use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use typed_arena::Arena;

use crate::formulas::ctl_formula_macros as f;
use crate::formulas::{CTLFactory, CTLFormula};
use crate::models::Model;

struct CTLCheckerInner<'a> {
    map: HashMap<Arc<CTLFormula>, &'a HashSet<usize>>,
    arena: &'a Arena<HashSet<usize>>,
}
impl<'a> CTLCheckerInner<'a> {
    fn map(&self) -> &HashMap<Arc<CTLFormula>, &'a HashSet<usize>> {
        &self.map
    }
    fn memoise_alloc(
        &mut self,
        formula: Arc<CTLFormula>,
        ret: HashSet<usize>,
    ) -> &'a HashSet<usize> {
        let ret_ref = self.arena.alloc(ret);
        self.map.insert(formula.clone(), ret_ref);
        ret_ref
    }
    fn memoise_ref(
        &mut self,
        formula: Arc<CTLFormula>,
        ret: &'a HashSet<usize>,
    ) -> &'a HashSet<usize> {
        self.map.insert(formula.clone(), ret);
        ret
    }
    fn sat_ex(&mut self, formula: Arc<CTLFormula>, model: &Model) -> HashSet<usize> {
        model.pre_e_idx(self.check(formula, model))
    }
    fn sat_eu(
        &mut self,
        formula1: Arc<CTLFormula>,
        formula2: Arc<CTLFormula>,
        model: &Model,
    ) -> HashSet<usize> {
        // We are using Cow only to have some type generic over T and &T,
        // as our initial `solve()` returns a reference, but calls to pre_a return owned values.
        let mut set = Cow::Borrowed(self.check(formula2, model));
        let base = self.check(formula1, model);
        loop {
            let next = model
                .pre_e_idx(&set)
                .intersection(base)
                .copied()
                .collect::<HashSet<usize>>()
                .union(&set)
                .copied()
                .collect::<HashSet<usize>>();
            if next == *set {
                return match set {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(_) => next,
                };
            }
            set = Cow::Owned(next);
        }
    }
    fn sat_af(&mut self, formula: Arc<CTLFormula>, model: &Model) -> HashSet<usize> {
        // We are using Cow only to have some type generic over T and &T,
        // as our initial `solve()` returns a reference, but calls to pre_a return owned values.
        let mut set = Cow::Borrowed(self.check(formula, model));
        loop {
            let next: HashSet<usize> = model.pre_a_idx(&set).union(&set).copied().collect();
            if next == *set {
                return match set {
                    Cow::Owned(owned) => owned,
                    Cow::Borrowed(_) => next,
                };
            }
            set = Cow::Owned(next);
        }
    }
    fn check(&mut self, formula: Arc<CTLFormula>, model: &Model) -> &'a HashSet<usize> {
        if let Some(ret) = self.map.get(&formula) {
            return ret;
        }
        use CTLFormula as F;
        match formula.as_ref() {
            F::Top => self.memoise_alloc(formula.clone(), model.all_idx()),
            F::Bot => self.memoise_alloc(formula.clone(), HashSet::new()),
            F::Atomic(var) => {
                self.memoise_alloc(formula.clone(), model.all_containing_idx(&var.inner))
            }
            F::Neg(inner) => {
                let ret = model.all_except_idx(self.check(inner.clone(), model));
                self.memoise_alloc(formula, ret)
            }
            F::And(lhs, rhs) => {
                let ret = self
                    .check(lhs.clone(), model)
                    .intersection(self.check(rhs.clone(), model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::Or(lhs, rhs) => {
                let ret = self
                    .check(lhs.clone(), model)
                    .union(self.check(rhs.clone(), model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::ImpliesR(lhs, rhs) => {
                let ret = self.check(f::or!(f::neg!(lhs.clone()), rhs.clone()), model);
                self.memoise_ref(formula, ret)
            }
            F::ImpliesL(lhs, rhs) => {
                let ret = self.check(f::or!(lhs.clone(), f::neg!(rhs.clone())), model);
                self.memoise_ref(formula, ret)
            }
            F::BiImplies(lhs, rhs) => {
                let ret = self.check(
                    f::and!(
                        f::impies_r!(lhs.clone(), rhs.clone()),
                        f::impies_r!(rhs.clone(), lhs.clone())
                    ),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::AX(inner) => {
                let ret = self.check(f::neg!(f::ex!(f::neg!(inner.clone()))), model);
                self.memoise_ref(formula, ret)
            }
            F::EX(inner) => {
                let ret = self.sat_ex(inner.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            // yuk
            F::AU(lhs, rhs) => {
                let ret = self.check(
                    f::or!(
                        f::neg!(f::eu!(
                            f::neg!(rhs.clone()),
                            f::and!(f::neg!(lhs.clone()), f::neg!(rhs.clone()))
                        )),
                        f::eg!(f::neg!(rhs.clone()))
                    ),
                    model,
                );
                self.memoise_ref(formula, ret)
            }
            F::EU(lhs, rhs) => {
                let ret = self.sat_eu(lhs.clone(), rhs.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            F::EF(inner) => {
                let ret = self.check(f::eu!(f::top!(), inner.clone()), model);
                self.memoise_ref(formula, ret)
            }
            F::AF(inner) => {
                let ret = self.sat_af(inner.clone(), model);
                self.memoise_alloc(formula, ret)
            }
            F::EG(inner) => {
                let ret = self.check(f::neg!(f::af!(f::neg!(inner.clone()))), model);
                self.memoise_ref(formula, ret)
            }
            F::AG(inner) => {
                let ret = self.check(f::neg!(f::ef!(f::neg!(inner.clone()))), model);
                self.memoise_ref(formula, ret)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CTLChecker {
    model: Model,
    formulas: CTLFactory,
    cache: HashMap<Arc<CTLFormula>, HashSet<usize>>,
}
impl CTLChecker {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            formulas: CTLFactory::default(),
            cache: HashMap::new(),
        }
    }
    pub fn get_model(&self) -> &Model {
        &self.model
    }
    pub fn check(&mut self, formula: Arc<CTLFormula>) -> HashSet<String> {
        let formula = self.formulas.create(formula);

        // We need to clone the current cache because we want to modify it
        // Otherwise we would have two mutable references out to self.
        let map: HashMap<Arc<CTLFormula>, &HashSet<usize>> =
            self.cache.iter().map(|(k, v)| (k.clone(), v)).collect();

        // We need to create an arena because we want to memoise
        // and memoising without arena would mean storing in the hashmap,
        // wich might get realocated and thus invalidating pointers.
        let arena: Arena<HashSet<usize>> = Arena::new();

        // Once this is in place, we can create the solver, solve,
        // and get the reference map out of it again,
        // which needs to be cloned because it won't live as long as self
        // We can perform this clone while transforming it from indexes to names.
        let mut solver = CTLCheckerInner { map, arena: &arena };
        let ret = self.model.get_names(solver.check(formula, &self.model));

        let cache_update: HashMap<Arc<CTLFormula>, HashSet<usize>> = solver
            .map()
            .iter()
            // We only need to take the value out of it wasn't in cache already
            .filter(|(k, _v)| !self.cache.contains_key(k.as_ref()))
            // We could try to move from arena, but cloning shouldn't cost too much
            .map(|(k, &v)| (k.clone(), v.clone()))
            .collect();

        // Once everything is cloned and `solver` (with its reference to map) is dropped,
        // we can extend the self cache and return.
        self.cache.extend(cache_update);
        ret
    }

    // This function is only there so I can play with the checker from python,
    // and insert different algorithms.
    pub(super) fn update_cache(
        &mut self,
        formula: Arc<CTLFormula>,
        res: HashSet<String>,
    ) -> Option<()> {
        let indexes = self.model.get_idxs(&res)?;
        self.cache.insert(formula, indexes);
        Some(())
    }
}
