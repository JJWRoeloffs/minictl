// use rowan::{GreenNode, GreenNodeBuilder};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    TOKEN_WHITESPACE = 0,
    TOKEN_COMMENT, // Comments with -- to newline
    TOKEN_ERROR,   // Anything that cannot be properly parsed.

    TOKEN_VARNAME, // Any nondescript variable
    TOKEN_NONE,    // none

    TOKEN_L_PAREN,   // (
    TOKEN_R_PAREN,   // )
    TOKEN_COLON,     // :
    TOKEN_SEMICOLON, // ;
    TOKEN_COMMA,     // ,
    TOKEN_DOT,       // .
    TOKEN_DOUBLEDOT, // ..

    TOKEN_AND, // and
    TOKEN_OR,  // or
    TOKEN_IF,  // if

    TOKEN_NEG,   // !
    TOKEN_PLUS,  // +
    TOKEN_MINUS, // -
    TOKEN_LE,    // <=
    TOKEN_LT,    // <
    TOKEN_GE,    // >=
    TOKEN_GT,    // >
    TOKEN_EQ,    // =
    TOKEN_NEQ,   // !=

    TOKEN_IMPLIES_R,       // ->
    TOKEN_IMPLIES_L,       // <-
    TOKEN_IMPLIES_BI,      // <->
    TOKEN_TRUE,            // true
    TOKEN_FALSE,           // false
    TOKEN_BOOLEAN_LITERAL, // boolean

    TOKEN_X, // X (Modal operator)
    TOKEN_F, // F (Modal operator)
    TOKEN_G, // G (Modal operator)
    TOKEN_U, // U (Modal operator)
    TOKEN_A, // A (Modal operator)
    TOKEN_E, // E (Modal operator)
    TOKEN_K, // K (Modal operator)

    TOKEN_GROUPNAME, // A group in Groups, or any variable previously defined as such.
    TOKEN_START_GROUPEXPR, // < of group expression, e.g. <g1>Xp
    TOKEN_END_GROUPEXPR, // > of group expression, e.g. <g1>Xp
    TOKEN_GROUP_OPENCURLY, // { of group definition. e.g. g1 = {A1, A2}
    TOKEN_GROUP_CLOSECURLY, // } of group definition. e.g. g1 = {A1, A2}
    TOKEN_GROUPS_ASSIGN, // = of group definition. e.g. g1 = {A1, A2}

    TOKEN_ENUM_LITERAL,    // Any value previously defined as part of an enum.
    TOKEN_ENUM_OPENCURLY,  // { of an enum definition, e.g. a: {b1, b2}
    TOKEN_ENUM_CLOSECURLY, // } of an enum definition, e.g. a: {b1, b2}

    TOKEN_INT_LITERAL, // any integer.

    TOKEN_SET_OPENCURLY, // { for sets (which is, anything that is not a group or enum)
    TOKEN_SET_CLOSECURLY, // { for sets (which is, anything that is not a group or enum)

    TOKEN_INNER_BLOCK_NAME,        // The name of an inner block. e.g. Vars:
    TOKEN_BEGIN_INNER_BLOCK,       // The : of an inner block
    TOKEN_END_INNER_BLOCK,         // The "end" of an inner bock
    TOKEN_END_INNER_BLOCK_NAME,    // The name after the end of the inner block
    TOKEN_INNER_BLOCK_ASSIGN,      // = in an assignment, e.g. Actions = {none}
    TOKEN_INNER_BLOCK_ASSIGN_NAME, // the name of an inner block assigned with =

    TOKEN_SEMANTICS,            // The Semantics=something; statement at the start.
    TOKEN_BEGIN_AGENT,          // The start "Agent" of an Agent block.
    TOKEN_AGENT_NAME,           // The name an agent has.
    TOKEN_BEGIN_EVALUATION,     // The start "Evaluation"
    TOKEN_BEGIN_INIT_STATES,    // The start "InitGroups"
    TOKEN_BEGIN_GROUPS,         // The start "Groups"
    TOKEN_BEGIN_FAIRNESS,       // The start "Fairness"
    TOKEN_BEGIN_FORMULAE,       // The start "Formulae"
    TOKEN_END_OUTER_BLOCK,      // The "end" of an outer block.
    TOKEN_END_OUTER_BLOCK_NAME, // The name after the end of an outer block.

    NODE_ENUM_DECL,
    NODE_BOOL_DECL,
    NODE_INT_DECL,
    NODE_GROUP_DECL,

    NODE_FORMULA,
    NODE_FORMULA_ROOT,
    NODE_EXPRESSION,
    NODE_EXPRESSION_ROOT,

    NODE_BLOCk,

    ROOT,
}
use SyntaxKind::*;

