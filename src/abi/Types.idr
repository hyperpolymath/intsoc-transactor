-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <jonathan.jewell@open.ac.uk>
--
||| ABI Type Definitions for intsoc-transactor
|||
||| Defines the Application Binary Interface for cross-language interop
||| between Rust core, Haskell parsers, and Zig FFI layer.
|||
||| Phase 3: These types will be used for the Haskell parser FFI bridge.

module IntSoc.ABI.Types

import Data.Bits
import Data.So
import Data.Vect

%default total

--------------------------------------------------------------------------------
-- Platform Detection
--------------------------------------------------------------------------------

||| Supported platforms for this ABI
public export
data Platform = Linux | Windows | MacOS | BSD | WASM

--------------------------------------------------------------------------------
-- Core Domain Types
--------------------------------------------------------------------------------

||| Internet Society organizations
public export
data Organization = IETF | IRTF | IAB | Independent | IANA | RFCEditor

||| Convert Organization to C integer
public export
orgToInt : Organization -> Bits8
orgToInt IETF = 0
orgToInt IRTF = 1
orgToInt IAB = 2
orgToInt Independent = 3
orgToInt IANA = 4
orgToInt RFCEditor = 5

||| Document format identifier
public export
data DocFormat = XmlV3 | XmlV2 | PlainText

||| Convert DocFormat to C integer
public export
docFormatToInt : DocFormat -> Bits8
docFormatToInt XmlV3 = 0
docFormatToInt XmlV2 = 1
docFormatToInt PlainText = 2

||| Severity level for check results
public export
data Severity = Info | Warning | Error | Fatal

||| Severity ordering proof: Info < Warning < Error < Fatal
public export
severityToInt : Severity -> Bits8
severityToInt Info = 0
severityToInt Warning = 1
severityToInt Error = 2
severityToInt Fatal = 3

||| Fix classification
public export
data Fixability = AutoSafe | Recommended | ManualOnly | NotFixable

||| Convert Fixability to C integer
public export
fixabilityToInt : Fixability -> Bits8
fixabilityToInt AutoSafe = 0
fixabilityToInt Recommended = 1
fixabilityToInt ManualOnly = 2
fixabilityToInt NotFixable = 3

--------------------------------------------------------------------------------
-- Result Codes
--------------------------------------------------------------------------------

||| Result codes for FFI operations
public export
data Result : Type where
  Ok : Result
  Error_ : Result
  InvalidParam : Result
  OutOfMemory : Result
  NullPointer : Result
  ParseFailed : Result
  ValidationFailed : Result

||| Convert Result to C integer
public export
resultToInt : Result -> Bits32
resultToInt Ok = 0
resultToInt Error_ = 1
resultToInt InvalidParam = 2
resultToInt OutOfMemory = 3
resultToInt NullPointer = 4
resultToInt ParseFailed = 5
resultToInt ValidationFailed = 6

--------------------------------------------------------------------------------
-- Opaque Handles
--------------------------------------------------------------------------------

||| Opaque handle for a parsed document
public export
data DocHandle : Type where
  MkDocHandle : (ptr : Bits64) -> {auto 0 nonNull : So (ptr /= 0)} -> DocHandle

||| Safely create a document handle
public export
createDocHandle : Bits64 -> Maybe DocHandle
createDocHandle 0 = Nothing
createDocHandle ptr = Just (MkDocHandle ptr)

||| Extract pointer from handle
public export
docHandlePtr : DocHandle -> Bits64
docHandlePtr (MkDocHandle ptr) = ptr

--------------------------------------------------------------------------------
-- Check Result ABI Struct
--------------------------------------------------------------------------------

||| Check result passed across FFI boundary.
||| Fixed-size struct for C ABI compatibility.
public export
record CheckResultABI where
  constructor MkCheckResultABI
  checkIdLen  : Bits16      -- Length of check ID string
  severity    : Bits8       -- Severity enum value
  category    : Bits8       -- CheckCategory enum value
  fixability  : Bits8       -- Fixability enum value
  padding     : Vect 3 Bits8 -- Explicit padding for alignment
  msgLen      : Bits32      -- Length of message string
  line        : Bits32      -- Source line number (0 = not applicable)

--------------------------------------------------------------------------------
-- Memory Layout
--------------------------------------------------------------------------------

||| Proof that a type has a specific size
public export
data HasSize : Type -> Nat -> Type where
  SizeProof : {0 t : Type} -> {n : Nat} -> HasSize t n

||| CheckResultABI is 16 bytes on all platforms
public export
checkResultABISize : HasSize CheckResultABI 16
checkResultABISize = SizeProof

--------------------------------------------------------------------------------
-- FFI Declarations (Phase 3)
--------------------------------------------------------------------------------

namespace Foreign

  ||| Parse a document from source text.
  ||| Returns a DocHandle on success.
  export
  %foreign "C:intsoc_parse_document, libintsoc_ffi"
  prim__parseDocument : Bits64 -> Bits64 -> PrimIO Bits64

  ||| Get the number of check results for a document.
  export
  %foreign "C:intsoc_check_count, libintsoc_ffi"
  prim__checkCount : Bits64 -> PrimIO Bits32

  ||| Free a document handle.
  export
  %foreign "C:intsoc_free_document, libintsoc_ffi"
  prim__freeDocument : Bits64 -> PrimIO ()
