# SPDX-License-Identifier: PMPL-1.0-or-later
#
# boilerplate.ncl -- Boilerplate text contracts and canonical texts.
#
# Internet-Drafts and RFCs carry mandatory legal and procedural
# boilerplate defined by the IETF Trust and documented in the Trust
# Legal Provisions (TLP).  This file codifies the boilerplate variants
# so that they can be validated and injected by the toolchain.
#
# Reference: RFC 5378, BCP 78, IETF Trust Legal Provisions
# https://trustee.ietf.org/documents/trust-legal-provisions/

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

let is_nonempty_string = fun s =>
  std.is_string s && std.string.length (std.string.trim s) > 0
in

# ---------------------------------------------------------------------------
# BoilerplateVariant
# ---------------------------------------------------------------------------
# Identifies which boilerplate set a document must carry.

let boilerplate_variant_values = [
  "trust200902",
  "noModificationTrust200902",
  "noDerivativesTrust200902",
  "pre5378Trust200902",
  "iana-submission",
  "rfc-editor",
]
in

let BoilerplateVariant = std.contract.from_predicate (fun value =>
  std.is_string value
  && std.array.any (fun v => v == value) boilerplate_variant_values
)
in

# ---------------------------------------------------------------------------
# BoilerplateSection
# ---------------------------------------------------------------------------
# A single block of boilerplate text (e.g., "Status of This Memo",
# "Copyright Notice").

let BoilerplateSection = {
  id
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Machine-readable section identifier",
  heading
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "Human-readable section heading",
  text
    | String
    | std.contract.from_predicate is_nonempty_string
    | doc "The boilerplate text body",
  required
    | Bool
    | default = true
    | doc "Whether this section is mandatory",
  position
    | String
    | std.contract.from_predicate (fun v =>
        std.is_string v
        && std.array.any (fun p => p == v) ["front", "back", "any"]
      )
    | default = "front"
    | doc "Where in the document the section must appear",
}
in

# ---------------------------------------------------------------------------
# BoilerplateSet
# ---------------------------------------------------------------------------
# A complete set of boilerplate sections for one variant.

let BoilerplateSet = {
  variant
    | BoilerplateVariant
    | doc "Which IPR / submission variant this set covers",
  sections
    | Array BoilerplateSection
    | std.contract.from_predicate (fun a => std.is_array a && std.array.length a > 0)
    | doc "Ordered list of boilerplate sections",
  applicable_streams
    | Array String
    | default = []
    | doc "Stream types this boilerplate set applies to (empty = all)",
}
in

# ---------------------------------------------------------------------------
# Canonical boilerplate texts
# ---------------------------------------------------------------------------
# These are the verbatim texts mandated by the IETF Trust Legal Provisions
# (TLP) version 5.0, effective 15 September 2009.
#
# Note: {{DRAFT_NAME}}, {{EXPIRY_DATE}}, and {{YEAR}} are placeholders
# replaced at render time by the template engine.

let status_of_this_memo_individual =
  "This Internet-Draft is submitted in full conformance with the "
  ++ "provisions of BCP 78 and BCP 79.  Internet-Drafts are working "
  ++ "documents of the Internet Engineering Task Force (IETF).  Note "
  ++ "that other groups may also distribute working documents as "
  ++ "Internet-Drafts.  The list of current Internet-Drafts is at "
  ++ "https://datatracker.ietf.org/drafts/current/."
in

let status_of_this_memo_ietf_wg =
  "This Internet-Draft is submitted in full conformance with the "
  ++ "provisions of BCP 78 and BCP 79.  Internet-Drafts are working "
  ++ "documents of the Internet Engineering Task Force (IETF).  Note "
  ++ "that other groups may also distribute working documents as "
  ++ "Internet-Drafts.  The list of current Internet-Drafts is at "
  ++ "https://datatracker.ietf.org/drafts/current/."
in

let status_of_this_memo_independent =
  "This document is not an Internet Standards Track specification; it "
  ++ "is published for informational purposes.  This is a contribution "
  ++ "to the RFC Series, independently of any other RFC stream.  The "
  ++ "RFC Editor has chosen to publish this document at its discretion "
  ++ "and makes no statement about its value for implementation or "
  ++ "deployment."
in

let status_of_this_memo_irtf =
  "This Internet-Draft is submitted in full conformance with the "
  ++ "provisions of BCP 78 and BCP 79.  This document is a product of "
  ++ "the Internet Research Task Force (IRTF).  The IRTF publishes the "
  ++ "results of Internet-related research and development activities."
in

let status_of_this_memo_iab =
  "This Internet-Draft is submitted in full conformance with the "
  ++ "provisions of BCP 78 and BCP 79.  This document is a product of "
  ++ "the Internet Architecture Board (IAB) and represents information "
  ++ "that the IAB has deemed valuable to provide to the Internet "
  ++ "community."
in

let expiry_notice =
  "This Internet-Draft will expire on {{EXPIRY_DATE}}."
in

let copyright_notice_trust200902 =
  "Copyright (c) {{YEAR}} IETF Trust and the persons identified as the "
  ++ "document authors.  All rights reserved.  This document is subject "
  ++ "to BCP 78 and the IETF Trust's Legal Provisions Relating to IETF "
  ++ "Documents (https://trustee.ietf.org/license-info) in effect on "
  ++ "the date of publication of this document.  Please review these "
  ++ "documents carefully, as they describe your rights and "
  ++ "restrictions with respect to this document.  Code Components "
  ++ "extracted from this document must include Revised BSD License text "
  ++ "as described in Section 4.e of the Trust Legal Provisions and are "
  ++ "provided without warranty as described in the Revised BSD License."
