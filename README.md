# Game of Life Example

[![Demo](https://img.shields.io/website?label=demo&url=https%3A%2F%2Fexamples.yew.rs%2Fgame_of_life)](https://examples.yew.rs/game_of_life)

This example boasts a complete implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life).
You can manually toggle cells by clicking on them or create a random layout by pressing the "Random" button.

## Running

This example is quite resource intensive; it's recommended that you only use it with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

- Uses [`gloo_timer`](https://gloo-rs.web.app/docs/timer) to automatically step the simulation.
- Logs to the console using the [`weblog`](https://crates.io/crates/weblog) crate.

# Ising

Some thoughts on a new, more expansive, architecture:

Custon Types (and subtypes)
- `Struct Lattice`
  - `Struct Lattice2D`
  - `Struct Lattice3D`
  - `Struct Lattice1D`
  - `Struct LatticeND`
    - `Struct SpinHalf`
    - other spin systems
    - `Struct XYModel`
- `Struct graph`

They have shared behaviour which we implement with Traits
- updating / time propagation
- initializing randomly, (+ many random schemes), deterministically



