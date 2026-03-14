-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

-- | ABNF (Augmented BNF, RFC 5234) parser.
--
-- Phase 2: Will parse ABNF grammars found in IETF RFCs and validate
-- protocol specifications against their formal grammars.
module Parser.BNFAugmented
  ( -- * Types
    ABNFRule(..)
  , ABNFElement(..)
    -- * Parsing
  , parseABNF
  ) where

import Data.Text (Text)

-- | An ABNF rule definition.
data ABNFRule = ABNFRule
  { ruleName       :: Text
  , ruleDefinition :: [ABNFElement]
  }
  deriving stock (Show, Eq)

-- | An element within an ABNF rule.
data ABNFElement
  = Literal Text
  | RuleRef Text
  | Alternation [ABNFElement]
  | Concatenation [ABNFElement]
  | Repetition (Maybe Int) (Maybe Int) ABNFElement
  | Group [ABNFElement]
  | Optional [ABNFElement]
  deriving stock (Show, Eq)

-- | Parse an ABNF grammar.
--
-- TODO (Phase 2): Implement with megaparsec
parseABNF :: Text -> Either Text [ABNFRule]
parseABNF _input = Left "Phase 2: Not yet implemented"
