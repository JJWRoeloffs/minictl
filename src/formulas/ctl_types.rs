use std::collections::HashSet;

use super::MLVariable;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct CTLVariable {
    pub inner: String,
}
impl CTLVariable {
    pub(crate) fn new(inner: String) -> Self {
        Self { inner }
    }
}
impl MLVariable for CTLVariable {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum CTLFormula {
    Top,
    Bot,
    Atomic(CTLVariable),
    Neg(Box<CTLFormula>),
    And(Box<CTLFormula>, Box<CTLFormula>),
    Or(Box<CTLFormula>, Box<CTLFormula>),
    ImpliesR(Box<CTLFormula>, Box<CTLFormula>),
    ImpliesL(Box<CTLFormula>, Box<CTLFormula>),
    BiImplies(Box<CTLFormula>, Box<CTLFormula>),
    EX(Box<CTLFormula>),
    EF(Box<CTLFormula>),
    EG(Box<CTLFormula>),
    EU(Box<CTLFormula>, Box<CTLFormula>),
    AX(Box<CTLFormula>),
    AF(Box<CTLFormula>),
    AG(Box<CTLFormula>),
    AU(Box<CTLFormula>, Box<CTLFormula>),
}

impl CTLFormula {
    fn collect_subformulas_into(&self, out: &mut HashSet<CTLFormula>) {
        if !out.insert(self.clone()) {
            return;
        }
        match self {
            CTLFormula::Top | CTLFormula::Bot | CTLFormula::Atomic(_) => {}
            CTLFormula::Neg(f)
            | CTLFormula::EX(f)
            | CTLFormula::EF(f)
            | CTLFormula::EG(f)
            | CTLFormula::AX(f)
            | CTLFormula::AF(f)
            | CTLFormula::AG(f) => {
                f.collect_subformulas_into(out);
            }
            CTLFormula::And(l, r)
            | CTLFormula::Or(l, r)
            | CTLFormula::ImpliesR(l, r)
            | CTLFormula::ImpliesL(l, r)
            | CTLFormula::BiImplies(l, r)
            | CTLFormula::EU(l, r)
            | CTLFormula::AU(l, r) => {
                l.collect_subformulas_into(out);
                r.collect_subformulas_into(out);
            }
        }
    }
    pub fn collect_subformulas(&self) -> HashSet<CTLFormula> {
        let mut out = HashSet::new();
        self.collect_subformulas_into(&mut out);
        out
    }
    pub fn total_size(&self) -> usize {
        use CTLFormula as F;
        match self {
            F::Atomic(_) => 1,
            F::Top | F::Bot => 1,
            F::And(lhs, rhs)
            | F::Or(lhs, rhs)
            | F::ImpliesR(lhs, rhs)
            | F::ImpliesL(lhs, rhs)
            | F::BiImplies(lhs, rhs)
            | F::EU(lhs, rhs)
            | F::AU(lhs, rhs) => 1 + lhs.total_size() + rhs.total_size(),
            F::EX(inner)
            | F::AX(inner)
            | F::EF(inner)
            | F::AF(inner)
            | F::EG(inner)
            | F::AG(inner)
            | F::Neg(inner) => 1 + inner.total_size(),
        }
    }
}

pub(crate) mod ctl_formula_macros {
    #![allow(unused)]

    macro_rules! top {
        () => {
            Box::new(CTLFormula::Top)
        };
    }
    pub(crate) use top;

    macro_rules! bot {
        () => {
            Box::new(CTLFormula::Bot)
        };
    }
    pub(crate) use bot;

    macro_rules! atom {
        ($inner:expr) => {
            Box::new(CTLFormula::Atomic(CTLVariable::new($inner)))
        };
    }
    pub(crate) use atom;

    macro_rules! neg {
        ($inner:expr) => {
            Box::new(CTLFormula::Neg($inner))
        };
    }
    pub(crate) use neg;

    macro_rules! and {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::And($lhs, $rhs))
        };
    }
    pub(crate) use and;

    macro_rules! or {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::Or($lhs, $rhs))
        };
    }
    pub(crate) use or;

    macro_rules! impies_r {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::ImpliesR($lhs, $rhs))
        };
    }
    pub(crate) use impies_r;

    macro_rules! impies_l {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::ImpliesL($lhs, $rhs))
        };
    }
    pub(crate) use impies_l;

    macro_rules! implies_bi {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::BiImplies($lhs, $rhs))
        };
    }
    pub(crate) use implies_bi;

    macro_rules! ex {
        ($inner:expr) => {
            Box::new(CTLFormula::EX($inner))
        };
    }
    pub(crate) use ex;

    macro_rules! ax {
        ($inner:expr) => {
            Box::new(CTLFormula::AX($inner))
        };
    }
    pub(crate) use ax;

    macro_rules! ef {
        ($inner:expr) => {
            Box::new(CTLFormula::EF($inner))
        };
    }
    pub(crate) use ef;

    macro_rules! af {
        ($inner:expr) => {
            Box::new(CTLFormula::AF($inner))
        };
    }
    pub(crate) use af;

    macro_rules! eg {
        ($inner:expr) => {
            Box::new(CTLFormula::EG($inner))
        };
    }
    pub(crate) use eg;

    macro_rules! ag {
        ($inner:expr) => {
            Box::new(CTLFormula::AG($inner))
        };
    }
    pub(crate) use ag;

    macro_rules! eu {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::EU($lhs, $rhs))
        };
    }
    pub(crate) use eu;

    macro_rules! au {
        ($lhs:expr, $rhs:expr) => {
            Box::new(CTLFormula::AU($lhs, $rhs))
        };
    }
    pub(crate) use au;
}
