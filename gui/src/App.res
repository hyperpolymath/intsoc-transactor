// SPDX-License-Identifier: PMPL-1.0-or-later

/// Internet Society Transactor - Main Application Module
///
/// TEA (The Elm Architecture) application for the intsoc-transactor desktop GUI.
/// Provides a four-panel workflow for Internet-Draft lifecycle management:
///   1. Editor   - Load and view document source
///   2. Checker  - Validate documents against IETF/IRTF/IAB/ISE/IANA rules
///   3. Fixer    - Generate and apply fixes (AutoSafe/Recommended/ManualOnly)
///   4. Submitter - Submit to Datatracker or appropriate endpoint

open Tea_Html

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// The four main views of the application workflow
type currentView =
  | Editor
  | Checker
  | Fixer
  | Submitter

/// Internet Society submission stream selection
type streamSelection =
  | IetfIndividual
  | IetfWorkingGroup(string)
  | IrtfResearchGroup(string)
  | IrtfIndividual
  | IabDocument
  | IndependentSubmission
  | IanaRegistry(string)

/// Loading state for async operations
type loadingState =
  | Idle
  | Loading
  | Complete
  | Failed(string)

/// Application model - the single source of truth
type model = {
  /// Current active view/tab
  currentView: currentView,
  /// Raw document source text
  documentSource: string,
  /// File path of the loaded document (if from disk)
  filePath: option<string>,
  /// Document name parsed from source
  documentName: string,
  /// Selected submission stream
  streamSelection: streamSelection,
  /// Working group or research group name (for stream types that need it)
  groupName: string,
  /// Check results from validation
  checkSummary: option<Tauri.checkSummary>,
  /// Whether a check is in progress
  checkState: loadingState,
  /// Fix result from the fixer
  fixResult: option<Tauri.fixResult>,
  /// Whether a fix is in progress
  fixState: loadingState,
  /// Whether to apply only AutoSafe fixes
  fixAutoOnly: bool,
  /// Whether to preview fixes without applying
  fixDryRun: bool,
  /// Submission status
  submissionStatus: option<Tauri.submissionStatus>,
  /// Whether a submission query is in progress
  submitState: loadingState,
  /// Status bar message
  statusMessage: string,
}

// ---------------------------------------------------------------------------
// Msg
// ---------------------------------------------------------------------------

/// All possible user and system messages
type msg =
  // Navigation
  | SwitchView(currentView)
  // Editor actions
  | UpdateSource(string)
  | OpenFile
  | FileOpened(string, string) // (path, content)
  | SaveFile
  | FileSaved
  // Stream selection
  | SetStream(streamSelection)
  | SetGroupName(string)
  // Checker actions
  | RunCheck
  | CheckCompleted(Tauri.checkSummary)
  | CheckFailed(string)
  // Fixer actions
  | RunFix
  | ToggleFixAutoOnly
  | ToggleFixDryRun
  | FixCompleted(Tauri.fixResult)
  | FixFailed(string)
  | ApplyFixedSource(string)
  // Submitter actions
  | QueryStatus
  | StatusReceived(Tauri.submissionStatus)
  | StatusFailed(string)
  // General
  | SetStatusMessage(string)
  | NoOp

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

