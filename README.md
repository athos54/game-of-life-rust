# This is an implementation of Conway game of life on rust

The only purpose of this repo is learn rust basics. So, if you are a pro on rust and want help me to improve, feel free to make a pull request with your improves, if posible, explaining the reasons

[Conway game of life rules](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

## How to run

`git clone https://github.com/athos54/game-of-life-rust`

`cargo build --release`

`./target/release/game_of_life2 [table size] [generation time on ms]`

for example

`./target/release/game_of_life2 10 1000`

## TODOS

> Improve the way of each cell calc the live around