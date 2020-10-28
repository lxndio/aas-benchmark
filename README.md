# aas-benchmark
A collection of pattern matching algorithms and a tool to benchmark the algorithms against each other.

## Table of Contents

1. [Build Instructions](#Build-Instructions)
  1. [Steps](#Steps)
2. [Usage Instructions](#Usage-Instructions)
  1. [Examples](#Examples)
  2. [List of Algorithms](#List-of-Algorithms)
  3. [List of Command-Line Arguments](#List-of-Command-Line-Arguments)

## Build Instructions

If you don't want to build aas-benchmark yourself, you can also download a prebuilt release version. Building the tool yourself, however, should be the prefered way if you want the latest features as the release version may not be updated regularly.

### Steps

How can follow these steps to compile aas-benchmark yourself:

1. Make sure that you have the Rust compiler as well as Cargo installed. Preferably, use [rustup](https://rustup.rs/) to install the entire Rust toolchain.
2. Clone or download this repository to a local directory.
3. Open a terminal and navigate to the directory where the `Cargo.toml` file is located.
4. Run `cargo build --release` to compile aas-benchmark.
  - Alternatively, you can run `cargo run --release` to compile and run aas-benchmark.
  - Using this, you can append command-line arguments after a double dash: `cargo run --release -- --arguments --here`.
5. You will find an executable file in the `target/release` subdirectory.

## Usage Instructions

This part of the README will explain in further detail how to use aas-benchmark using some examples. Make sure you've read the chapter [Build instructions](#Build-instructions).

### Examples

#### Simple example

Let's have a look at the most simple way to benchmark an algorithm:

```
aas-benchmark naive -t 1000000 -p 5
```

This command will benchmark the algorithm `naive`. You can find a complete [list of supported algorithms](#List-of-Algorithms) below.

It is required to always give a text source and a pattern source as argument. Here we are using the `-t 1000000` argument to generate a random text with a length of 1 000 000 bytes. The `-p 5` argument specifies that the pattern should be a substring of the text with a length of 5 bytes.

#### Benchmarking multiple executions

You can simply benchmark multiple executions using the same text and pattern to smooth out deviations in time:

```
aas-benchmark naive -t 1000000 -p 5 -n 10
```

This is the same example as [above](#Simple-example) but with the `-n 10` argument added. This argument specifies that the given algorithm(s) should be executed 10 times each using the same text and pattern.

#### Benchmarking multiple algorithms

It is possible to benchmark multiple algorithms at the same time. Let's expand the example from [above](#Benchmarking-multiple-executions) to do so:

```
aas-benchmark naive,horspool -t 1000000 -p 5 -n 10
```

This command will benchmark the naive algorithm 10 times using a randomly generated text and pattern and then benchmark the Horspool algorithm 10 times using the same text and pattern.

### List of Algorithms

Currently, these algorithms are supported:

Algorithm                | Command-line argument name            |
------------------------ | ------------------------------------- |
Horspool                 | `horspool`                            |
Naive approach           | `naive`                               |
Knuth-Morris-Pratt (KMP) | `kmp` or `kmp-classic`                |
Shift-And                | `shift-and`                           |

### List of Command-Line Arguments

You can run `aas-benchmark --help` to get a list of available arguments.
