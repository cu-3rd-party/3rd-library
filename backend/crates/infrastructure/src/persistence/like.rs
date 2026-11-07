//! `ILIKE` в памяти: `%` — любая подстрока, `_` — один символ, `\` экранирует следующий символ
//! (как в PostgreSQL по умолчанию). Сравнение без учёта регистра: шаблон и текст в нижнем регистре.

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Any,
    One,
    Char(char),
}

#[derive(Debug, Clone)]
pub(crate) enum Pattern {
    /// `%literal%` — поиск подстроки (обычный случай: строка поиска без спецсимволов).
    Contains(String),
    Like(Vec<Token>),
}

impl Pattern {
    /// `pattern` — шаблон ILIKE как есть.
    pub(crate) fn new(pattern: &str) -> Self {
        let mut tokens = Vec::new();
        let mut chars = pattern.chars().flat_map(char::to_lowercase);
        while let Some(char) = chars.next() {
            tokens.push(match char {
                '%' => Token::Any,
                '_' => Token::One,
                '\\' => match chars.next() {
                    Some(escaped) => Token::Char(escaped),
                    // одинокий `\` в конце PostgreSQL считает ошибкой шаблона; здесь — просто символ
                    None => Token::Char('\\'),
                },
                char => Token::Char(char),
            });
        }
        tokens.dedup_by(|a, b| *a == Token::Any && *b == Token::Any);

        if let [Token::Any, middle @ .., Token::Any] = tokens.as_slice()
            && middle.iter().all(|token| matches!(token, Token::Char(_)))
        {
            let literal = middle
                .iter()
                .map(|token| match token {
                    Token::Char(char) => *char,
                    _ => unreachable!(),
                })
                .collect();
            return Self::Contains(literal);
        }
        Self::Like(tokens)
    }

    /// Шаблон «подстрока `search`» (`%search%`, спецсимволы внутри работают как в ILIKE).
    pub(crate) fn contains(search: &str) -> Self {
        Self::new(&format!("%{search}%"))
    }

    /// `text` — уже в нижнем регистре (см. [`lowercase`]).
    pub(crate) fn matches(&self, text: &str) -> bool {
        match self {
            Self::Contains(literal) => text.contains(literal.as_str()),
            Self::Like(tokens) => {
                let text: Vec<char> = text.chars().collect();
                like(tokens, &text)
            }
        }
    }
}

pub(crate) fn lowercase(text: &str) -> String {
    text.chars().flat_map(char::to_lowercase).collect()
}

/// Классический жадный разбор с откатом к последнему `%`: O(n·m) в худшем случае.
fn like(tokens: &[Token], text: &[char]) -> bool {
    let (mut t, mut s) = (0, 0);
    let mut backtrack: Option<(usize, usize)> = None;
    while s < text.len() {
        match tokens.get(t) {
            Some(Token::Any) => {
                backtrack = Some((t, s));
                t += 1;
                continue;
            }
            Some(Token::One) => {
                t += 1;
                s += 1;
                continue;
            }
            Some(Token::Char(char)) if *char == text[s] => {
                t += 1;
                s += 1;
                continue;
            }
            _ => {}
        }
        match backtrack {
            Some((any, start)) => {
                t = any + 1;
                s = start + 1;
                backtrack = Some((any, start + 1));
            }
            None => return false,
        }
    }
    tokens[t..].iter().all(|token| *token == Token::Any)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ilike(text: &str, pattern: &str) -> bool {
        Pattern::new(pattern).matches(&lowercase(text))
    }

    #[test]
    fn substring_is_case_insensitive() {
        assert!(ilike("Algebra Notes #1", "%NOTES%"));
        assert!(!ilike("Algebra", "%physics%"));
        assert!(ilike("anything", "%%"));
        assert!(matches!(Pattern::contains("Author 1"), Pattern::Contains(_)));
    }

    #[test]
    fn wildcards_and_escapes() {
        assert!(ilike("a1b", "%a_b%"));
        assert!(!ilike("ab", "%a_b%"));
        assert!(ilike("x a%b y", "%a\\%b%"));
        assert!(!ilike("x axb y", "%a\\%b%"));
        assert!(ilike("abc", "a%c"));
        assert!(!ilike("abcd", "a%c"));
        assert!(ilike("aXbYc", "a%b%c"));
        assert!(ilike("", "%"));
        assert!(!ilike("", "_"));
    }
}
