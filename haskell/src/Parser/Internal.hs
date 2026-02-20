-- SPDX-License-Identifier: PMPL-1.0-or-later
-- Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <jonathan.jewell@open.ac.uk>

-- | Internal parser utilities.
module Parser.Internal
  ( -- * Utility parsers
    whitespace
  , lexeme
  , symbol
  ) where

import Data.Text (Text)
import Data.Void (Void)
import Text.Megaparsec
import Text.Megaparsec.Char
import qualified Text.Megaparsec.Char.Lexer as L

type Parser = Parsec Void Text

-- | Consume whitespace.
whitespace :: Parser ()
whitespace = L.space space1 (L.skipLineComment ";") (L.skipBlockComment "/*" "*/")

-- | Parse a lexeme followed by whitespace.
lexeme :: Parser a -> Parser a
lexeme = L.lexeme whitespace

-- | Parse a symbol followed by whitespace.
symbol :: Text -> Parser Text
symbol = L.symbol whitespace
