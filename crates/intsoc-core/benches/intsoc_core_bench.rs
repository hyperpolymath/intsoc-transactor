// SPDX-License-Identifier: PMPL-1.0-or-later
// SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell

//! intsoc-core benchmarks — domain model construction, state machine transitions,
//! and draft name parsing.
//!
//! Covers the hot paths used during batch document processing:
//! - `InternetDraft::parse_draft_name` — called on every document ingest
//! - `IetfState::valid_transitions` — called on every state machine step
//! - `StateMachine::transition` — full state advance including history append
//! - `Organization::datatracker_base` + `uses_datatracker` — called per request

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use intsoc_core::{
    document::InternetDraft,
    organization::Organization,
    state::{IetfState, StateMachine, StreamState},
    stream::Stream,
    validation::{CheckCategory, CheckResult, CheckSummary, Fixability, Severity},
};

// ============================================================================
// Draft name parsing benchmarks
// ============================================================================

/// Benchmark parsing a well-formed draft name (common fast path).
fn bench_parse_draft_name_valid(c: &mut Criterion) {
    let draft_names = [
        "draft-ietf-quic-transport-34",
        "draft-ietf-tls-rfc8446bis-09",
        "draft-iab-protocol-maintenance-14",
        "draft-irtf-cfrg-hpke-12",
        "draft-ietf-httpbis-semantics-19",
    ];

    c.bench_function("parse_draft_name_valid", |b| {
        b.iter(|| {
            for name in &draft_names {
                black_box(InternetDraft::parse_draft_name(black_box(name)));
            }
        })
    });
}

/// Benchmark parsing an invalid draft name (error path).
fn bench_parse_draft_name_invalid(c: &mut Criterion) {
    let invalid_names = [
        "not-a-draft",
        "draft-no-version",
        "draft-",
        "",
        "RFC7230",
    ];

    c.bench_function("parse_draft_name_invalid", |b| {
        b.iter(|| {
            for name in &invalid_names {
                black_box(InternetDraft::parse_draft_name(black_box(name)));
            }
        })
    });
}

// ============================================================================
// State machine benchmarks
// ============================================================================

/// Benchmark querying valid transitions from each IETF state.
///
/// This is called on every UI render and state display, so it needs
/// to be cheap even for complex state graphs.
fn bench_ietf_valid_transitions(c: &mut Criterion) {
    let states = [
        IetfState::Draft,
        IetfState::IdnitsCheck,
        IetfState::WgDocument,
        IetfState::IesgEvaluation,
        IetfState::Approved,
    ];

    c.bench_function("ietf_valid_transitions", |b| {
        b.iter(|| {
            for state in &states {
                black_box(state.valid_transitions());
            }
        })
    });
}

/// Benchmark creating a new StateMachine starting from the initial IETF state.
fn bench_state_machine_new(c: &mut Criterion) {
    c.bench_function("state_machine_new_ietf", |b| {
        b.iter(|| black_box(StateMachine::<IetfState>::new()))
    });
}

/// Benchmark a single state transition (Draft -> IdnitsCheck).
fn bench_state_machine_single_transition(c: &mut Criterion) {
    c.bench_function("state_machine_transition_single", |b| {
        b.iter(|| {
            let mut sm = StateMachine::<IetfState>::new();
            black_box(sm.transition(black_box(IetfState::IdnitsCheck)))
        })
    });
}

/// Benchmark walking the full happy-path IETF lifecycle:
/// Draft -> IdnitsCheck -> WgDocument -> ... -> Published
fn bench_state_machine_full_lifecycle(c: &mut Criterion) {
    // Represents the happy-path transitions for a WG document.
    let lifecycle = [
        IetfState::IdnitsCheck,
        IetfState::IndividualSubmitted,
        IetfState::WgAdopted,
        IetfState::WgDocument,
        IetfState::WgLastCall,
        IetfState::WaitingForWriteup,
        IetfState::AdEvaluation,
        IetfState::IesgEvaluation,
        IetfState::IesgLastCall,
        IetfState::Approved,
        IetfState::RfcEditorQueue,
        IetfState::Auth48,
        IetfState::Published,
    ];

    c.bench_function("state_machine_full_lifecycle", |b| {
        b.iter(|| {
            let mut sm = StateMachine::<IetfState>::new();
            for next_state in &lifecycle {
                let _ = sm.transition(black_box(next_state.clone()));
            }
            black_box(sm.is_complete())
        })
    });
}

/// Benchmark checking `is_terminal` for all IETF states.
fn bench_is_terminal_all_states(c: &mut Criterion) {
    let states = [
        IetfState::Draft,
        IetfState::Published,
        IetfState::Withdrawn,
        IetfState::Expired,
        IetfState::WgDocument,
    ];

    c.bench_function("is_terminal_all_states", |b| {
        b.iter(|| {
            for s in &states {
                black_box(s.is_terminal());
            }
        })
    });
}

// ============================================================================
// Organization benchmarks
// ============================================================================

/// Benchmark organization metadata lookups (called per HTTP request).
fn bench_organization_metadata(c: &mut Criterion) {
    let orgs = [
        Organization::Ietf,
        Organization::Irtf,
        Organization::Iab,
        Organization::Independent,
        Organization::Iana,
        Organization::RfcEditor,
    ];

    c.bench_function("organization_datatracker_base", |b| {
        b.iter(|| {
            for org in &orgs {
                black_box(org.datatracker_base());
                black_box(org.uses_datatracker());
            }
        })
    });
}

// ============================================================================
// ValidationReport benchmarks
// ============================================================================

/// Benchmark building a ValidationReport from N check results.
fn bench_validation_report_build(c: &mut Criterion) {
    let make_result = |i: u32, severity: Severity| CheckResult {
        check_id: format!("check-{i:04}"),
        severity,
        message: format!("Check {i} finding"),
        location: None,
        category: CheckCategory::Boilerplate,
        fixable: Fixability::AutoSafe,
        suggestion: None,
    };

    let mut group = c.benchmark_group("validation_report_n");
    for n in [8usize, 32, 128, 512] {
        let results: Vec<CheckResult> = (0..n as u32)
            .map(|i| make_result(i, if i % 3 == 0 { Severity::Error } else { Severity::Warning }))
            .collect();
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| black_box(CheckSummary::from_results(black_box(results.clone()))))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_draft_name_valid,
    bench_parse_draft_name_invalid,
    bench_ietf_valid_transitions,
    bench_state_machine_new,
    bench_state_machine_single_transition,
    bench_state_machine_full_lifecycle,
    bench_is_terminal_all_states,
    bench_organization_metadata,
    bench_validation_report_build,
);
criterion_main!(benches);
