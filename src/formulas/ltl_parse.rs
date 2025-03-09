use std::iter::Peekable;
use std::str::Chars;
use std::sync::Arc;
use thiserror::Error;

use super::ltl_types::{LTLFormula, LTLVariable};

#[derive(Debug, Clone, PartialEq)]
enum LTLToken {
    Top,
    Bot,
    Variable(String),
    Not,
    And,
    Or,
    ImpliesR,
    ImpliesL,
    BiImplies,
    X,
    F,
    G,
    U,
    W,
    R,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Error)]
pub enum LTLParseError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unknown keyword: {0}")]
    UnknownKeyword(String),
    #[error("Unexpected Token: {0}")]
    UnexpectedToken(String),
    #[error("Unexpected end of input: {0}")]
    UnexpectedEndOfInput(String),
}

struct LTLLexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> LTLLexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn expect_char(&mut self, expected: char, err_str: &str) -> Result<(), LTLParseError> {
        match self.chars.next() {
            Some(a) if a == expected => Ok(()),
            Some(other) => Err(LTLParseError::UnexpectedCharacter(other)),
            None => {
                let err_message = format!("{}, got end of input", err_str);
                Err(LTLParseError::UnexpectedEndOfInput(err_message))
            }
        }
    }

    fn next_token(&mut self) -> Option<Result<LTLToken, LTLParseError>> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ch if ch.is_whitespace() => {
                    self.chars.next();
                }
                '(' => {
                    self.chars.next();
                    return Some(Ok(LTLToken::LParen));
                }
                ')' => {
                    self.chars.next();
                    return Some(Ok(LTLToken::RParen));
                }
                '!' => {
                    self.chars.next();
                    return Some(Ok(LTLToken::Not));
                }
                '-' => {
                    self.chars.next();
                    return Some(
                        self.expect_char('>', "Expected > after - that didn't start with <")
                            .map(|_| LTLToken::ImpliesR),
                    );
                }
                '<' => {
                    self.chars.next();
                    // I thought I wasn't programming in Go... ???
                    if let Err(error) = self.expect_char('-', "Expected - after <") {
                        return Some(Err(error));
                    }
                    if self.chars.peek() == Some(&'>') {
                        self.chars.next();
                        return Some(Ok(LTLToken::BiImplies));
                    }
                    return Some(Ok(LTLToken::ImpliesL));
                }
                ch if ch.is_lowercase() => return self.consume_lowercase_variable_or_keyword(),
                ch if ch.is_uppercase() => return self.consume_keyword(),
                _ => return Some(Err(LTLParseError::UnexpectedCharacter(ch))),
            }
        }
        None
    }

    fn consume_lowercase_variable_or_keyword(&mut self) -> Option<Result<LTLToken, LTLParseError>> {
        let mut name = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_lowercase() || ch.is_numeric() {
                name.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        match name.as_str() {
            "and" => Some(Ok(LTLToken::And)),
            "or" => Some(Ok(LTLToken::Or)),
            _ => Some(Ok(LTLToken::Variable(name))),
        }
    }

    fn consume_keyword(&mut self) -> Option<Result<LTLToken, LTLParseError>> {
        use LTLToken as T;
        let mut name = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_uppercase() {
                name.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        match name.as_str() {
            "TOP" => Some(Ok(T::Top)),
            "BOT" => Some(Ok(T::Bot)),
            "X" => Some(Ok(T::X)),
            "F" => Some(Ok(T::F)),
            "G" => Some(Ok(T::G)),
            "U" => Some(Ok(T::U)),
            "W" => Some(Ok(T::W)),
            "R" => Some(Ok(T::R)),
            _ => Some(Err(LTLParseError::UnknownKeyword(name))),
        }
    }
}

impl Iterator for LTLLexer<'_> {
    type Item = Result<LTLToken, LTLParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

struct LTLParser<'a> {
    tokens: Peekable<LTLLexer<'a>>,
}

impl<'a> LTLParser<'a> {
    fn new(lexer: LTLLexer<'a>) -> Self {
        Self {
            tokens: lexer.peekable(),
        }
    }
    fn expect_token(&mut self, expected: LTLToken, err_str: &str) -> Result<(), LTLParseError> {
        match self.tokens.next() {
            Some(Ok(a)) if a == expected => Ok(()),
            Some(other) => {
                let err_message = format!("{}, got {:?}", err_str, other);
                Err(LTLParseError::UnexpectedToken(err_message))
            }
            None => {
                let err_message = format!("{}, got end of input", err_str);
                Err(LTLParseError::UnexpectedEndOfInput(err_message))
            }
        }
    }
    fn parse(&mut self) -> Result<Arc<LTLFormula>, LTLParseError> {
        self.parse_expression(1)
    }
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Arc<LTLFormula>, LTLParseError> {
        use LTLFormula as F;
        use LTLToken as T;
        let mut left = self.parse_primary()?;

        while let Some(Ok(token)) = self.tokens.peek() {
            let token_precedence = match token {
                T::ImpliesR | T::ImpliesL | T::BiImplies => 1,
                T::Or => 2,
                T::And => 3,
                T::U | T::W | T::R => 4,
                _ => break,
            };

            if token_precedence < min_precedence {
                break;
            }

            let token = self.tokens.next().unwrap()?;
            let right = self.parse_expression(token_precedence + 1)?;

            left = Arc::new(match token {
                T::ImpliesR => F::ImpliesR(left, right),
                T::ImpliesL => F::ImpliesL(left, right),
                T::BiImplies => F::BiImplies(left, right),
                T::Or => F::Or(left, right),
                T::And => F::And(left, right),
                T::U => F::U(left, right),
                T::W => F::W(left, right),
                T::R => F::R(left, right),
                _ => unreachable!(),
            });
        }

        Ok(left)
    }
    fn parse_primary(&mut self) -> Result<Arc<LTLFormula>, LTLParseError> {
        use LTLFormula as F;
        use LTLToken as T;
        match self.tokens.next() {
            Some(Ok(T::Top)) => Ok(Arc::new(F::Top)),
            Some(Ok(T::Bot)) => Ok(Arc::new(F::Bot)),
            Some(Ok(T::Variable(var))) => Ok(Arc::new(F::Atomic(LTLVariable::new(var)))),
            Some(Ok(T::Not)) => Ok(Arc::new(F::Neg(self.parse_primary()?))),
            Some(Ok(T::X)) => Ok(Arc::new(F::X(self.parse_primary()?))),
            Some(Ok(T::F)) => Ok(Arc::new(F::F(self.parse_primary()?))),
            Some(Ok(T::G)) => Ok(Arc::new(F::G(self.parse_primary()?))),
            Some(Ok(T::LParen)) => {
                let expr = self.parse_expression(1)?;
                self.expect_token(T::RParen, "Expected closing parentheses")?;
                Ok(expr)
            }
            Some(Ok(other)) => Err(LTLParseError::UnexpectedToken(format!("{:?}", other))),
            Some(Err(error)) => Err(error),
            None => Err(LTLParseError::UnexpectedEndOfInput(
                "Expected primary expression".to_owned(),
            )),
        }
    }
}

#[inline(always)]
pub fn parse_ltl(input: &str) -> Result<Arc<LTLFormula>, LTLParseError> {
    let lexer = LTLLexer::new(input);
    let mut parser = LTLParser::new(lexer);
    parser.parse()
}
