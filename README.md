# This is an implementation of Conway game of life on rust

[Conway game of life rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

## How to run

`git clone https://github.com/athos54/game-of-life-rust`

`cargo build --release`

`./target/release/game_of_life2 [table size] [generation time on ms]`

for example

`./target/release/game_of_life2 10 1000`

## TODOS

> Improve the way of each cell calc the live around