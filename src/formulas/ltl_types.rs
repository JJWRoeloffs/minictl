// Allowing unused while LTLChecker isn't implemented.
#![allow(unused)]

use super::MLVariable;
use std::collections::HashMap;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct LTLVariable {
    pub inner: String,
}
impl LTLVariable {
    pub(crate) fn new(inner: String) -> Self {
        Self { inner }
    }
}
impl MLVariable for LTLFormula {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum LTLFormula {
    Top,
    Bot,
    Atomic(LTLVariable),
    Neg(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Or(Box<LTLFormula>, Box<LTLFormula>),
    ImpliesR(Box<LTLFormula>, Box<LTLFormula>),
    ImpliesL(Box<LTLFormula>, Box<LTLFormula>),
    BiImplies(Box<LTLFormula>, Box<LTLFormula>),
    X(Box<LTLFormula>),
    F(Box<LTLFormula>),
    G(Box<LTLFormula>),
    U(Box<LTLFormula>, Box<LTLFormula>),
    W(Box<LTLFormula>, Box<LTLFormula>),
    R(Box<LTLFormula>, Box<LTLFormula>),
}

impl LTLFormula {
    pub(crate) fn for_each_child(&self, mut f: impl FnMut(&LTLFormula)) {
        use LTLFormula as F;
        match self {
            F::X(x) | F::F(x) | F::G(x) | F::Neg(x) => f(x),
            F::And(l, r)
            | F::Or(l, r)
            | F::ImpliesR(l, r)
            | F::ImpliesL(l, r)
            | F::BiImplies(l, r)
            | F::U(l, r)
            | F::W(l, r)
            | F::R(l, r) => {
                f(l);
                f(r);
            }
            F::Top | F::Bot | F::Atomic(_) => {}
        }
    }
    pub fn total_size(&self) -> usize {
        let mut sum = 1;
        self.for_each_child(|child| {
            sum += child.total_size();
        });
        sum
    }
}

pub(crate) mod ltl_formula_macros {
    #![allow(unused)]

    macro_rules! top {
        () => {
            Box::new(LTLFormula::Top)
        };
    }
    pub(crate) use top;

    macro_rules! bot {
        () => {
            Box::new(LTLFormula::Bot)
        };
    }
    pub(crate) use bot;

    macro_rules! atom {
        ($inner:expr) => {
            Box::new(LTLFormula::Atomic(LTLVariable::new($inner)))
        };
    }
    pub(crate) use atom;

    macro_rules! neg {
        ($inner:expr) => {
            Box::new(LTLFormula::Neg($inner))
        };
    }
    pub(crate) use neg;

    macro_rules! and {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::And($lhs, $rhs))
        };
    }
    pub(crate) use and;

    macro_rules! or {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::Or($lhs, $rhs))
        };
    }
    pub(crate) use or;

    macro_rules! impies_r {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::ImpliesR($lhs, $rhs))
        };
    }
    pub(crate) use impies_r;

    macro_rules! impies_l {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::ImpliesL($lhs, $rhs))
        };
    }
    pub(crate) use impies_l;

    macro_rules! implies_bi {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::BiImplies($lhs, $rhs))
        };
    }
    pub(crate) use implies_bi;

    macro_rules! x {
        ($inner:expr) => {
            Box::new(LTLFormula::X($inner))
        };
    }
    pub(crate) use x;

    macro_rules! f {
        ($inner:expr) => {
            Box::new(LTLFormula::F($inner))
        };
    }
    pub(crate) use f;

    macro_rules! g {
        ($inner:expr) => {
            Box::new(LTLFormula::G($inner))
        };
    }
    pub(crate) use g;

    macro_rules! u {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::U($lhs, $rhs))
        };
    }
    pub(crate) use u;

    macro_rules! w {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::W($lhs, $rhs))
        };
    }
    pub(crate) use w;

    macro_rules! r {
        ($lhs:expr, $rhs:expr) => {
            Box::new(LTLFormula::R($lhs, $rhs))
        };
    }
    pub(crate) use r;
}
