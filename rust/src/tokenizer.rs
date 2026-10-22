//! Source text to tokens. Mirrors `src/gui/source-text.ts` so the GUI's
//! formatter and step mode split source the way the interpreter reads it.
//!
//! Whitespace is the only delimiter. At the start of an atom, `#` opens a
//! comment that runs to the end of the line and `'` opens a string that is
//! closed by a `'` followed by whitespace or the end of input. Anything else
//! is a word running to the next whitespace.

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Str(String),
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        match chars[i] {
            '#' => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            '\'' => {
                let start = i + 1;
                let mut j = start;
                loop {
                    if j >= chars.len() {
                        return Err("unclosed string".to_string());
                    }
                    if chars[j] == '\'' && chars.get(j + 1).is_none_or(|c| c.is_whitespace()) {
                        break;
                    }
                    j += 1;
                }
                tokens.push(Token::Str(chars[start..j].iter().collect()));
                i = j + 1;
            }
            _ => {
                let start = i;
                while i < chars.len() && !chars[i].is_whitespace() {
                    i += 1;
                }
                tokens.push(Token::Word(chars[start..i].iter().collect()));
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_words_strings_and_comments() {
        let tokens = tokenize("1 'a b' # note\n[ ADD ]").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Word("1".into()),
                Token::Str("a b".into()),
                Token::Word("[".into()),
                Token::Word("ADD".into()),
                Token::Word("]".into()),
            ]
        );
    }

    #[test]
    fn quote_inside_a_string_needs_whitespace_to_close() {
        assert_eq!(
            tokenize("'it's' ").unwrap(),
            vec![Token::Str("it's".into())]
        );
    }

    #[test]
    fn unclosed_string_is_an_error() {
        assert!(tokenize("'abc").is_err());
    }
}
