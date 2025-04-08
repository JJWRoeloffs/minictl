#             _         _        _    _
#            (_)       (_)      | |  | |
#  _ __ ___   _  _ __   _   ___ | |_ | |
# | '_ ` _ \ | || '_ \ | | / __|| __|| |
# | | | | | || || | | || || (__ | |_ | |
# |_| |_| |_||_||_| |_||_| \___| \__||_|
#
# All docstrings and type information is provided in this file.
# This is technically not the proper way to do it, however,
# all popular editors support docstrings from a `.pyi` file,
# and not all (ahum, vscode) support the proper method of adding
# directly to `help()`

from typing import Callable, List, Dict, Set

def hello_world() -> str:
    """A method to be used exclusively for checking if everything is installed
    Returns (does not print) the string "Hello World"
    """

class LTLFormula:
    """The python view into the LTLFormula.
    This class is frozen. Objects, once created, cannot be modified.

    In python, either create this litterally through the constructor,
    like `LTLFormula("And", LTLFormula("p"), LTLFormula("q"))` or though the
    .parse method like: LTLFormula.parse("p and q")

    Implements `__str__`, `__eq__`, and `__hash__`.
    """

    name: str
    arguments: List[LTLFormula]
    def __init__(self, name: str, *args: LTLFormula) -> None: ...
    @staticmethod
    def parse(formula: str) -> LTLFormula:
        """Parse a string representing an LTLFormula into an LTLFormula
        The parser allows formulas with the following elements:
            - Variables, that must start with a lowercase letter, and be followed by:
                - More lowercase letters
                - Numbers
                - The special characters "=" and "_"
            - "TOP" and "BOT" to represent "⊤" and "⊥" respectively.
            - "!", which means "not" / "¬" and binds tightly
            - Any of the modal operators "X", "F", and "G", which bind tightly
            - The modal operators "U", "W", and "R".
            - "or" and "and" (surrounded by spaces) to represent "∨" and "∧"
            - "->", "<-", and "<->" to represent right, left, and bi-implication, which bind loosely
            - Any amount of brackets "(.)" surrounding formulas to change variable binding.
        """

    def __eq__(self, value: object, /) -> bool: ...
    def __str__(self) -> str: ...
    def __hash__(self) -> int: ...

class CTLFormula:
    """The python view into the CTLFormula.
    This class is frozen. Objects, once created, cannot be modified.

    In python, either create this litterally through the constructor,
    like `CTLFormula("and", CTLFormula("p"), CTLFormula("q"))` or though the
    .parse method like: CTLFormula.parse("p and q")

    Implements `__str__`, `__eq__`, and `__hash__`.
    """

    name: str
    arguments: List[CTLFormula]
    def __init__(self, name: str, *args: CTLFormula) -> None: ...
    @staticmethod
    def parse(formula: str) -> CTLFormula:
        """Parse a string representing an LTLFormula into an LTLFormula
        The parser allows formulas with the following elements:
            - Variables, that must start with a lowercase letter, and be followed by:
                - More lowercase letters
                - Numbers
                - The special characters "=" and "_"
            - "TOP" and "BOT" to represent "⊤" and "⊥" respectively.
            - "!", which means "not" / "¬" and binds tightly
            - Any of the modal operators "EX", "AX", "EF", "AF", "EG", and "AG", which bind tightly
            - The modal operators "E[. U .]" and "A[. U .]", with something valid in place of the dots.
                - The square brackets instead of round ones are mandatory, and make it unambiguous.
            - "or" and "and" (surrounded by spaces) to represent "∨" and "∧"
            - "->", "<-", and "<->" to represent right, left, and bi-implication, which bind loosely
            - Any amount of brackets "(.)" surrounding formulas to change variable binding.
        """

    def __eq__(self, value: object, /) -> bool: ...
    def __str__(self) -> str: ...
    def __hash__(self) -> int: ...

class State:
    """The Python view into the State
    This class is frozen. Objects, once created, cannot be modified.

    You can create them with the State("name", {"var1", "var2"}) constructor,
    providing the state name and a set of variables that are true in the state.
    """

    name: str
    variables: Set[str]
    def __init__(self, name: str, variables: Set[str]): ...
    def contains(self, var: str) -> bool:
        """Returns whether this state has the input variable set as true"""

class Model:
    """The python view into the Model
    This class is frozen. Objects, once created, cannot be modified.
    This class does not expose any public fields. It can only be inspected through methods.

    You can create them with the Model([s1, s2], {"s1": ["s1"], "s2": ["s2"]}) constructor,
    providing a list of states and a hashmap that represents the kripke frame.
    This constructor throws a value error when the arguments do not lead to a valid frame,
    e.g. when not all states have outgoing edges, or if edges point to unknown states.
    """

    def __init__(self, states: List[State], edges: Dict[str, list[str]]) -> None: ...
    def get_state(self, which: str) -> State:
        """Get the state with input name"""

    def get_states(self) -> List[State]:
        """Get all states in this model"""

    def all(self) -> Set[str]:
        """Get all names of states in this model"""

    def all_containing(self, var: str) -> Set[str]:
        """Get all names of states in which the input variable is true in the model"""

    def all_except(self, names: Set[str]) -> set[str]:
        """Get all states except those with the input names"""

    def pre_e(self, names: Set[str]) -> set[str]:
        """Get all states that have at least one outgoing edge to a state which name is
        in the input set of names
        pre_e(Y) = {s ∈ S | exists s', (s -> s' and s' ∈ Y)}
        """

    def pre_a(self, names: Set[str]) -> set[str]:
        """Get all states of which all outgoing edges are to a state which name is
        in the input set of names
        pre_a(Y) = {s ∈ S | for all s', (s -> s' implies s' ∈ Y)}
        """

    def get_next(self, name: str) -> Set[str]:
        """Get the set of names of all the states that the input state
        has outgoing connections to.
        """

# fmt: off
class CTLChecker:
    """
    The Python view into the CTL Checker
    Though this class is not frozen, you cannot modify it directly.
    The object will update itself on calls of `check` by updating the cache.
    This means subsequent calls of `check` will be increasingly faster.

    In Python, you can create this class from a model with the
    CTLChecker(model) constructor.
    """
    def __init__(self, model: Model) -> None: ...
    def check(self, formula: CTLFormula, debug: bool = False) -> Set[str]:
        """Returns the names of the set of states in which the provided formula is true,
        if the parameter "debug" is passed as `True`, this will fail early in case a custom
        algorithm is applied and its output is not the expected output.
        """
    def is_modified(self) -> bool:
        """Returns whether this checker has a custom algorithm applied"""

    def get_model(self) -> Model:
        """Returns the model with which the checker was created"""
    def set_custom(
        self,
        target: str,
        func: Callable[[Set[str], Model], set[str]] | Callable[[set[str], set[str], Model], set[str]],
    ) -> None:
        """ Set a new custom algorithm for the checker. This is only possible if it
        has not checked any formulas jet for cache invalidation reasons.
        These functions must:
           - As their first argument(s) take the set of states in which the dependent formulas are true
           - As the last argument, take an argument "model", of type "Model".

        For example, for the algorithm for EFϕ, the first argument is the set of states in which ϕ is true,
        and the second argument is `model` and for the algorithm for E[ϕUψ] must take as its first argument
        the set of states in which ϕ is true,as the second argument the set of states in which ψ is true,
        and as a third argument `model`
        """
