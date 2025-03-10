from minictl.models import Model
from minictl.formulas import CTLFormula

from typing import Callable

# fmt: off
class CTLChecker:
    def __init__(self, model: Model) -> None: ...
    def check(self, formula: CTLFormula, debug: bool = False) -> set[str]: ...
    def is_modified(self) -> bool: ...
    def set_custom(
        self,
        target: str,
        func: Callable[[set[str], Model], set[str]] | Callable[[set[str], set[str], Model], set[str]],
    ) -> None: ...
