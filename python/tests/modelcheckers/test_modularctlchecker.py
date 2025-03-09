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
        checker.add_custom("EF", ef_correct)
        assert checker.check_with_custom(CTLFormula.parse("EFp"), debug=True) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
        assert checker.check_with_custom(CTLFormula.parse("EFq")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
