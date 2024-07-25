#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    TOKEN_WHITESPACE = 0,
    TOKEN_COMMENT, // Comments with -- to newline

    TOKEN_ERR_UNEXPECTED_KEYWORD, // A keyword in a place there shouldn't be one.
    TOKEN_ERR_EXPECTED_NAME, // A place where a name (like that of an agent or variable), was expected
    TOKEN_ERR_INVALID_CLOSE_BLOCK, // An end that doesn't end what it should.
    TOKEN_ERR_UNEXPECTED_TOPLEVEL, // Something at toplevel that shouldn't be there.
    TOKEN_ERR_UNEXPECTED_BLOCK, // Something inner block that shouldn't be there.
    TOKEN_ERR_UNDEFINED_NAME, // Something at toplevel that shouldn't be there.
    TOKEN_ERROR,             // Anything else that cannot be properly parsed.

    TOKEN_VARNAME, // Any nondescript variable

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

    TOKEN_NEG,    // !
    TOKEN_LE,     // <=
    TOKEN_LT,     // <
    TOKEN_GE,     // >=
    TOKEN_GT,     // >
    TOKEN_EQ,     // =
    TOKEN_NEQ,    // <>
    TOKEN_PLUS,   // +
    TOKEN_MINUS,  // -
    TOKEN_TIMES,  // *
    TOKEN_DEVIDE, // /

    TOKEN_IMPLIES_R,       // ->
    TOKEN_IMPLIES_L,       // <-
    TOKEN_IMPLIES_BI,      // <->
    TOKEN_BITAND,          // &
    TOKEN_BITOR,           // |
    TOKEN_BITNOT,          // ~
    TOKEN_BITXOR,          // ^
    TOKEN_TRUE,            // true
    TOKEN_FALSE,           // false
    TOKEN_BOOLEAN_LITERAL, // boolean

    TOKEN_AG,  // AG (Modal Operator)
    TOKEN_EG,  // EG (Modal Operator)
    TOKEN_AX,  // AX (Modal Operator)
    TOKEN_EX,  // EX (Modal Operator)
    TOKEN_X,   // X (Modal Operator)
    TOKEN_F,   // F (Modal Operator)
    TOKEN_G,   // G (Modal Operator)
    TOKEN_AF,  // AF (Modal Operator)
    TOKEN_EF,  // EF (Modal Operator)
    TOKEN_A,   // A (Modal Operator)
    TOKEN_E,   // E (Modal Operator)
    TOKEN_U,   // UNTIL (Modal Operator)
    TOKEN_K,   // K (Modal Operator)
    TOKEN_GK,  // GK (Modal Operator)
    TOKEN_GCK, // GCK (Modal Operator)
    TOKEN_O,   // O (Modal Operator)
    TOKEN_DK,  // DK (Modal Operator)

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

    TOKEN_ENVIRONMENT, // Reserved agent name for an environment.
    TOKEN_OTHER,       // The Other: statement in blocks
    TOKEN_ACTION,      // The Action statement in blocks
    TOKEN_NONE,        // The none statement in blocks.

    TOKEN_REDSTATES,            // RedStates (Inner block name)
    TOKEN_GREENSTATES,          // GreenStates (Inner block name)
    TOKEN_ACTIONS,              // Actions (Inner block name)
    TOKEN_PROTOCOL,             // Protocol (Inner block name)
    TOKEN_EVOLUTION,            // Evolution (Inner block name)
    TOKEN_OBSVARS,              // Obsvars (Inner block name)
    TOKEN_LOBSVARS,             // Lobsvars (Inner block name)
    TOKEN_VARS,                 // Vars (Inner block name)
    TOKEN_BEGIN_INNER_BLOCK,    // The : of an inner block
    TOKEN_END_INNER_BLOCK,      // The "end" of an inner bock
    TOKEN_END_INNER_BLOCK_NAME, // The name after the end of the inner block
    TOKEN_INNER_BLOCK_ASSIGN,   // = in an assignment, e.g. Actions = {none}

    TOKEN_SEMANTICS_SA,         // Semantics=SingleAssignment;
    TOKEN_SEMANTICS_MA,         // Semantics=MultiAssignment;
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

    ROOT, // Note: the root has to always be the last one. The code depends on this.
}
pub use SyntaxKind::*;

impl SyntaxKind {
    /// Returns whether or not the token is an error
    pub fn is_err(&self) -> bool {
        matches!(
            self,
            TOKEN_ERR_UNEXPECTED_KEYWORD
                | TOKEN_ERR_EXPECTED_NAME
                | TOKEN_ERR_INVALID_CLOSE_BLOCK
                | TOKEN_ERR_UNEXPECTED_TOPLEVEL
                | TOKEN_ERR_UNEXPECTED_BLOCK
                | TOKEN_ERR_UNDEFINED_NAME
                | TOKEN_ERROR
        )
    }
    /// Returns if the token is an error, whitespace or comment.
    /// This is to say, returns if this token should be skipped over by the parser.
    pub fn is_trivia(self) -> bool {
        matches!(self, TOKEN_COMMENT | TOKEN_WHITESPACE) || self.is_err()
    }
}
