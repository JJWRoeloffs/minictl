#             _         _        _    _
#            (_)       (_)      | |  | |
#  _ __ ___   _  _ __   _   ___ | |_ | |
# | '_ ` _ \ | || '_ \ | | / __|| __|| |
# | | | | | || || | | || || (__ | |_ | |
# |_| |_| |_||_||_| |_||_| \___| \__||_|
#
# In this tutorial, we'll look at examples of how to use minictl
# as a python library to work with CTL model checking, going over all functions
# you'll reasonably nead when learning to do model checking
#
# For a more detailed description of what is possible, it is best to read the
# tests at `tests/python` and the type information at `minictl.pyi`, as those provide an
# exhaustive list of all supported classes fields and methods with examples.
# This is expecially true if you plan to work on the project itself.
#
# ---- FORMULAS ----
#
# First of all, you might want to create a formula: let's say `p ∧ q`.
# The barest way to do it is to create the formula object entirely from scratch:

from minictl import CTLChecker, CTLFormula

p_and_q = CTLFormula("And", CTLFormula("p"), CTLFormula("q"))

# This, however, is quite cumbersome. It reveals how the underlying structure of these
# formulas are worked out, but doesn't provide much in the way of convenience.
#
# Instead, formulas can usually be best created with the `.parse()` static method:

p_and_q = CTLFormula.parse("p and q")

# For CTL, the parser allows formulas with the following elements:
#     - Variables, that must start with a lowercase letter, and be followed by:
#         - More lowercase letters
#         - Numbers
#         - The special characters "=" and "_"
#     - "TOP" and "BOT" to represent "⊤" and "⊥" respectively.
#     - "!", which means "not" / "¬" and binds tightly
#     - Any of the modal operators "EX", "AX", "EF", "AF", "EG", and "AG", which bind tightly
#     - The modal operators "E[. U .]" and "A[. U .]", with something valid in place of the dots.
#         - The square brackets instead of round ones are mandatory, and make it unambiguous.
#     - "or" and "and" (surrounded by spaces) to represent "∨" and "∧"
#     - "->", "<-", and "<->" to represent right, left, and bi-implication, which bind loosely
#     - Any amount of brackets "(.)" surrounding formulas to change variable binding.
#
# For example:

formula = CTLFormula.parse("!BOT -> !AG(p <-> q or A[zUw])")

# It is possible to print out a string representation of the formulas, but this is not pretty,
# as all braces are printed, not just the mandatory ones:

print(formula)

#
# ---- MODELS ----
#
# States, which are a part of a model, can be created by providing a state name, and a set of
# variables that are true in that state. The state name must be unique, but is not used for
# other reasons, so name them however is of use to you.
# You can check if a variable is true in a state with the `.contains()` method.

from minictl import State

state = State("s1", {"p", "q"})
print(state.contains("p"))
print(state.contains("r"))

# A model is a collection of states and a list of edges between states where all states
# must have outgoing edges.
# You can create them by providing a list of states, and a dictionary that represents the edges,
# where each state name is a key, with its value the list of names of states it has outgoing edges to.
# You can get a state out of a model by using the `.get_state()` method.
# You can get all names of states in a model by using the `.all()` method.
# You can get all names of states containing some variable by using the `.all_containing()` method.

from minictl import Model

model = Model([state], {"s1": ["s1"]})
our_state = model.get_state("s1")
print(our_state.contains("p"), our_state.contains("r"))
print(model.all())
print(model.all_containing("p"))

#
# ---- Checker ----
#
# You can create a checker by providing it with a model. You can then call the `.check()`
# method with a formula to get the set of state names in which the formula is true.

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

checker = CTLChecker(model)
formula = CTLFormula.parse("E[pU!q]")
print(checker.check(formula))


# You can provide custom algorithms to the checker if it has not jet checked any formula.
# These functions must:
#    - As their first argument(s) take the set of states in which the dependent formulas are true
#    - As the last argument, take an argument "model", of type "Model".
#
# For example, for the algorithm for EFϕ, the first argument is the set of states in which ϕ is true,
# and the second argument is `model`, like so:
from copy import copy


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


# And the algorithm for E[ϕUψ] must take as its first argument the set of states in which ϕ is true,
# as the second argument the set of states in which ψ is true, and as a third argument `model` like so:


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


# These could then be inserted in the model checker like so:
checker = CTLChecker(model)
checker.set_custom("EF", ef)
checker.set_custom("EU", eu)

formula = CTLFormula.parse("E[pU!q]")
print(checker.check(formula))
