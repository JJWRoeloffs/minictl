use super::MLVariable;
use std::collections::HashMap;
use std::rc::Rc;

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
    Neg(Rc<CTLFormula>),
    And(Rc<CTLFormula>, Rc<CTLFormula>),
    Or(Rc<CTLFormula>, Rc<CTLFormula>),
    ImpliesR(Rc<CTLFormula>, Rc<CTLFormula>),
    ImpliesL(Rc<CTLFormula>, Rc<CTLFormula>),
    BiImplies(Rc<CTLFormula>, Rc<CTLFormula>),
    EX(Rc<CTLFormula>),
    EF(Rc<CTLFormula>),
    EG(Rc<CTLFormula>),
    EU(Rc<CTLFormula>, Rc<CTLFormula>),
    AX(Rc<CTLFormula>),
    AF(Rc<CTLFormula>),
    AG(Rc<CTLFormula>),
    AU(Rc<CTLFormula>, Rc<CTLFormula>),
}

impl CTLFormula {
    pub(crate) fn memoize(
        &self,
        cache: &mut HashMap<CTLFormula, Rc<CTLFormula>>,
    ) -> Rc<CTLFormula> {
        use CTLFormula as F;
        if let Some(cached) = cache.get(self) {
            return cached.clone();
        }

        let result = match self {
            F::Top => Rc::new(F::Top),
            F::Bot => Rc::new(F::Bot),
            F::Atomic(v) => Rc::new(F::Atomic(v.clone())),
            F::Neg(inner) => Rc::new(F::Neg(inner.memoize(cache))),
            F::And(lhs, rhs) => Rc::new(F::And(lhs.memoize(cache), rhs.memoize(cache))),
            F::Or(lhs, rhs) => Rc::new(F::Or(lhs.memoize(cache), rhs.memoize(cache))),
            F::ImpliesR(lhs, rhs) => Rc::new(F::ImpliesR(lhs.memoize(cache), rhs.memoize(cache))),
            F::ImpliesL(lhs, rhs) => Rc::new(F::ImpliesL(lhs.memoize(cache), rhs.memoize(cache))),
            F::BiImplies(lhs, rhs) => Rc::new(F::BiImplies(lhs.memoize(cache), rhs.memoize(cache))),
            F::EX(inner) => Rc::new(F::EX(inner.memoize(cache))),
            F::AX(inner) => Rc::new(F::AX(inner.memoize(cache))),
            F::EF(inner) => Rc::new(F::EF(inner.memoize(cache))),
            F::AF(inner) => Rc::new(F::AF(inner.memoize(cache))),
            F::EG(inner) => Rc::new(F::EG(inner.memoize(cache))),
            F::AG(inner) => Rc::new(F::AG(inner.memoize(cache))),
            F::EU(lhs, rhs) => Rc::new(F::EU(lhs.memoize(cache), rhs.memoize(cache))),
            F::AU(lhs, rhs) => Rc::new(F::AU(lhs.memoize(cache), rhs.memoize(cache))),
        };

        cache.insert(self.clone(), result.clone());
        result
    }
    pub(crate) fn total_size(&self) -> usize {
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

#[derive(Debug, Default, Clone)]
pub struct CTLFactory {
    cache: HashMap<CTLFormula, Rc<CTLFormula>>,
}

impl CTLFactory {
    pub fn new(cache: HashMap<CTLFormula, Rc<CTLFormula>>) -> Self {
        Self { cache }
    }
    pub fn create(&mut self, formula: Rc<CTLFormula>) -> Rc<CTLFormula> {
        formula.memoize(&mut self.cache)
    }

    pub fn actual_size(&self) -> usize {
        self.cache.len()
    }
}

#[inline(always)]
pub fn memoize_ctl(formula: &CTLFormula) -> Rc<CTLFormula> {
    let mut cache = HashMap::new();
    formula.memoize(&mut cache)
}

pub(crate) mod ctl_formula_macros {
    #![allow(unused)]

    macro_rules! top {
        () => {
            Rc::new(CTLFormula::Top)
        };
    }
    pub(crate) use top;

    macro_rules! bot {
        () => {
            Rc::new(CTLFormula::Bop)
        };
    }
    pub(crate) use bot;

    macro_rules! atom {
        ($name:ident) => {
            Rc::new(CTLFormula::Atomic(CTLVariable::new(
                stringify!($name).to_string(),
            )))
        };
    }
    pub(crate) use atom;

    macro_rules! neg {
        ($inner:expr) => {
            Rc::new(CTLFormula::Neg($inner))
        };
    }
    pub(crate) use neg;

    macro_rules! and {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::And($lhs, $rhs))
        };
    }
    pub(crate) use and;

    macro_rules! or {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::Or($lhs, $rhs))
        };
    }
    pub(crate) use or;

    macro_rules! impies_r {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::ImpliesR($lhs, $rhs))
        };
    }
    pub(crate) use impies_r;

    macro_rules! impies_l {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::ImpliesL($lhs, $rhs))
        };
    }
    pub(crate) use impies_l;

    macro_rules! implies_bi {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::BiImplies($lhs, $rhs))
        };
    }
    pub(crate) use implies_bi;

    macro_rules! ex {
        ($inner:expr) => {
            Rc::new(CTLFormula::EX($inner))
        };
    }
    pub(crate) use ex;

    macro_rules! ax {
        ($inner:expr) => {
            Rc::new(CTLFormula::AX($inner))
        };
    }
    pub(crate) use ax;

    macro_rules! ef {
        ($inner:expr) => {
            Rc::new(CTLFormula::EF($inner))
        };
    }
    pub(crate) use ef;

    macro_rules! af {
        ($inner:expr) => {
            Rc::new(CTLFormula::AF($inner))
        };
    }
    pub(crate) use af;

    macro_rules! eg {
        ($inner:expr) => {
            Rc::new(CTLFormula::EG($inner))
        };
    }
    pub(crate) use eg;

    macro_rules! ag {
        ($inner:expr) => {
            Rc::new(CTLFormula::AG($inner))
        };
    }
    pub(crate) use ag;

    macro_rules! eu {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::EU($lhs, $rhs))
        };
    }
    pub(crate) use eu;

    macro_rules! au {
        ($lhs:expr, $rhs:expr) => {
            Rc::new(CTLFormula::AU($lhs, $rhs))
        };
    }
    pub(crate) use au;
}
