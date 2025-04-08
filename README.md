# minictl

A small model checker for Computational Tree Logic. It is not implemented to be the fastest or the most featureful, instead, it is written for a Mini-Master Project at _vu Amsterdam_ to be used as a playground for several courses on Modal Logic.

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

### Installing

The package can be installed with a simple `pip install minictl`.

If pip complaints that `cargo` is not installed, this is because I did not compile the package for your combination of OS and python version (For example, `macOS Sequoia, python3.9`) This is especially likely for MacOS when using Apple Scilicon and a Python version below 3.11, as these are not supported by the tools upstream. If you run into this error and are on MacOS, use to python 3.11 or greater.

If there is no python version supported for your OS, or you wish to use a different python version than I support (>= 3.9), the error can also be fixed by installing the rust toolchain on your computer, which you can do using `rustup` as explained [here](https://www.rust-lang.org/tools/install), proceeding with a default instalation. After rust is installed, the `pip install` command will work.

### Usage

A simple tutorial for the python library can be found under `teaching_materials/minictl_into.py`

More advanced documentation of the python library can be found as docstrings in the `minictl.pyi` file, and as examples in the `tests/python/*.py` files.

### Development

##### Python

For testing, install `maturin` through cargo with `cargo install maturin`. Once installed, the editable package can be installed with `pip install -e .[dev]`, where this same command is run to re-compile the code. To show Rust compiler warnings, `maturin develop` can be run to compile the python part of the code.

To run the tests run `python -m pytest python/tests/`, and to run the formatter run `black python/`
