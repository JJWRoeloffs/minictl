#  _______  _____             ______  ______  _____  _____    _       _____  _____  _    _  _______
# |__   __||  __ \     /\    |  ____||  ____||_   _|/ ____|  | |     |_   _|/ ____|| |  | ||__   __|
#    | |   | |__) |   /  \   | |__   | |__     | | | |       | |       | | | |  __ | |__| |   | |
#    | |   |  _  /   / /\ \  |  __|  |  __|    | | | |       | |       | | | | |_ ||  __  |   | |
#    | |   | | \ \  / ____ \ | |     | |      _| |_| |____   | |____  _| |_| |__| || |  | |   | |
#    |_|   |_|  \_\/_/    \_\|_|     |_|     |_____|\_____|  |______||_____|\_____||_|  |_|   |_|
#
# What is model checking?
#
# Put simply, it has three steps:
# 1) Turning a system into a kripke frame
# 2) Turning things you want to be true about that system into formulas.
# 3) Checking if/where those formulas are true in the constructed frame.
#
# Profesionaly, one is done by computer with human assistance, two is done by humans,
# and three is done entirely by computer.
# In this example, we'll do 1 and 2 by hand, and then do 3 by computer for the simplest
# example I could think of: a traffic light.
#
# Consider a single traffic light that has the following states: red, yellow, green.
# After red comes green, after green comes yellow, and after yellow comes red again.
# Let's say we want to prove that there always is a yellow light after green.
# More formally: green -> AX(yellow)
#
# Let's start by turning the traffic light system into a Kripke frame.
# For that, we first have to specify our three possible states.
# We do this by passing the state name and the set of variables true in that state to the
# State constructor.

from minictl import State

s1 = State("s1", {"red"})
s2 = State("s2", {"yellow"})
s3 = State("s3", {"green"})

# We are drawing three states, and initiating three boolean variables: red, yellow, and green,
# each of these variables is only true in one state, and false in the other two.
#
# One we have the states, we want to draw the arrows between them; we want to construct
# a model out of them. We do this by passing the states, and the connections in between the states
# to the Model constructor.

from minictl import Model

model = Model(
    [s1, s2, s3],
    {"s1": ["s3"], "s2": ["s1"], "s3": ["s2"]},
)

# Now that we have the model, we want to have our formula to test. We said before that
# this will be green -> AX(yellow). We can construct this formula by simply parsing it out:

from minictl import CTLFormula

formula = CTLFormula.parse("green -> AX(yellow)")

# Now that we have this, we want to do our final step: checking if the formula is true
# in the model we created.
# The model checker, however, returns the _set of states_ in which the formula is true,
# which makes sense, a formula isn't true or false in a model, it is true or false in a state.
# In this case, we will want to know if this formula is true in _all_ states, but, an alternative
# is to ask if a formula is true in some specified _initial_ state.
#
# To check if a formula is true in all states, we first run the model checker to get the
# set of states, and then check if this is indeed all states:

from minictl import CTLChecker

checker = CTLChecker(model)
true_states = checker.check(formula)
print(true_states)
print(true_states == checker.get_model().all())

# This was a really simple example, and you could have probably done this a lot quicker by hand.
# However, as the systems to check become more complex, doing this by hand slowly becomes
# infeasable.
