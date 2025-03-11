use crate::fen::parse_fen;
use crate::model::Position;
use crate::san::AnnotatedAlgebraicTurn;
use std::collections::HashMap;
use std::str::FromStr;

/// A type that can be used to parse Portable Game Notation (PGN).
///
/// The specification of PGN is available on the Internet Archive at
/// <https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt>.
struct PgnParser<'a> {
    content: &'a str,
}

impl<'a> PgnParser<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }

    /// Returns a boolean indicating whether there are remaining characters to read.
    fn can_read(&self) -> bool {
        !self.content.is_empty()
    }

    /// Tests if a character is a white space character.
    ///
    /// # Definition
    ///
    /// [The specification] defines white space characters in section 7:
    ///
    /// > White space characters include space, newline, and tab characters.
    ///
    /// This parser considers any Unicode character with the `White_Space` property as a white space
    /// character, which technically does not go against the specification (notice the use of the
    /// word "include" instead of, e.g., "are").
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    /// Advances the parser, skipping leading whitespace.
    ///
    /// Returns a boolean indicating whether there was some whitespace to skip.
    fn eat_whitespace(&mut self) -> bool {
        if self.content.chars().next().is_some_and(Self::is_whitespace) {
            self.content = self.content.trim_start();
            true
        } else {
            false
        }
    }

    /// Advances the parser until right after the first occurrence of the specified character.
    ///
    /// If the specified character does not appear, `false` is returned, and the parser is left
    /// unmodified. Otherwise, the iterator is advanced as described above, and `true` is returned.
    fn eat_until(&mut self, c: char) -> bool {
        match self.content.find(c) {
            None => false,
            Some(i) => {
                self.content = &self.content[i + c.len_utf8()..];
                true
            }
        }
    }

    /// Tries to skip a comment.
    ///
    /// Returns a boolean indicating whether a comment was skipped.
    ///
    /// # Definition
    ///
    /// [The specification] defines comments in section 5:
    ///
    /// > Comment text may appear in PGN data.  There are two kinds of comments.  The
    /// > first kind is the "rest of line" comment; this comment type starts with a
    /// > semicolon character and continues to the end of the line.  The second kind
    /// > starts with a left brace character and continues to the next right brace
    /// > character.  Comments cannot appear inside any token.
    /// >
    /// > Brace comments do not nest; a left brace character appearing in a brace comment
    /// > loses its special meaning and is ignored.  A semicolon appearing inside of a
    /// > brace comment loses its special meaning and is ignored.  Braces appearing
    /// > inside of a semicolon comments lose their special meaning and are ignored.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn eat_comment(&mut self) -> crate::Result<bool> {
        Ok(if self.content.starts_with(';') {
            if !self.eat_until('\n') {
                self.content = ""
            }
            true
        } else if self.content.starts_with('{') {
            if !self.eat_until('}') {
                Err("invalid PGN: unmatched opening curly brace")?
            }
            true
        } else {
            false
        })
    }

    /// Skips whitespace and comments.
    ///
    /// This essentially calls [`Self::eat_whitespace`] and [`Self::eat_comment`] until there is no
    /// whitespace or comment remaining.
    fn advance(&mut self) -> crate::Result<()> {
        while self.eat_whitespace() || self.eat_comment()? {}
        Ok(())
    }

    /// Tries to skip a self-terminating token.
    ///
    /// If the content of the parser starts with the specified string, advances the content to after
    /// the string, and returns `true`. Otherwise, returns `false`.
    fn eat_self_terminating(&mut self, token: &str) -> bool {
        match self.content.strip_prefix(token) {
            None => false,
            Some(remainder) => {
                self.content = remainder;
                true
            }
        }
    }

    fn is_tag_name_start(c: char) -> bool {
        c.is_ascii_alphanumeric()
    }

    fn is_tag_name_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /// Reads a tag name.
    ///
    /// If no tag name can be read, `None` is returned. If a tag name is read, a reference to the
    /// corresponding slice is returned, wrapped in `Some`.
    ///
    /// # Definition
    ///
    /// [The specification] defines tag names as symbol tokens with additional constraints:
    ///
    /// > A further restriction on tag names is that they are composed exclusively of
    /// > letters, digits, and the underscore character.  This is done to facilitate
    /// > mapping of tag names into key and attribute names for use with general purpose
    /// > database programs.
    ///
    /// Symbol tokens are defined in the following way:
    ///
    /// > A symbol token starts with a letter or digit character and is immediately
    /// > followed by a sequence of zero or more symbol continuation characters.  These
    /// > continuation characters are letter characters ("A-Za-z"), digit characters
    /// > ("0-9"), the underscore ("_"), the plus sign ("+"), the octothorpe sign ("#"),
    /// > the equal sign ("="), the colon (":"),  and the hyphen ("-").  Symbols are used
    /// > for a variety of purposes.  All characters in a symbol are significant.  A
    /// > symbol token is terminated just prior to the first non-symbol character
    /// > following the symbol character sequence.  Currently, a symbol is limited to a
    /// > maximum of 255 characters in length.
    ///
    /// The length limit on symbols is not enforced by this parser.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn read_tag_name<'b>(&'b mut self) -> Option<&'a str> {
        let i = self.content.find(|c| !Self::is_tag_name_char(c))?;
        let (token, remainder) = self.content.split_at(i);
        if token.starts_with(Self::is_tag_name_start) {
            self.content = remainder;
            Some(token)
        } else {
            None
        }
    }

    /// Parses a string token.
    ///
    /// If no string token can be parsed, the `Err` variant is returned. If a string is parsed, its
    /// value is returned, wrapped in `Ok`.
    ///
    /// # Definition
    ///
    /// [The specification] defines string tokens in section 7:
    ///
    /// > A string token is a sequence of zero or more printing characters delimited by a
    /// > pair of quote characters (ASCII decimal value 34, hexadecimal value 0x22).  An
    /// > empty string is represented by two adjacent quotes.  (Note: an apostrophe is
    /// > not a quote.)  A quote inside a string is represented by the backslash
    /// > immediately followed by a quote.  A backslash inside a string is represented by
    /// > two adjacent backslashes.  Strings are commonly used as tag pair values (see
    /// > below).  Non-printing characters like newline and tab are not permitted inside
    /// > of strings.  A string token is terminated by its closing quote.  Currently, a
    /// > string is limited to a maximum of 255 characters of data.
    ///
    /// The length limit is not enforced by this parser.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_string(&mut self) -> crate::Result<String> {
        if !self.content.starts_with('"') {
            return Err("invalid PGN: string token should start with a double quote")?;
        }
        let mut value = String::new();
        let mut escaped = false;
        for (i, c) in self.content.char_indices().skip(1) {
            if c.is_ascii_control() {
                return Err(
                    "invalid PGN: ASCII control characters are not allowed in string tokens",
                )?;
            }
            if escaped {
                match c {
                    '"' | '\\' => value.push(c),
                    _ => Err(format!(
                        "invalid PGN: illegal escape sequence in string token: \"\\{}\"",
                        c
                    ))?,
                }
                escaped = false
            } else if c == '\\' {
                escaped = true
            } else if c == '"' {
                let end = i + '"'.len_utf8();
                self.content = &self.content[end..];
                return Ok(value);
            } else {
                value.push(c)
            }
        }
        Err("invalid PGN: unclosed string token")?
    }

    /// Parses the tag pair section of a game.
    ///
    /// Returns a `HashMap` containing each `(tag_name, tag_value)` pair.
    ///
    /// # Definition
    ///
    /// [The specification] defines the tag pair section in section 8.1.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_tag_pair_section<'b>(&'b mut self) -> crate::Result<HashMap<&'a str, String>> {
        let mut pairs = HashMap::new();
        while {
            self.eat_whitespace();
            self.eat_self_terminating("[")
        } {
            let name = self
                .read_tag_name()
                .ok_or("invalid PGN: missing tag name")?;
            self.eat_whitespace();
            let value = self.parse_string()?;
            self.eat_whitespace();
            if !self.eat_self_terminating("]") {
                Err("invalid PGN: unclosed tag pair")?
            }
            if pairs.insert(name, value).is_some() {
                Err(format!("invalid PGN: \"{}\" key defined twice", name))?
            }
        }
        Ok(pairs)
    }

    /// Parses an integer token.
    ///
    /// If no integer token can be parsed, `None` is returned. Otherwise, the integer that was
    /// parsed is returned, wrapped in `Some`.
    ///
    /// # Definition
    ///
    /// [The specification] defines integer tokens in section 7:
    ///
    /// > An integer token is a sequence of one or more decimal digit characters.
    /// > [...]
    /// > An integer token is terminated just prior to the first non-symbol character
    /// > following the integer digit sequence.
    ///
    /// Disregardfully of the official definition, this parser considers an integer token to end
    /// just prior to the first non-decimal digit character following the integer digit sequence.
    /// This ensures an integer token can be interpreted as an integer.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_integer(&mut self) -> Option<usize> {
        let i = self.content.find(|c: char| !c.is_ascii_digit())?;
        let (token, remainder) = self.content.split_at(i);
        if token.is_empty() {
            return None;
        }
        // `token.parse()` should never return `None` because `token` was checked to only contain
        // ASCII digits.
        let n = token.parse().unwrap();
        self.content = remainder;
        Some(n)
    }

    /// Parses a movetext move number indication.
    ///
    /// If no move number indication can be parsed, `None` is returned. Otherwise, the move number
    /// is returned, wrapped in `Some`.
    ///
    /// # Definition
    ///
    /// [The specification] defines move number indications in section 8.2.2:
    ///
    /// > A move number indication is composed of one or more adjacent digits (an integer
    /// > token) followed by zero or more periods.  The integer portion of the indication
    /// > gives the move number of the immediately following white move (if present) and
    /// > also the immediately following black move (if present).
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_move_number_indication(&mut self) -> Option<usize> {
        let n = self.parse_integer()?;
        self.eat_whitespace();
        // I don't think the specification allows the periods to be separated by whitespace.
        // "one or more white space characters may appear between the digit sequence and the
        // period(s)"
        while self.eat_self_terminating(".") {}
        Some(n)
    }

    /// Reads a movetext move.
    ///
    /// If a movetext SAN-notated move is read, a slice containing the entire SAN-notated turn,
    /// wrapped in `Some`, is returned. Otherwise, `None` is returned.
    fn read_movetext_san<'b>(&'b mut self) -> Option<&'a str> {
        let i = self
            .content
            .find(Self::is_whitespace)
            .unwrap_or(self.content.len());
        let (san, remainder) = self.content.split_at(i);
        self.content = remainder;
        if san.is_empty() { None } else { Some(san) }
    }

    /// Parses a movetext move.
    ///
    /// If no movetext SAN-notated move can be parsed, the `Err` variant is returned. Otherwise, the
    /// parsed [`AnnotatedAlgebraicTurn`] is returned, wrapped in `Ok`.
    fn parse_movetext_move(&mut self) -> crate::Result<AnnotatedAlgebraicTurn> {
        self.read_movetext_san()
            .ok_or("invalid PGN: expected movetext SAN")?
            .parse()
    }

    /// Parses a movetext numeric annotation glyph.
    ///
    /// [The specification] defines numeric annotation glyphs in section  8.2.4:
    ///
    /// > An NAG (Numeric Annotation Glyph) is a movetext element that is used to
    /// > indicate a simple annotation in a language independent manner.  An NAG is
    /// > formed from a dollar sign ("$") with a non-negative decimal integer suffix.
    /// > The non-negative integer must be from zero to 255 in value.
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_movetext_nag(&mut self) -> crate::Result<Option<u8>> {
        if self.eat_self_terminating("$") {
            let message = "invalid PGN: numeric annotation glyph should be a non-negative integer between 0 and 255";
            Ok(Some(
                self.parse_integer()
                    .ok_or(message)?
                    .try_into()
                    .map_err(|_| message)?,
            ))
        } else {
            Ok(None)
        }
    }

    /// Tests if the next token marks the end of an `<element-sequence>`.
    ///
    /// This is used internally by [`parse_element_sequence`].
    fn is_element_sequence_end(&self) -> bool {
        self.content.starts_with("1-0")
            || self.content.starts_with("0-1")
            || self.content.starts_with("1/2-1/2")
            || self.content.starts_with('*')
            || self.content.starts_with(')')
    }

    /// Parses an `<element-sequence>`, as defined in section 18 of [the specification].
    ///
    /// This returns a vector containing the parsed moved from the sequence.
    ///
    /// # Definition
    ///
    /// ```grammar
    /// <element-sequence> ::= <element> <element-sequence>
    ///                        <recursive-variation> <element-sequence>
    ///                        <empty>
    ///
    /// <element> ::= <move-number-indication>
    ///               <SAN-move>
    ///               <numeric-annotation-glyph>
    ///
    /// <recursive-variation> ::= ( <element-sequence> )
    /// ```
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn parse_element_sequence(&mut self) -> crate::Result<Vec<AnnotatedAlgebraicTurn>> {
        let mut turns = Vec::new();
        while {
            self.advance()?;
            !self.is_element_sequence_end()
        } {
            if self.eat_self_terminating("(") {
                // Recursive annotation variation (section 8.2.5).
                self.advance()?;
                self.parse_element_sequence()?;
                self.advance()?;
                if !self.eat_self_terminating(")") {
                    Err("invalid PGN recursive annotation variation: missing closing parenthesis")?
                }
            } else {
                // The specification does not allow invalid move number indications.
                if let Some(n) = self.parse_move_number_indication() {
                    let expected = turns.len() / 2 + 1;
                    if n != expected {
                        Err(format!(
                            "invalid PGN: expected move number {} but found {}",
                            expected, n
                        ))?
                    }
                    self.advance()?;
                }
                turns.push(self.parse_movetext_move()?);
                self.advance()?;
                self.parse_movetext_nag()?;
            }
        }
        Ok(turns)
    }

    /// Parses a game termination marker.
    ///
    /// [The specification] defines game termination markers in section 8.2.6:
    ///
    /// > Each movetext section has exactly one game termination marker; the marker
    /// > always occurs as the last element in the movetext.  The game termination marker
    /// > is a symbol that is one of the following four values: "1-0" (White wins), "0-1"
    /// > (Black wins), "1/2-1/2" (drawn game), and "*" (game in progress, result
    /// > unknown, or game abandoned).  Note that the digit zero is used in the above;
    /// > not the upper case letter "O".  The game termination marker appearing in the
    /// > movetext of a game must match the value of the game's Result tag pair.  (While
    /// > the marker appears as a string in the Result tag, it appears as a symbol
    /// > without quotes in the movetext.)
    ///
    /// [the specification]: https://ia902908.us.archive.org/26/items/pgn-standard-1994-03-12/PGN_standard_1994-03-12.txt
    fn eat_game_termination_marker(&mut self) -> bool {
        self.eat_self_terminating("1-0")
            || self.eat_self_terminating("0-1")
            || self.eat_self_terminating("1/2-1/2")
            || self.eat_self_terminating("*")
    }

    fn parse_movetext_section(&mut self) -> crate::Result<Vec<AnnotatedAlgebraicTurn>> {
        let turns = self.parse_element_sequence()?;
        if !self.eat_game_termination_marker() {
            Err("invalid PGN: game does not terminate properly")?
        }
        Ok(turns)
    }
}

