use std::iter::Peekable;
use std::sync::Arc;
use std::str::Chars;
use thiserror::Error;

use super::ctl_types::{CTLFormula, CTLVariable};

#[derive(Debug, Clone, PartialEq)]
enum CTLToken {
    Top,
    Bot,
    Variable(String),
    Not,
    And,
    Or,
    ImpliesR,
    ImpliesL,
    BiImplies,
    EX,
    AX,
    EF,
    AF,
    EG,
    AG,
    E,
    A,
    U,
    LParen,
    RParen,
    LSquare,
    RSquare,
}

#[derive(Debug, PartialEq, Error)]
pub enum CTLParseError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unknown keyword: {0}")]
    UnknownKeyword(String),
    #[error("Unexpected Token: {0}")]
    UnexpectedToken(String),
    #[error("Unexpected end of input: {0}")]
    UnexpectedEndOfInput(String),
}

struct CTLLexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CTLLexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn expect_char(&mut self, expected: char, err_str: &str) -> Result<(), CTLParseError> {
        match self.chars.next() {
            Some(a) if a == expected => Ok(()),
            Some(other) => Err(CTLParseError::UnexpectedCharacter(other)),
            None => {
                let err_message = format!("{}, got end of input", err_str);
                Err(CTLParseError::UnexpectedEndOfInput(err_message))
            }
        }
    }

    fn next_token(&mut self) -> Option<Result<CTLToken, CTLParseError>> {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ch if ch.is_whitespace() => {
                    self.chars.next();
                }
                '(' => {
                    self.chars.next();
                    return Some(Ok(CTLToken::LParen));
                }
                ')' => {
                    self.chars.next();
                    return Some(Ok(CTLToken::RParen));
                }
                '[' => {
                    self.chars.next();
                    return Some(Ok(CTLToken::LSquare));
                }
                ']' => {
                    self.chars.next();
                    return Some(Ok(CTLToken::RSquare));
                }
                '!' => {
                    self.chars.next();
                    return Some(Ok(CTLToken::Not));
                }
                '-' => {
                    self.chars.next();
                    return Some(
                        self.expect_char('>', "Expected > after - that didn't start with <")
                            .map(|_| CTLToken::ImpliesR),
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
                        return Some(Ok(CTLToken::BiImplies));
                    }
                    return Some(Ok(CTLToken::ImpliesL));
                }
                ch if ch.is_lowercase() => return self.consume_lowercase_variable_or_keyword(),
                ch if ch.is_uppercase() => return self.consume_keyword(),
                _ => return Some(Err(CTLParseError::UnexpectedCharacter(ch))),
            }
        }
        None
    }

    fn consume_lowercase_variable_or_keyword(&mut self) -> Option<Result<CTLToken, CTLParseError>> {
        let mut name = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch.is_lowercase() {
                name.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        match name.as_str() {
            "and" => Some(Ok(CTLToken::And)),
            "or" => Some(Ok(CTLToken::Or)),
            _ => Some(Ok(CTLToken::Variable(name))),
        }
    }

    fn consume_keyword(&mut self) -> Option<Result<CTLToken, CTLParseError>> {
        use CTLToken as T;
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
            "EX" => Some(Ok(T::EX)),
            "AX" => Some(Ok(T::AX)),
            "EF" => Some(Ok(T::EF)),
            "AF" => Some(Ok(T::AF)),
            "EG" => Some(Ok(T::EG)),
            "AG" => Some(Ok(T::AG)),
            "E" => Some(Ok(T::E)),
            "A" => Some(Ok(T::A)),
            "U" => Some(Ok(T::U)),
            _ => Some(Err(CTLParseError::UnknownKeyword(name))),
        }
    }
}

impl Iterator for CTLLexer<'_> {
    type Item = Result<CTLToken, CTLParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

