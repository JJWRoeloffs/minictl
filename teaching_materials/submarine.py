#   _____  _    _  ____   __  __            _____   _____  _   _  ______
#  / ____|| |  | ||  _ \ |  \/  |    /\    |  __ \ |_   _|| \ | ||  ____|
# | (___  | |  | || |_) || \  / |   /  \   | |__) |  | |  |  \| || |__
#  \___ \ | |  | ||  _ < | |\/| |  / /\ \  |  _  /   | |  | . ` ||  __|
#  ____) || |__| || |_) || |  | | / ____ \ | | \ \  _| |_ | |\  || |____
# |_____/  \____/ |____/ |_|  |_|/_/    \_\|_|  \_\|_____||_| \_||______|
#
#
# This exercise is inspired by the paper Verifying Fault Tolerance and Self-Diagnosability
# of an Autonomous Underwater Vehicle by Jonathan Ezekiel, Alessio Lomuscio, Levente Molnar
# and Sandor M. Veres. in the Proceedings of the 22nd International Joint Conference
# on Artificial intelligence, pages 1659-1664.
# DOI: doi.org/10.5591/978-1-57735-516-8%2FIJCAI11-279
#
# (Before following this exercise, it is smart to look at the minictl usage guide,
# as this is an example for CTL model checking in general, not minictl specifically)
#
# In this exercise, you'll do the model checking of a small submarine. Its state will be
# described by three boolean variables: "surface" (meaning the submarine is on the surface,
# e.g. at least partially above water), "hatch" (the hatch is open (true) or closed (false)),
# and "sunk" (The submarine is sunk). The submarine can perform the actions: {up, down, open, close}# The actions have the following effects:
# up: nothing while on the surface, goes to the surface when up
# down: nothing while underwater, goes underwater when on the surface
# open: nothing while the hatch is open, opens hatch when closed
# close: nothing while the hatch is closed, closes hatch while opened.
# Aditionally:
#     - if open is performed when underwater, sunk becomes true
#     - If down is performed when the hatch is open, sunk becomes true
#     - Once sunk is true, the only valid action is down which changes nothing
#
# Exercise 1)
# Create the State Transition System (Kripke frame) that describes this system.
# It might be smart to draw it out on paper before you make it in code.

from minictl import State, Model

# TODO: make the model that describes the submarine.
s1 = State("s1", set())
model = Model([s1], {"s1": ["s1"]})

# Exercise 2)
# Consider the state s in which the submarine is on the surface, and open and sunk are false.
#     a) Express in CTL: on all paths, always, the submarine is not sunk.
#     b) Is this formula true in the state s? Explain, referring to the truth definitions.
#     c) Use the model checker to verify your answer to b.

from minictl import CTLChecker

checker = CTLChecker(model)
# TODO: Use the model checker to check 2c) in the state s in the model created in Ex. 1.
# (To check if a formula is true in a state, calculate the set of states in which the
# formula is true, and then check if the target state is in that computed set.)

# Exercise 3)
# Consider the state s from Ex. 2.
#     a) Express in CTL: There exists a path where in some future state the submarine
#        is not on the surface and not sunk, and until that state, it holds that the
#        submarine was also not sunk.
#     b) Is this formula true in the state s? Explain, referring to the truth definitions.
#     c) Use the model checker to verify your answer to b.

# TODO: Use the model checker to check 3c) in the state s in the model created in Ex. 1.

# Exercise 4)
# Consider the state s from Ex. 2.
# Express the following statements in CTL, check if they are true in the state s
# using the model checker, and explain why in one or two sentences of natural language.
#    1) On all paths, there is a future state where "surface" is true and "open" and
#       "sunk" are false.
#    2) On all paths everywhere, if "surface" is true, then it is possible to make it
#       false in the future.
#    3) On all paths everywhere, if "open" is false then it is possible to make it
#       true in the future
#    4) There is a path where from some future state, the submarine is always not
#       on the surface but also not sunk.
#    5) There is no path where at some point the submarine is sunk and then not sunk.

# TODO: Create all the formulas as CTLFormulas, and then check them in the state s
# in the model created in Ex 1.