in

let copyright_notice_no_modification =
  copyright_notice_trust200902
  ++ "  This document may not be modified, and derivative works of it "
  ++ "may not be created, except to format it for publication as an "
  ++ "RFC or to translate it into languages other than English."
in

let copyright_notice_no_derivatives =
  copyright_notice_trust200902
  ++ "  This document may not be modified, and derivative works of it "
  ++ "may not be created, and it may not be published except as an "
  ++ "Internet-Draft."
in

let copyright_notice_pre5378 =
  "Copyright (c) {{YEAR}} IETF Trust and the persons identified as the "
  ++ "document authors.  All rights reserved.  This document is subject "
  ++ "to BCP 78 and the IETF Trust's Legal Provisions Relating to IETF "
  ++ "Documents (https://trustee.ietf.org/license-info) in effect on "
  ++ "the date of publication of this document.  Please review these "
  ++ "documents carefully, as they describe your rights and "
  ++ "restrictions with respect to this document."
in

# ---------------------------------------------------------------------------
# Assembled boilerplate sets
# ---------------------------------------------------------------------------

let trust200902_set = {
  variant = "trust200902",
  applicable_streams = [
    "ietf-individual",
    "ietf-wg",
    "ietf-standards-track",
    "ietf-informational",
    "ietf-experimental",
    "ietf-bcp",
    "ietf-bis",
    "irtf-rg",
    "irtf-individual",
    "iab-document",
    "iab-statement",
    "independent",
  ],
  sections = [
    {
      id = "status-of-this-memo",
      heading = "Status of This Memo",
      text = status_of_this_memo_individual,
      required = true,
      position = "front",
    },
    {
      id = "expiry-notice",
      heading = "Expiry Notice",
      text = expiry_notice,
      required = true,
      position = "front",
    },
    {
      id = "copyright-notice",
      heading = "Copyright Notice",
      text = copyright_notice_trust200902,
      required = true,
      position = "front",
    },
  ],
}
in

let no_modification_set = {
  variant = "noModificationTrust200902",
  applicable_streams = [
    "ietf-individual",
    "irtf-individual",
    "independent",
  ],
  sections = [
    {
      id = "status-of-this-memo",
      heading = "Status of This Memo",
      text = status_of_this_memo_individual,
      required = true,
      position = "front",
    },
    {
      id = "expiry-notice",
      heading = "Expiry Notice",
      text = expiry_notice,
      required = true,
      position = "front",
    },
    {
      id = "copyright-notice",
      heading = "Copyright Notice",
      text = copyright_notice_no_modification,
      required = true,
      position = "front",
    },
  ],
}
in

let no_derivatives_set = {
  variant = "noDerivativesTrust200902",
  applicable_streams = [
    "ietf-individual",
    "irtf-individual",
    "independent",
  ],
  sections = [
    {
      id = "status-of-this-memo",
      heading = "Status of This Memo",
      text = status_of_this_memo_individual,
      required = true,
      position = "front",
    },
    {
      id = "expiry-notice",
      heading = "Expiry Notice",
      text = expiry_notice,
      required = true,
      position = "front",
    },
    {
      id = "copyright-notice",
      heading = "Copyright Notice",
      text = copyright_notice_no_derivatives,
      required = true,
      position = "front",
    },
  ],
}
in

let pre5378_set = {
  variant = "pre5378Trust200902",
  applicable_streams = [],
  sections = [
    {
      id = "status-of-this-memo",
      heading = "Status of This Memo",
      text = status_of_this_memo_individual,
      required = true,
      position = "front",
    },
    {
      id = "expiry-notice",
      heading = "Expiry Notice",
      text = expiry_notice,
      required = true,
      position = "front",
    },
    {
      id = "copyright-notice",
      heading = "Copyright Notice",
      text = copyright_notice_pre5378,
      required = true,
      position = "front",
    },
  ],
}
in

# ---------------------------------------------------------------------------
# Stream-specific status-of-this-memo texts
# ---------------------------------------------------------------------------
# Keyed by stream group so the template engine can select the right variant.

let stream_status_texts = {
  ietf_individual  = status_of_this_memo_individual,
  ietf_wg          = status_of_this_memo_ietf_wg,
  irtf             = status_of_this_memo_irtf,
  iab              = status_of_this_memo_iab,
  independent      = status_of_this_memo_independent,
}
in

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

{
  BoilerplateVariant,
  BoilerplateSection,
  BoilerplateSet,

  # Canonical sets
  trust200902_set,
  no_modification_set,
  no_derivatives_set,
  pre5378_set,

  # Per-stream status-of-this-memo paragraphs
  stream_status_texts,

  # Raw copyright texts (for direct use / composition)
  copyright_notice_trust200902,
  copyright_notice_no_modification,
  copyright_notice_no_derivatives,
  copyright_notice_pre5378,

  # Placeholder tokens replaced at render time
  placeholders = {
    draft_name = "{{DRAFT_NAME}}",
    expiry_date = "{{EXPIRY_DATE}}",
    year = "{{YEAR}}",
  },
}
