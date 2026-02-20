# SPDX-License-Identifier: PMPL-1.0-or-later
#
# submission.ncl -- Submission validation contracts.
#
# Defines the stream taxonomy and per-stream submission requirements that
# mirror intsoc_core::stream::Stream.  Used by policies/stream-rules.ncl
# and by templates to declare which constraints apply to a draft.

let { DraftName, Author, IprType, Category, .. } =
  import "./draft.ncl"
in

# ---------------------------------------------------------------------------
# StreamType
# ---------------------------------------------------------------------------
# Canonical identifiers for every submission stream.
# These correspond to the variant names in the Rust `Stream` enum,
# lowercased and hyphenated for Nickel/JSON ergonomics.

let stream_type_values = [
  # IETF streams
  "ietf-individual",
  "ietf-wg",
  "ietf-standards-track",
  "ietf-informational",
  "ietf-experimental",
  "ietf-bcp",
  "ietf-bis",
  # IRTF streams
  "irtf-rg",
  "irtf-individual",
  # IAB streams
  "iab-document",
  "iab-statement",
  # Independent
  "independent",
  # IANA
  "iana-registry-request",
  "iana-parameter-assignment",
  # RFC Editor
  "rfc-editor-errata",
  "rfc-editor-editorial",
]
in

let StreamType = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) stream_type_values
)
in

# ---------------------------------------------------------------------------
# OrganizationType
# ---------------------------------------------------------------------------
# The parent organisation that owns the stream.

let org_values = [
  "ietf",
  "irtf",
  "iab",
  "independent",
  "iana",
  "rfc-editor",
]
in

let OrganizationType = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) org_values
)
in

# ---------------------------------------------------------------------------
# WorkingGroupRef
# ---------------------------------------------------------------------------
# Abbreviated WG/RG identifier (e.g. "httpbis", "cfrg").

let WorkingGroupRef = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.string.length value > 0
  && std.string.is_match "^[a-z][a-z0-9-]*$" value
)
in

# ---------------------------------------------------------------------------
# SubmissionRequirements
# ---------------------------------------------------------------------------
# Declares what a submission MUST provide for a given stream.
# Instantiated per-stream in policies/stream-rules.ncl.

let SubmissionRequirements = {
  stream
    | StreamType
    | doc "Which submission stream these requirements apply to",
  organization
    | OrganizationType
    | doc "Parent organisation for the stream",

  # -- Naming ---------------------------------------------------------------
  name_prefix_pattern
    | String
    | doc "Regex that the draft name must match (excluding version suffix)",
  requires_wg_prefix
    | Bool
    | default = false
    | doc "Whether the name must contain a working group slug",

  # -- Required metadata ----------------------------------------------------
  requires_abstract
    | Bool
    | default = true
    | doc "Whether an abstract section is mandatory",
  requires_iana_considerations
    | Bool
    | default = false
    | doc "Whether an IANA Considerations section is mandatory",
  requires_security_considerations
    | Bool
    | default = true
    | doc "Whether a Security Considerations section is mandatory",

  # -- IPR ------------------------------------------------------------------
  allowed_ipr_types
    | Array IprType
    | default = ["trust200902"]
    | doc "IPR types permitted for this stream",
  default_ipr
    | IprType
    | default = "trust200902"
    | doc "Default IPR type when none is specified",

  # -- Category / intended status -------------------------------------------
  allowed_categories
    | Array Category
    | doc "Permitted intended-status values for this stream",

  # -- Structural requirements ----------------------------------------------
  required_sections
    | Array String
    | default = []
    | doc "Section headings that MUST appear in the document body",
  requires_boilerplate
    | Bool
    | default = true
    | doc "Whether standard boilerplate text is required",
  boilerplate_id
    | String
    | default = "trust200902"
    | doc "Identifier for the boilerplate variant to use",

  # -- Author requirements --------------------------------------------------
  min_authors
    | Number
    | default = 1
    | doc "Minimum number of authors",
  max_authors
    | Number
    | default = 5
    | doc "Maximum recommended authors (soft limit, warning only)",

  # -- Dates ----------------------------------------------------------------
  expiry_days
    | Number
    | default = 185
    | doc "Number of days after submission before the draft expires",
}
in

# ---------------------------------------------------------------------------
# SubmissionContext
# ---------------------------------------------------------------------------
# Runtime context passed from the Rust layer when evaluating a draft
# against its requirements.

let SubmissionContext = {
  stream
    | StreamType
    | doc "The submission stream for this draft",
  draft_name
    | String
    | doc "Full draft name including version",
  authors_count
    | Number
    | doc "Number of authors on the document",
  has_abstract
    | Bool
    | doc "Whether the document includes an abstract",
  has_boilerplate
    | Bool
    | doc "Whether required boilerplate text is present",
  has_iana_considerations
    | Bool
    | doc "Whether an IANA Considerations section exists",
  has_security_considerations
    | Bool
    | doc "Whether a Security Considerations section exists",
  ipr
    | String
    | doc "IPR declaration on the document",
  category
    | String
    | doc "Intended status / category",
  wg
    | String
    | optional
    | doc "Working group abbreviation, if applicable",
}
in

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

{
  StreamType,
  OrganizationType,
  WorkingGroupRef,
  SubmissionRequirements,
  SubmissionContext,
  stream_type_values,
  org_values,
}
