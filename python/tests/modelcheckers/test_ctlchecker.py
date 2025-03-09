from minictl.formulas import CTLFormula
from minictl.modelcheckers import CTLChecker
from minictl.models import State, Model


class TestCheckerBasics:
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

    def test_topbot(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("TOP")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("BOT")) == set()

    def test_vars(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p")) == {"s1", "s2", "s3"}
        assert checker.check(CTLFormula.parse("q")) == {"s2", "s3", "s5", "s6"}

    def test_neg(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("!p")) == {"s4", "s5", "s6"}
        assert checker.check(CTLFormula.parse("!q")) == {"s1", "s4"}

    def test_and(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p and q")) == {"s2", "s3"}

    def test_or(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p or q")) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }

    def test_implies_r(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p -> q")) == {
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_implies_l(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p <- q")) == {"s1", "s2", "s3", "s4"}

    def test_implies_bi(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("p <-> q")) == {"s2", "s3", "s4"}

    def test_ax(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("AXp")) == {"s1", "s2", "s6"}
        assert checker.check(CTLFormula.parse("AXq")) == set()

    def test_ex(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("EXp")) == {"s1", "s2", "s4", "s6"}
        assert checker.check(CTLFormula.parse("EXq")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_au(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("A[qUp]")) == {"s1", "s2", "s3", "s6"}
        assert checker.check(CTLFormula.parse("A[pUq]")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_eu(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("E[pUq]")) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("E[qUp]")) == {
            "s1",
            "s2",
            "s3",
            "s5",
            "s6",
        }

    def test_af(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("AFq")) == {
            "s2",
            "s3",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("AFp")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_ef(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("EFp")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
        assert checker.check(CTLFormula.parse("EFq")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test_ag(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("AGp")) == set()
        assert checker.check(CTLFormula.parse("AGq")) == set()

    def test_eg(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("EGp")) == {"s1", "s2"}
        assert checker.check(CTLFormula.parse("EGq")) == {"s2", "s3", "s5", "s6"}


# These come from lecture and workgroup slides of the course.
# I thought making it do all assignments would be a good and funny test. :P
class TestCheckerFormulas:
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

    def test1(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("E[!pU!q]")) == {"s1", "s4", "s5", "s6"}

    def test2(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("E[pU!q]")) == {"s1", "s2", "s3", "s4"}

    def test3(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("A[pU(EXq)]")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }

    def test4(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("AX EXp")) == {"s4", "s5", "s6"}

    def test5(self):
        checker = CTLChecker(self.model)
        assert checker.check(CTLFormula.parse("AF EGp")) == {
            "s1",
            "s2",
            "s3",
            "s4",
            "s5",
            "s6",
        }