fn is_valid_start_var_char(c: &char) -> bool {
    c.is_alphabetic()
}
fn is_valid_var_char(c: &char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '$' | '^' | '@' | '#')
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang {}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context<'a> {
    AgentDef,
    AgentBlock,
    OuterBlock { name: &'a str },
    InnerBlock { name: &'a str },
    InnerBlockAssign,
    InnerBlockBegin,
    BlockEnded,
    EnumDef,
    GroupDef,
    SetDef,
    GroupExpr,
}

#[derive(Clone, Copy)]
struct State<'a> {
    input: &'a str,
    offset: usize,
}
impl State<'_> {
    fn remaining(&self) -> &str {
        &self.input[self.offset..]
    }
    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }
    fn peekchar(&self) -> Option<char> {
        self.remaining().chars().find(|c| !c.is_whitespace())
    }
    fn peek2(&self) -> (Option<char>, Option<char>) {
        (
            self.remaining().chars().next(),
            self.remaining().chars().next(),
        )
    }
    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(c) = c {
            self.offset += c.len_utf8();
        }
        c
    }
    fn starts_with_bump(&mut self, s: &str) -> bool {
        let starts_with = self.remaining().starts_with(s);
        if starts_with {
            self.offset += s.len();
        }
        starts_with
    }
    fn str_since<'a>(&self, past: State<'a>) -> &'a str {
        &past.input[past.offset..self.offset]
    }
    fn consume_while<F>(&mut self, mut f: F) -> usize
    where
        F: FnMut(char) -> bool,
    {
        let len = self
            .remaining()
            .chars()
            .take_while(|&c| f(c))
            .map(char::len_utf8)
            .sum::<usize>();
        self.offset += len;
        len
    }
    fn consume_var(&mut self) -> Option<&str> {
        if !is_valid_start_var_char(&self.remaining().chars().next()?) {
            return None;
        }
        let len = self
            .remaining()
            .chars()
            .take_while(is_valid_var_char)
            .map(char::len_utf8)
            .sum::<usize>();
        let name = &self.input[self.offset..self.offset + len];
        self.offset += len;
        Some(name)
    }
    fn consume_err(&mut self) {
        let consumed = self.consume_while(|c| c != '\n');
        if consumed == 0 {
            self.consume_while(|_| true);
        }
    }
}
impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.input, other.input) && self.offset == other.offset
    }
}
impl Eq for State<'_> {}

pub type Token<'a> = (SyntaxKind, &'a str);