/// Initial state: empty editor, IETF Individual stream selected
let init = (): (model, Tea_Cmd.t<msg>) => {
  let model = {
    currentView: Editor,
    documentSource: "",
    filePath: None,
    documentName: "",
    streamSelection: IetfIndividual,
    groupName: "",
    checkSummary: None,
    checkState: Idle,
    fixResult: None,
    fixState: Idle,
    fixAutoOnly: true,
    fixDryRun: true,
    submissionStatus: None,
    submitState: Idle,
    statusMessage: "Ready. Open a document or paste source to begin.",
  }
  (model, Tea_Cmd.none)
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

/// Convert stream selection to a hint string for the backend
let streamToHint = (stream: streamSelection): option<string> => {
  switch stream {
  | IetfIndividual => Some("ietf-individual")
  | IetfWorkingGroup(wg) => Some("ietf-wg:" ++ wg)
  | IrtfResearchGroup(rg) => Some("irtf-rg:" ++ rg)
  | IrtfIndividual => Some("irtf-individual")
  | IabDocument => Some("iab")
  | IndependentSubmission => Some("independent")
  | IanaRegistry(reg) => Some("iana:" ++ reg)
  }
}

/// TEA update function: process a message and return the next model + commands
let update = (model: model, msg: msg): (model, Tea_Cmd.t<msg>) => {
  switch msg {
  // -- Navigation --
  | SwitchView(view) => ({...model, currentView: view}, Tea_Cmd.none)

  // -- Editor --
  | UpdateSource(source) => ({...model, documentSource: source}, Tea_Cmd.none)

  | OpenFile =>
    let cmd = Tea_Cmd.call(callbacks => {
      let _ = Tauri.Dialog.open_({
        multiple: false,
        directory: false,
        filters: Tauri.Dialog.draftFileFilters,
        title: "Open Internet-Draft",
      })->Promise.then(result => {
        switch Nullable.toOption(result) {
        | Some(path) =>
          Tauri.Fs.readTextFile(path)->Promise.then(content => {
            callbacks.enqueue(FileOpened(path, content))
            Promise.resolve()
          })
        | None => Promise.resolve()
        }
      })->Promise.catch(_err => {
        callbacks.enqueue(SetStatusMessage("Failed to open file"))
        Promise.resolve()
      })
    })
    ({...model, statusMessage: "Opening file..."}, cmd)

  | FileOpened(path, content) => (
      {
        ...model,
        documentSource: content,
        filePath: Some(path),
        statusMessage: "Loaded: " ++ path,
        // Reset downstream state when a new file is loaded
        checkSummary: None,
        checkState: Idle,
        fixResult: None,
        fixState: Idle,
        submissionStatus: None,
        submitState: Idle,
      },
      Tea_Cmd.none,
    )

  | SaveFile =>
    switch model.filePath {
    | Some(path) =>
      let cmd = Tea_Cmd.call(callbacks => {
        let _ = Tauri.Fs.writeTextFile(path, model.documentSource)->Promise.then(_ => {
          callbacks.enqueue(FileSaved)
          Promise.resolve()
        })->Promise.catch(_err => {
          callbacks.enqueue(SetStatusMessage("Failed to save file"))
          Promise.resolve()
        })
      })
      ({...model, statusMessage: "Saving..."}, cmd)
    | None =>
      let cmd = Tea_Cmd.call(callbacks => {
        let _ = Tauri.Dialog.save({
          filters: Tauri.Dialog.draftFileFilters,
          title: "Save Internet-Draft",
          defaultPath: None,
        })->Promise.then(result => {
          switch Nullable.toOption(result) {
          | Some(path) =>
            Tauri.Fs.writeTextFile(path, model.documentSource)->Promise.then(_ => {
              callbacks.enqueue(FileOpened(path, model.documentSource))
              callbacks.enqueue(FileSaved)
              Promise.resolve()
            })
          | None => Promise.resolve()
          }
        })->Promise.catch(_err => {
          callbacks.enqueue(SetStatusMessage("Failed to save file"))
          Promise.resolve()
        })
      })
      ({...model, statusMessage: "Save as..."}, cmd)
    }

  | FileSaved => ({...model, statusMessage: "File saved."}, Tea_Cmd.none)

  // -- Stream selection --
  | SetStream(stream) => ({...model, streamSelection: stream}, Tea_Cmd.none)
  | SetGroupName(name) => ({...model, groupName: name}, Tea_Cmd.none)

  // -- Checker --
  | RunCheck =>
    if model.documentSource === "" {
      ({...model, statusMessage: "No document source to check."}, Tea_Cmd.none)
    } else {
      let cmd = Tea_Cmd.call(callbacks => {
        let _ = Tauri.checkDocument(
          model.documentSource,
          streamToHint(model.streamSelection),
        )->Promise.then(summary => {
          callbacks.enqueue(CheckCompleted(summary))
          Promise.resolve()
        })->Promise.catch(err => {
          let errMsg: string = %raw(`String(err)`)
          callbacks.enqueue(CheckFailed(errMsg))
          Promise.resolve()
        })
      })
      ({...model, checkState: Loading, statusMessage: "Running checks..."}, cmd)
    }

  | CheckCompleted(summary) => (
      {
        ...model,
        checkSummary: Some(summary),
        checkState: Complete,
        statusMessage: `Check complete: ${Int.toString(summary.error_count)} error(s), ${Int.toString(summary.warning_count)} warning(s)`,
      },
      Tea_Cmd.none,
    )

  | CheckFailed(error) => (
      {
        ...model,
        checkState: Failed(error),
        statusMessage: "Check failed: " ++ error,
      },
      Tea_Cmd.none,
    )

  // -- Fixer --
  | RunFix =>
    if model.documentSource === "" {
      ({...model, statusMessage: "No document source to fix."}, Tea_Cmd.none)
    } else {
      let cmd = Tea_Cmd.call(callbacks => {
        let _ = Tauri.fixDocument(
          model.documentSource,
          model.fixAutoOnly,
          model.fixDryRun,
        )->Promise.then(result => {
          callbacks.enqueue(FixCompleted(result))
          Promise.resolve()
        })->Promise.catch(err => {
          let errMsg: string = %raw(`String(err)`)
          callbacks.enqueue(FixFailed(errMsg))
          Promise.resolve()
        })
      })
      ({...model, fixState: Loading, statusMessage: "Generating fixes..."}, cmd)
    }

  | ToggleFixAutoOnly => ({...model, fixAutoOnly: !model.fixAutoOnly}, Tea_Cmd.none)
  | ToggleFixDryRun => ({...model, fixDryRun: !model.fixDryRun}, Tea_Cmd.none)

  | FixCompleted(result) => (
      {
        ...model,
        fixResult: Some(result),
        fixState: Complete,
        statusMessage: `Fix complete: ${Int.toString(result.auto_safe_applied)} auto-safe, ${Int.toString(result.recommended_applied)} recommended, ${Int.toString(result.manual_remaining)} manual remaining`,
      },
      Tea_Cmd.none,
    )

  | FixFailed(error) => (
      {
        ...model,
        fixState: Failed(error),
        statusMessage: "Fix failed: " ++ error,
      },
      Tea_Cmd.none,
    )

  | ApplyFixedSource(source) => (
      {
        ...model,
        documentSource: source,
        statusMessage: "Fixed source applied to editor.",
        // Reset check/fix state since the source changed
        checkSummary: None,
        checkState: Idle,
        fixResult: None,
        fixState: Idle,
      },
      Tea_Cmd.none,
    )

  // -- Submitter --
  | QueryStatus =>
    if model.documentName === "" && model.documentSource === "" {
      ({...model, statusMessage: "No document loaded to query status for."}, Tea_Cmd.none)
    } else {
      let docName = if model.documentName !== "" {
        model.documentName
      } else {
        "unknown-draft"
      }
      let cmd = Tea_Cmd.call(callbacks => {
        let _ = Tauri.getSubmissionStatus(docName)->Promise.then(status => {
          callbacks.enqueue(StatusReceived(status))
          Promise.resolve()
        })->Promise.catch(err => {
          let errMsg: string = %raw(`String(err)`)
          callbacks.enqueue(StatusFailed(errMsg))
          Promise.resolve()
        })
      })
      ({...model, submitState: Loading, statusMessage: "Querying submission status..."}, cmd)
    }

  | StatusReceived(status) => (
      {
        ...model,
        submissionStatus: Some(status),
        submitState: Complete,
        statusMessage: "Status: " ++ status.state,
      },
      Tea_Cmd.none,
    )

  | StatusFailed(error) => (
      {
        ...model,
        submitState: Failed(error),
        statusMessage: "Status query failed: " ++ error,
      },
      Tea_Cmd.none,
    )

  // -- General --
  | SetStatusMessage(message) => ({...model, statusMessage: message}, Tea_Cmd.none)
  | NoOp => (model, Tea_Cmd.none)
  }
}

// ---------------------------------------------------------------------------
// View helpers
// ---------------------------------------------------------------------------

/// Render a navigation tab button
let viewTab = (label: string, targetView: currentView, activeView: currentView): Tea_Html.t<msg> => {
  let isActive = targetView === activeView
  let activeClass = isActive ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300"
  button(
    list{
      Attrs.class_("px-4 py-2 rounded-t-lg font-medium text-sm transition-colors " ++ activeClass),
      Events.onClick(SwitchView(targetView)),
      Attrs.ariaPressed(isActive),
    },
    list{text(label)},
  )
}

/// Render a severity badge
let viewSeverityBadge = (severity: Tauri.severity): Tea_Html.t<msg> => {
  let (colorClass, label) = switch severity {
  | Fatal => ("bg-red-800 text-white", "FATAL")
  | Error => ("bg-red-600 text-white", "ERROR")
  | Warning => ("bg-yellow-500 text-gray-900", "WARN")
  | Info => ("bg-blue-500 text-white", "INFO")
  }
  span(
    list{Attrs.class_("px-2 py-0.5 rounded text-xs font-mono font-bold " ++ colorClass)},
    list{text(label)},
  )
}

/// Render a fixability badge
let viewFixabilityBadge = (fixable: Tauri.fixability): Tea_Html.t<msg> => {
  let (colorClass, label) = switch fixable {
  | AutoSafe => ("bg-green-600 text-white", "auto-fix")
  | Recommended => ("bg-blue-600 text-white", "recommended")
  | ManualOnly => ("bg-orange-600 text-white", "manual")
  | NotFixable => ("bg-gray-500 text-white", "n/a")
  }
  span(
    list{Attrs.class_("px-2 py-0.5 rounded text-xs font-mono " ++ colorClass)},
    list{text(label)},
  )
}

/// Render the loading indicator
let viewLoadingIndicator = (label: string): Tea_Html.t<msg> => {
  div(
    list{Attrs.class_("flex items-center gap-2 text-blue-600 py-4")},
    list{
      span(list{Attrs.class_("animate-spin inline-block w-4 h-4 border-2 border-blue-600 border-t-transparent rounded-full")}, list{}),
      text(label),
    },
  )
}

// ---------------------------------------------------------------------------
// View: Editor panel
// ---------------------------------------------------------------------------

/// Editor view: document source editing, file open/save, stream selection
let viewEditor = (model: model): Tea_Html.t<msg> => {
  div(
    list{Attrs.class_("space-y-4")},
    list{
      // Toolbar
      div(
        list{Attrs.class_("flex items-center gap-2 flex-wrap")},
        list{
          button(
            list{
              Attrs.class_("px-3 py-1.5 bg-blue-600 text-white rounded text-sm hover:bg-blue-700"),
              Events.onClick(OpenFile),
            },
            list{text("Open File")},
          ),
          button(
            list{
              Attrs.class_("px-3 py-1.5 bg-gray-600 text-white rounded text-sm hover:bg-gray-700"),
              Events.onClick(SaveFile),
            },
            list{text("Save")},
          ),
          // File path display
          switch model.filePath {
          | Some(path) =>
            span(
              list{Attrs.class_("text-sm text-gray-500 ml-2 truncate max-w-md")},
              list{text(path)},
            )
          | None =>
            span(
              list{Attrs.class_("text-sm text-gray-400 ml-2 italic")},
              list{text("No file loaded")},
            )
          },
        },
      ),
      // Stream selection
      div(
        list{Attrs.class_("flex items-center gap-3 flex-wrap")},
        list{
          label(
            list{Attrs.class_("text-sm font-medium text-gray-700")},
            list{text("Stream:")},
          ),
          select(
            list{
              Attrs.class_("px-2 py-1 border border-gray-300 rounded text-sm"),
              Events.onChange(value => {
                switch value {
                | "ietf-individual" => SetStream(IetfIndividual)
                | "ietf-wg" => SetStream(IetfWorkingGroup(model.groupName))
                | "irtf-rg" => SetStream(IrtfResearchGroup(model.groupName))
                | "irtf-individual" => SetStream(IrtfIndividual)
                | "iab" => SetStream(IabDocument)
                | "independent" => SetStream(IndependentSubmission)
                | "iana" => SetStream(IanaRegistry(model.groupName))
                | _ => NoOp
                }
              }),
            },
            list{
              option(list{Attrs.value("ietf-individual")}, list{text("IETF Individual")}),
              option(list{Attrs.value("ietf-wg")}, list{text("IETF Working Group")}),
              option(list{Attrs.value("irtf-rg")}, list{text("IRTF Research Group")}),
              option(list{Attrs.value("irtf-individual")}, list{text("IRTF Individual")}),
              option(list{Attrs.value("iab")}, list{text("IAB Document")}),
              option(list{Attrs.value("independent")}, list{text("Independent Submission")}),
              option(list{Attrs.value("iana")}, list{text("IANA Registry Request")}),
            },
          ),
          // Group name input (shown for streams that need it)
          switch model.streamSelection {
          | IetfWorkingGroup(_) | IrtfResearchGroup(_) | IanaRegistry(_) =>
            input(
              list{
                Attrs.class_("px-2 py-1 border border-gray-300 rounded text-sm w-40"),
                Attrs.placeholder("Group/registry name"),
                Attrs.value(model.groupName),
                Events.onInput(value => SetGroupName(value)),
              },
              list{},
            )
          | _ => noNode
          },
        },
      ),
      // Source editor textarea
      div(
        list{Attrs.class_("relative")},
        list{
          textarea(
            list{
              Attrs.class_(
                "w-full h-96 font-mono text-sm p-3 border border-gray-300 rounded-lg " ++
                "bg-gray-50 focus:bg-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 " ++
                "resize-y",
              ),
              Attrs.placeholder(
                "Paste your Internet-Draft source here, or use Open File to load from disk.\n\n" ++
                "Supports RFC XML v3, RFC XML v2, and plain text formats.",
              ),
              Attrs.value(model.documentSource),
              Events.onInput(value => UpdateSource(value)),
              Attrs.ariaLabel("Document source editor"),
            },
            list{},
          ),
          // Character count
          div(
            list{Attrs.class_("absolute bottom-2 right-2 text-xs text-gray-400")},
            list{text(Int.toString(String.length(model.documentSource)) ++ " chars")},
          ),
        },
      ),
    },
  )
}

// ---------------------------------------------------------------------------
// View: Checker panel
// ---------------------------------------------------------------------------

/// Checker view: run validation, display results table
let viewChecker = (model: model): Tea_Html.t<msg> => {
  div(
    list{Attrs.class_("space-y-4")},
    list{
      // Toolbar
      div(
        list{Attrs.class_("flex items-center gap-3")},
        list{
          button(
            list{
              Attrs.class_("px-4 py-2 bg-green-600 text-white rounded font-medium hover:bg-green-700 disabled:opacity-50"),
              Events.onClick(RunCheck),
              Attrs.disabled(model.checkState === Loading || model.documentSource === ""),
            },
            list{text("Run Checks")},
          ),
          switch model.checkState {
          | Loading => viewLoadingIndicator("Checking...")
          | _ => noNode
          },
        },
      ),
      // Results
      switch model.checkSummary {
      | None =>
        switch model.checkState {
        | Failed(err) =>
          div(
            list{Attrs.class_("p-4 bg-red-50 border border-red-200 rounded-lg text-red-800")},
            list{text("Error: " ++ err)},
          )
        | _ =>
          div(
            list{Attrs.class_("p-8 text-center text-gray-500")},
            list{text("Run checks to validate the document against submission requirements.")},
          )
        }
      | Some(summary) =>
        div(
          list{Attrs.class_("space-y-4")},
          list{
            // Summary bar
            div(
              list{Attrs.class_("flex gap-4 p-3 bg-gray-50 rounded-lg text-sm")},
              list{
                span(
                  list{Attrs.class_("font-bold " ++ (summary.error_count > 0 ? "text-red-600" : "text-green-600"))},
                  list{text(Int.toString(summary.error_count) ++ " errors")},
                ),
                span(
                  list{Attrs.class_("text-yellow-600 font-medium")},
                  list{text(Int.toString(summary.warning_count) ++ " warnings")},
                ),
                span(
                  list{Attrs.class_("text-blue-600")},
                  list{text(Int.toString(summary.info_count) ++ " info")},
                ),
                span(list{Attrs.class_("text-gray-400")}, list{text("|")}),
                span(
                  list{Attrs.class_("text-green-600")},
                  list{text(Int.toString(summary.auto_fixable_count) ++ " auto-fixable")},
                ),
                span(
                  list{Attrs.class_("text-blue-600")},
                  list{text(Int.toString(summary.recommended_fixable_count) ++ " recommended")},
                ),
                span(
                  list{Attrs.class_("text-orange-600")},
                  list{text(Int.toString(summary.manual_only_count) ++ " manual")},
                ),
              },
            ),
            // Results table
            div(
              list{Attrs.class_("overflow-x-auto")},
              list{
                table(
                  list{Attrs.class_("w-full text-sm border-collapse")},
                  list{
                    thead(
                      list{},
                      list{
                        tr(
                          list{Attrs.class_("bg-gray-100 border-b")},
                          list{
                            th(list{Attrs.class_("text-left p-2 font-medium")}, list{text("Severity")}),
                            th(list{Attrs.class_("text-left p-2 font-medium")}, list{text("Check ID")}),
                            th(list{Attrs.class_("text-left p-2 font-medium")}, list{text("Message")}),
                            th(list{Attrs.class_("text-left p-2 font-medium")}, list{text("Fix")}),
                            th(list{Attrs.class_("text-left p-2 font-medium")}, list{text("Suggestion")}),
                          },
                        ),
                      },
                    ),
                    tbody(
                      list{},
                      List.fromArray(
                        Array.map(summary.results, result => {
                          tr(
                            list{Attrs.class_("border-b hover:bg-gray-50")},
                            list{
                              td(list{Attrs.class_("p-2")}, list{viewSeverityBadge(result.severity)}),
                              td(
                                list{Attrs.class_("p-2 font-mono text-xs")},
                                list{text(result.check_id)},
                              ),
                              td(list{Attrs.class_("p-2")}, list{text(result.message)}),
                              td(list{Attrs.class_("p-2")}, list{viewFixabilityBadge(result.fixable)}),
                              td(
                                list{Attrs.class_("p-2 text-gray-600 text-xs")},
                                list{
                                  text(
                                    switch Nullable.toOption(result.suggestion) {
                                    | Some(s) => s
                                    | None => "-"
                                    },
                                  ),
                                },
                              ),
                            },
                          )
                        }),
                      ),
                    ),
                  },
                ),
              },
            ),
          },
        )
      },
    },
  )
}

// ---------------------------------------------------------------------------
// View: Fixer panel
// ---------------------------------------------------------------------------

/// Fixer view: configure fix options, run fixer, preview diff, apply
let viewFixer = (model: model): Tea_Html.t<msg> => {
  div(
    list{Attrs.class_("space-y-4")},
    list{
      // Options bar
      div(
        list{Attrs.class_("flex items-center gap-4 flex-wrap")},
        list{
          button(
            list{
              Attrs.class_("px-4 py-2 bg-purple-600 text-white rounded font-medium hover:bg-purple-700 disabled:opacity-50"),
              Events.onClick(RunFix),
              Attrs.disabled(model.fixState === Loading || model.documentSource === ""),
            },
            list{text("Generate Fixes")},
          ),
          // Auto-only toggle
          label(
            list{Attrs.class_("flex items-center gap-2 text-sm cursor-pointer")},
            list{
              input(
                list{
                  Attrs.type_("checkbox"),
                  Attrs.checked(model.fixAutoOnly),
                  Events.onClick(ToggleFixAutoOnly),
                },
                list{},
              ),
              text("Auto-safe only"),
            },
          ),
          // Dry-run toggle
          label(
            list{Attrs.class_("flex items-center gap-2 text-sm cursor-pointer")},
            list{
              input(
                list{
                  Attrs.type_("checkbox"),
                  Attrs.checked(model.fixDryRun),
                  Events.onClick(ToggleFixDryRun),
                },
                list{},
              ),
              text("Preview only (dry run)"),
            },
          ),
          switch model.fixState {
          | Loading => viewLoadingIndicator("Generating fixes...")
          | _ => noNode
          },
        },
      ),
      // Fix results
      switch model.fixResult {
      | None =>
        switch model.fixState {
        | Failed(err) =>
          div(
            list{Attrs.class_("p-4 bg-red-50 border border-red-200 rounded-lg text-red-800")},
            list{text("Error: " ++ err)},
          )
        | _ =>
          div(
            list{Attrs.class_("p-8 text-center text-gray-500")},
            list{
              text("Run the fixer to generate an automatic fix plan."),
              br(),
              text("AutoSafe fixes are applied without review. Recommended fixes should be reviewed. ManualOnly fixes require human intervention."),
            },
          )
        }
      | Some(result) =>
        div(
          list{Attrs.class_("space-y-4")},
          list{
            // Fix summary
            div(
              list{Attrs.class_("flex gap-4 p-3 bg-gray-50 rounded-lg text-sm")},
              list{
                span(
                  list{Attrs.class_("text-green-600 font-bold")},
                  list{text(Int.toString(result.auto_safe_applied) ++ " auto-safe applied")},
                ),
                span(
                  list{Attrs.class_("text-blue-600 font-medium")},
                  list{text(Int.toString(result.recommended_applied) ++ " recommended applied")},
                ),
                span(
                  list{Attrs.class_("text-orange-600")},
                  list{text(Int.toString(result.manual_remaining) ++ " manual remaining")},
                ),
              },
            ),
            // Diff preview
            if result.diff_preview !== "" {
              div(
                list{Attrs.class_("space-y-2")},
                list{
                  h4(
                    list{Attrs.class_("text-sm font-medium text-gray-700")},
                    list{text("Diff Preview:")},
                  ),
                  pre(
                    list{
                      Attrs.class_(
                        "p-3 bg-gray-900 text-gray-100 rounded-lg text-xs font-mono " ++
                        "overflow-x-auto max-h-80 overflow-y-auto",
                      ),
                    },
                    list{code(list{}, list{text(result.diff_preview)})},
                  ),
                },
              )
            } else {
              noNode
            },
            // Apply button (only if not dry-run or has fixed source)
            if result.success && result.fixed_source !== "" {
              button(
                list{
                  Attrs.class_("px-4 py-2 bg-green-600 text-white rounded font-medium hover:bg-green-700"),
                  Events.onClick(ApplyFixedSource(result.fixed_source)),
                },
                list{text("Apply Fixed Source to Editor")},
              )
            } else {
              noNode
            },
          },
        )
      },
    },
  )
}

// ---------------------------------------------------------------------------
// View: Submitter panel
// ---------------------------------------------------------------------------

/// Submitter view: query Datatracker status, submission guidance
let viewSubmitter = (model: model): Tea_Html.t<msg> => {
  div(
    list{Attrs.class_("space-y-4")},
    list{
      // Query toolbar
      div(
        list{Attrs.class_("flex items-center gap-3")},
        list{
          button(
            list{
              Attrs.class_("px-4 py-2 bg-indigo-600 text-white rounded font-medium hover:bg-indigo-700 disabled:opacity-50"),
              Events.onClick(QueryStatus),
              Attrs.disabled(model.submitState === Loading),
            },
            list{text("Query Submission Status")},
          ),
          switch model.submitState {
          | Loading => viewLoadingIndicator("Querying...")
          | _ => noNode
          },
        },
      ),
      // Status display
      switch model.submissionStatus {
      | None =>
        switch model.submitState {
        | Failed(err) =>
          div(
            list{Attrs.class_("p-4 bg-red-50 border border-red-200 rounded-lg text-red-800")},
            list{text("Error: " ++ err)},
          )
        | _ =>
          div(
            list{Attrs.class_("space-y-3 p-6 bg-gray-50 rounded-lg")},
            list{
              h3(list{Attrs.class_("font-medium text-gray-800")}, list{text("Submission Workflow")}),
              ol(
                list{Attrs.class_("list-decimal ml-6 space-y-1 text-sm text-gray-600")},
                list{
                  li(list{}, list{text("Load your Internet-Draft in the Editor tab")}),
                  li(list{}, list{text("Run checks in the Checker tab to validate")}),
                  li(list{}, list{text("Apply fixes in the Fixer tab if needed")}),
                  li(list{}, list{text("Query submission status here to check Datatracker state")}),
                  li(list{}, list{text("Submit via IETF Datatracker (manual submission for now)")}),
                },
              ),
              p(
                list{Attrs.class_("text-xs text-gray-400 mt-2")},
                list{text("Automated Datatracker submission will be available in a future release.")},
              ),
            },
          )
        }
      | Some(status) =>
        div(
          list{Attrs.class_("space-y-3")},
          list{
            // Status card
            div(
              list{Attrs.class_("p-4 bg-white border border-gray-200 rounded-lg shadow-sm space-y-2")},
              list{
                div(
                  list{Attrs.class_("flex justify-between items-center")},
                  list{
                    h3(
                      list{Attrs.class_("font-bold text-gray-800")},
                      list{text(status.document_name)},
                    ),
                    span(
                      list{
                        Attrs.class_(
                          "px-3 py-1 rounded-full text-sm font-medium " ++
                          (status.submitted ? "bg-green-100 text-green-800" : "bg-gray-100 text-gray-600"),
                        ),
                      },
                      list{text(status.state)},
                    ),
                  },
                ),
                div(
                  list{Attrs.class_("grid grid-cols-2 gap-2 text-sm")},
                  list{
                    div(
                      list{},
                      list{
                        span(list{Attrs.class_("text-gray-500")}, list{text("Stream: ")}),
                        span(list{Attrs.class_("font-medium")}, list{text(status.stream)}),
                      },
                    ),
                    div(
                      list{},
                      list{
                        span(list{Attrs.class_("text-gray-500")}, list{text("Submitted: ")}),
                        span(
                          list{Attrs.class_("font-medium")},
                          list{text(status.submitted ? "Yes" : "No")},
                        ),
                      },
                    ),
                  },
                ),
                p(
                  list{Attrs.class_("text-sm text-gray-600 mt-1")},
                  list{text(status.message)},
                ),
                switch Nullable.toOption(status.datatracker_url) {
                | Some(url) =>
                  a(
                    list{
                      Attrs.class_("text-blue-600 hover:underline text-sm"),
                      Attrs.href(url),
                    },
                    list{text("View on Datatracker")},
                  )
                | None => noNode
                },
              },
            ),
          },
        )
      },
    },
  )
}

// ---------------------------------------------------------------------------
// View: Main
// ---------------------------------------------------------------------------

/// Top-level view function: renders the full application shell
let view = (model: model): Tea_Vdom.t<msg> => {
  div(
    list{Attrs.class_("min-h-screen bg-gray-100 flex flex-col")},
    list{
      // Header
      header(
        list{Attrs.class_("bg-white shadow-sm border-b border-gray-200")},
        list{
          div(
            list{Attrs.class_("max-w-7xl mx-auto px-4 py-3 flex items-center justify-between")},
            list{
              div(
                list{Attrs.class_("flex items-center gap-3")},
                list{
                  h1(
                    list{Attrs.class_("text-lg font-bold text-gray-900")},
                    list{text("intsoc-transactor")},
                  ),
                  span(
                    list{Attrs.class_("text-xs bg-gray-200 text-gray-600 px-2 py-0.5 rounded-full")},
                    list{text("v0.1.0")},
                  ),
                },
              ),
              span(
                list{Attrs.class_("text-xs text-gray-400")},
                list{text("Internet Society Document Processing")},
              ),
            },
          ),
        },
      ),
      // Tab navigation
      nav(
        list{
          Attrs.class_("max-w-7xl mx-auto px-4 pt-4 flex gap-1"),
          Attrs.role("tablist"),
          Attrs.ariaLabel("Application views"),
        },
        list{
          viewTab("Editor", Editor, model.currentView),
          viewTab("Checker", Checker, model.currentView),
          viewTab("Fixer", Fixer, model.currentView),
          viewTab("Submitter", Submitter, model.currentView),
        },
      ),
      // Main content area
      main(
        list{
          Attrs.class_("flex-1 max-w-7xl mx-auto px-4 py-4 w-full"),
          Attrs.role("tabpanel"),
        },
        list{
          div(
            list{Attrs.class_("bg-white rounded-lg shadow-sm border border-gray-200 p-6")},
            list{
              switch model.currentView {
              | Editor => viewEditor(model)
              | Checker => viewChecker(model)
              | Fixer => viewFixer(model)
              | Submitter => viewSubmitter(model)
              },
            },
          ),
        },
      ),
      // Status bar
      footer(
        list{Attrs.class_("bg-white border-t border-gray-200")},
        list{
          div(
            list{Attrs.class_("max-w-7xl mx-auto px-4 py-2 flex items-center justify-between")},
            list{
              span(
                list{Attrs.class_("text-xs text-gray-500"), Attrs.ariaLive("polite")},
                list{text(model.statusMessage)},
              ),
              div(
                list{Attrs.class_("flex items-center gap-3 text-xs text-gray-400")},
                list{
                  switch model.filePath {
                  | Some(_) => span(list{}, list{text("File loaded")})
                  | None => span(list{}, list{text("No file")})
                  },
                  switch model.checkSummary {
                  | Some(s) =>
                    span(
                      list{},
                      list{
                        text(
                          s.error_count === 0
                            ? "Checks: PASS"
                            : "Checks: " ++ Int.toString(s.error_count) ++ " errors",
                        ),
                      },
                    )
                  | None => noNode
                  },
                },
              ),
            },
          ),
        },
      ),
    },
  )
}

// ---------------------------------------------------------------------------
// Subscriptions
// ---------------------------------------------------------------------------

/// Application subscriptions.
/// Currently none, but this is where Tauri backend event listeners would go
/// (e.g., file watcher, Datatracker push notifications).
let subscriptions = (_model: model): Tea_Sub.t<msg> => {
  Tea_Sub.none
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/// Start the application.
/// This is the module entry point; side-effect runs on import.
let _app = Tea_App.standardProgram(
  ~init,
  ~update,
  ~view,
  ~subscriptions,
  (),
)
