import pytest
from minictl.models import Model, State


class TestModelBasics:
    s1 = State("s1", {"p", "q"})
    s2 = State("s2", set())
    s3 = State("s3", {"q"})

    def test_getstate(self):
        model = Model([self.s3], {"s3": ["s3"]})
        instate = model.get_state("s3")
        assert instate.name == "s3"
        assert instate.contains("q")
        assert not instate.contains("p")

    def test_all(self):
        model = Model([self.s3], {"s3": ["s3"]})
        assert model.all() == {"s3"}
        model = Model(
            [self.s1, self.s2, self.s3], {"s1": ["s1"], "s2": ["s2"], "s3": ["s3"]}
        )
        assert model.all() == {"s1", "s2", "s3"}

    def test_all_containing(self):
        model = Model(
            [self.s1, self.s2, self.s3], {"s1": ["s1"], "s2": ["s2"], "s3": ["s3"]}
        )
        assert model.all_containing("p") == {"s1"}
        assert model.all_containing("q") == {"s1", "s3"}

    def test_all_except(self):
        model = Model(
            [self.s1, self.s2, self.s3], {"s1": ["s1"], "s2": ["s2"], "s3": ["s3"]}
        )
        assert model.all_except(set()) == {"s1", "s2", "s3"}
        assert model.all_except({"s2"}) == {"s1", "s3"}
        assert model.all_except({"s1", "s2", "s3"}) == set()


class TestModelCreationErrors:
    s1 = State("s1", {"p", "q"})
    s2 = State("s2", set())

    def test_state_not_mentionned(self):
        with pytest.raises(ValueError):
            Model([self.s1, self.s2], {"s1": ["s1"]})

    def test_empty_edge_list(self):
        with pytest.raises(ValueError):
            Model([self.s1, self.s2], {"s1": ["s1"], "s2": []})

    def test_unused_edge(self):
        with pytest.raises(ValueError):
            Model([self.s1], {"s1": ["s1"], "s2": ["s1"]})

    def test_dangeling_edge(self):
        with pytest.raises(ValueError):
            Model([self.s1], {"s1": ["s1", "s2"]})
