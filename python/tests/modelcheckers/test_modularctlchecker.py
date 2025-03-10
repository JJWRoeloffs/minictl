from minictl.formulas import CTLFormula
from minictl.modelcheckers import CTLChecker
from minictl.models import Model, State
from copy import copy


def ef_correct(states: set[str], model: Model) -> set[str]:
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


def ef_empty(states: set[str], model: Model) -> set[str]:
    return set()


# def eu(lhs: set[str], rhs: set[str], model: Model) -> set[str]:
#     states = rhs
#     while True:
#         next_states = copy(states)


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
        checker.set_custom("EF", ef_correct)
        assert checker.check(CTLFormula.parse("EFp"), debug=True) == {
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

    def test_is_modified(self):
        checker = CTLChecker(self.model)
        assert not checker.is_modified()
        checker.set_custom("EF", ef_correct)
        assert checker.is_modified()
        checker.set_custom("EF", ef_empty)
        assert checker.is_modified()
