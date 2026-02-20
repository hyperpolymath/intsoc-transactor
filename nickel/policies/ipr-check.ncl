# SPDX-License-Identifier: PMPL-1.0-or-later
#
# ipr-check.ncl -- IPR (Intellectual Property Rights) validation policy.
#
# Implements the IPR declaration rules from RFC 8179 (BCP 79) and the
# IETF Trust Legal Provisions (TLP).  Validates that a document's IPR
# declaration is consistent with its submission stream, intended status,
# and boilerplate.
#
# Reference specifications:
#   - RFC 8179 (BCP 79) -- Intellectual Property Rights in IETF Technology
#   - RFC 5378 (BCP 78) -- Rights Contributors Provide to the IETF Trust
#   - IETF Trust Legal Provisions (TLP) v5.0

let { IprType, Category, .. } =
  import "../contracts/draft.ncl"
in

let { StreamType, SubmissionRequirements, .. } =
  import "../contracts/submission.ncl"
in

let { BoilerplateVariant, .. } =
  import "../contracts/boilerplate.ncl"
in

let { rules_by_stream, .. } =
  import "./stream-rules.ncl"
in

# ---------------------------------------------------------------------------
# IPR type metadata
# ---------------------------------------------------------------------------
# Describes the rights and restrictions for each IPR declaration type.

let ipr_metadata = {
  trust200902 = {
    ipr = "trust200902",
    label = "IETF Trust (TLP 5.0)",
    allows_modification = true,
    allows_derivatives = true,
    allows_publication_as_rfc = true,
    requires_bsd_license_for_code = true,
    notes = "Standard IETF Trust licence.  Permits modification, derivatives, "
      ++ "and publication as an RFC.  Code components must carry Revised BSD.",
  },
  noModificationTrust200902 = {
    ipr = "noModificationTrust200902",
    label = "No Modification (TLP 5.0)",
    allows_modification = false,
    allows_derivatives = true,
    allows_publication_as_rfc = true,
    requires_bsd_license_for_code = true,
    notes = "Prohibits modification except for RFC formatting and translation.  "
      ++ "Only permitted for individual and independent streams.",
  },
  noDerivativesTrust200902 = {
    ipr = "noDerivativesTrust200902",
    label = "No Derivatives (TLP 5.0)",
    allows_modification = false,
    allows_derivatives = false,
    allows_publication_as_rfc = false,
    requires_bsd_license_for_code = false,
    notes = "Prohibits modification, derivatives, and publication as an RFC.  "
      ++ "The document may only be published as an Internet-Draft.  Intended "
      ++ "for time-limited contributions where full rights are not granted.",
  },
  pre5378Trust200902 = {
    ipr = "pre5378Trust200902",
    label = "Pre-RFC 5378 (Legacy)",
    allows_modification = false,
    allows_derivatives = false,
    allows_publication_as_rfc = true,
    requires_bsd_license_for_code = false,
    notes = "For documents containing material contributed before 10 November "
      ++ "2008 (the effective date of BCP 78/RFC 5378).  Carries legacy "
      ++ "restrictions on the pre-5378 portions.",
  },
}
in

# ---------------------------------------------------------------------------
# Stream-IPR compatibility matrix
# ---------------------------------------------------------------------------
# Encodes which IPR types are valid for each stream category.
# Working group documents MUST use trust200902; restrictive IPR types
# are only available for individual / independent streams.

let stream_ipr_compatibility = {
  # IETF streams
  "ietf-individual" = {
    allowed = ["trust200902", "noModificationTrust200902", "noDerivativesTrust200902"],
    recommended = "trust200902",
    notes = "Individual submissions permit restrictive IPR for initial sharing.",
  },
  "ietf-wg" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "WG documents MUST use trust200902.  The WG consensus process "
      ++ "requires full rights for the community.",
  },
  "ietf-standards-track" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "Standards track requires full trust200902 for implementability.",
  },
  "ietf-informational" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "IETF informational stream uses trust200902.",
  },
  "ietf-experimental" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "Experimental documents need trust200902 for implementation.",
  },
  "ietf-bcp" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "Best Current Practice documents MUST use trust200902.",
  },
  "ietf-bis" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "BIS (replacement) documents inherit the original RFC's IPR plus trust200902.",
  },

  # IRTF streams
  "irtf-rg" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "Research group documents follow the same IPR rules as IETF WG documents.",
  },
  "irtf-individual" = {
    allowed = ["trust200902", "noModificationTrust200902", "noDerivativesTrust200902"],
    recommended = "trust200902",
    notes = "IRTF individual submissions may use restrictive IPR types.",
  },

  # IAB streams
  "iab-document" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "IAB documents always use trust200902.",
  },
  "iab-statement" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "IAB statements always use trust200902.",
  },

  # Independent
  "independent" = {
    allowed = ["trust200902", "noModificationTrust200902", "noDerivativesTrust200902"],
    recommended = "trust200902",
    notes = "Independent stream permits restrictive IPR at ISE discretion.",
  },

  # IANA
  "iana-registry-request" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "IANA submissions always use trust200902.",
  },
  "iana-parameter-assignment" = {
    allowed = ["trust200902"],
    recommended = "trust200902",
    notes = "IANA parameter assignments always use trust200902.",
  },
}
in

