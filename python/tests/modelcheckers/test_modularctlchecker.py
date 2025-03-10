from copy import copy

import pytest

from minictl.formulas import CTLFormula
from minictl.modelcheckers import CTLChecker
from minictl.models import Model, State


def ef(states: set[str], model: Model) -> set[str]:
    while True:
        next_states = copy(states)

        for s in model.all():
            reachables = model.get_next(s)
            if reachables.intersection(states):
                next_states.add(s)

        if next_states == states:
            return states
        else:
            states = next_states


def eu(lhs: set[str], rhs: set[str], model: Model) -> set[str]:
    states = rhs
    while True:
        next_states = copy(states)

        for s in model.all():
            if s in lhs:
                reachables = model.get_next(s)
                if reachables.intersection(states):
                    next_states.add(s)

        if next_states == states:
            return states
        else:
            states = next_states


def empty(states: set[str], model: Model) -> set[str]:
    return set()


class TestModularChecker:
    s1 = State("s1", {"p"})
    s2 = State("s2", {"p", "q"})
    s3 = State("s3", {"p", "q"})
    s4 = State("s4", set())
    s5 = State("s5", {"q"})
    s6 = State("s6", {"q"})
    model = Model(
        [s1, s2, s3, s4, s5, s6],
        {
            "s1": ["s1", "s2", "s3"],
            "s2": ["s1", "s2", "s3"],
            "s3": ["s4", "s5"],
            "s4": ["s1", "s6"],
            "s5": ["s4", "s6"],
            "s6": ["s1", "s2"],
        },
    )

    def test_ef_correct(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EF", ef)
        assert checker.check(CTLFormula.parse("EFp")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("EFq")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_ef_correct_debug(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EF", ef)
        assert checker.check(CTLFormula.parse("EFp"), debug=True) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("EFq"), debug=True) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_ef_incorrect(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EF", empty)
        assert checker.check(CTLFormula.parse("EFp")) == set()
        assert checker.check(CTLFormula.parse("EFq")) == set()

    def test_ef_incorrect_debug(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EF", empty)
        with pytest.raises(RuntimeError):
            assert checker.check(CTLFormula.parse("EFp"), debug=True)
        with pytest.raises(RuntimeError):
            assert checker.check(CTLFormula.parse("EFq"), debug=True)

    def test_eu_correct(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EU", eu)
        assert checker.check(CTLFormula.parse("E[pUq]")) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("E[qUp]")) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }

    def test_eu_correct_debug(self):
        checker = CTLChecker(self.model)
        checker.set_custom("EU", eu)
        assert checker.check(CTLFormula.parse("E[pUq]"), debug=True) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("E[qUp]"), debug=True) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }

    def test_is_modified(self):
        checker = CTLChecker(self.model)
        assert not checker.is_modified()
        checker.set_custom("EF", ef)
        assert checker.is_modified()
        checker.set_custom("EF", empty)
        assert checker.is_modified()
