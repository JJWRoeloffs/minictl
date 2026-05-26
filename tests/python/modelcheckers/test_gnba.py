import pytest
from minictl import GNBATransition, GNBA


class TestGNBABasics:
    t1 = GNBATransition("s1", "s2", {"p", "q"})
    t2 = GNBATransition("s1", "s1", set())

    def test_to_from_existing(self):
        gnba = GNBA(["s1", "s2"], ["s1"], [self.t1, self.t2], [{"t2"}])
        assert gnba.transition_from("s1") == [self.t1, self.t2]
        assert gnba.transition_to("s1") == [self.t2]
        assert gnba.transition_from("s2") == []
        assert gnba.transition_to("s2") == [self.t1]

    def test_to_from_contains(self):
        gnba = GNBA(["s1", "s2"], ["s1"], [self.t1, self.t2], [{"t2"}])
        transitions = gnba.transition_to("s2")
        assert transitions is not None and len(transitions) == 1
        assert transitions[0].contains("p")

    def test_to_from_nonexisting(self):
        gnba = GNBA(["s1", "s2"], ["s1"], [self.t1, self.t2], [{"t2"}])
        with pytest.raises(ValueError):
            gnba.transition_from("s3")
        with pytest.raises(ValueError):
            gnba.transition_to("s3")
