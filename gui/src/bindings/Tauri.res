// SPDX-License-Identifier: PMPL-1.0-or-later

/// Backend IPC Bindings for intsoc-transactor
///
/// ReScript bindings to the backend APIs via RuntimeBridge for:
/// - Core invoke/event system (commands, listeners, emitters)
/// - Window management (title, minimize, maximize, close)
/// - Shell plugin (execute external tools like idnits)
/// - Path plugin (app data, config, home directories)
/// - Dialog plugin (file open/save dialogs)
/// - Filesystem plugin (read/write document files)
///
/// Custom backend commands for the intsoc-transactor:
/// - check_document: validate an Internet-Draft
/// - fix_document: generate and apply fixes
/// - get_submission_status: query Datatracker submission state
///
/// Uses RuntimeBridge for Gossamer dispatch (Tauri support removed).

/// Generic backend invoke result (promise-based)
type invokeResult<'a> = promise<'a>

// ---------------------------------------------------------------------------
// Core: invoke, listen, emit — via RuntimeBridge
// ---------------------------------------------------------------------------

/// Invoke a backend command via Gossamer
let invoke = RuntimeBridge.invoke

/// Event payload wrapper from the backend event system
type eventPayload<'a> = {payload: 'a}

/// Unlisten handle returned by listen()
type unlisten = unit => unit

/// Listen for events emitted by the backend
let listen = RuntimeBridge.Event.listen

/// Emit an event to the backend
let emit = RuntimeBridge.Event.emit

// ---------------------------------------------------------------------------
// Window module — via RuntimeBridge (Gossamer IPC)
// ---------------------------------------------------------------------------

/// Window management operations via Gossamer backend
module Window = {
  type windowLabel = string

  /// Get the current window label from the Gossamer runtime.
  let getCurrent = (): {"label": windowLabel} => {
    {"label": "main"}
  }

  /// Set the window title via Gossamer IPC.
  let setTitle = (title: string): invokeResult<unit> => {
    invoke("__gossamer_window_set_title", {"title": title})
  }

  /// Set fullscreen mode via Gossamer IPC.
  let setFullscreen = (fullscreen: bool): invokeResult<unit> => {
    invoke("__gossamer_window_set_fullscreen", {"fullscreen": fullscreen})
  }

  /// Minimize the window via Gossamer IPC.
  let minimize = (): invokeResult<unit> => {
    invoke("__gossamer_window_minimize", {})
  }

  /// Maximize the window via Gossamer IPC.
  let maximize = (): invokeResult<unit> => {
    invoke("__gossamer_window_maximize", {})
  }

  /// Close the window via Gossamer IPC.
  let close = (): invokeResult<unit> => {
    invoke("__gossamer_window_close", {})
  }
}

// ---------------------------------------------------------------------------
// Shell module — via RuntimeBridge.Shell
// ---------------------------------------------------------------------------

/// Shell operations for running external tools (idnits, xml2rfc, etc.)
module Shell = {
  type childProcess = RuntimeBridge.Shell.childProcess

  /// Run idnits on a document file and return the output
  let runIdnits = (filePath: string): invokeResult<childProcess> => {
    RuntimeBridge.Shell.execute("idnits", [filePath])
  }

  /// Run xml2rfc to convert XML to text output
  let runXml2rfc = (filePath: string, outputPath: string): invokeResult<childProcess> => {
    RuntimeBridge.Shell.execute("xml2rfc", ["--text", "--out=" ++ outputPath, filePath])
  }
}

// ---------------------------------------------------------------------------
// Path module — via RuntimeBridge.Path
// ---------------------------------------------------------------------------

/// Filesystem path resolution (platform-aware)
module Path = {
  let appDataDir = RuntimeBridge.Path.appDataDir
  let appConfigDir = RuntimeBridge.Path.appConfigDir
  let homeDir = RuntimeBridge.Path.homeDir
  let desktopDir = RuntimeBridge.Path.desktopDir
  let documentDir = RuntimeBridge.Path.documentDir
}

// ---------------------------------------------------------------------------
// Dialog module — via RuntimeBridge.Dialog
// ---------------------------------------------------------------------------

/// Native file dialogs for opening and saving documents
module Dialog = {
  type fileFilter = {
    name: string,
    extensions: array<string>,
  }

