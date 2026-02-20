# SPDX-License-Identifier: PMPL-1.0-or-later
#
# stream-rules.ncl -- Per-stream submission rules and policies.
#
# Defines the SubmissionRequirements record for every stream type
# recognised by intsoc-transactor.  The Rust policy engine
# (intsoc_nickel::policy) loads this file and evaluates the incoming
# document context against the matching stream's requirements.
#
# Reference specifications:
#   - RFC 4844 (IETF stream)
#   - RFC 5742 (IESG role in IRTF / Independent streams)
#   - RFC 4846 (Independent Submission stream)
#   - RFC 6635 (RFC Editor model)
#   - BCP 9   (Internet Standards Process)

let { IprType, Category, .. } =
  import "../contracts/draft.ncl"
in

let { SubmissionRequirements, SubmissionContext, StreamType, .. } =
  import "../contracts/submission.ncl"
in

# ---------------------------------------------------------------------------
# Per-stream requirement definitions
# ---------------------------------------------------------------------------

let ietf_individual
  | SubmissionRequirements
  = {
  stream = "ietf-individual",
  organization = "ietf",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = [
    "trust200902",
    "noModificationTrust200902",
    "noDerivativesTrust200902",
  ],
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
}
in

let ietf_wg
  | SubmissionRequirements
  = {
  stream = "ietf-wg",
  organization = "ietf",
  name_prefix_pattern = "^draft-ietf-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = true,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["std", "info", "exp", "bcp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
    "IANA Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let ietf_standards_track
  | SubmissionRequirements
  = {
  stream = "ietf-standards-track",
  organization = "ietf",
  name_prefix_pattern = "^draft-(ietf-[a-z][a-z0-9]+-|[a-z][a-z0-9]+-)[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["std"],
  required_sections = [
    "Introduction",
    "Security Considerations",
    "IANA Considerations",
    "Conventions and Definitions",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let ietf_informational
  | SubmissionRequirements
  = {
  stream = "ietf-informational",
  organization = "ietf",
  name_prefix_pattern = "^draft-(ietf-[a-z][a-z0-9]+-|[a-z][a-z0-9]+-)[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["info"],
  required_sections = [
    "Introduction",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let ietf_experimental
  | SubmissionRequirements
  = {
  stream = "ietf-experimental",
  organization = "ietf",
  name_prefix_pattern = "^draft-(ietf-[a-z][a-z0-9]+-|[a-z][a-z0-9]+-)[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["exp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let ietf_bcp
  | SubmissionRequirements
  = {
  stream = "ietf-bcp",
  organization = "ietf",
  name_prefix_pattern = "^draft-(ietf-[a-z][a-z0-9]+-|[a-z][a-z0-9]+-)[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["bcp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
    "IANA Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let ietf_bis
  | SubmissionRequirements
  = {
  stream = "ietf-bis",
  organization = "ietf",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["std", "info", "exp", "bcp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
    "IANA Considerations",
    "Changes from RFC XXXX",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let irtf_rg
  | SubmissionRequirements
  = {
  stream = "irtf-rg",
  organization = "irtf",
  name_prefix_pattern = "^draft-irtf-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = true,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["info", "exp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let irtf_individual
  | SubmissionRequirements
  = {
  stream = "irtf-individual",
  organization = "irtf",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = [
    "trust200902",
    "noModificationTrust200902",
    "noDerivativesTrust200902",
  ],
  default_ipr = "trust200902",
  allowed_categories = ["info", "exp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let iab_document
  | SubmissionRequirements
  = {
  stream = "iab-document",
  organization = "iab",
  name_prefix_pattern = "^draft-iab-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["info", "bcp"],
  required_sections = [
    "Introduction",
    "Security Considerations",
    "IAB Members at the Time of Approval",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 10,
  expiry_days = 185,
}
in

let iab_statement
  | SubmissionRequirements
  = {
  stream = "iab-statement",
  organization = "iab",
  name_prefix_pattern = "^draft-iab-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = false,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["info"],
  required_sections = [
    "Introduction",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 10,
  expiry_days = 185,
}
in

let independent_submission
  | SubmissionRequirements
  = {
  stream = "independent",
  organization = "independent",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = false,
  requires_security_considerations = true,
  allowed_ipr_types = [
    "trust200902",
    "noModificationTrust200902",
    "noDerivativesTrust200902",
  ],
  default_ipr = "trust200902",
  allowed_categories = ["info", "exp", "historic"],
  required_sections = [
    "Introduction",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let iana_registry_request
  | SubmissionRequirements
  = {
  stream = "iana-registry-request",
  organization = "iana",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["std", "info"],
  required_sections = [
    "Introduction",
    "IANA Considerations",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

let iana_parameter_assignment
  | SubmissionRequirements
  = {
  stream = "iana-parameter-assignment",
  organization = "iana",
  name_prefix_pattern = "^draft-[a-z][a-z0-9]+-[a-z]",
  requires_wg_prefix = false,
  requires_abstract = true,
  requires_iana_considerations = true,
  requires_security_considerations = true,
  allowed_ipr_types = ["trust200902"],
  default_ipr = "trust200902",
  allowed_categories = ["std", "info"],
  required_sections = [
    "Introduction",
    "IANA Considerations",
    "Security Considerations",
  ],
  requires_boilerplate = true,
  boilerplate_id = "trust200902",
  min_authors = 1,
  max_authors = 5,
  expiry_days = 185,
}
in

# ---------------------------------------------------------------------------
# Lookup helper
# ---------------------------------------------------------------------------
# Returns the SubmissionRequirements for a given stream type string.
# Used by the Rust policy engine after JSON export.

let rules_by_stream = {
  "ietf-individual"          = ietf_individual,
  "ietf-wg"                  = ietf_wg,
  "ietf-standards-track"     = ietf_standards_track,
  "ietf-informational"       = ietf_informational,
  "ietf-experimental"        = ietf_experimental,
  "ietf-bcp"                 = ietf_bcp,
  "ietf-bis"                 = ietf_bis,
  "irtf-rg"                  = irtf_rg,
  "irtf-individual"          = irtf_individual,
  "iab-document"             = iab_document,
  "iab-statement"            = iab_statement,
  "independent"              = independent_submission,
  "iana-registry-request"    = iana_registry_request,
  "iana-parameter-assignment" = iana_parameter_assignment,
}
in

# ---------------------------------------------------------------------------
# Naming convention rules (supplementary)
# ---------------------------------------------------------------------------
# These describe the naming pattern expectations beyond what the regex in
# name_prefix_pattern captures.  Exported for documentation and CLI help.

let naming_conventions = {
  ietf_individual = {
    pattern = "draft-<author-surname>-<descriptive-slug>-<version>",
    example = "draft-jewell-http-430-consent-required-00",
    notes = "Author surname must be lowercase ASCII; slug uses lowercase and hyphens only.",
  },
  ietf_wg = {
    pattern = "draft-ietf-<wg-abbrev>-<descriptive-slug>-<version>",
    example = "draft-ietf-httpbis-retrofit-01",
    notes = "WG abbreviation must match an active IETF working group.",
  },
  irtf_rg = {
    pattern = "draft-irtf-<rg-abbrev>-<descriptive-slug>-<version>",
    example = "draft-irtf-cfrg-curves-02",
    notes = "RG abbreviation must match an active IRTF research group.",
  },
  iab = {
    pattern = "draft-iab-<descriptive-slug>-<version>",
    example = "draft-iab-protocol-maintenance-00",
    notes = "All IAB documents use the 'draft-iab-' prefix.",
  },
  independent = {
    pattern = "draft-<author-surname>-<descriptive-slug>-<version>",
    example = "draft-smith-vcard-extensions-00",
    notes = "Same format as individual IETF but processed by ISE.",
  },
}
in

# ---------------------------------------------------------------------------
# Exports
# ---------------------------------------------------------------------------

{
  # All rules indexed by stream type
  rules_by_stream,

  # Individual rule sets for direct import
  ietf_individual,
  ietf_wg,
  ietf_standards_track,
  ietf_informational,
  ietf_experimental,
  ietf_bcp,
  ietf_bis,
  irtf_rg,
  irtf_individual,
  iab_document,
  iab_statement,
  independent_submission,
  iana_registry_request,
  iana_parameter_assignment,

  # Naming documentation
  naming_conventions,
}
