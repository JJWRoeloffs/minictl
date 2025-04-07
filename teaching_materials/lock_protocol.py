#  _       ____    _____  _  __    _____   _____    ____  _______  ____    _____  ____   _
# | |     / __ \  / ____|| |/ /   |  __ \ |  __ \  / __ \|__   __|/ __ \  / ____|/ __ \ | |
# | |    | |  | || |     | ' /    | |__) || |__) || |  | |  | |  | |  | || |    | |  | || |
# | |    | |  | || |     |  <     |  ___/ |  _  / | |  | |  | |  | |  | || |    | |  | || |
# | |____| |__| || |____ | . \    | |     | | \ \ | |__| |  | |  | |__| || |____| |__| || |____
# |______|\____/  \_____||_|\_\   |_|     |_|  \_\ \____/   |_|   \____/  \_____|\____/ |______|
#
#
# In this example, we will look at a lock/unlock protocol checker.
# Let's say we are writing an operating system, and want to make sure a file
# cannot be written to by two different programs simultaneously.
#
# Depending on the needs, this file access might be implemented in several different ways,
# with different effects. All of these will have to be checked to make sure they work.
#
# We will do this by creating a "lock" that has to be acquired before the resource is accessed,
# much like these things are implemented in practice.
#
# More formally, we want to expose an API that allows to:
#     - acquire_lock
#     - access_resource
#     - release_lock
#
# And we want to verify that:
#     - Safety:   `AG(step=access_resource -> lock=held)` ("No state accesses the resource without holding the lock")
#     - Liveness: `AG(lock=held -> AF(lock=free))`    ("Every acquired lock is eventually released")


# Let's start by defining some state to expose:

from dataclasses import dataclass
from enum import StrEnum, auto


class Step(StrEnum):
    acquire_lock = auto()
    access_resource = auto()
    release_lock = auto()
    done = auto()


class Lock(StrEnum):
    free = auto()
    held = auto()


@dataclass
class LockState:
    step: Step
    lock: Lock

    def get_vars(self) -> set[str]:
        """Get the state variables as strings to be able to use them as prepositional variables"""
        return {f"step={self.step}", f"lock={self.lock}"}

    def get_name(self) -> str:
        """Get a unique state name, which we'll just implement as listing the variables"""
        return "_".join(self.get_vars())


# For example:
my_state = LockState(Step.done, Lock.free)
print(my_state)


# We'll then allow the user to define some resource protocol, which, in practice, is just a function that takes
# a LockState object, and returns a list of LockState objects that are the possible next states.
# This way, this protocol can be _executed_ by a template method that reads the protocol at every state, and looks at
# what it should do (deciding randomly or based on unpredictable factors in case there are multiple next states),
# but it can also be formally checked, which is what we'll focus on for this example.
#
# A generic protocol might look like this:


def simple_protocol(lockstate: LockState) -> list[LockState]:
    if lockstate.step == Step.acquire_lock:
        return [LockState(Step.access_resource, Lock.held)]

    if lockstate.step == Step.access_resource:
        return [LockState(Step.release_lock, Lock.held)]

    if lockstate.step == Step.release_lock:
        return [LockState(Step.done, Lock.free)]

    return [lockstate]


# Now, we want to _verify_ this (or any other) protocol.
# To do this, we will write a `verify_protocol` function, that takes _any_ function
# from state to state, and returns if it satisfies some constraints it also takes as input.
# You don't have to worry about the implementation of this, and most professional model checkers
# will have this build-in: allowing you to verify protocols in a formal "state goes to next state"
# language directly.
# However, having it explicit like this is more fun, as it allows you to look at what's going on,
# without having to learn a whole new formal language to specify these kinds of things.
#
# If you are wondering: this could be further expanded to multiple initial states, more complicated protocols,
# or a plethora of other additions that proper model checkers will implement to make themselves useful
#
# This is also to say this code is designed to be readable, not to be fast/efficient. You can do better!
# In fact, proper model checkers won't even _initiate_ all the states, and instead use OBDDs,
# as the number of states a protocol has grows exponentially with program complexity.

from minictl import CTLChecker, CTLFormula, Model, State
from typing import Callable


