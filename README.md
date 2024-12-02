# Advent of Code 2024

This repository contains my solutions to [Advent of Code 2024](https://adventofcode.com/2024/). Per the requests of the owners, I am considering the following:

* Solutions will not be uploaded until after the leaderboard is full.
* Inputs, example inputs, puzzle text, etc from the project will not be included here.

You'll have to follow along with the project to know what's going on and what input data to provide.

## Usage

```
Usage: adventofcode2024 [OPTIONS] --day <DAY>

Options:
  -d, --day <DAY>        Day of the month to run for, or "all"
  -f, --folder <FOLDER>  Folder path for input files [default: ./inputs]
  -h, --help             Print help
  -V, --version          Print version
```

Create a folder called `inputs` and copy your input text into files named like `day6.txt`, `day12.txt`, etc. It defaults to looking for this folder in your current working directory, otherwise use the `-f` flag to point to it.

It's probably easiest to just run this out of the project folder with `cargo`, otherwise feel free to compile it.

```
# Run for day 2
cargo run -- -d 2
```

```
cargo build --release
sudo cp target/release/adventofcode2024 /usr/local/bin/ # or wherever you want it.
adventofcode2024 -d 2 -f ~/inputs
```

## Unit tests and why they're initially broken

The unit tests are based on example input data in the puzzle text. This cannot be redistributed, so I've kept it all in an ignored file. Rename `example-inputs-template.toml` to `example-inputs.toml`, then add the example puzzle input referenced in any given field. The unit tests should then pass.

Note that this is unnecessary to run the application with real input text. This was just for testing along the way.

```
cargo test day0_hello_world
```

## Surprise bonus commits

I'll be going back and improving old solutions as I read more documentation and learn more efficient practices with Rust.