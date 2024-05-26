# MiniCTL

A small symbolic model checker for Computational Tree Logic. It is not implemented to be the fastest or the most featureful, instead, it is written for a Mini-Master Project at _vu Amsterdam_ to be used as a playground for the Bachelor course on Modal Logic. It implements the subset of ISPL (Interpreted Systems Programming Language) that is needed to express CTL, without any extensions from LTL or SML.

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
