# MiniCTL

A small model checker for Computational Tree Logic. It is not implemented to be the fastest or the most featureful, instead, it is written for a Mini-Master Project at _vu Amsterdam_ to be used as a playground for the Bachelor course on Modal Logic.

For any Finite-state model $\mathcal{M}$, and a CTL formula $\phi$, MiniCTL can compute $\|\phi\|_{\mathcal{M}}$, which is to say, the set of states in which $\phi$ holds.

On top of Prepositional Logic ($\phi ::= p | \top | \bot | \neg \phi | \phi \land \phi | \phi \lor \phi | \phi \rightarrow \phi | \phi \leftrightarrow \phi$), it supports the CTL Modal operators:

- $\mathrm{A} X\phi$
- $\mathrm{E} X \phi$
- $\mathrm{A} F\phi$
- $\mathrm{E} F \phi$
- $\mathrm{A} G\phi$
- $\mathrm{E} G\phi$
- $\mathrm{A} (\phi U \psi)$
- $\mathrm{E} (\phi U \psi)$

### Development

##### Python

For testing, install `maturin` through cargo with `cargo install maturin`. Once installed, the editable package can be installed with `pip install -e .[dev]`, where this same command is run to re-compile the code. To show Rust compiler warnings, `maturin develop` can be run to compile the python part of the code.

To run the tests run `python -m pytest python/tests/`, and to run the formatter run `black python/`
