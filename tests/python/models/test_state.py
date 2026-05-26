from minictl import State


class TestState:
    def test_creation(self):
        state = State("s1", {"p", "q"})
        assert state.name == "s1"
        assert state.variables == {"p", "q"}

    def test_contains(self):
        state = State("s1", {"p", "q"})
        assert state.contains("p")
        assert state.contains("q")
        assert not state.contains("r")
        assert not state.contains("s1")

    def test_eq(self):
        state1 = State("s1", {"p"})
        state2 = State("s1", {"p"})
        assert state1 == state2
