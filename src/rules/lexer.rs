#[derive(Debug)]
pub enum Symbol {
    LeftCurly,
    RightCurly,
    Comma,
    Colon,
    Arrow,
    SemiColon,
}

impl PartialEq<Self> for Symbol {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Symbol::LeftCurly, Symbol::LeftCurly) => true,
            (Symbol::RightCurly, Symbol::RightCurly) => true,
            (Symbol::Comma, Symbol::Comma) => true,
            (Symbol::Colon, Symbol::Colon) => true,
            (Symbol::Arrow, Symbol::Arrow) => true,
            (Symbol::SemiColon, Symbol::SemiColon) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Symbol(Symbol),
    String(String),
    Space,
}

impl PartialEq<Self> for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TokenKind::Identifier(s1), TokenKind::Identifier(s2)) => s1 == s2,
            (TokenKind::Symbol(s1), TokenKind::Symbol(s2)) => s1 == s2,
            (TokenKind::String(s1), TokenKind::String(s2)) => s1 == s2,
            (TokenKind::Space, TokenKind::Space) => true,
            _ => false,
        }
    }
}

impl Eq for TokenKind {}

pub fn tokenize(data: &str) -> Vec<TokenKind> {
    let mut stream = data.chars().peekable();
    let mut tokens = Vec::new();
    while let Some(c1) = stream.next() {
        // identifiers
        if c1.is_alphanumeric() {
            let mut identifier = String::new();
            identifier.push(c1);
            while let Some(c2) = stream.peek() {
                if c2.is_alphanumeric() {
                    identifier.push(*c2);
                    stream.next();
                } else {
                    break;
                }
            }
            tokens.push(TokenKind::Identifier(identifier));
            continue;
        }

        // symbols
        match c1 {
            '{' => {
                tokens.push(TokenKind::Symbol(Symbol::LeftCurly));
                continue;
            }
            '}' => {
                tokens.push(TokenKind::Symbol(Symbol::RightCurly));
                continue;
            }
            ',' => {
                tokens.push(TokenKind::Symbol(Symbol::Comma));
                continue;
            }
            ':' => {
                tokens.push(TokenKind::Symbol(Symbol::Colon));
                continue;
            }
            ';' => {
                tokens.push(TokenKind::Symbol(Symbol::SemiColon));
                continue;
            }
            '=' => {
                if let Some(c2) = stream.peek() {
                    if c2.eq(&'>') {
                        stream.next();
                        tokens.push(TokenKind::Symbol(Symbol::Arrow));
                        continue;
                    }
                } else {
                    panic!("Unexpected token: {}", c1)
                }
            }
            _ => {}
        }

        // spaces
        if c1.is_whitespace() || c1 == '\n' || c1 == '\r' {
            while let Some(c2) = stream.peek() {
                if c2.is_whitespace() || c2 == &'\n' || c2 == &'\r' {
                    stream.next();
                } else {
                    break;
                }
            }
            tokens.push(TokenKind::Space);
            continue;
        }

        // strings
        if c1.eq(&'\'') {
            let mut string = String::new();
            while let Some(c2) = stream.peek() {
                if c2 == &'\'' {
                    stream.next();
                    break;
                } else {
                    string.push(*c2);
                    stream.next();
                }
            }
            tokens.push(TokenKind::String(string));
            continue;
        }

        // TODO: handle errors
        panic!("Unexpected token: {}", c1)
    }
    tokens
}

#[cfg(test)]
mod test {
    macro_rules! token {
        ($kind:ident) => {
            super::TokenKind::$kind
        };
        (Identifier, $value:expr) => {
            super::TokenKind::Identifier($value.to_string())
        };
        (String, $value:expr) => {
            super::TokenKind::String($value.to_string())
        };
        (Symbol, $value:ident) => {
            super::TokenKind::Symbol(super::Symbol::$value)
        };
    }

    #[test]
    fn test_rule_from_src() {
        let code = r#"
            EXT {
                pdf,doc,docx => 'Documents';
                mp3,mp4 => 'Music';
            }
        "#;
        let tokens = super::tokenize(code);
        assert_eq!(tokens, vec![
            token!(Space),
            token!(Identifier, "EXT"),
            token!(Space),
            token!(Symbol, LeftCurly),
            token!(Space),
            token!(Identifier, "pdf"),
            token!(Symbol, Comma),
            token!(Identifier, "doc"),
            token!(Symbol, Comma),
            token!(Identifier, "docx"),
            token!(Space),
            token!(Symbol, Arrow),
            token!(Space),
            token!(String, "Documents"),
            token!(Symbol, SemiColon),
            token!(Space),
            token!(Identifier, "mp3"),
            token!(Symbol, Comma),
            token!(Identifier, "mp4"),
            token!(Space),
            token!(Symbol, Arrow),
            token!(Space),
            token!(String, "Music"),
            token!(Symbol, SemiColon),
            token!(Space),
            token!(Symbol, RightCurly),
            token!(Space),
        ]);
    }
}