pub struct Tokenizer<'a> {
    state: State<'a>,
    ctx: Vec<Context<'a>>,
    enumliterals: RefCell<Vec<&'a str>>,
    groupnames: RefCell<Vec<&'a str>>,
    agentnames: RefCell<Vec<String>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            ctx: Vec::new(),
            state: State { input, offset: 0 },
            enumliterals: Vec::new().into(),
            groupnames: Vec::new().into(),
            agentnames: Vec::new().into(),
        }
    }
}
impl Tokenizer<'_> {
    fn pop_ctx(&mut self, ctx: Context) {
        debug_assert_eq!(self.ctx.last(), Some(&ctx));
        self.ctx.pop();
    }

    fn next_inner(&mut self) -> Option<SyntaxKind> {
        let start = self.state;

        if self.state.remaining().is_empty() {
            return None;
        }

        if self.state.consume_while(char::is_whitespace) > 0 {
            return Some(TOKEN_WHITESPACE);
        }

        if self.state.starts_with_bump("--") {
            self.state.consume_while(|c| c != '\n');
            return Some(TOKEN_COMMENT);
        }

        if self.ctx.last() == Some(&Context::AgentDef) {
            self.pop_ctx(Context::AgentDef);
            self.ctx.push(Context::AgentBlock);
            match self.state.consume_var() {
                Some(name) => {
                    self.agentnames.borrow_mut().push(name.to_string());
                    return Some(TOKEN_AGENT_NAME);
                }
                None => {
                    self.state.consume_err();
                    return Some(TOKEN_ERROR);
                }
            }
        }

        if self.ctx.last() == Some(&Context::BlockEnded) {
            self.pop_ctx(Context::BlockEnded);
            match self.ctx.last().copied() {
                Some(Context::AgentBlock) => match self.state.consume_var() {
                    Some("Agent") => {
                        self.pop_ctx(Context::AgentBlock);
                        return Some(TOKEN_END_OUTER_BLOCK_NAME);
                    }
                    Some(_) => return Some(TOKEN_ERROR),
                    None => {
                        self.state.consume_err();
                        return Some(TOKEN_ERROR);
                    }
                },
                Some(Context::OuterBlock { name }) => match self.state.consume_var() {
                    Some(varname) if varname == name => {
                        self.ctx.pop();
                        return Some(TOKEN_END_OUTER_BLOCK_NAME);
                    }
                    Some(_) => return Some(TOKEN_ERROR),
                    None => {
                        self.state.consume_err();
                        return Some(TOKEN_ERROR);
                    }
                },
                Some(Context::InnerBlock { name }) => match self.state.consume_var() {
                    Some(varname) if varname == name => {
                        self.ctx.pop();
                        return Some(TOKEN_END_INNER_BLOCK_NAME);
                    }
                    Some(_) => return Some(TOKEN_ERROR),
                    None => {
                        self.state.consume_err();
                        return Some(TOKEN_ERROR);
                    }
                },
                _ => {
                    if self.state.consume_var().is_none() {
                        self.state.consume_err()
                    }
                    return Some(TOKEN_ERROR);
                }
            }
        }

        if self.ctx.is_empty() {
            match self.state.consume_var() {
                Some("Semantics") => {
                    self.state.consume_while(|c| c != ';');
                    self.next()?;
                    return Some(TOKEN_SEMANTICS);
                }
                Some("Agent") => {
                    self.ctx.push(Context::AgentDef);
                    return Some(TOKEN_BEGIN_AGENT);
                }
                Some("Evaluation") => {
                    self.ctx.push(Context::OuterBlock { name: "Evaluation" });
                    return Some(TOKEN_BEGIN_EVALUATION);
                }
                Some("InitStates") => {
                    self.ctx.push(Context::OuterBlock { name: "InitStates" });
                    return Some(TOKEN_BEGIN_INIT_STATES);
                }
                Some("Groups") => {
                    self.ctx.push(Context::OuterBlock { name: "Groups" });
                    return Some(TOKEN_BEGIN_GROUPS);
                }
                Some("Fairness") => {
                    self.ctx.push(Context::OuterBlock { name: "Fairness" });
                    return Some(TOKEN_BEGIN_FAIRNESS);
                }
                Some("Formulae") => {
                    self.ctx.push(Context::OuterBlock { name: "Formulae" });
                    return Some(TOKEN_BEGIN_FORMULAE);
                }
                _ => {
                    self.state.consume_err();
                    return Some(TOKEN_ERROR);
                }
            }
        }

        Some(match self.state.next()? {
            '.' if self.state.peek() == Some('.') => {
                self.next().unwrap();
                TOKEN_DOUBLEDOT
            }
            '.' => TOKEN_DOT,
            '(' => TOKEN_L_PAREN,
            ')' => TOKEN_R_PAREN,
            '{' => match self.ctx.last() {
                Some(&Context::OuterBlock { name: "Groups" }) => {
                    self.ctx.push(Context::GroupDef);
                    TOKEN_GROUP_OPENCURLY
                }
                Some(&Context::InnerBlock { name: _ }) => {
                    self.ctx.push(Context::EnumDef);
                    TOKEN_ENUM_OPENCURLY
                }
                _ => {
                    self.ctx.push(Context::SetDef);
                    TOKEN_SET_OPENCURLY
                }
            },
            '}' => match self.ctx.last() {
                Some(Context::GroupDef) => {
                    self.pop_ctx(Context::GroupDef);
                    TOKEN_GROUP_CLOSECURLY
                }
                Some(Context::EnumDef) => {
                    self.pop_ctx(Context::EnumDef);
                    TOKEN_ENUM_CLOSECURLY
                }
                Some(Context::SetDef) => {
                    self.pop_ctx(Context::SetDef);
                    TOKEN_SET_CLOSECURLY
                }
                _ => TOKEN_ERROR,
            },
            ':' if self.ctx.last() == Some(&Context::InnerBlockBegin) => {
                self.pop_ctx(Context::InnerBlockBegin);
                TOKEN_BEGIN_INNER_BLOCK
            }
            ':' => TOKEN_COLON,
            ';' => TOKEN_SEMICOLON,
            ',' => TOKEN_COMMA,
            '!' if self.state.peek() == Some('=') => {
                self.next().unwrap();
                TOKEN_NEQ
            }
            '!' => TOKEN_NEG,
            '<' if self.state.peek2() == (Some('-'), Some('>')) => {
                self.next().unwrap();
                self.next().unwrap();
                TOKEN_IMPLIES_BI
            }
            '<' if self.state.peek() == Some('-') => {
                self.next().unwrap();
                TOKEN_IMPLIES_L
            }
            '<' if self.state.peek() == Some('=') => {
                self.next().unwrap();
                TOKEN_LE
            }
            '>' if self.state.peek() == Some('=') => {
                self.next().unwrap();
                TOKEN_GE
            }
            '-' if self.state.peek() == Some('>') => {
                self.next().unwrap();
                TOKEN_IMPLIES_R
            }
            '-' => TOKEN_MINUS,
            '+' => TOKEN_PLUS,
            '=' if self.ctx.last() == Some(&Context::OuterBlock { name: "Groups" }) => {
                TOKEN_GROUPS_ASSIGN
            }
            '=' if self.ctx.last() == Some(&Context::InnerBlockAssign) => {
                self.pop_ctx(Context::InnerBlockAssign);
                TOKEN_INNER_BLOCK_ASSIGN
            }
            '=' => TOKEN_EQ,
            '>' if self.ctx.last() == Some(&Context::GroupExpr) => {
                self.pop_ctx(Context::GroupExpr);
                TOKEN_END_GROUPEXPR
            }
            '>' => TOKEN_GT,
            // This character is the reason why all this is so hard.
            '<' => {
                let next_item_start_offset = self
                    .state
                    .remaining()
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                let next_item_start = self.state.offset + next_item_start_offset;
                let next_item_valid_char = is_valid_start_var_char(
                    &self
                        .state
                        .remaining()
                        .chars()
                        .skip(next_item_start_offset)
                        .next()
                        .unwrap_or('0'), // this depends on the fact that 0 is not a valid char.
                );
                let next_item_len = self
                    .state
                    .remaining()
                    .chars()
                    .skip(next_item_start_offset)
                    .take_while(is_valid_var_char)
                    .map(|c| c.len_utf8())
                    .sum::<usize>();
                if !next_item_valid_char
                    || !self.groupnames.borrow().contains(
                        &&self.state.input[next_item_start..next_item_start + next_item_len],
                    )
                {
                    TOKEN_LT
                } else {
                    self.ctx.push(Context::GroupExpr);
                    TOKEN_START_GROUPEXPR
                }
            }
            '0'..='9' => {
                self.state.consume_while(|c| c.is_ascii_digit());
                TOKEN_INT_LITERAL
            }
            'X' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_X,
            'F' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_F,
            'G' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_G,
            'U' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_U,
            'A' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_A,
            'E' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_E,
            'K' if self.ctx.last() == Some(&Context::OuterBlock { name: "Formulae" }) => TOKEN_K,
            character if is_valid_start_var_char(&character) => {
                self.state.consume_while(|c| is_valid_var_char(&c));
                match self.state.str_since(start) {
                    "end" => match self.ctx.last() {
                        Some(Context::AgentBlock) => {
                            self.ctx.push(Context::BlockEnded);
                            TOKEN_END_OUTER_BLOCK
                        }
                        Some(Context::OuterBlock { name: _ }) => {
                            self.ctx.push(Context::BlockEnded);
                            TOKEN_END_OUTER_BLOCK
                        }
                        Some(Context::InnerBlock { name: _ }) => {
                            self.ctx.push(Context::BlockEnded);
                            TOKEN_END_INNER_BLOCK
                        }
                        _ => TOKEN_ERROR,
                    },
                    "none" => TOKEN_NONE,
                    "and" => TOKEN_AND,
                    "or" => TOKEN_OR,
                    "if" => TOKEN_IF,
                    "true" => TOKEN_TRUE,
                    "false" => TOKEN_FALSE,
                    "boolean" => TOKEN_BOOLEAN_LITERAL,
                    var => match self.ctx.last() {
                        Some(&Context::EnumDef) => {
                            self.enumliterals.borrow_mut().push(var);
                            TOKEN_ENUM_LITERAL
                        }
                        Some(&Context::OuterBlock { name: "Groups" }) => {
                            self.groupnames.borrow_mut().push(var);
                            TOKEN_GROUPNAME
                        }
                        Some(&Context::GroupDef) => {
                            if self.agentnames.borrow().iter().any(|i| i == var) {
                                TOKEN_AGENT_NAME
                            } else {
                                TOKEN_ERROR
                            }
                        }
                        Some(&Context::GroupExpr) => {
                            if self.groupnames.borrow().contains(&var) {
                                TOKEN_GROUPNAME
                            } else {
                                TOKEN_ERROR
                            }
                        }
                        Some(&Context::AgentBlock) if self.state.peekchar() == Some('=') => {
                            self.ctx.push(Context::InnerBlockAssign);
                            TOKEN_INNER_BLOCK_ASSIGN_NAME
                        }
                        Some(&Context::AgentBlock) if self.state.peekchar() == Some(':') => {
                            self.ctx.push(Context::InnerBlock { name: var });
                            self.ctx.push(Context::InnerBlockBegin);
                            TOKEN_INNER_BLOCK_NAME
                        }
                        Some(&Context::OuterBlock { name: _ })
                            if self.state.peekchar() == Some(':') =>
                        {
                            self.ctx.push(Context::InnerBlock { name: var });
                            self.ctx.push(Context::InnerBlockBegin);
                            TOKEN_INNER_BLOCK_NAME
                        }
                        _ => {
                            if self.enumliterals.borrow().contains(&var) {
                                TOKEN_ENUM_LITERAL
                            } else if self.agentnames.borrow().iter().any(|i| i == var) {
                                TOKEN_AGENT_NAME
                            } else {
                                TOKEN_VARNAME
                            }
                        }
                    },
                }
            }
            _ => TOKEN_ERROR,
        })
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let start = self.state;
        self.next_inner()
            .map(|syntax_kind| (syntax_kind, self.state.str_since(start)))
    }
}

/// A convenience function for tokenizing the given input
pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    Tokenizer::new(input).collect()
}
