// SPDX-License-Identifier: PMPL-1.0-or-later

/// TEA HTML - Convenience wrappers for building HTML views.
///
/// Provides element constructors, attribute helpers, and event helpers
/// layered on top of Tea_Vdom. Modelled on the PanLL TEA implementation.

open Tea_Vdom

/// Re-export Vdom types for convenience
type t<'msg> = Tea_Vdom.t<'msg>

/// Create a text node
let text = Tea_Vdom.text

/// Create an element node
let node = Tea_Vdom.node

/// No node (renders nothing)
let noNode: t<'msg> = Text("")

/// Common HTML elements
let div = (attrs, children) => node("div", attrs, children)
let span = (attrs, children) => node("span", attrs, children)
let p = (attrs, children) => node("p", attrs, children)
let h1 = (attrs, children) => node("h1", attrs, children)
let h2 = (attrs, children) => node("h2", attrs, children)
let h3 = (attrs, children) => node("h3", attrs, children)
let h4 = (attrs, children) => node("h4", attrs, children)
let h5 = (attrs, children) => node("h5", attrs, children)
let pre = (attrs, children) => node("pre", attrs, children)
let code = (attrs, children) => node("code", attrs, children)
let strong = (attrs, children) => node("strong", attrs, children)
let em = (attrs, children) => node("em", attrs, children)
let br = () => node("br", list{}, list{})
let hr = () => node("hr", list{}, list{})
let button = (attrs, children) => node("button", attrs, children)
let input = (attrs, children) => node("input", attrs, children)
let textarea = (attrs, children) => node("textarea", attrs, children)
let select = (attrs, children) => node("select", attrs, children)
let option = (attrs, children) => node("option", attrs, children)
let label = (attrs, children) => node("label", attrs, children)
let a = (attrs, children) => node("a", attrs, children)
let img = (attrs, children) => node("img", attrs, children)
let ul = (attrs, children) => node("ul", attrs, children)
let ol = (attrs, children) => node("ol", attrs, children)
let li = (attrs, children) => node("li", attrs, children)
let form = (attrs, children) => node("form", attrs, children)
let table = (attrs, children) => node("table", attrs, children)
let thead = (attrs, children) => node("thead", attrs, children)
let tbody = (attrs, children) => node("tbody", attrs, children)
let tr = (attrs, children) => node("tr", attrs, children)
let th = (attrs, children) => node("th", attrs, children)
let td = (attrs, children) => node("td", attrs, children)
let header = (attrs, children) => node("header", attrs, children)
let footer = (attrs, children) => node("footer", attrs, children)
let main = (attrs, children) => node("main", attrs, children)
let nav = (attrs, children) => node("nav", attrs, children)
let section = (attrs, children) => node("section", attrs, children)
let article = (attrs, children) => node("article", attrs, children)
let aside = (attrs, children) => node("aside", attrs, children)
let details = (attrs, children) => node("details", attrs, children)
let summary = (attrs, children) => node("summary", attrs, children)

/// Attribute helpers module
module Attrs = {
  let class_ = Tea_Vdom.class_
  let id = Tea_Vdom.id
  let style = Tea_Vdom.style
  let placeholder = Tea_Vdom.placeholder
  let value = Tea_Vdom.value
  let title = Tea_Vdom.title
  let href = Tea_Vdom.href
  let src = Tea_Vdom.src
  let alt = Tea_Vdom.alt
  let disabled = Tea_Vdom.disabled
  let checked = Tea_Vdom.checked
  let type_ = Tea_Vdom.type_
  let name = Tea_Vdom.name
  let for_ = Tea_Vdom.for_
  let rows = Tea_Vdom.rows
  let cols = Tea_Vdom.cols
  let readonly = Tea_Vdom.readonly
  let selected = Tea_Vdom.selected

  // ARIA accessibility
  let ariaLabel = Tea_Vdom.ariaLabel
  let ariaLive = Tea_Vdom.ariaLive
  let ariaExpanded = Tea_Vdom.ariaExpanded
  let ariaHidden = Tea_Vdom.ariaHidden
  let ariaPressed = Tea_Vdom.ariaPressed
  let ariaCurrent = Tea_Vdom.ariaCurrent
  let ariaDescribedBy = Tea_Vdom.ariaDescribedBy
  let role = Tea_Vdom.role
}

/// Event helpers module
module Events = {
  let onClick = Tea_Vdom.onClick
  let onInput = Tea_Vdom.onInput
  let onChange = Tea_Vdom.onChange
  let onSubmit = Tea_Vdom.onSubmit
  let onMouseEnter = Tea_Vdom.onMouseEnter
  let onMouseLeave = Tea_Vdom.onMouseLeave
  let onFocus = Tea_Vdom.onFocus
  let onBlur = Tea_Vdom.onBlur
}

/// Map the message type of a virtual DOM tree
let map = Tea_Vdom.map