def verify_protocol(
    protocol: Callable[[LockState], list[LockState]], constraints: list[CTLFormula]
) -> list[bool]:
    initial_state = LockState(Step.acquire_lock, Lock.free)
    queue: list[LockState] = [initial_state]
    states: list[State] = []
    transitions: dict[str, list[str]] = {}

    while queue:
        current = queue.pop()
        if any(current.get_name() == state.name for state in states):
            continue

        states.append(State(current.get_name(), current.get_vars()))
        next_states = protocol(current)
        transitions[current.get_name()] = [state.get_name() for state in next_states]
        queue.extend(next_states)

    model = Model(states, transitions)
    checker = CTLChecker(model)

    return [initial_state.get_name() in checker.check(f) for f in constraints]


# We can now use this function to test for the Safety and Liveness constraints:

safety = CTLFormula.parse("AG(step=access_resource -> lock=held)")
liveness = CTLFormula.parse("AG(lock=held -> AF(lock=free))")


def print_verification_report(protocol: Callable[[LockState], list[LockState]]):
    print("--- Verification report ---")
    [safe_true, live_true] = verify_protocol(protocol, [safety, liveness])
    print(f"For {protocol.__name__}, Safety is {safe_true}")
    print(f"For {protocol.__name__}, Liveness is {live_true}\n")


print_verification_report(simple_protocol)

# We can also add different protocols to check, now, let's say we, for example, want to retry
# when we cannot hold the lock:


def retry_protocol(lockstate: LockState) -> list[LockState]:
    if lockstate.step == Step.acquire_lock:
        if lockstate.lock == Lock.free:
            return [LockState(Step.access_resource, Lock.held)]
        else:
            return [LockState(Step.acquire_lock, Lock.free)]

    if lockstate.step == Step.access_resource:
        return [LockState(Step.release_lock, Lock.held)]

    if lockstate.step == Step.release_lock:
        return [LockState(Step.done, Lock.free)]

    return [lockstate]


print_verification_report(retry_protocol)

# If we then forget to free the lock, we can see that safety is satisfied, but liveness is not:


def incorrect_retry_protocol(lockstate: LockState) -> list[LockState]:
    if lockstate.step == Step.acquire_lock:
        if lockstate.lock == Lock.free:
            return [LockState(Step.access_resource, Lock.held)]
        else:
            return [LockState(Step.acquire_lock, Lock.free)]

    if lockstate.step == Step.access_resource:
        return [LockState(Step.release_lock, Lock.held)]

    if lockstate.step == Step.release_lock:
        return [LockState(Step.done, Lock.held)]

    return [lockstate]


print_verification_report(incorrect_retry_protocol)


# You can do a lot more, and, believe it or not, but this simple implementation is already enough to check
# some basic professional protocols in a way that would be done in industry!
#
# Hopefully, after reading this, you'll have an understanding of how these things might be used in practice.
#
# There is one thing you might be worried about: "The current protocols don't go back to start"
# All of them indeed just go to "done" and stay there, there is no way for them to acquire something again.
#
# More formally, none of them satisfy this constraint: `AG(lock=held -> AF(lock=free and step=acquire_lock))`,
# which we can see as an extended livelyness constraint. This will be the subject of the exercise of this example.
#
#
#  ______                        _
# |  ____|                      (_)
# | |__   __  __ ___  _ __  ___  _  ___   ___
# |  __|  \ \/ // _ \| '__|/ __|| |/ __| / _ \
# | |____  >  <|  __/| |  | (__ | |\__ \|  __/
# |______|/_/\_\\___||_|   \___||_||___/ \___|
#
# Implement some protocol that satisfies the extended liveness constraint:

extended_liveness = CTLFormula.parse(
    "AG(lock=held -> AF(lock=free and step=acquire_lock))"
)

# as well as the constraint that the resource must be acquirable:
aquireable_liveness = CTLFormula.parse("EF(step=access_resource)")


def print_exercise_verification_report(
    protocol: Callable[[LockState], list[LockState]]
):
    print("--- Exercise Verification report ---")
    [safe_true, extended_live_true, aquireable_live_true] = verify_protocol(
        protocol, [safety, extended_liveness, aquireable_liveness]
    )
    print(f"For {protocol.__name__}, Safety is {safe_true}")
    print(f"For {protocol.__name__}, Aquireable Liveness is {aquireable_live_true}")
    print(f"For {protocol.__name__}, Extended Liveness is {extended_live_true}\n")


# It is currently false for all previously done ones:
print_exercise_verification_report(simple_protocol)
print_exercise_verification_report(retry_protocol)


def my_protocol(lockstate: LockState) -> list[LockState]:

    # TODO: Implement extended liveness

    return [lockstate]


print_exercise_verification_report(my_protocol)
