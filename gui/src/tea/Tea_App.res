// SPDX-License-Identifier: PMPL-1.0-or-later

/// TEA Application - The core TEA runtime.
///
/// Manages the model/update/view lifecycle, command execution,
/// subscription management, and virtual DOM rendering.
/// Ported from the PanLL TEA implementation for intsoc-transactor.

/// Configuration for a TEA program
type programConfig<'model, 'msg> = {
  init: unit => ('model, Tea_Cmd.t<'msg>),
  update: ('model, 'msg) => ('model, Tea_Cmd.t<'msg>),
  view: 'model => Tea_Vdom.t<'msg>,
  subscriptions: 'model => Tea_Sub.t<'msg>,
}

/// Internal mutable state for the running application
type appState<'model, 'msg> = {
  mutable model: 'model,
  mutable currentSub: Tea_Sub.t<'msg>,
  mutable subscriptionCleanup: unit => unit,
  mutable isDispatching: bool,
  mutable messageQueue: array<'msg>,
  mutable renderState: option<Tea_Render.renderState<'msg>>,
  mutable container: option<Tea_Render.domElement>,
}

/// Handle returned from starting a program, allows shutdown and model inspection
type programInterface<'msg, 'model> = {
  shutdown: unit => unit,
  getModel: unit => 'model,
}

/// Start a standard TEA program with subscriptions.
///
/// This is the full TEA runtime supporting init, update, view,
/// and subscriptions. The application mounts to the #app DOM element.
let standardProgram = (
  ~init: unit => ('model, Tea_Cmd.t<'msg>),
  ~update: ('model, 'msg) => ('model, Tea_Cmd.t<'msg>),
  ~view: 'model => Tea_Vdom.t<'msg>,
  ~subscriptions: 'model => Tea_Sub.t<'msg>,
  (),
): programInterface<'msg, 'model> => {
  let config = {init, update, view, subscriptions}
  let (initialModel, initialCmd) = config.init()
  let state: appState<'model, 'msg> = {
    model: initialModel,
    currentSub: Tea_Sub.none,
    subscriptionCleanup: () => (),
    isDispatching: false,
    messageQueue: [],
    renderState: None,
    container: None,
  }
  let rec dispatch = (msg: 'msg): unit => {
    if state.isDispatching {
      Array.push(state.messageQueue, msg)
    } else {
      state.isDispatching = true
      processMessage(msg)
      while Array.length(state.messageQueue) > 0 {
        let queuedMsg = Array.shift(state.messageQueue)
        switch queuedMsg {
        | Some(m) => processMessage(m)
        | None => ()
        }
      }
      state.isDispatching = false
    }
  }
  and processMessage = (msg: 'msg): unit => {
    let (newModel, cmd) = config.update(state.model, msg)
    state.model = newModel
    render()
    updateSubscriptions()
    Tea_Cmd.execute(cmd, dispatch)
  }
  and render = (): unit => {
    let vdom = config.view(state.model)
    switch (state.container, state.renderState) {
    | (Some(container), Some(renderState)) =>
      Tea_Render.update(container, vdom, renderState)
    | (Some(container), None) =>
      let renderState = Tea_Render.createState(dispatch)
      Tea_Render.render(container, vdom, renderState)
      state.renderState = Some(renderState)
    | (None, _) =>
      switch Tea_Render.mount("#app", vdom, dispatch) {
      | Some(renderState) =>
        state.renderState = Some(renderState)
        state.container = Tea_Render.querySelector("#app")
      | None => Console.warn("No #app element found")
      }
    }
  }
  and updateSubscriptions = (): unit => {
    let newSub = config.subscriptions(state.model)
    let oldKeys = Tea_Sub.getKeys(state.currentSub)
    let newKeys = Tea_Sub.getKeys(newSub)
    let changed = Array.length(oldKeys) !== Array.length(newKeys) ||
      Array.some(oldKeys, key => !Array.includes(newKeys, key))
    if changed {
      state.subscriptionCleanup()
      state.currentSub = newSub
      state.subscriptionCleanup = Tea_Sub.enable(newSub, dispatch)
    }
  }
  render()
  let initialSub = config.subscriptions(state.model)
  state.currentSub = initialSub
  state.subscriptionCleanup = Tea_Sub.enable(initialSub, dispatch)
  Tea_Cmd.execute(initialCmd, dispatch)
  {
    shutdown: () => state.subscriptionCleanup(),
    getModel: () => state.model,
  }
}

/// Start a simple TEA program without subscriptions.
///
/// Convenience wrapper around standardProgram for applications that
/// do not need external event subscriptions.
let simpleProgram = (
  ~init: unit => ('model, Tea_Cmd.t<'msg>),
  ~update: ('model, 'msg) => ('model, Tea_Cmd.t<'msg>),
  ~view: 'model => Tea_Vdom.t<'msg>,
  (),
): programInterface<'msg, 'model> => {
  standardProgram(~init, ~update, ~view, ~subscriptions=_ => Tea_Sub.none, ())
}
