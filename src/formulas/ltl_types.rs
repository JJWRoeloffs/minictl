use super::MLVariable;
use std::collections::HashMap;
use std::rc::Rc;

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
    Neg(Rc<LTLFormula>),
    And(Rc<LTLFormula>, Rc<LTLFormula>),
    Or(Rc<LTLFormula>, Rc<LTLFormula>),
    ImpliesR(Rc<LTLFormula>, Rc<LTLFormula>),
    ImpliesL(Rc<LTLFormula>, Rc<LTLFormula>),
    BiImplies(Rc<LTLFormula>, Rc<LTLFormula>),
    X(Rc<LTLFormula>),
    F(Rc<LTLFormula>),
    G(Rc<LTLFormula>),
    U(Rc<LTLFormula>, Rc<LTLFormula>),
    W(Rc<LTLFormula>, Rc<LTLFormula>),
    R(Rc<LTLFormula>, Rc<LTLFormula>),
}

impl LTLFormula {
    pub(crate) fn memoize(
        &self,
        cache: &mut HashMap<LTLFormula, Rc<LTLFormula>>,
    ) -> Rc<LTLFormula> {
        use LTLFormula as F;
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
            F::X(inner) => Rc::new(F::X(inner.memoize(cache))),
            F::F(inner) => Rc::new(F::F(inner.memoize(cache))),
            F::G(inner) => Rc::new(F::G(inner.memoize(cache))),
            F::U(lhs, rhs) => Rc::new(F::U(lhs.memoize(cache), rhs.memoize(cache))),
            F::W(lhs, rhs) => Rc::new(F::W(lhs.memoize(cache), rhs.memoize(cache))),
            F::R(lhs, rhs) => Rc::new(F::R(lhs.memoize(cache), rhs.memoize(cache))),
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
    cache: HashMap<LTLFormula, Rc<LTLFormula>>,
}

impl LTLFactory {
    pub fn new(cache: HashMap<LTLFormula, Rc<LTLFormula>>) -> Self {
        Self { cache }
    }
    pub fn create(&mut self, formula: LTLFormula) -> Rc<LTLFormula> {
        formula.memoize(&mut self.cache)
    }

    pub fn actual_size(&self) -> usize {
        self.cache.len()
    }
}

#[inline(always)]
pub fn memoize_ltl(formula: &LTLFormula) -> Rc<LTLFormula> {
    let mut cache = HashMap::new();
    formula.memoize(&mut cache)
}
