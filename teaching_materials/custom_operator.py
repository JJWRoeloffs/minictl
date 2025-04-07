#   _____  _    _   _____  _______  ____   __  __     ____   _____   ______  _____          _______  ____   _____
#  / ____|| |  | | / ____||__   __|/ __ \ |  \/  |   / __ \ |  __ \ |  ____||  __ \     /\ |__   __|/ __ \ |  __ \
# | |     | |  | || (___     | |  | |  | || \  / |  | |  | || |__) || |__   | |__) |   /  \   | |  | |  | || |__) |
# | |     | |  | | \___ \    | |  | |  | || |\/| |  | |  | ||  ___/ |  __|  |  _  /   / /\ \  | |  | |  | ||  _  /
# | |____ | |__| | ____) |   | |  | |__| || |  | |  | |__| || |     | |____ | | \ \  / ____ \ | |  | |__| || | \ \
#  \_____| \____/ |_____/    |_|   \____/ |_|  |_|   \____/ |_|     |______||_|  \_\/_/    \_\|_|   \____/ |_|  \_\
#
#
# (Before following this assignment, it is smart to look at the minictl usage guide,
# as this is an assignment for CTL model checking in general, not minictl specfically)
#
# In this assignment, we will look at defining our own custom operator,
# and providing an algorithm for it.
#
# In definition of E[ϕUψ], ϕ does not need to hold true in the state in which ψ is true,
# only in preciding states. However, it is not imposible to imagine a modality in which
# ϕ does need to hold true, even in the state in which ψ is true.
# In this assignment, you will write an implemenation of a model checking algorithm for this
# custom version of E[ϕUψ].
#
# Consider the following model (it might be smart to draw it out):

from minictl import State, Model, CTLFormula, CTLChecker

s1 = State("s1", {"p"})
s2 = State("s2", set())
s3 = State("s3", {"p", "q"})
s4 = State("s4", {"q"})
s5 = State("s5", {"p", "q"})
model = Model(
    [s1, s2, s3, s4, s5],
    {
        "s1": ["s2"],
        "s2": ["s1"],
        "s3": ["s1", "s4"],
        "s4": ["s4"],
        "s5": ["s2", "s4", "s5"],
    },
)

checker = CTLChecker(model)
print(checker.check(CTLFormula.parse("E[pU!q]")))

# Let's consider the formula E[pU!q]. Before starting, it might be smart to consider in
# what states this formula is true now (you can just run the model checker to check),
# and then in what state this formula would be true, if we change the definition to
# make it such that p does need to hold true, even in the state in which !q is true.
#
# What do you expect the set of states in which E[pU!q] holds to be with the new
# definition of E[ϕUψ]?
#
# Once you're done thinking, you can try to implement it. There is some scaffold code to
# to get you started below (Also don't forget to check the `minictl_intro.py` file for help,)
# Good luck!
from copy import copy


def new_eu(lhs: set[str], rhs: set[str], model: Model) -> set[str]:
    # TODO: Add your implementation here

    raise NotImplementedError


checker = CTLChecker(model)
checker.set_custom("EU", new_eu)
# (Note that you cannot base `debug=True`, as that function expects the actual definition of EU,
# Not our custom one.)
print(checker.check(CTLFormula.parse("E[pU!q]")))
