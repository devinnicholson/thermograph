# Thermograph

[![Crates.io](https://img.shields.io/crates/v/thermograph.svg)](https://crates.io/crates/thermograph)
[![Docs.rs](https://docs.rs/thermograph/badge.svg)](https://docs.rs/thermograph)

Thermograph is a Rust library for Combinatorial Game Theory (CGT) and Surreal Numbers.

## Overview

Thermograph provides the mathematical primitives necessary to construct, simplify, and evaluate Canonical Game Trees. While classical chess engines output a scalar Win/Draw/Loss probability, Thermograph represents game values as exact combinatorial forms. This allows for precise analysis of independent sub-games and their algebraic sums.

## Features

- **Canonical Game Trees**: Construct rigorous `{ Left | Right }` game options.
- **Surreal Arithmetic**: Native support for dyadic rationals (e.g., $1/2, 3/4$) and integers.
- **Infinitesimal Algebra**: Evaluation of nimbers ($\ast$) and combinatorial infinitesimals ($\uparrow, \downarrow$).
- **Thermodynamic Analysis**: Calculate the combinatorial temperature $t(G)$ and stopping value $m(G)$ of arbitrary game trees.

## Example Usage

```rust
use thermograph::CGTValue;

// Define an integer value
let advantage = CGTValue::Integer(1);

// Define the infinitesimal Nimber * (Star)
let star = CGTValue::Star;

// Define a canonical game tree: { 1 | * }
let game = CGTValue::GameTree {
    left: vec![advantage],
    right: vec![star],
};

let temp = game.temperature();
```

## Research Context

Thermograph is designed to replace traditional scalar loss functions in Reinforcement Learning (like MSE) with a Game-Theoretic Surreal Loss formulation, enabling Neural Networks to learn tactical volatility (temperature) explicitly.
