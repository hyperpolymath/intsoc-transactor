# SPDX-License-Identifier: PMPL-1.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

# Consent Pipeline

## Overview

The consent pipeline connects three components that together enable
consent-aware document processing and publication for Internet Society
standards work:

```
intsoc-transactor  -->  consent-aware-http  -->  branch-newspaper
  (check/fix/submit)    (consent middleware)     (publication platform)
```

## Components

### intsoc-transactor (this repo)

**Purpose:** Check, fix, and submit documents across all Internet Society
streams (IETF, IRTF, IAB, Independent Stream, IANA, RFC Editor).

**What it does:**
- Parses RFC XML v3 and plain-text Internet-Drafts
- Validates documents against per-stream requirements (idnits, metadata, SPDX)
- Classifies fixes as AutoSafe, Recommended, or ManualOnly
- Tracks document lifecycle with per-stream state machines (20+ states for IETF)
- Provides CLI (`intsoc check/fix/submit/status/init`) and Gossamer desktop GUI

**Key crates:**
- `intsoc-core` — domain model, state machines, validation framework
- `intsoc-parser` — RFC XML v3 and plain-text parsing
- `intsoc-fixer` — fix engine with safety classification
- `intsoc-nickel` — Nickel template rendering and policy validation
- `intsoc-api` — IETF Datatracker and IANA API clients
- `intsoc-cli` — CLI binary

### consent-aware-http (planned)

**Purpose:** HTTP middleware layer that enforces consent semantics on document
submissions and API interactions.

**Status:** Not yet created as a standalone repository.

**Planned responsibilities:**
- Enforce HTTP 430 Consent Required responses where consent has not been given
- Track consent state across document submission workflows
- Bridge between intsoc-transactor's submission engine and downstream publication
- Provide consent audit trails for governance compliance

**Design intent:** When intsoc-transactor submits a document or interacts with
external APIs (IETF Datatracker, IANA registries), consent-aware-http ensures
that all required consents (author consent, IPR declarations, publication
consent) have been obtained before the request proceeds.

### branch-newspaper

**Purpose:** Phoenix LiveView application for citizen journalists and union
branches, with decentralised content storage.

**What it does:**
- Meeting minutes management (create, edit, organise)
- IPFS integration for decentralised, immutable content storage
- Real-time UI via Phoenix LiveView
- Tag-based organisation for content discovery

**Tech stack:** Elixir, Phoenix 1.8.1, LiveView 1.1.0, SQLite3/PostgreSQL, IPFS

**Connection to the pipeline:** branch-newspaper is the publication endpoint
where processed and consent-verified documents are made available to union
branches and citizen journalists. Content that passes through intsoc-transactor
(validated, fixed) and consent-aware-http (consent-verified) can be published
through branch-newspaper's IPFS-backed storage.

## How They Connect

```
1. Author creates/edits an Internet-Draft
              |
              v
2. intsoc-transactor: check + fix
   - Validates RFC XML structure
   - Checks SPDX headers, metadata, idnits
   - Applies AutoSafe fixes
   - Tracks state machine transitions
              |
              v
3. consent-aware-http: consent gate (planned)
   - Verifies author consent, IPR declarations
   - Enforces HTTP 430 where consent missing
   - Maintains consent audit trail
              |
              v
4. branch-newspaper: publish
   - Stores validated content on IPFS
   - Makes documents available via LiveView UI
   - Tags and organises for discovery
```

## Current Status

| Component | Status | Repository |
|-----------|--------|-----------|
| intsoc-transactor | Active development | This repo |
| consent-aware-http | Planned | Not yet created |
| branch-newspaper | Active development | [branch-newspaper](https://github.com/hyperpolymath/branch-newspaper) |

## See Also

- [intsoc-transactor README](../README.adoc) — full project documentation
- [branch-newspaper](https://github.com/hyperpolymath/branch-newspaper) — publication platform
- [HTTP 430 Consent Required](https://datatracker.ietf.org/doc/draft-jewell-http-430-consent-required/) — the consent HTTP status code
