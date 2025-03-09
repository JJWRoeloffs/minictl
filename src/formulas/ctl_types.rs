use super::MLVariable;
use std::collections::HashMap;
use std::sync::Arc;

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

// We are using Arc here because pyo3 demands it.
// pyo3 exported classes need to be sendable between threads, even when
// not planning to do multithreading. While I can try to implement Send for
// `HashMap<Rc<CTLFormula>, HashSet<usize>>` in some safe manner, I think
// the overhead of Arc over Rc is low enough I'll just do it this way.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum CTLFormula {
    Top,
    Bot,
    Atomic(CTLVariable),
    Neg(Arc<CTLFormula>),
    And(Arc<CTLFormula>, Arc<CTLFormula>),
    Or(Arc<CTLFormula>, Arc<CTLFormula>),
    ImpliesR(Arc<CTLFormula>, Arc<CTLFormula>),
    ImpliesL(Arc<CTLFormula>, Arc<CTLFormula>),
    BiImplies(Arc<CTLFormula>, Arc<CTLFormula>),
    EX(Arc<CTLFormula>),
    EF(Arc<CTLFormula>),
    EG(Arc<CTLFormula>),
    EU(Arc<CTLFormula>, Arc<CTLFormula>),
    AX(Arc<CTLFormula>),
    AF(Arc<CTLFormula>),
    AG(Arc<CTLFormula>),
    AU(Arc<CTLFormula>, Arc<CTLFormula>),
}

impl CTLFormula {
    pub(crate) fn memoize(
        &self,
        cache: &mut HashMap<CTLFormula, Arc<CTLFormula>>,
    ) -> Arc<CTLFormula> {
        use CTLFormula as F;
        if let Some(cached) = cache.get(self) {
            return cached.clone();
        }

        let result = match self {
            F::Top => Arc::new(F::Top),
            F::Bot => Arc::new(F::Bot),
            F::Atomic(v) => Arc::new(F::Atomic(v.clone())),
            F::Neg(inner) => Arc::new(F::Neg(inner.memoize(cache))),
            F::And(lhs, rhs) => Arc::new(F::And(lhs.memoize(cache), rhs.memoize(cache))),
            F::Or(lhs, rhs) => Arc::new(F::Or(lhs.memoize(cache), rhs.memoize(cache))),
            F::ImpliesR(lhs, rhs) => Arc::new(F::ImpliesR(lhs.memoize(cache), rhs.memoize(cache))),
            F::ImpliesL(lhs, rhs) => Arc::new(F::ImpliesL(lhs.memoize(cache), rhs.memoize(cache))),
            F::BiImplies(lhs, rhs) => {
                Arc::new(F::BiImplies(lhs.memoize(cache), rhs.memoize(cache)))
            }
            F::EX(inner) => Arc::new(F::EX(inner.memoize(cache))),
            F::AX(inner) => Arc::new(F::AX(inner.memoize(cache))),
            F::EF(inner) => Arc::new(F::EF(inner.memoize(cache))),
            F::AF(inner) => Arc::new(F::AF(inner.memoize(cache))),
            F::EG(inner) => Arc::new(F::EG(inner.memoize(cache))),
            F::AG(inner) => Arc::new(F::AG(inner.memoize(cache))),
            F::EU(lhs, rhs) => Arc::new(F::EU(lhs.memoize(cache), rhs.memoize(cache))),
            F::AU(lhs, rhs) => Arc::new(F::AU(lhs.memoize(cache), rhs.memoize(cache))),
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
    cache: HashMap<CTLFormula, Arc<CTLFormula>>,
}

impl CTLFactory {
    pub fn new(cache: HashMap<CTLFormula, Arc<CTLFormula>>) -> Self {
        Self { cache }
    }
    pub fn create(&mut self, formula: Arc<CTLFormula>) -> Arc<CTLFormula> {
        formula.memoize(&mut self.cache)
    }

    pub fn actual_size(&self) -> usize {
        self.cache.len()
    }
    pub(crate) fn get_cache(&self) -> &HashMap<CTLFormula, Arc<CTLFormula>> {
        &self.cache
    }
}

#[inline(always)]
pub fn memoize_ctl(formula: &CTLFormula) -> Arc<CTLFormula> {
    let mut cache = HashMap::new();
    formula.memoize(&mut cache)
}

pub(crate) mod ctl_formula_macros {
    #![allow(unused)]

    macro_rules! top {
        () => {
            Arc::new(CTLFormula::Top)
        };
    }
    pub(crate) use top;

    macro_rules! bot {
        () => {
            Arc::new(CTLFormula::Bop)
        };
    }
    pub(crate) use bot;

    macro_rules! atom {
        ($name:ident) => {
            Arc::new(CTLFormula::Atomic(CTLVariable::new(
                stringify!($name).to_string(),
            )))
        };
    }
    pub(crate) use atom;

    macro_rules! neg {
        ($inner:expr) => {
            Arc::new(CTLFormula::Neg($inner))
        };
    }
    pub(crate) use neg;

    macro_rules! and {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::And($lhs, $rhs))
        };
    }
    pub(crate) use and;

    macro_rules! or {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::Or($lhs, $rhs))
        };
    }
    pub(crate) use or;

    macro_rules! impies_r {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::ImpliesR($lhs, $rhs))
        };
    }
    pub(crate) use impies_r;

    macro_rules! impies_l {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::ImpliesL($lhs, $rhs))
        };
    }
    pub(crate) use impies_l;

    macro_rules! implies_bi {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::BiImplies($lhs, $rhs))
        };
    }
    pub(crate) use implies_bi;

    macro_rules! ex {
        ($inner:expr) => {
            Arc::new(CTLFormula::EX($inner))
        };
    }
    pub(crate) use ex;

    macro_rules! ax {
        ($inner:expr) => {
            Arc::new(CTLFormula::AX($inner))
        };
    }
    pub(crate) use ax;

    macro_rules! ef {
        ($inner:expr) => {
            Arc::new(CTLFormula::EF($inner))
        };
    }
    pub(crate) use ef;

    macro_rules! af {
        ($inner:expr) => {
            Arc::new(CTLFormula::AF($inner))
        };
    }
    pub(crate) use af;

    macro_rules! eg {
        ($inner:expr) => {
            Arc::new(CTLFormula::EG($inner))
        };
    }
    pub(crate) use eg;

    macro_rules! ag {
        ($inner:expr) => {
            Arc::new(CTLFormula::AG($inner))
        };
    }
    pub(crate) use ag;

    macro_rules! eu {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::EU($lhs, $rhs))
        };
    }
    pub(crate) use eu;

    macro_rules! au {
        ($lhs:expr, $rhs:expr) => {
            Arc::new(CTLFormula::AU($lhs, $rhs))
        };
    }
    pub(crate) use au;
}
