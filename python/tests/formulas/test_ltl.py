import string
import pytest

from minictl.formulas import LTLFormula as LF


class TestCreationErrors:
    @pytest.mark.parametrize("variable_name", string.ascii_uppercase)
    def test_variable_lowercase(self, variable_name):
        with pytest.raises(ValueError):
            LF(variable_name)

    @pytest.mark.parametrize("formula_type", ["p", "q", "TOP", "BOT"])
    def test_no_args(self, formula_type):
        for i in range(1, 100):
            with pytest.raises(ValueError):
                LF(formula_type, *[LF("q") for _ in range(i)])

    @pytest.mark.parametrize("formula_type", ["Neg", "X", "F", "G"])
    def test_one_args(self, formula_type):
        for i in range(2, 100):
            with pytest.raises(ValueError):
                LF(formula_type, *[LF("q") for _ in range(i)])
        with pytest.raises(ValueError):
            LF(formula_type)
        with pytest.raises(TypeError):
            LF(formula_type, "Something that is not an LTLFormula")

    @pytest.mark.parametrize(
        "formula_type",
        ["And", "Or", "ImpliesR", "ImpliesL", "BiImplies", "U", "W", "R"],
    )
    def test_two_args(self, formula_type):
        for i in range(3, 100):
            with pytest.raises(ValueError):
                LF(formula_type, *[LF("q") for _ in range(i)])
        for i in range(2):
            with pytest.raises(ValueError):
                LF(formula_type, *[LF("q") for _ in range(i)])
        with pytest.raises(TypeError):
            LF(formula_type, "Something that is", "not an LTLFormula")


class TestParse:
    def test_var(self):
        assert LF("p") == LF.parse("p")

    def test_top(self):
        assert LF("TOP") == LF.parse("TOP")

    def test_bot(self):
        assert LF("BOT") == LF.parse("BOT")

    def test_neg(self):
        assert LF("Neg", LF("p")) == LF.parse("!p")

    def test_x(self):
        assert LF("X", LF("p")) == LF.parse("Xp")

    def test_f(self):
        assert LF("F", LF("p")) == LF.parse("Fp")

    def test_g(self):
        assert LF("G", LF("p")) == LF.parse("Gp")

    def test_and(self):
        assert LF("And", LF("p"), LF("q")) == LF.parse("p and q")

    def test_or(self):
        assert LF("Or", LF("p"), LF("q")) == LF.parse("p or q")

    def test_implies_right(self):
        assert LF("ImpliesR", LF("p"), LF("q")) == LF.parse("p -> q")

    def test_implies_left(self):
        assert LF("ImpliesL", LF("p"), LF("q")) == LF.parse("p <- q")

    def test_implies_bi(self):
        assert LF("BiImplies", LF("p"), LF("q")) == LF.parse("p <-> q")

    def test_u(self):
        assert LF("U", LF("p"), LF("q")) == LF.parse("pUq")

    def test_w(self):
        assert LF("W", LF("p"), LF("q")) == LF.parse("pWq")

    def test_r(self):
        assert LF("R", LF("p"), LF("q")) == LF.parse("pRq")


class TestSymbols:
    def test_top(self):
        assert str(LF.parse("TOP")) == "⊤"

    def test_bot(self):
        assert str(LF.parse("BOT")) == "⊥"

    def test_neg(self):
        assert str(LF.parse("!p")) == "¬(p)"

    def test_x(self):
        assert str(LF.parse("Xp")) == "X(p)"

    def test_f(self):
        assert str(LF.parse("Fp")) == "F(p)"

    def test_g(self):
        assert str(LF.parse("Gp")) == "G(p)"

    def test_and(self):
        assert str(LF.parse("p and q")) == "(p)∧(q)"

    def test_or(self):
        assert str(LF.parse("p or q")) == "(p)∧(q)"

    def test_implies_right(self):
        assert str(LF.parse("p -> q")) == "(p)→(q)"

    def test_implies_left(self):
        assert str(LF.parse("p <- q")) == "(p)←(q)"

    def test_implies_bi(self):
        assert str(LF.parse("p <-> q")) == "(p)↔(q)"

    def test_u(self):
        assert str(LF.parse("pUq")) == "(p)U(q)"

    def test_w(self):
        assert str(LF.parse("pWq")) == "(p)W(q)"

    def test_r(self):
        assert str(LF.parse("pRq")) == "(p)R(q)"

    def test_complex_1(self):
        assert (
            str(LF.parse("pU(q and p) -> Xz or G(rUw)"))
            == "((p)U((q)∧(p)))→((X(z))∧(G((r)U(w))))"
        )

    def test_complex_2(self):
        assert (
            str(LF.parse("!BOT -> !G(p <-> q or zWw)"))
            == "(¬(⊥))→(¬(G((p)↔((q)∧((z)W(w))))))"
        )


class TestEq:
    def test_implies_right(self):
        assert LF.parse("(p) -> q") == LF.parse("p -> (q)") == LF.parse("(p -> q)")

    def test_implies_left(self):
        assert LF.parse("(p) <- q") == LF.parse("p <- (q)") == LF.parse("(p <- q)")

    def test_complex(self):
        assert (
            LF.parse("(!TOP -> (Xq)U(Fp)) <-> TOP")
            == LF.parse("((!(TOP) -> (X(q))U(F(p))) <-> (TOP))")
            == LF.parse("(!TOP -> Xq U Fp) <-> TOP")
        )
