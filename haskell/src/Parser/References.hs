-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

-- | Reference parser and validator.
--
-- Phase 2: Will parse and validate references against the RFC reference
-- database, checking for outdated references, broken links, and
-- normative/informative classification correctness.
module Parser.References
  ( -- * Types
    Reference(..)
  , ReferenceType(..)
    -- * Parsing
  , parseReference
  , validateReference
  ) where

import Data.Text (Text)

-- | A parsed reference.
data Reference = Reference
  { refAnchor :: Text
  , refTitle  :: Text
  , refType   :: ReferenceType
  , refTarget :: Maybe Text
  }
  deriving stock (Show, Eq)

-- | Reference classification.
data ReferenceType
  = Normative
  | Informative
  deriving stock (Show, Eq)

-- | Parse a single reference entry.
--
-- TODO (Phase 2): Implement with megaparsec
parseReference :: Text -> Either Text Reference
parseReference _input = Left "Phase 2: Not yet implemented"

-- | Validate a reference against the RFC database.
--
-- TODO (Phase 2): Implement with HTTP client
validateReference :: Reference -> IO (Either Text ())
validateReference _ref = pure (Left "Phase 2: Not yet implemented")
