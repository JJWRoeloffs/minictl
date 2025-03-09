from minictl.models import Model
from minictl.formulas import CTLFormula

from typing import Callable

# fmt: off
class CTLChecker:
    def __init__(self, model: Model) -> None: ...
    def check(self, formula: CTLFormula) -> set[str]: ...
    def check_with_custom(self, formula: CTLFormula, debug: bool = False) -> set[str]: ...
    def add_custom(
        self,
        target: str,
        func: Callable[[set[str], Model], set[str]] | Callable[[set[str], set[str], Model], set[str]],
    ) -> None: ...
