// SPDX-License-Identifier: PMPL-1.0-or-later

/// RuntimeBridge — Unified IPC bridge for intsoc-transactor.
///
/// Detects the available runtime (Gossamer, Tauri, or browser-only) and
/// dispatches `invoke` calls to the appropriate backend. This allows all
/// command modules to use a single import instead of binding directly
/// to `@tauri-apps/api/core`.
///
/// Priority order:
///   1. Gossamer (`window.__gossamer_invoke`)  — own stack, preferred
///   2. Tauri    (`window.__TAURI_INTERNALS__`) — legacy, transition
///   3. Browser  (direct HTTP fetch)            — development fallback
///
/// Migration path: Tauri.res replaces
///   `@module("@tauri-apps/api/core") external invoke: ...`
/// with
///   `let invoke = RuntimeBridge.invoke`

// ---------------------------------------------------------------------------
// Raw external bindings — exactly one of these will be available at runtime
// ---------------------------------------------------------------------------

/// Gossamer IPC: injected by gossamer_channel_open() into the webview.
%%raw(`
function isGossamerRuntime() {
  return typeof window !== 'undefined'
    && typeof window.__gossamer_invoke === 'function';
}
`)
@val external isGossamerRuntime: unit => bool = "isGossamerRuntime"

%%raw(`
function gossamerInvoke(cmd, args) {
  return window.__gossamer_invoke(cmd, args);
}
`)
@val external gossamerInvoke: (string, 'a) => promise<'b> = "gossamerInvoke"

/// Tauri IPC: injected by the Tauri runtime into the webview.
%%raw(`
function isTauriRuntime() {
  return typeof window !== 'undefined'
    && window.__TAURI_INTERNALS__ != null
    && !window.__TAURI_INTERNALS__.__BROWSER_SHIM__;
}
`)
@val external isTauriRuntime: unit => bool = "isTauriRuntime"

@module("@tauri-apps/api/core")
external tauriInvoke: (string, 'a) => promise<'b> = "invoke"

// ---------------------------------------------------------------------------
// Unified invoke — detects runtime and dispatches
// ---------------------------------------------------------------------------

/// The runtime currently in use. Cached after first detection for performance.
type runtime =
  | Gossamer
  | Tauri
  | BrowserOnly

%%raw(`
var _detectedRuntime = null;
function detectRuntime() {
  if (_detectedRuntime !== null) return _detectedRuntime;
  if (typeof window !== 'undefined' && typeof window.__gossamer_invoke === 'function') {
    _detectedRuntime = 'gossamer';
  } else if (typeof window !== 'undefined' && window.__TAURI_INTERNALS__ != null && !window.__TAURI_INTERNALS__.__BROWSER_SHIM__) {
    _detectedRuntime = 'tauri';
  } else {
    _detectedRuntime = 'browser';
  }
  return _detectedRuntime;
}
`)
@val external detectRuntimeRaw: unit => string = "detectRuntime"

/// Detect and return the current runtime.
let detectRuntime = (): runtime => {
  switch detectRuntimeRaw() {
  | "gossamer" => Gossamer
  | "tauri" => Tauri
  | _ => BrowserOnly
  }
}

/// Invoke a backend command through whatever runtime is available.
///
/// - On Gossamer: calls `window.__gossamer_invoke(cmd, args)`
/// - On Tauri:    calls `window.__TAURI_INTERNALS__.invoke(cmd, args)`
/// - On browser:  rejects with a descriptive error
///
/// This is the primary function all command modules should use.
let invoke = (cmd: string, args: 'a): promise<'b> => {
  if isGossamerRuntime() {
    gossamerInvoke(cmd, args)
  } else if isTauriRuntime() {
    tauriInvoke(cmd, args)
  } else {
    Promise.reject(
      JsError.throwWithMessage(
        `No desktop runtime — "${cmd}" requires Gossamer or Tauri`,
      ),
    )
  }
}

/// Check whether any desktop runtime is available.
let hasDesktopRuntime = (): bool => {
  isGossamerRuntime() || isTauriRuntime()
}

/// Get a human-readable name for the current runtime.
let runtimeName = (): string => {
  switch detectRuntime() {
  | Gossamer => "Gossamer"
  | Tauri => "Tauri"
  | BrowserOnly => "Browser"
  }
}

// ---------------------------------------------------------------------------
// Dialog abstraction — Gossamer dialogs vs Tauri plugin-dialog
// ---------------------------------------------------------------------------

module Dialog = {
  @module("@tauri-apps/plugin-dialog")
  external tauriOpenRaw: JSON.t => promise<Nullable.t<JSON.t>> = "open"

  @module("@tauri-apps/plugin-dialog")
  external tauriSaveRaw: JSON.t => promise<Nullable.t<JSON.t>> = "save"

  /// Open a file picker dialog.
  let open = (opts: JSON.t): promise<Nullable.t<JSON.t>> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_dialog_open", opts)
    } else if isTauriRuntime() {
      tauriOpenRaw(opts)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — file dialogs require Gossamer or Tauri",
        ),
      )
    }
  }

  /// Open a save dialog.
  let save = (opts: JSON.t): promise<Nullable.t<JSON.t>> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_dialog_save", opts)
    } else if isTauriRuntime() {
      tauriSaveRaw(opts)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — save dialogs require Gossamer or Tauri",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Filesystem abstraction — Gossamer fs vs Tauri plugin-fs
// ---------------------------------------------------------------------------

module Fs = {
  @module("@tauri-apps/plugin-fs")
  external tauriReadTextFileRaw: string => promise<string> = "readTextFile"

  @module("@tauri-apps/plugin-fs")
  external tauriWriteTextFileRaw: (string, string) => promise<unit> = "writeTextFile"

  /// Read a text file from the local filesystem.
  let readTextFile = (path: string): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_fs_read_text", {"path": path})
    } else if isTauriRuntime() {
      tauriReadTextFileRaw(path)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — filesystem access requires Gossamer or Tauri",
        ),
      )
    }
  }

  /// Write a text file to the local filesystem.
  let writeTextFile = (path: string, contents: string): promise<unit> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_fs_write_text", {"path": path, "contents": contents})
    } else if isTauriRuntime() {
      tauriWriteTextFileRaw(path, contents)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — filesystem access requires Gossamer or Tauri",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Shell abstraction — Gossamer shell vs Tauri plugin-shell
// ---------------------------------------------------------------------------

module Shell = {
  type command
  type childProcess = {
    code: int,
    stdout: string,
    stderr: string,
  }

  @module("@tauri-apps/plugin-shell")
  external tauriCommand: (string, array<string>) => command = "Command"

  @send
  external tauriExecute: command => promise<childProcess> = "execute"

  /// Execute a shell command through whatever runtime is available.
  /// On Gossamer, routes through IPC. On Tauri, uses plugin-shell.
  let execute = (program: string, args: array<string>): promise<childProcess> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_shell_execute", {"program": program, "args": args})
    } else if isTauriRuntime() {
      let cmd = tauriCommand(program, args)
      tauriExecute(cmd)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — shell execution requires Gossamer or Tauri",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Path abstraction — Gossamer paths vs Tauri api/path
// ---------------------------------------------------------------------------

module Path = {
  @module("@tauri-apps/api/path")
  external tauriAppDataDir: unit => promise<string> = "appDataDir"

  @module("@tauri-apps/api/path")
  external tauriAppConfigDir: unit => promise<string> = "appConfigDir"

  @module("@tauri-apps/api/path")
  external tauriHomeDir: unit => promise<string> = "homeDir"

  @module("@tauri-apps/api/path")
  external tauriDesktopDir: unit => promise<string> = "desktopDir"

  @module("@tauri-apps/api/path")
  external tauriDocumentDir: unit => promise<string> = "documentDir"

  /// Resolve the app data directory.
  let appDataDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "appData"})
    } else if isTauriRuntime() {
      tauriAppDataDir()
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer or Tauri"),
      )
    }
  }

  /// Resolve the app config directory.
  let appConfigDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "appConfig"})
    } else if isTauriRuntime() {
      tauriAppConfigDir()
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer or Tauri"),
      )
    }
  }

  /// Resolve the home directory.
  let homeDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "home"})
    } else if isTauriRuntime() {
      tauriHomeDir()
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer or Tauri"),
      )
    }
  }

  /// Resolve the desktop directory.
  let desktopDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "desktop"})
    } else if isTauriRuntime() {
      tauriDesktopDir()
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer or Tauri"),
      )
    }
  }

  /// Resolve the documents directory.
  let documentDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "document"})
    } else if isTauriRuntime() {
      tauriDocumentDir()
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer or Tauri"),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Event abstraction — Gossamer events vs Tauri api/event
// ---------------------------------------------------------------------------

module Event = {
  /// Event payload wrapper
  type eventPayload<'a> = {payload: 'a}

  /// Unlisten handle
  type unlisten = unit => unit

  @module("@tauri-apps/api/event")
  external tauriListen: (string, eventPayload<'a> => unit) => promise<unlisten> = "listen"

  @module("@tauri-apps/api/event")
  external tauriEmit: (string, 'payload) => promise<unit> = "emit"

  /// Listen for events from the backend.
  let listen = (event: string, handler: eventPayload<'a> => unit): promise<unlisten> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_event_listen", {"event": event})
    } else if isTauriRuntime() {
      tauriListen(event, handler)
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — events require Gossamer or Tauri"),
      )
    }
  }

  /// Emit an event to the backend.
  let emit = (event: string, payload: 'a): promise<unit> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_event_emit", {"event": event, "payload": payload})
    } else if isTauriRuntime() {
      tauriEmit(event, payload)
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — events require Gossamer or Tauri"),
      )
    }
  }
}
