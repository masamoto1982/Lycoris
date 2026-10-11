//! The reading machinery behind `tokenizer::tokenize_with_spans`: the cursor
//! that walks the source keeping its line and column, and the memo that
//! turns a repeated lexeme into a copy of its first token.
//!
//! Split out of `tokenizer` when that file outgrew the file-size budget in
//! docs/dev/specification-implementation-rules.md. What a lexeme *means* —
//! the number grammar, the delimiter rule, the string sub-grammar — stays
//! there; this is only how the text is walked.

use crate::tokenizer::SourceSpan;

/// Where each lexeme already read first became a token, so that a repeat is
/// a copy of that token rather than a second reading.
///
/// A lexeme of up to seven bytes — nearly every Word, binding and number a
/// program writes — packs into one machine word together with its length,
/// and a small direct-mapped table answers it with one compare: hashing the
/// text and comparing it byte by byte against a map's key was a measurable
/// share of reading every token. A longer lexeme, or one whose slot another
/// holds, is looked up by its text.
pub(crate) struct LexemeMemo<'a> {
    recent: [(u64, usize); 64],
    all: crate::fast_hash::FastMap<&'a str, usize>,
}

impl Default for LexemeMemo<'_> {
    fn default() -> Self {
        Self {
            recent: [(0, 0); 64],
            all: Default::default(),
        }
    }
}

impl<'a> LexemeMemo<'a> {
    /// The packed form of the `len` bytes at `start`, or 0 for a lexeme too
    /// long to pack. A packed key is never 0: its top byte is the length.
    #[inline]
    pub(crate) fn key(bytes: &[u8], start: usize, len: usize) -> u64 {
        if len > 7 {
            return 0;
        }
        let packed = match bytes.get(start..start + 8) {
            Some(window) => {
                let word = u64::from_le_bytes(window.try_into().expect("an eight-byte window"));
                word & ((1u64 << (8 * len)) - 1)
            }
            None => bytes[start..start + len]
                .iter()
                .rev()
                .fold(0u64, |word, &b| (word << 8) | u64::from(b)),
        };
        packed | ((len as u64) << 56)
    }

    #[inline]
    fn slot(key: u64) -> usize {
        (key.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 58) as usize
    }

    /// The index of the token `lexeme` (packed as `key`) first read as.
    #[inline]
    pub(crate) fn find(&mut self, key: u64, lexeme: &'a str) -> Option<usize> {
        if key != 0 {
            let (held, first) = self.recent[Self::slot(key)];
            if held == key {
                return Some(first);
            }
        }
        let first = *self.all.get(lexeme)?;
        if key != 0 {
            self.recent[Self::slot(key)] = (key, first);
        }
        Some(first)
    }

    pub(crate) fn remember(&mut self, key: u64, lexeme: &'a str, first: usize) {
        if key != 0 {
            self.recent[Self::slot(key)] = (key, first);
        }
        self.all.insert(lexeme, first);
    }
}

/// How many runs of non-whitespace bytes `input` holds, ASCII whitespace
/// only: about the number of tokens it reads as. Only a capacity — a string
/// or comment holding spaces makes it more, other whitespace makes it fewer,
/// and neither changes what is read.
pub(crate) fn ascii_word_count(input: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for &byte in input.as_bytes() {
        let space = byte.is_ascii_whitespace();
        count += usize::from(!space && !in_word);
        in_word = !space;
    }
    count
}

/// A position in the source being tokenized, advanced one character at a
/// time so that its line and column stay those of the next character.
pub(crate) struct Cursor<'a> {
    pub(crate) input: &'a str,
    pub(crate) pos: usize,
    pub(crate) here: SourceSpan,
}

impl Cursor<'_> {
    #[inline]
    pub(crate) fn peek(&self) -> Option<char> {
        // Source is nearly all ASCII, which is its own character; decoding
        // UTF-8 from a fresh slice per character was most of this loop.
        match *self.input.as_bytes().get(self.pos)? {
            byte if byte.is_ascii() => Some(char::from(byte)),
            _ => self.input[self.pos..].chars().next(),
        }
    }

    /// Advance to the next whitespace character or the end of input. A word
    /// holds no newline, so only the column moves; its ASCII bytes are taken
    /// a byte at a time, and anything else falls back to `peek`/`bump`.
    #[inline]
    pub(crate) fn skip_word(&mut self) {
        let bytes = self.input.as_bytes();
        while let Some(&byte) = bytes.get(self.pos) {
            if byte.is_ascii() {
                // `char::is_whitespace` on ASCII, vertical tab included.
                if matches!(byte, b' ' | b'\t' | b'\n' | b'\x0B' | b'\x0C' | b'\r') {
                    return;
                }
                self.pos += 1;
                self.here.column += 1;
            } else {
                match self.peek() {
                    Some(c) if !c.is_whitespace() => self.bump(),
                    _ => return,
                }
            }
        }
    }

    /// Advance to byte offset `end`, a character boundary: a column per
    /// character and a fresh line after each newline, as `bump` moves. A
    /// UTF-8 continuation byte is the one kind that begins no character.
    #[inline]
    pub(crate) fn advance_to(&mut self, end: usize) {
        for &byte in &self.input.as_bytes()[self.pos..end] {
            if byte == b'\n' {
                self.here.line += 1;
                self.here.column = 1;
            } else if byte & 0xC0 != 0x80 {
                self.here.column += 1;
            }
        }
        self.pos = end;
    }

    #[inline]
    pub(crate) fn bump(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
            if c == '\n' {
                self.here.line += 1;
                self.here.column = 1;
            } else {
                self.here.column += 1;
            }
        }
    }
}
