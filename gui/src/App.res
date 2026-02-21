// SPDX-License-Identifier: PMPL-1.0-or-later

/**
 * Intsoc-Transactor GUI — Main Application Kernel (ReScript).
 *
 * This module implements the "The Elm Architecture" (TEA) orchestrator for 
 * the desktop interface. It manages the global state of the transactor, 
 * coordinating between the WebView and the high-assurance Rust backend.
 *
 * WORKFLOW PANELS:
 * 1. **Editor**: Document authoring and filesystem I/O.
 * 2. **Checker**: Deterministic validation against IETF/IANA standards.
 * 3. **Fixer**: Automated remediation of identified issues.
 * 4. **Submitter**: Interaction with the authoritative Datatracker API.
 */

open Tea_Html

// MODEL: The Single Source of Truth for the entire GUI.
type model = {
  currentView: currentView,
  documentSource: string, // The raw text/XML content.
  filePath: option<string>,
  checkSummary: option<Tauri.checkSummary>,
  checkState: loadingState,
  fixResult: option<Tauri.fixResult>,
  fixState: loadingState,
}

/**
 * UPDATE: The deterministic state transition function.
 *
 * ASYNCHRONOUS PATTERN: 
 * - User triggers an action (e.g., `RunCheck`).
 * - Update function returns a new model (`checkState: Loading`) and 
 *   a Command (`Tea_Cmd.call`) to invoke the Rust backend via Tauri.
 * - When the Rust backend returns, it enqueues a completion message 
 *   (`CheckCompleted`), which the Update function then processes.
 */
let update = (model: model, msg: msg): (model, Tea_Cmd.t<msg>) => {
  switch msg {
  | SwitchView(view) => ({...model, currentView: view}, Tea_Cmd.none)
  
  | RunCheck =>
    // COMMAND: Offload the complex parsing/validation logic to Rust.
    let cmd = Tea_Cmd.call(callbacks => {
      Tauri.checkDocument(model.documentSource, None)
      ->Promise.then(summary => {
        callbacks.enqueue(CheckCompleted(summary))
        Promise.resolve()
      })
    })
    ({...model, checkState: Loading}, cmd)

  | CheckCompleted(summary) => ({...model, checkSummary: Some(summary), checkState: Complete}, Tea_Cmd.none)
  // ... [Other message handlers]
  }
}
