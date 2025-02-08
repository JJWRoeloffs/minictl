import string
import pytest

from minictl.formulas import CTLFormula as CF


class TestCreationErrors:
    @pytest.mark.parametrize("variable_name", string.ascii_uppercase)
    def test_variable_lowercase(self, variable_name):
        with pytest.raises(ValueError):
            CF(variable_name)

    @pytest.mark.parametrize("formula_type", ["p", "q", "TOP", "BOT"])
    def test_no_args(self, formula_type):
        for i in range(1, 100):
            with pytest.raises(ValueError):
                CF(formula_type, *[CF("q") for _ in range(i)])

    @pytest.mark.parametrize(
        "formula_type", ["Neg", "EX", "AX", "EF", "AF", "EG", "AG"]
    )
    def test_one_args(self, formula_type):
        for i in range(2, 100):
            with pytest.raises(ValueError):
                CF(formula_type, *[CF("q") for _ in range(i)])
        with pytest.raises(ValueError):
            CF(formula_type)
        with pytest.raises(TypeError):
            CF(formula_type, "Something that is not an LTLFormula")

    @pytest.mark.parametrize(
        "formula_type",
        ["And", "Or", "ImpliesR", "ImpliesL", "BiImplies", "EU", "AU"],
    )
    def test_two_args(self, formula_type):
        for i in range(3, 100):
            with pytest.raises(ValueError):
                CF(formula_type, *[CF("q") for _ in range(i)])
        for i in range(2):
            with pytest.raises(ValueError):
                CF(formula_type, *[CF("q") for _ in range(i)])
        with pytest.raises(TypeError):
            CF(formula_type, "Something that is", "not an LTLFormula")


class TestParse:
    def test_var(self):
        assert CF("p") == CF.parse("p")

    def test_top(self):
        assert CF("TOP") == CF.parse("TOP")

    def test_bot(self):
        assert CF("BOT") == CF.parse("BOT")

    def test_neg(self):
        assert CF("Neg", CF("p")) == CF.parse("!p")

    def test_ex(self):
        assert CF("EX", CF("p")) == CF.parse("EXp")

    def test_ax(self):
        assert CF("AX", CF("p")) == CF.parse("AXp")

    def test_ef(self):
        assert CF("EF", CF("p")) == CF.parse("EFp")

    def test_af(self):
        assert CF("AF", CF("p")) == CF.parse("AFp")

    def test_eg(self):
        assert CF("EG", CF("p")) == CF.parse("EGp")

    def test_ag(self):
        assert CF("AG", CF("p")) == CF.parse("AGp")

    def test_and(self):
        assert CF("And", CF("p"), CF("q")) == CF.parse("p and q")

    def test_or(self):
        assert CF("Or", CF("p"), CF("q")) == CF.parse("p or q")

    def test_implies_right(self):
        assert CF("ImpliesR", CF("p"), CF("q")) == CF.parse("p -> q")

    def test_implies_left(self):
        assert CF("ImpliesL", CF("p"), CF("q")) == CF.parse("p <- q")

    def test_implies_bi(self):
        assert CF("BiImplies", CF("p"), CF("q")) == CF.parse("p <-> q")

    def test_eu(self):
        assert CF("EU", CF("p"), CF("q")) == CF.parse("E[pUq]")

    def test_au(self):
        assert CF("AU", CF("p"), CF("q")) == CF.parse("A[pUq]")


class TestSymbols:
    def test_top(self):
        assert str(CF.parse("TOP")) == "⊤"

    def test_bot(self):
        assert str(CF.parse("BOT")) == "⊥"

    def test_neg(self):
        assert str(CF.parse("!p")) == "¬(p)"

    def test_ex(self):
        assert str(CF.parse("EXp")) == "EX(p)"

    def test_ax(self):
        assert str(CF.parse("AXp")) == "AX(p)"

    def test_ef(self):
        assert str(CF.parse("EFp")) == "EF(p)"

    def test_af(self):
        assert str(CF.parse("AFp")) == "AF(p)"

    def test_eg(self):
        assert str(CF.parse("EGp")) == "EG(p)"

    def test_ag(self):
        assert str(CF.parse("AGp")) == "AG(p)"

    def test_and(self):
        assert str(CF.parse("p and q")) == "(p)∧(q)"

    def test_or(self):
        assert str(CF.parse("p or q")) == "(p)∧(q)"

    def test_implies_right(self):
        assert str(CF.parse("p -> q")) == "(p)→(q)"

    def test_implies_left(self):
        assert str(CF.parse("p <- q")) == "(p)←(q)"

    def test_implies_bi(self):
        assert str(CF.parse("p <-> q")) == "(p)↔(q)"

    def test_eu(self):
        assert str(CF.parse("E[pUq]")) == "E[(p)U(q)]"

    def test_au(self):
        assert str(CF.parse("A[pUq]")) == "A[(p)U(q)]"

    def test_complex_1(self):
        assert (
            str(CF.parse("E[pU(q and p)] -> AXz or EG(A[rUw])"))
            == "(E[(p)U((q)∧(p))])→((AX(z))∧(EG(A[(r)U(w)])))"
        )

    def test_complex_2(self):
        assert (
            str(CF.parse("!BOT -> !AG(p <-> q or A[zUw])"))
            == "(¬(⊥))→(¬(AG((p)↔((q)∧(A[(z)U(w)])))))"
        )


class TestEq:
    def test_implies_right(self):
        assert CF.parse("(p) -> q") == CF.parse("p -> (q)") == CF.parse("(p -> q)")

    def test_implies_left(self):
        assert CF.parse("(p) <- q") == CF.parse("p <- (q)") == CF.parse("(p <- q)")

    def test_complex(self):
        assert (
            CF.parse("(!TOP -> A[(AXq)U(EFp)]) <-> TOP")
            == CF.parse("((!(TOP) -> A[(AX(q))U(EF(p))]) <-> (TOP))")
            == CF.parse("(!TOP -> A[AXq U EFp]) <-> TOP")
        )
