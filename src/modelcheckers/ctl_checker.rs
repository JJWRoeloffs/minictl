use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use typed_arena::Arena;

use crate::formulas::ctl_formula_macros as f;
use crate::formulas::CTLFormula;
use crate::models::Model;

struct CTLCheckerInner<'a> {
    map: HashMap<&'a CTLFormula, &'a HashSet<usize>>,
    result_arena: &'a Arena<HashSet<usize>>,
    formula_arena: &'a Arena<CTLFormula>,
}
impl<'a> CTLCheckerInner<'a> {
    fn map(&self) -> &HashMap<&'a CTLFormula, &'a HashSet<usize>> {
        &self.map
    }
    fn memoise_alloc(
        &mut self,
        formula: &'a CTLFormula,
        ret: HashSet<usize>,
    ) -> &'a HashSet<usize> {
        let ret_ref = self.result_arena.alloc(ret);
        self.map.insert(formula, ret_ref);
        ret_ref
    }
    fn memoise_ref(
        &mut self,
        formula: &'a CTLFormula,
        ret: &'a HashSet<usize>,
    ) -> &'a HashSet<usize> {
        self.map.insert(formula, ret);
        ret
    }
    fn sat_ex(&mut self, formula: &'a CTLFormula, model: &Model) -> HashSet<usize> {
        model.pre_e_idx(self.check(formula, model))
    }
    fn sat_eu(
        &mut self,
        formula1: &'a CTLFormula,
        formula2: &'a CTLFormula,
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
                return next;
            }
            set = Cow::Owned(next);
        }
    }
    fn sat_af(&mut self, formula: &'a CTLFormula, model: &Model) -> HashSet<usize> {
        // We are using Cow only to have some type generic over T and &T,
        // as our initial `solve()` returns a reference, but calls to pre_a return owned values.
        let mut set = Cow::Borrowed(self.check(formula, model));
        loop {
            let next: HashSet<usize> = model.pre_a_idx(&set).union(&set).copied().collect();
            if next == *set {
                return next;
            }
            set = Cow::Owned(next);
        }
    }
    fn check(&mut self, formula: &'a CTLFormula, model: &Model) -> &'a HashSet<usize> {
        if let Some(ret) = self.map.get(formula) {
            return ret;
        }
        use CTLFormula as F;
        // We do the re-writing as we go instead of on the formula immediately.
        // I know it's probably slower this way, but it's also kinda fine.
        match formula {
            F::Top => self.memoise_alloc(formula, model.all_idx()),
            F::Bot => self.memoise_alloc(formula, HashSet::new()),
            F::Atomic(var) => self.memoise_alloc(formula, model.all_containing_idx(&var.inner)),
            F::Neg(inner) => {
                let ret = model.all_except_idx(self.check(inner, model));
                self.memoise_alloc(formula, ret)
            }
            F::And(lhs, rhs) => {
                let ret = self
                    .check(lhs, model)
                    .intersection(self.check(rhs, model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::Or(lhs, rhs) => {
                let ret = self
                    .check(lhs, model)
                    .union(self.check(rhs, model))
                    .copied()
                    .collect();
                self.memoise_alloc(formula, ret)
            }
            F::ImpliesR(lhs, rhs) => {
                let rewritten = self
                    .formula_arena
                    .alloc(F::Or(f::neg!(lhs.clone()), rhs.clone()));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::ImpliesL(lhs, rhs) => {
                let rewritten = self
                    .formula_arena
                    .alloc(F::Or(lhs.clone(), f::neg!(rhs.clone())));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::BiImplies(lhs, rhs) => {
                let rewritten = self.formula_arena.alloc(F::And(
                    f::impies_r!(lhs.clone(), rhs.clone()),
                    f::impies_r!(rhs.clone(), lhs.clone()),
                ));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::AX(inner) => {
                let rewritten = self
                    .formula_arena
                    .alloc(F::Neg(f::ex!(f::neg!(inner.clone()))));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::EX(inner) => {
                let ret = self.sat_ex(inner, model);
                self.memoise_alloc(formula, ret)
            }
            // yuk
            F::AU(lhs, rhs) => {
                let rewritten = self.formula_arena.alloc(F::Or(
                    f::neg!(f::eu!(
                        f::neg!(rhs.clone()),
                        f::and!(f::neg!(lhs.clone()), f::neg!(rhs.clone()))
                    )),
                    f::eg!(f::neg!(rhs.clone())),
                ));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::EU(lhs, rhs) => {
                let ret = self.sat_eu(lhs, rhs, model);
                self.memoise_alloc(formula, ret)
            }
            F::EF(inner) => {
                let rewritten = self.formula_arena.alloc(F::EU(f::top!(), inner.clone()));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::AF(inner) => {
                let ret = self.sat_af(inner, model);
                self.memoise_alloc(formula, ret)
            }
            F::EG(inner) => {
                let rewritten = self
                    .formula_arena
                    .alloc(F::Neg(f::af!(f::neg!(inner.clone()))));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
            F::AG(inner) => {
                let rewritten = self
                    .formula_arena
                    .alloc(F::Neg(f::ef!(f::neg!(inner.clone()))));
                let ret = self.check(rewritten, model);
                self.memoise_ref(rewritten, ret)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CTLChecker {
    model: Model,
    cache: HashMap<Box<CTLFormula>, HashSet<usize>>,
}
impl CTLChecker {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            cache: HashMap::new(),
        }
    }
    pub fn get_model(&self) -> &Model {
        &self.model
    }
    pub fn check(&mut self, formula: &CTLFormula) -> HashSet<String> {
        // This function fights the borrow checker a lot.
        // There might be a better way to do it, as I'm not _that_ good at rust,
        // but I tried quite a few things already.

        // We need to clone the current cache because we want to modify it
        // Otherwise we would have two mutable references out to self.
        let map: HashMap<&CTLFormula, &HashSet<usize>> =
            self.cache.iter().map(|(k, v)| (k.as_ref(), v)).collect();

        // We need to create an arena because we want to memoise
        // and memoising without arena would mean storing in the hashmap,
        // wich might get realocated and thus invalidating pointers.
        // We cannot own the arenea, because we want Clone to be implemented for Python.
        let result_arena: Arena<HashSet<usize>> = Arena::new();
        let formula_arena: Arena<CTLFormula> = Arena::new();

        // Once this is in place, we can create the solver, solve,
        // and get the reference map out of it again,
        // which needs to be cloned because it won't live as long as self
        // We can perform this clone while transforming it from indexes to names.
        let mut solver = CTLCheckerInner {
            map,
            result_arena: &result_arena,
            formula_arena: &formula_arena,
        };
        let ret = self.model.get_names(solver.check(formula, &self.model));

        let cache_update: HashMap<Box<CTLFormula>, HashSet<usize>> = solver
            .map()
            .iter()
            // We only need to take the value out of it wasn't in cache already
            .filter(|(&k, _v)| !self.cache.contains_key(k))
            // We could try to move from arena, but cloning shouldn't cost too much
            .map(|(&k, &v)| (Box::new(k.clone()), v.clone()))
            .collect();

        // Once everything is cloned and `solver` (with its reference to map) is dropped,
        // we can extend the self cache and return.
        self.cache.extend(cache_update);
        ret
    }

    // This function is only there so I can play with the checker from python,
    // and insert different algorithms.
    pub(super) fn update_cache(&mut self, formula: CTLFormula, res: HashSet<String>) -> Option<()> {
        let indexes = self.model.get_idxs(&res)?;
        self.cache.insert(Box::new(formula), indexes);
        Some(())
    }
}