#[derive(Debug)]
pub struct PgnGame {
    /// The starting position, if specified.
    pub starting_position: Position,
    /// The successive turns of the game.
    ///
    /// Does not contain numeric annotation glyphs.
    pub turns: Vec<AnnotatedAlgebraicTurn>,
}

impl PgnGame {
    /// Returns the number of halfmoves in this game.
    pub fn len(&self) -> usize {
        self.turns.len()
    }
}

impl FromStr for PgnGame {
    type Err = String;

    fn from_str(s: &str) -> crate::Result<Self> {
        let mut parser = PgnParser::new(s);
        let tag_pairs = parser.parse_tag_pair_section()?;
        let turns = parser.parse_movetext_section()?;
        parser.advance()?;
        if parser.can_read() {
            Err("the PGN function accepts a single PGN game")?
        }
        let starting_position = match tag_pairs.get("SetUp").map(String::as_ref) {
            Some("1") => {
                let fen = tag_pairs
                    .get("FEN")
                    .ok_or("invalid PGN: missing FEN tag (SetUp tag is set to \"1\")")?;
                parse_fen(fen)?
            }
            Some("0") | None => {
                if tag_pairs.contains_key("FEN") {
                    Err("warning: PGN contains a FEN tag, but SetUp tag is not set to \"1\"")?
                }
                Position::default()
            }
            Some(v) => Err(format!("invalid PGN: illegal value for tag SetUp: {:?}", v))?,
        };
        Ok(Self {
            starting_position,
            turns,
        })
    }
}
