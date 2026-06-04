// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
/* SPDX-License-Identifier: MPL-2.0 */
/* Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) */

/*
 * GHC RTS initialization stub.
 *
 * When the Haskell parser is compiled as a shared library for FFI,
 * the GHC runtime system needs to be initialized before any Haskell
 * functions can be called. This stub provides the init/deinit functions.
 *
 * Phase 2: Will be called from the Rust FFI layer.
 */

#include "HsFFI.h"

void intsoc_parser_hs_init(void) {
    static int initialized = 0;
    if (!initialized) {
        int argc = 1;
        char *argv[] = { "intsoc-parser-hs", NULL };
        char **pargv = argv;
        hs_init(&argc, &pargv);
        initialized = 1;
    }
}

void intsoc_parser_hs_deinit(void) {
    hs_exit();
}
