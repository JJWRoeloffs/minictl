use super::MLVariable;
use std::collections::HashMap;
use std::sync::Arc;

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

// Using Arc<> might be a bit of a weird choise,
// but I want to play around with multi-threading options,
// so I might as well just arc all of it already.
// If rust had second-order types, I'd have used that, but alas.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum LTLFormula {
    Top,
    Bot,
    Atomic(LTLVariable),
    Neg(Arc<LTLFormula>),
    And(Arc<LTLFormula>, Arc<LTLFormula>),
    Or(Arc<LTLFormula>, Arc<LTLFormula>),
    ImpliesR(Arc<LTLFormula>, Arc<LTLFormula>),
    ImpliesL(Arc<LTLFormula>, Arc<LTLFormula>),
    BiImplies(Arc<LTLFormula>, Arc<LTLFormula>),
    X(Arc<LTLFormula>),
    F(Arc<LTLFormula>),
    G(Arc<LTLFormula>),
    U(Arc<LTLFormula>, Arc<LTLFormula>),
    W(Arc<LTLFormula>, Arc<LTLFormula>),
    R(Arc<LTLFormula>, Arc<LTLFormula>),
}

impl LTLFormula {
    pub(crate) fn memoize(
        &self,
        cache: &mut HashMap<LTLFormula, Arc<LTLFormula>>,
    ) -> Arc<LTLFormula> {
        use LTLFormula as F;
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
            F::X(inner) => Arc::new(F::X(inner.memoize(cache))),
            F::F(inner) => Arc::new(F::F(inner.memoize(cache))),
            F::G(inner) => Arc::new(F::G(inner.memoize(cache))),
            F::U(lhs, rhs) => Arc::new(F::U(lhs.memoize(cache), rhs.memoize(cache))),
            F::W(lhs, rhs) => Arc::new(F::W(lhs.memoize(cache), rhs.memoize(cache))),
            F::R(lhs, rhs) => Arc::new(F::R(lhs.memoize(cache), rhs.memoize(cache))),
        };

        cache.insert(self.clone(), result.clone());
        result
    }
    pub(crate) fn total_size(&self) -> usize {
        use LTLFormula as F;
        match self {
            F::Atomic(_) => 1,
            F::Top | F::Bot => 1,
            F::And(lhs, rhs)
            | F::Or(lhs, rhs)
            | F::ImpliesR(lhs, rhs)
            | F::ImpliesL(lhs, rhs)
            | F::BiImplies(lhs, rhs)
            | F::U(lhs, rhs)
            | F::W(lhs, rhs)
            | F::R(lhs, rhs) => 1 + lhs.total_size() + rhs.total_size(),
            F::X(inner) | F::F(inner) | F::G(inner) | F::Neg(inner) => 1 + inner.total_size(),
        }
    }
}

#[derive(Debug, Default)]
pub struct LTLFactory {
    cache: HashMap<LTLFormula, Arc<LTLFormula>>,
}

impl LTLFactory {
    pub fn new(cache: HashMap<LTLFormula, Arc<LTLFormula>>) -> Self {
        Self { cache }
    }
    pub fn create(&mut self, formula: LTLFormula) -> Arc<LTLFormula> {
        formula.memoize(&mut self.cache)
    }

    pub fn actual_size(&self) -> usize {
        self.cache.len()
    }
}

#[inline(always)]
pub fn memoize_ltl(formula: &LTLFormula) -> Arc<LTLFormula> {
    let mut cache = HashMap::new();
    formula.memoize(&mut cache)
}
