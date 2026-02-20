# SPDX-License-Identifier: PMPL-1.0-or-later
#
# individual-draft.ncl -- Template for IETF individual submission drafts.
#
# Exports a complete JSON structure with all fields required by the
# intsoc-transactor Rust backend (see intsoc_core::document::Document).
# Values marked {{...}} are placeholders that MUST be overridden by the
# user before rendering via `nickel export --format json`.
#
# Usage:
#   nickel export --format json \
#     --override 'name = "draft-jewell-http-430-consent-required-00"' \
#     --override 'title = "The 430 (Consent Required) HTTP Status Code"' \
#     nickel/templates/ietf/individual-draft.ncl

let { DraftMetadata, Author, IprType, Category, DocumentFormat, Reference, .. } =
  import "../../contracts/draft.ncl"
in

let { SubmissionRequirements, .. } =
  import "../../contracts/submission.ncl"
in

let { trust200902_set, stream_status_texts, .. } =
  import "../../contracts/boilerplate.ncl"
in

# ---------------------------------------------------------------------------
# Template defaults
# ---------------------------------------------------------------------------
# Override any of these fields when invoking `nickel export --override`.

let template = {
  # -- Identification -------------------------------------------------------
  name
    | String
    | doc "Full draft name with version (MUST override)"
    = "draft-AUTHOR-TOPIC-00",
  title
    | String
    | doc "Document title (MUST override)"
    = "TITLE PLACEHOLDER",
  format
    | DocumentFormat
    | doc "Source format"
    = "xml-v3",

  # -- Classification -------------------------------------------------------
  stream = "ietf-individual",
  category
    | Category
    | doc "Intended status (override as needed)"
    = "info",
  ipr
    | IprType
    | doc "IPR declaration"
    = "trust200902",
  version
    | Number
    | doc "Draft revision number"
    = 0,

  # -- Authors --------------------------------------------------------------
  authors
    | Array Author
    | doc "Document authors (MUST override)"
    = [
      {
        fullname = "Author Name",
        surname = "Name",
        initials = "A.",
        email = "author@example.com",
        organization = "Example Org",
      },
    ],

  # -- Content --------------------------------------------------------------
  abstract_text
    | String
    | doc "Abstract paragraph (MUST override)"
    = "This document defines ...",

  date
    | String
    | optional
    | doc "Document date in YYYY-MM-DD format (auto-filled if omitted)",

  # -- References ------------------------------------------------------------
  normative_references
    | Array Reference
    | doc "Normative references"
    = [],
  informative_references
    | Array Reference
    | doc "Informative references"
    = [],

  # -- RFC lifecycle ---------------------------------------------------------
  obsoletes
    | Array Number
    | doc "RFCs this document obsoletes"
    = [],
  updates
    | Array Number
    | doc "RFCs this document updates"
    = [],

  # -- Boilerplate -----------------------------------------------------------
  boilerplate = {
    variant = "trust200902",
    status_of_this_memo = stream_status_texts.ietf_individual,
    copyright_notice = trust200902_set.sections
      |> std.array.filter (fun s => s.id == "copyright-notice")
      |> std.array.first
      |> (fun s => s.text),
    expiry_notice = trust200902_set.sections
      |> std.array.filter (fun s => s.id == "expiry-notice")
      |> std.array.first
      |> (fun s => s.text),
  },

  # -- Submission requirements (informational, for tooling) ------------------
  submission_requirements = {
    stream = "ietf-individual",
    organization = "ietf",
    name_prefix_pattern = "^draft-[a-z]+-",
    requires_wg_prefix = false,
    requires_abstract = true,
    requires_iana_considerations = false,
    requires_security_considerations = true,
    allowed_ipr_types = ["trust200902", "noModificationTrust200902", "noDerivativesTrust200902"],
    default_ipr = "trust200902",
    allowed_categories = ["std", "info", "exp", "bcp", "historic"],
    required_sections = [
      "Introduction",
      "Security Considerations",
    ],
    requires_boilerplate = true,
    boilerplate_id = "trust200902",
    min_authors = 1,
    max_authors = 5,
    expiry_days = 185,
  },

  # -- Required document sections (skeleton) ---------------------------------
  sections = [
    {
      number = "1",
      heading = "Introduction",
      content = "",
    },
    {
      number = "2",
      heading = "Conventions and Definitions",
      content = "The key words \"MUST\", \"MUST NOT\", \"REQUIRED\", \"SHALL\", "
        ++ "\"SHALL NOT\", \"SHOULD\", \"SHOULD NOT\", \"RECOMMENDED\", \"NOT "
        ++ "RECOMMENDED\", \"MAY\", and \"OPTIONAL\" in this document are to be "
        ++ "interpreted as described in BCP 14 [RFC2119] [RFC8174] when, and "
        ++ "only when, they appear in all capitals, as shown here.",
    },
    {
      number = "3",
      heading = "Security Considerations",
      content = "",
    },
    {
      number = "4",
      heading = "IANA Considerations",
      content = "This document has no IANA actions.",
    },
  ],

  # -- Well-known normative references (convenience) -------------------------
  common_references = {
    rfc2119 = {
      anchor = "RFC2119",
      title = "Key words for use in RFCs to Indicate Requirement Levels",
      target = "https://www.rfc-editor.org/rfc/rfc2119",
      series_info = { name = "RFC", value = "2119" },
      authors = [
        {
          fullname = "Scott O. Bradner",
          surname = "Bradner",
          email = "sob@harvard.edu",
        },
      ],
      date = "1997-03",
    },
    rfc8174 = {
      anchor = "RFC8174",
      title = "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words",
      target = "https://www.rfc-editor.org/rfc/rfc8174",
      series_info = { name = "RFC", value = "8174" },
      authors = [
        {
          fullname = "Barry Leiba",
          surname = "Leiba",
          email = "barryleiba@computer.org",
        },
      ],
      date = "2017-05",
    },
  },
}
in

template
