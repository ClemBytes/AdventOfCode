# 🎄 Advent of Code 2015 🎄

This repository contains my solutions to [Advent of Code 2015](https://adventofcode.com/2015), a series of daily programming puzzles published in December 2015.

## ✨ About

Each day consists of two related puzzles that require problem-solving, algorithmic thinking, and some creative coding.  
This project is written in **Rust**, mainly for learning and fun.  
I plan to do some puzzles also in **Python** and then compare how fast they are.

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## 🧠 Structure

- Each puzzle is implemented in its own module under `src/days/`.
- The corresponding data files are in `inputs/` (these are my inputs, they are generated differently for each user when logged into AdventOfCode so yours are different, you can't use my results directly).
- Please note that for some days, I might have applied a change to the input file (for example in `day06`: I have deleted all the "turn ", transforming "turn on" into "on" and "turn off" into "off", in order to have more homogeneity with the "toggle" command).

Compile and run all in debug mode with:
```bash
cargo run
```

Or compile in release mode, then run in release mode (faster run):
```bash
cargo build --release
./target/release/AdventOfCode2015
```

## 🚧 Status

I'm solving the problems at my own pace and may refactor as I go.  
Progress so far:  
⭐⭐⭐⭐⭐⭐⭐☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆  
⭐⭐⭐⭐⭐⭐⭐☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆  
**14 / 50 stars**