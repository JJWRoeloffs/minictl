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

// Using Arc<> might be a bit of a weird choise,
// but I want to play around with multi-threading options,
// so I might as well just arc all of it already.
// If rust had second-order types, I'd have used that, but alas.
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
}

#[inline(always)]
pub fn memoize_ctl(formula: &CTLFormula) -> Arc<CTLFormula> {
    let mut cache = HashMap::new();
    formula.memoize(&mut cache)
}
