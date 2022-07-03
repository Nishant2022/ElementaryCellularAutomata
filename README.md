# Elementary Cellular Automaton

## About

This is Nishant Dash's implementation of the Elementary Cellular Automaton using Wolfram codes.
It was created in rust using the Bevy game engine and was compiled to Web Assembly.

## What is the Elementary Cellular Automaton?

The Elementary Cellular Automaton is a 1-dimensional cellular automaton.
There is a 2-dimensional grid for visualization purposes where each cell can be alive or dead.
Each cell follows the same rule, checking the cell directly above it as well as that cell's neighbors.
The rule is determined by the Wolfram code given, with rules ranging from 0 to 255. 
Each row is a "generation" and the rows are spread vertically to visualize each generation.

## My Version

My version allows you to view each rule with a starting row consisting of a single alive cell or a random first row.

The controls for interaction if you are on a computer are as follows:

* W: move view up
* S: move view down
* A: move view left
* D: move view right
* Q: zoom in
* E: zoom out
* Left Mouse Button: increase rule by 1
* Right Mouse Button: decrease rule by 1
* Middle Mouse Button: toggle random first row
* Mouse Scroll: change rule

## Build

If you have cargo install you can run:

```
cargo build
cargo run
```

To build for web run:
```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name elementary-cellular-automaton --out-dir wasm --target web target/wasm32-unknown-unknown/release/elementary-cellular-automaton.wasm
```