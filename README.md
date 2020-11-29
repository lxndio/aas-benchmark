# aas-benchmark
A collection of pattern matching algorithms and a tool to benchmark the algorithms against each other.

## Table of Contents

- [Build Instructions](#Build-Instructions)
  - [Steps](#Steps)
- [Usage Instructions](#Usage-Instructions)
  - [Specifying Algorithms](#Specifying-Algorithms)
    - [Benchmark All Algorithms at Once](#Benchmark-All-Algorithms-at-Once)
  - [Specifying a Number of Executions](#Specifying-a-Number-of-Executions)
  - [Specifying a Text Source](#Specifying-a-Text-Source)
    - [Random Generated Text](#Random-Generated-Text)
    - [Text From File](#Text-From-File)
  - [Specifying a Pattern Source](#Specifying-a-Pattern-Source)
    - [Pattern From Fixed Position in Text](#Pattern-From-Fixed-Position-in-Text)
    - [Pattern(s) From Random Position in Text](#Patterns-From-Random-Position-in-Text)
    - [Pattern From Argument](#Pattern-From-Argument)
    - [Pattern From File](#Pattern-From-File)
    - [Random Pattern](#Random-Pattern)
  - [Specifying a Seed](#Specifying-a-Seed)
  - [List of Algorithms](#List-of-Algorithms)
    - [Single Pattern Algorithms](#Single-Pattern-Algorithms)
    - [Algorithms Using a Suffix Array](#Algorithms-Using-a-Suffix-Array)
  - [List of Command-Line Arguments](#List-of-Command-Line-Arguments)

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

This part of the README will explain in further detail how to use aas-benchmark using some examples. Make sure you've read the chapter [Build instructions](#Build-Instructions).

### Specifying Algorithms

The tool requires one parameter which specifies the algorithm or algorithms that you want to benchmark. You can either set a single algorithm or multiple algorithms by giving a comma-seperated list. Notice that there mustn't be spaces around the commas.

```
aas-benchmark naive ...
aas-benchmark naive,horspool,kmp ...
```

#### Benchmark All Algorithms at Once

There is also a shortcut to benchmark all algorithms at once:

```
aas-benchmark all ...
```

### Specifying a Number of Executions

If you like, you can specify a number of executions for each algorithm. You could for example use

```
aas-benchmark naive,horspool -n 10 ...
```

to run both the `naive` and `horspool` algorithm 10 times to smooth out deviations in runtime. If you set different [pattern lengths](#Specifying-a-Pattern-Source), the tool will run the set number of executions for each algorithm and pattern length.

### Specifying a Text Source

#### Random Generated Text

You can generate a random text with a length of `m` bytes by using the `-t` or `--randomtext` argument:

```
aas-benchmark naive -t m ...
```

#### Text From File

It is possible to load a text as a UTF-8 string from a file by using `--textfromfile`:

```
aas-benchmark naive ... --textfromfile text.txt
```

This would load the content of the file `text.txt` as the text.

### Specifying a Pattern Source

#### Pattern From Fixed Position in Text

To use a pattern from a fixed position in the given text, set the `--patternfromtext` argument:

```
aas-benchmark naive ... --patternfromtext 10..15
```

This would take as the pattern the characters at positions 10 to 14 from the given text. Notice that the upper bound is exclusive.

#### Pattern(s) From Random Position in Text

To use a pattern from a random position in the given text, set the `-p` or `--randompatternfromtext` argument:

```
aas-benchmark naive ... -p 10
aas-benchmark naive ... -p 1..11
aas-benchmark naive ... -p 1..101,10
```

The first line would take a string of length 10 from a random position in the given text and set is a the pattern.

The second line determines a random position in the given text and takes strings of length 1, length 2 up to length 10 (upper bound is exclusive) as patterns.

The third line does the same as the second line with the difference that it has a step size of 10, so it would the strings of length 1, 11, 21, 31 and so on.

#### Pattern From Argument

You can also specify a pattern as a UTF-8 string by using the `--patternfromarg` argument:

```
aas-benchmark naive ... --patternfromarg abc
```

This command would set the pattern as `abc`.

#### Pattern From File

It is possible to load a pattern as a UTF-8 string from a file by using `--patternfromfile`:

```
aas-benchmark naive ... --patternfromfile pattern.txt
```

This would load the content of the file `pattern.txt` as the pattern.

#### Random Pattern

You can also use the `--randompattern` argument to generate a random pattern with a length of `m` bytes:

```
aas-benchmark naive ... --randompattern m
```

### Specifying a Seed

You can set a seed to make the generation of a random text and random patterns predictable using the `-s` or `--seed` argument:

```
aas-benchmark naive ... --seed 12345
```

### List of Algorithms

Currently, these algorithms are supported:

#### Single Pattern Algorithms

Algorithm                                      | Command-line argument name            |
---------------------------------------------- | ------------------------------------- |
Backward Nondeterministic DAWG Matching (BNDM) | `bndm`                                |
Horspool                                       | `horspool`                            |
Naive approach                                 | `naive`                               |
Knuth-Morris-Pratt (KMP)                       | `kmp` or `kmp-classic`                |
Shift-And                                      | `shift-and`                           |

#### Algorithms Using a Suffix Array

Algorithm        | Command-line argument name       |
---------------- | -------------------------------- |
Pattern Matching | `sa-match-slow`, `sa-match-fast` |

The algorithms ending with `-slow` use a very simple and slow approach to generating the required suffix array. The corresponding `-fast` variants use the SAIS algorithm to generate the required suffix array.

### List of Command-Line Arguments

You can run `aas-benchmark --help` to get a list of available arguments.
