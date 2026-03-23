// SPDX-License-Identifier: PMPL-1.0-or-later

/// RuntimeBridge — Unified IPC bridge for intsoc-transactor.
///
/// Dispatches `invoke` calls to the Gossamer backend via
/// `window.__gossamer_invoke`. Falls back to a descriptive error
/// in browser-only mode (e.g., during development without Gossamer).
///
/// Priority order:
///   1. Gossamer (`window.__gossamer_invoke`) — primary runtime
///   2. Browser  (descriptive error)          — development fallback
///
/// MIGRATION NOTE: Tauri support has been removed. All IPC now routes
/// through Gossamer exclusively. The Tauri `@module` externals and
/// `isTauriRuntime` checks have been deleted.

// ---------------------------------------------------------------------------
// Raw external bindings — Gossamer IPC injected by the Zig runtime
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

// ---------------------------------------------------------------------------
// Runtime detection
// ---------------------------------------------------------------------------

/// The runtime currently in use.
type runtime =
  | Gossamer
  | BrowserOnly

/// Detect the current runtime.
let detectRuntime = (): runtime => {
  if isGossamerRuntime() {
    Gossamer
  } else {
    BrowserOnly
  }
}

// ---------------------------------------------------------------------------
// Unified invoke — Gossamer IPC or descriptive error
// ---------------------------------------------------------------------------

/// Invoke a backend command through Gossamer.
///
/// - On Gossamer: calls `window.__gossamer_invoke(cmd, args)`
/// - On browser:  rejects with a descriptive error
///
/// This is the primary function all command modules should use.
let invoke = (cmd: string, args: 'a): promise<'b> => {
  if isGossamerRuntime() {
    gossamerInvoke(cmd, args)
  } else {
    Promise.reject(
      JsError.throwWithMessage(
        `No desktop runtime — "${cmd}" requires Gossamer`,
      ),
    )
  }
}

/// Check whether the Gossamer runtime is available.
let hasDesktopRuntime = (): bool => {
  isGossamerRuntime()
}

/// Get a human-readable name for the current runtime.
let runtimeName = (): string => {
  switch detectRuntime() {
  | Gossamer => "Gossamer"
  | BrowserOnly => "Browser"
  }
}

// ---------------------------------------------------------------------------
// Dialog abstraction — Gossamer dialogs
// ---------------------------------------------------------------------------

module Dialog = {
  /// Open a file picker dialog via Gossamer IPC.
  let open = (opts: JSON.t): promise<Nullable.t<JSON.t>> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_dialog_open", opts)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — file dialogs require Gossamer",
        ),
      )
    }
  }

  /// Open a save dialog via Gossamer IPC.
  let save = (opts: JSON.t): promise<Nullable.t<JSON.t>> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_dialog_save", opts)
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — save dialogs require Gossamer",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Filesystem abstraction — Gossamer fs
// ---------------------------------------------------------------------------

module Fs = {
  /// Read a text file from the local filesystem via Gossamer IPC.
  let readTextFile = (path: string): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_fs_read_text", {"path": path})
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — filesystem access requires Gossamer",
        ),
      )
    }
  }

  /// Write a text file to the local filesystem via Gossamer IPC.
  let writeTextFile = (path: string, contents: string): promise<unit> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_fs_write_text", {"path": path, "contents": contents})
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — filesystem access requires Gossamer",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Shell abstraction — Gossamer shell
// ---------------------------------------------------------------------------

module Shell = {
  type childProcess = {
    code: int,
    stdout: string,
    stderr: string,
  }

  /// Execute a shell command via Gossamer IPC.
  let execute = (program: string, args: array<string>): promise<childProcess> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_shell_execute", {"program": program, "args": args})
    } else {
      Promise.reject(
        JsError.throwWithMessage(
          "No desktop runtime — shell execution requires Gossamer",
        ),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Path abstraction — Gossamer paths
// ---------------------------------------------------------------------------

module Path = {
  /// Resolve the app data directory via Gossamer IPC.
  let appDataDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "appData"})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer"),
      )
    }
  }

  /// Resolve the app config directory via Gossamer IPC.
  let appConfigDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "appConfig"})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer"),
      )
    }
  }

  /// Resolve the home directory via Gossamer IPC.
  let homeDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "home"})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer"),
      )
    }
  }

  /// Resolve the desktop directory via Gossamer IPC.
  let desktopDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "desktop"})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer"),
      )
    }
  }

  /// Resolve the documents directory via Gossamer IPC.
  let documentDir = (): promise<string> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_path_resolve", {"kind": "document"})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — path resolution requires Gossamer"),
      )
    }
  }
}

// ---------------------------------------------------------------------------
// Event abstraction — Gossamer events
// ---------------------------------------------------------------------------

module Event = {
  /// Event payload wrapper
  type eventPayload<'a> = {payload: 'a}

  /// Unlisten handle
  type unlisten = unit => unit

  /// Listen for events from the Gossamer backend.
  let listen = (event: string, _handler: eventPayload<'a> => unit): promise<unlisten> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_event_listen", {"event": event})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — events require Gossamer"),
      )
    }
  }

  /// Emit an event to the Gossamer backend.
  let emit = (event: string, payload: 'a): promise<unit> => {
    if isGossamerRuntime() {
      gossamerInvoke("__gossamer_event_emit", {"event": event, "payload": payload})
    } else {
      Promise.reject(
        JsError.throwWithMessage("No desktop runtime — events require Gossamer"),
      )
    }
  }
}
