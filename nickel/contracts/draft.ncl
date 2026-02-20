# SPDX-License-Identifier: PMPL-1.0-or-later
#
# draft.ncl -- Metadata contracts for Internet-Draft validation.
#
# Defines structural and semantic contracts that mirror the Rust domain
# model in intsoc-core (document.rs).  These contracts are applied to
# configuration and template data before it reaches the Rust layer, so
# malformed metadata is caught early.

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Predicate: string is non-empty after trimming whitespace.
let is_nonempty_string = fun s =>
  std.is_string s && std.string.length (std.string.trim s) > 0
in

# Predicate: string contains only lowercase ASCII letters, digits, and
# hyphens -- the character set allowed in draft name segments.
let is_slug = fun s =>
  std.is_string s
  && std.string.length s > 0
  && std.string.is_match "^[a-z0-9][a-z0-9-]*$" s
in

# Predicate: string looks like a plausible e-mail address.
let is_email = fun s =>
  std.is_string s
  && std.string.is_match "^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$" s
in

# ---------------------------------------------------------------------------
# DraftName
# ---------------------------------------------------------------------------
# Must start with "draft-" and end with a two-digit version number
# separated by a hyphen.  Examples:
#   "draft-jewell-http-430-consent-required-00"
#   "draft-ietf-httpbis-retrofit-01"

let DraftName = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 8
  && std.string.is_match "^draft-[a-z0-9][a-z0-9-]*-[0-9]{2}$" value
)
in

# ---------------------------------------------------------------------------
# DraftNameBase
# ---------------------------------------------------------------------------
# The draft name without version suffix.  Used when the version is tracked
# separately (e.g., in templates where version is a numeric field).

let DraftNameBase = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 6
  && std.string.is_match "^draft-[a-z0-9][a-z0-9-]*[a-z0-9]$" value
)
in

# ---------------------------------------------------------------------------
# Author
# ---------------------------------------------------------------------------
# Mirrors intsoc_core::document::Author.

let Author = {
  fullname
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Full name of the author (e.g. 'Jonathan D.A. Jewell')",
  surname
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Surname / family name used for draft naming prefix",
  initials
    | String
    | optional
    | doc "Author initials (e.g. 'J.D.A.')",
  email
    | String
    | std.contract.from_predicate is_email
    | doc "Contact e-mail address",
  organization
    | String
    | optional
    | doc "Organisation affiliation",
  role
    | String
    | optional
    | doc "Role attribute for the author element (e.g. 'editor')",
}
in

# ---------------------------------------------------------------------------
# IprType
# ---------------------------------------------------------------------------
# Maps to intsoc_core::document::IprDeclaration.

let ipr_values = [
  "trust200902",
  "noModificationTrust200902",
  "noDerivativesTrust200902",
  "pre5378Trust200902",
]
in

let IprType = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) ipr_values
)
in

# ---------------------------------------------------------------------------
# Category
# ---------------------------------------------------------------------------
# Maps to intsoc_core::document::Category.

let category_values = [
  "std",
  "info",
  "exp",
  "bcp",
  "historic",
]
in

let Category = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) category_values
)
in

# ---------------------------------------------------------------------------
# DocumentFormat
# ---------------------------------------------------------------------------
# Mirrors intsoc_core::document::DocumentFormat.

let format_values = ["xml-v3", "xml-v2", "plain-text"]
in

let DocumentFormat = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) format_values
)
in

# ---------------------------------------------------------------------------
# SeriesInfo
# ---------------------------------------------------------------------------
# Reference series identifier (e.g., "RFC" / "1234").

let SeriesInfo = {
  name
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Series name, e.g. 'RFC', 'Internet-Draft'",
  value
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Series value, e.g. '7991', 'draft-foo-bar-00'",
}
in

# ---------------------------------------------------------------------------
# Reference
# ---------------------------------------------------------------------------
# A normative or informative reference to another document.

let Reference = {
  anchor
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "XML anchor id (e.g. 'RFC7991')",
  title
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Reference title",
  target
    | String
    | optional
    | doc "URL or URN target",
  series_info
    | SeriesInfo
    | optional
    | doc "Series information for the reference",
  authors
    | Array Author
    | default = []
    | doc "Reference authors",
  date
    | String
    | optional
    | doc "Publication date (YYYY-MM-DD or YYYY-MM or YYYY)",
}
in

# ---------------------------------------------------------------------------
# DraftMetadata
# ---------------------------------------------------------------------------
# Top-level metadata record that combines all contracts above.
# This is the primary contract applied to any draft configuration.

let DraftMetadata = {
  name
    | DraftName
    | doc "Full draft name including version (e.g. 'draft-jewell-http-430-consent-required-00')",
  title
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Human-readable document title",
  format
    | DocumentFormat
    | default = "xml-v3"
    | doc "Document source format",
  category
    | Category
    | doc "Intended status / document category",
  ipr
    | IprType
    | default = "trust200902"
    | doc "IPR declaration type (RFC 8179)",
  version
    | Number
    | std.contract.from_predicate (fun v => std.is_number v && v >= 0)
    | doc "Draft revision number (0-based integer)",
  authors
    | Array Author
    | std.contract.from_predicate (fun a => std.is_array a && std.array.length a > 0)
    | doc "One or more document authors (at least one required)",
  abstract_text
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Abstract / summary paragraph",
  date
    | String
    | optional
    | doc "Document date in YYYY-MM-DD format",
  normative_references
    | Array Reference
    | default = []
    | doc "Normative references",
  informative_references
    | Array Reference
    | default = []
    | doc "Informative references",
  obsoletes
    | Array Number
    | default = []
    | doc "RFC numbers this document obsoletes",
  updates
    | Array Number
    | default = []
    | doc "RFC numbers this document updates",
}
in

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

{
  DraftName,
  DraftNameBase,
  Author,
  IprType,
  Category,
  DocumentFormat,
  SeriesInfo,
  Reference,
  DraftMetadata,
}
