// SPDX-License-Identifier: PMPL-1.0-or-later

/// Tauri FFI Bindings for intsoc-transactor
///
/// ReScript bindings to the Tauri 2.0 APIs for:
/// - Core invoke/event system (commands, listeners, emitters)
/// - Window management (title, minimize, maximize, close)
/// - Shell plugin (execute external tools like idnits)
/// - Path plugin (app data, config, home directories)
/// - Dialog plugin (file open/save dialogs)
/// - Filesystem plugin (read/write document files)
///
/// Custom Tauri commands for the intsoc-transactor backend:
/// - check_document: validate an Internet-Draft
/// - fix_document: generate and apply fixes
/// - get_submission_status: query Datatracker submission state

/// Generic Tauri invoke result (promise-based)
type invokeResult<'a> = promise<'a>

// ---------------------------------------------------------------------------
// Core: invoke, listen, emit
// ---------------------------------------------------------------------------

/// Invoke a Tauri command on the Rust backend
@module("@tauri-apps/api/core")
external invoke: (string, 'params) => invokeResult<'result> = "invoke"

/// Event payload wrapper from Tauri event system
type eventPayload<'a> = {payload: 'a}

/// Unlisten handle returned by listen()
type unlisten = unit => unit

/// Listen for events emitted by the Tauri backend
@module("@tauri-apps/api/event")
external listen: (string, eventPayload<'a> => unit) => invokeResult<unlisten> = "listen"

/// Emit an event to the Tauri backend
@module("@tauri-apps/api/event")
external emit: (string, 'payload) => invokeResult<unit> = "emit"

// ---------------------------------------------------------------------------
// Window module
// ---------------------------------------------------------------------------

/// Window management operations (Tauri 2.0 window API)
module Window = {
  type windowLabel = string

  @module("@tauri-apps/api/window")
  external getCurrent: unit => {"label": windowLabel} = "getCurrent"

  @module("@tauri-apps/api/window")
  external setTitle: string => invokeResult<unit> = "setTitle"

  @module("@tauri-apps/api/window")
  external setFullscreen: bool => invokeResult<unit> = "setFullscreen"

  @module("@tauri-apps/api/window")
  external minimize: unit => invokeResult<unit> = "minimize"

  @module("@tauri-apps/api/window")
  external maximize: unit => invokeResult<unit> = "maximize"

  @module("@tauri-apps/api/window")
  external close: unit => invokeResult<unit> = "close"
}

// ---------------------------------------------------------------------------
// Shell module (tauri-plugin-shell)
// ---------------------------------------------------------------------------

/// Shell operations for running external tools (idnits, xml2rfc, etc.)
module Shell = {
  type command
  type childProcess = {
    code: int,
    stdout: string,
    stderr: string,
  }

  @module("@tauri-apps/plugin-shell")
  external command: (string, array<string>) => command = "Command"

  @send
  external execute: command => invokeResult<childProcess> = "execute"

  /// Run idnits on a document file and return the output
  let runIdnits = (filePath: string): invokeResult<childProcess> => {
    let cmd = command("idnits", [filePath])
    execute(cmd)
  }

  /// Run xml2rfc to convert XML to text output
  let runXml2rfc = (filePath: string, outputPath: string): invokeResult<childProcess> => {
    let cmd = command("xml2rfc", ["--text", "--out=" ++ outputPath, filePath])
    execute(cmd)
  }
}

// ---------------------------------------------------------------------------
// Path module
// ---------------------------------------------------------------------------

/// Filesystem path resolution (platform-aware)
module Path = {
  @module("@tauri-apps/api/path")
  external appDataDir: unit => invokeResult<string> = "appDataDir"

  @module("@tauri-apps/api/path")
  external appConfigDir: unit => invokeResult<string> = "appConfigDir"

  @module("@tauri-apps/api/path")
  external homeDir: unit => invokeResult<string> = "homeDir"

  @module("@tauri-apps/api/path")
  external desktopDir: unit => invokeResult<string> = "desktopDir"

  @module("@tauri-apps/api/path")
  external documentDir: unit => invokeResult<string> = "documentDir"
}

// ---------------------------------------------------------------------------
// Dialog module (tauri-plugin-dialog)
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

  @module("@tauri-apps/plugin-dialog")
  external open_: openDialogOptions => invokeResult<Nullable.t<string>> = "open"

  @module("@tauri-apps/plugin-dialog")
  external save: saveDialogOptions => invokeResult<Nullable.t<string>> = "save"

  /// Pre-configured filter for Internet-Draft files
  let draftFileFilters: array<fileFilter> = [
    {name: "RFC XML v3", extensions: ["xml"]},
    {name: "Plain Text", extensions: ["txt"]},
    {name: "All Files", extensions: ["*"]},
  ]
}

// ---------------------------------------------------------------------------
// Filesystem module (tauri-plugin-fs)
// ---------------------------------------------------------------------------

/// Filesystem read/write operations for document files
module Fs = {
  @module("@tauri-apps/plugin-fs")
  external readTextFile: string => invokeResult<string> = "readTextFile"

  @module("@tauri-apps/plugin-fs")
  external writeTextFile: (string, string) => invokeResult<unit> = "writeTextFile"
}

// ---------------------------------------------------------------------------
// Custom intsoc-transactor Tauri commands
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

/// Check a document for issues via the Tauri backend.
/// Invokes the check_document Rust command.
let checkDocument = (source: string, streamHint: option<string>): invokeResult<checkSummary> => {
  invoke("check_document", {"source": source, "stream_hint": streamHint})
}

/// Fix a document via the Tauri backend.
/// Invokes the fix_document Rust command with the given fix level.
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