# ---------------------------------------------------------------------------
# Category-IPR compatibility
# ---------------------------------------------------------------------------
# Additional constraints based on intended status.

let category_ipr_rules = {
  std = {
    category = "std",
    label = "Standards Track",
    requires_trust200902 = true,
    allows_no_modification = false,
    allows_no_derivatives = false,
    notes = "Standards Track documents MUST grant full modification rights "
      ++ "so the community can produce revisions and errata.",
  },
  info = {
    category = "info",
    label = "Informational",
    requires_trust200902 = false,
    allows_no_modification = true,
    allows_no_derivatives = true,
    notes = "Informational documents may carry restrictive IPR when "
      ++ "submitted on individual or independent streams.",
  },
  exp = {
    category = "exp",
    label = "Experimental",
    requires_trust200902 = false,
    allows_no_modification = true,
    allows_no_derivatives = true,
    notes = "Experimental documents may carry restrictive IPR when on "
      ++ "individual or independent streams.",
  },
  bcp = {
    category = "bcp",
    label = "Best Current Practice",
    requires_trust200902 = true,
    allows_no_modification = false,
    allows_no_derivatives = false,
    notes = "BCP documents MUST grant full modification rights.",
  },
  historic = {
    category = "historic",
    label = "Historic",
    requires_trust200902 = false,
    allows_no_modification = true,
    allows_no_derivatives = true,
    notes = "Historic documents may carry any permitted IPR type.",
  },
}
in

# ---------------------------------------------------------------------------
# IPR disclosure requirements
# ---------------------------------------------------------------------------
# Per RFC 8179 Section 6, authors MUST disclose any known IPR.

let disclosure_requirements = {
  personal_ipr_disclosure = {
    required = true,
    deadline = "before or at the time of submission",
    target_url = "https://datatracker.ietf.org/ipr/new-personal/",
    notes = "Each author MUST disclose any IPR they are personally aware of "
      ++ "that is or may be relevant to the technologies described.",
  },
  third_party_ipr_disclosure = {
    required = false,
    encouraged = true,
    target_url = "https://datatracker.ietf.org/ipr/new-third-party/",
    notes = "Any person aware of third-party IPR claims SHOULD file a "
      ++ "disclosure.  This is encouraged but not mandatory.",
  },
  update_obligation = {
    required = true,
    notes = "If new IPR is discovered after initial submission, an updated "
      ++ "disclosure MUST be filed.",
  },
}
in

# ---------------------------------------------------------------------------
# Boilerplate-IPR consistency
# ---------------------------------------------------------------------------
# The boilerplate variant in the document text must match the IPR declaration
# in the XML header.

let boilerplate_ipr_mapping = {
  "trust200902" = "trust200902",
  "noModificationTrust200902" = "noModificationTrust200902",
  "noDerivativesTrust200902" = "noDerivativesTrust200902",
  "pre5378Trust200902" = "pre5378Trust200902",
}
in

# ---------------------------------------------------------------------------
# Pre-5378 material rules
# ---------------------------------------------------------------------------
# Special handling for documents containing material contributed before
# 10 November 2008.

let pre5378_rules = {
  cutoff_date = "2008-11-10",
  requires_legacy_notice = true,
  legacy_notice_text =
    "This document may contain material from IETF Documents or IETF "
    ++ "Contributions published or made publicly available before November "
    ++ "10, 2008.  The person(s) controlling the copyright in some of this "
    ++ "material may not have granted the IETF Trust the right to allow "
    ++ "modifications of such material outside the IETF Standards Process.",
  applies_to = ["pre5378Trust200902"],
  notes = "If any portion of the document predates RFC 5378, the "
    ++ "pre5378Trust200902 IPR type MUST be used and the legacy "
    ++ "notice included in the boilerplate.",
}
in

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

{
  # IPR type descriptions
  ipr_metadata,

  # Per-stream compatibility
  stream_ipr_compatibility,

  # Per-category constraints
  category_ipr_rules,

  # Disclosure obligations
  disclosure_requirements,

  # Boilerplate-to-IPR mapping
  boilerplate_ipr_mapping,

  # Legacy material rules
  pre5378_rules,
}
