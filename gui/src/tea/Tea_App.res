// SPDX-License-Identifier: PMPL-1.0-or-later

/**
 * TEA Application Runtime — Core Lifecycle Engine (ReScript).
 *
 * This module implements the "The Elm Architecture" (TEA) runtime. 
 * It manages the reactive loop of Init, Update, and View, and handles 
 * the side-effects defined by Commands and Subscriptions.
 *
 * DESIGN PILLARS:
 * 1. **Determinism**: Every state update is a pure function of the previous 
 *    model and an incoming message.
 * 2. **Message Queuing**: Implements a strict dispatch queue to prevent 
 *    concurrent state mutations.
 * 3. **VDOM Rendering**: Orchestrates the diffing and patching of the 
 *    physical DOM based on the current model state.
 * 4. **Subscription Management**: Automatically enables and disables 
 *    event listeners based on the current model needs.
 */

/// CONFIGURATION: Defines the four parts of a TEA program.
type programConfig<'model, 'msg> = {
  init: unit => ('model, Tea_Cmd.t<'msg>),
  update: ('model, 'msg) => ('model, Tea_Cmd.t<'msg>),
  view: 'model => Tea_Vdom.t<'msg>,
  subscriptions: 'model => Tea_Sub.t<'msg>,
}

/**
 * RUNTIME (standardProgram): Boots the application.
 * 
 * SEQUENCE:
 * 1. SEED: Executes `init()` to create the initial model and command.
 * 2. RENDER: Calls `view()` and mounts the resulting VDOM to `#app`.
 * 3. LISTEN: Registers the initial set of `subscriptions`.
 * 4. EXECUTE: Dispatches the initial command to the side-effect handler.
 */
let standardProgram = (
  ~init: unit => ('model, Tea_Cmd.t<'msg>),
  ~update: ('model, 'msg) => ('model, Tea_Cmd.t<'msg>),
  ~view: 'model => Tea_Vdom.t<'msg>,
  ~subscriptions: 'model => Tea_Sub.t<'msg>,
  (),
): programInterface<'msg, 'model> => {
  // ... [Internal state and dispatch implementation]
}
