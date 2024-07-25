#![allow(unused)]

use std::{collections::VecDeque, fmt};

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, TextRange, TextSize};

use crate::ispl::{
    SyntaxKind::{self, *},
    Token, TokenSet,
};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    /// Unexpected is used when the cause cannot be specified further
    Unexpected(TextRange),
    /// UnexpectedTopLevel is used when there are additional tokens to the root in the tree
    UnexpectedTopLevel(TextRange),
    /// UnexpectedBlock is used when a block (with `BLOCKNAME:` or Agent BLOCKNAME) is started
    /// with a BLOCKNAME not allowed in this context.
    UnexpectedBlock(TextRange),
    /// UnexpectedWanted is used when specific tokens are expected, but different one is found
    UnexpectedWanted(SyntaxKind, TextRange, Box<[SyntaxKind]>),
    /// UnexpectedEnding is used when the wrong block type follows the `end` keyword.
    UnexpectedEnding(TextRange),
    /// UnexpectedEOF is used when the end of file is reached, while tokens are still expected
    UnexpectedEOF,
    /// UnexpectedEOFWanted is used when specific tokens are expected, but the end of file is reached
    UnexpectedEOFWanted(Box<[SyntaxKind]>),
    /// DuplicatedNames is used when enum names are duplicated, e.g. `{ a, a }`
    DuplicatedNames(TextRange, String),

    /// RecursionLimitExceeded is used when we're unable to parse further due to likely being close to
    /// a stack overflow.
    RecursionLimitExceeded,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Unexpected(range) => {
                write!(
                    f,
                    "error node at {}..{}",
                    usize::from(range.start()),
                    usize::from(range.end())
                )
            }
            ParseError::UnexpectedTopLevel(range) => {
                write!(
                    f,
                    "unexpected top-level token at {}..{}",
                    usize::from(range.start()),
                    usize::from(range.end())
                )
            }
            ParseError::UnexpectedBlock(range) => {
                write!(
                    f,
                    "unexpected block {}..{}",
                    usize::from(range.start()),
                    usize::from(range.end())
                )
            }
            ParseError::UnexpectedWanted(got, range, kinds) => write!(
                f,
                "unexpected {:?} at {}..{}, wanted any of {:?}",
                got,
                usize::from(range.start()),
                usize::from(range.end()),
                kinds
            ),
            ParseError::UnexpectedEnding(range) => {
                write!(
                    f,
                    "unexpected end of block type {}..{}",
                    usize::from(range.start()),
                    usize::from(range.end())
                )
            }
            ParseError::UnexpectedEOF => write!(f, "unexpected end of file"),
            ParseError::UnexpectedEOFWanted(kinds) => {
                write!(f, "unexpected end of file, wanted any of {:?}", kinds)
            }
            ParseError::DuplicatedNames(range, ident) => {
                write!(
                    f,
                    "value `{}` is duplicated in {}..{}",
                    ident,
                    usize::from(range.start()),
                    usize::from(range.end())
                )
            }
            ParseError::RecursionLimitExceeded => write!(f, "recursion limit exceeded"),
        }
    }
}

impl std::error::Error for ParseError {}