  type openDialogOptions = {
    multiple: bool,
    directory: bool,
    filters: array<fileFilter>,
    title: string,
  }

  type saveDialogOptions = {
    filters: array<fileFilter>,
    title: string,
    defaultPath: option<string>,
  }

  /// Open a file dialog via Gossamer IPC.
  let open_ = (opts: openDialogOptions): invokeResult<Nullable.t<string>> => {
    RuntimeBridge.Dialog.open(JSON.Encode.object(Dict.fromArray([
      ("multiple", JSON.Encode.bool(opts.multiple)),
      ("directory", JSON.Encode.bool(opts.directory)),
      ("title", JSON.Encode.string(opts.title)),
    ])))
  }

  /// Save file dialog via Gossamer IPC.
  let save = (opts: saveDialogOptions): invokeResult<Nullable.t<string>> => {
    RuntimeBridge.Dialog.save(JSON.Encode.object(Dict.fromArray([
      ("title", JSON.Encode.string(opts.title)),
    ])))
  }

  /// Pre-configured filter for Internet-Draft files
  let draftFileFilters: array<fileFilter> = [
    {name: "RFC XML v3", extensions: ["xml"]},
    {name: "Plain Text", extensions: ["txt"]},
    {name: "All Files", extensions: ["*"]},
  ]
}

// ---------------------------------------------------------------------------
// Filesystem module — via RuntimeBridge.Fs
// ---------------------------------------------------------------------------

/// Filesystem read/write operations for document files
module Fs = {
  let readTextFile = RuntimeBridge.Fs.readTextFile
  let writeTextFile = RuntimeBridge.Fs.writeTextFile
}

// ---------------------------------------------------------------------------
// Custom intsoc-transactor backend commands
// ---------------------------------------------------------------------------

/// Severity level for check results (mirrors intsoc_core::validation::Severity)
type severity =
  | @as("Info") Info
  | @as("Warning") Warning
  | @as("Error") Error
  | @as("Fatal") Fatal

/// Fixability classification (mirrors intsoc_core::validation::Fixability)
type fixability =
  | @as("AutoSafe") AutoSafe
  | @as("Recommended") Recommended
  | @as("ManualOnly") ManualOnly
  | @as("NotFixable") NotFixable

/// Check category (mirrors intsoc_core::validation::CheckCategory)
type checkCategory =
  | @as("Boilerplate") Boilerplate
  | @as("Date") Date
  | @as("Header") Header
  | @as("References") References
  | @as("Sections") Sections
  | @as("TextFormat") TextFormat
  | @as("Xml") Xml
  | @as("IanaSections") IanaSections
  | @as("DraftName") DraftName
  | @as("Ipr") Ipr

/// A single check result from the backend
type checkResult = {
  check_id: string,
  severity: severity,
  message: string,
  location: Nullable.t<string>,
  category: checkCategory,
  fixable: fixability,
  suggestion: Nullable.t<string>,
}

/// Summary of all check results
type checkSummary = {
  results: array<checkResult>,
  error_count: int,
  warning_count: int,
  info_count: int,
  auto_fixable_count: int,
  recommended_fixable_count: int,
  manual_only_count: int,
}

/// Fix result from the backend
type fixResult = {
  success: bool,
  fixed_source: string,
  diff_preview: string,
  auto_safe_applied: int,
  recommended_applied: int,
  manual_remaining: int,
}

/// Submission status from the Datatracker or other endpoints
type submissionStatus = {
  document_name: string,
  stream: string,
  state: string,
  submitted: bool,
  datatracker_url: Nullable.t<string>,
  message: string,
}

/// Check a document for issues via the backend.
/// Invokes the check_document command.
let checkDocument = (source: string, streamHint: option<string>): invokeResult<checkSummary> => {
  invoke("check_document", {"source": source, "stream_hint": streamHint})
}

/// Fix a document via the backend.
/// Invokes the fix_document command with the given fix level.
let fixDocument = (
  source: string,
  autoOnly: bool,
  dryRun: bool,
): invokeResult<fixResult> => {
  invoke("fix_document", {"source": source, "auto_only": autoOnly, "dry_run": dryRun})
}

/// Get the submission status for a document.
/// Queries the IETF Datatracker or appropriate endpoint.
let getSubmissionStatus = (documentName: string): invokeResult<submissionStatus> => {
  invoke("get_submission_status", {"document_name": documentName})
}
