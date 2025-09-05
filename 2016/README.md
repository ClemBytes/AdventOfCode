# 🎄 Advent of Code 2016 🎄

This repository contains my solutions to [Advent of Code 2016](https://adventofcode.com/2016), a series of daily programming puzzles published in December 2015.

## ✨ About

Each day consists of two related puzzles that require problem-solving, algorithmic thinking, and some creative coding.  
This project is written in **Rust**, mainly for learning and fun.  
I plan to do some puzzles also in **Python** and then compare how fast they are.

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## 🧠 Structure

- Each puzzle is implemented in its own module under `src/`.
- The corresponding data files are in `inputs/`, but I don't publish them here like [required by the author](https://adventofcode.com/2015/about) (these are my inputs, they are generated differently for each user when logged into AdventOfCode so yours are different, you can't use my results directly).
- Please note that for some days, I might have applied a change to the input file:
    - For day 11, I didn't want to parse the short input and directly hardcoded the info I needed in an easy-to-use format.

### Debug mode
Compile and run **all** with:
```bash
cargo test
```

Compile and run **a specific day XX** with:
```bash
cargo test -- dayXX
```

Compile and run **a specific day XX** and **print logs** with:
```bash
cargo test -- --nocapture dayXX
```

### Release mode (faster run)
Compile and run **all** with:
```bash
cargo test --release
```

Compile and run **a specific day XX** with:
```bash
cargo test --release -- dayXX
```

Compile and run **a specific day XX** and **print logs** with:
```bash
cargo test --release -- --nocapture dayXX
```

## 🚧 Status

I'm solving the problems at my own pace and may refactor as I go.  

Progress so far:  
⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐☆☆   
⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐⭐☆☆   
**46 / 50 stars**