struct CTLParser<'a> {
    tokens: Peekable<CTLLexer<'a>>,
}
impl<'a> CTLParser<'a> {
    fn new(lexer: CTLLexer<'a>) -> Self {
        Self {
            tokens: lexer.peekable(),
        }
    }
    fn expect_token(&mut self, expected: CTLToken, err_str: &str) -> Result<(), CTLParseError> {
        match self.tokens.next() {
            Some(Ok(a)) if a == expected => Ok(()),
            Some(other) => {
                let err_message = format!("{}, got {:?}", err_str, other);
                Err(CTLParseError::UnexpectedToken(err_message))
            }
            None => {
                let err_message = format!("{}, got end of input", err_str);
                Err(CTLParseError::UnexpectedEndOfInput(err_message))
            }
        }
    }
    fn parse(&mut self) -> Result<Arc<CTLFormula>, CTLParseError> {
        self.parse_expression(1)
    }
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Arc<CTLFormula>, CTLParseError> {
        use CTLFormula as F;
        use CTLToken as T;
        let mut left = self.parse_primary()?;

        while let Some(Ok(token)) = self.tokens.peek() {
            let token_precedence = match token {
                T::ImpliesR | T::ImpliesL | T::BiImplies => 1,
                T::Or => 2,
                T::And => 3,
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
                _ => unreachable!(),
            });
        }

        Ok(left)
    }
    fn parse_primary(&mut self) -> Result<Arc<CTLFormula>, CTLParseError> {
        use CTLFormula as F;
        use CTLToken as T;
        match self.tokens.next() {
            Some(Ok(T::Top)) => Ok(Arc::new(F::Top)),
            Some(Ok(T::Bot)) => Ok(Arc::new(F::Bot)),
            Some(Ok(T::Variable(var))) => Ok(Arc::new(F::Atomic(CTLVariable::new(var)))),
            Some(Ok(T::Not)) => Ok(Arc::new(F::Neg(self.parse_primary()?))),
            Some(Ok(T::EX)) => Ok(Arc::new(F::EX(self.parse_primary()?))),
            Some(Ok(T::AX)) => Ok(Arc::new(F::AX(self.parse_primary()?))),
            Some(Ok(T::EF)) => Ok(Arc::new(F::EF(self.parse_primary()?))),
            Some(Ok(T::AF)) => Ok(Arc::new(F::AF(self.parse_primary()?))),
            Some(Ok(T::EG)) => Ok(Arc::new(F::EG(self.parse_primary()?))),
            Some(Ok(T::AG)) => Ok(Arc::new(F::AG(self.parse_primary()?))),
            Some(Ok(T::LParen)) => {
                let expr = self.parse_expression(1)?;
                self.expect_token(T::RParen, "Expected closing parentheses")?;
                Ok(expr)
            }
            Some(Ok(T::E)) => {
                self.expect_token(T::LSquare, "Expected [ after E for E[pUq] construction")?;
                let left = self.parse_primary()?;
                self.expect_token(T::U, "Expected U after E[ for E[pUq] construction")?;
                let right = self.parse_primary()?;
                self.expect_token(T::RSquare, "Expected ] after E[.U. for E[pUq] construction")?;
                Ok(Arc::new(F::EU(left, right)))
            }
            Some(Ok(T::A)) => {
                self.expect_token(T::LSquare, "Expected [ after A for A[pUq] construction")?;
                let left = self.parse_primary()?;
                self.expect_token(T::U, "Expected U after A[ for A[pUq] construction")?;
                let right = self.parse_primary()?;
                self.expect_token(T::RSquare, "Expected ] after A[.U. for A[pUq] construction")?;
                Ok(Arc::new(F::AU(left, right)))
            }
            Some(Ok(other)) => Err(CTLParseError::UnexpectedToken(format!("{:?}", other))),
            Some(Err(error)) => Err(error),
            None => Err(CTLParseError::UnexpectedEndOfInput(
                "Expected primary expression".to_owned(),
            )),
        }
    }
}

#[inline(always)]
pub fn parse_ctl(input: &str) -> Result<Arc<CTLFormula>, CTLParseError> {
    let lexer = CTLLexer::new(input);
    let mut parser = CTLParser::new(lexer);
    parser.parse()
}
