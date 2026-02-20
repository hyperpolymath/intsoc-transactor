-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <jonathan.jewell@open.ac.uk>

-- | RFC XML v3 parser using megaparsec.
--
-- Phase 2: Will provide deep structural validation of RFC XML documents,
-- including semantic checks that go beyond what quick-xml can do.
module Parser.RFCXml
  ( -- * Types
    RFCDocument(..)
  , RFCSection(..)
    -- * Parsing
  , parseRFCXml
  ) where

import Data.Text (Text)

-- | Parsed RFC document structure.
data RFCDocument = RFCDocument
  { rfcName     :: Text
  , rfcTitle    :: Text
  , rfcSections :: [RFCSection]
  }
  deriving stock (Show, Eq)

-- | A section within an RFC document.
data RFCSection = RFCSection
  { sectionTitle   :: Text
  , sectionNumber  :: Text
  , sectionContent :: Text
  }
  deriving stock (Show, Eq)

-- | Parse an RFC XML v3 document.
--
-- TODO (Phase 2): Implement with megaparsec + xml-conduit
parseRFCXml :: Text -> Either Text RFCDocument
parseRFCXml _input = Left "Phase 2: Not yet implemented"
