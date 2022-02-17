# Ising

This project is a precursor to an improved `ising-lib` rust crate. It displays the ising model propagation. The improved `ising-lib` will be designed with both computation and exposition in mind, so that you can easily use it in a playground for displaying simulations, but also so that you can actually measure interesting properties with it. Including ising model simulations of interesting graph structures, such as those found in management science papers (as in Song et Al (link comming soon))

## Running

This example is quite resource intensive; it's recommended that you only use it with the `--release` flag:

```bash
trunk serve --release
```

## Concepts

- Uses [`gloo_timer`](https://gloo-rs.web.app/docs/timer) to automatically step the simulation.
- Logs to the console using the [`weblog`](https://crates.io/crates/weblog) crate.

## Some thoughts on a new, more expansive, architecture:

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



