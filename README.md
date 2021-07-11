# aas-benchmark

[![codecov](https://codecov.io/gh/lxndio/aas-benchmark/branch/main/graph/badge.svg?token=EwcxS3GDKU)](https://codecov.io/gh/lxndio/aas-benchmark)

A collection of pattern matching algorithms and a tool to benchmark the algorithms against each other.

## Table of Contents

- [aas-benchmark](#aas-benchmark)
  - [Table of Contents](#table-of-contents)
  - [Build Instructions](#build-instructions)
    - [Steps](#steps)
  - [Usage Instructions](#usage-instructions)
    - [Specifying Algorithms](#specifying-algorithms)
      - [Benchmark All Algorithms at Once](#benchmark-all-algorithms-at-once)
    - [Specifying a Number of Executions](#specifying-a-number-of-executions)
    - [Specifying a Text Source](#specifying-a-text-source)
      - [Random Generated Text](#random-generated-text)
      - [Text From File](#text-from-file)
    - [Specifying a Pattern Source](#specifying-a-pattern-source)
    - [Specifying a Seed](#specifying-a-seed)
    - [Other Arguments](#other-arguments)
    - [List of Algorithms](#list-of-algorithms)
      - [Single Pattern Algorithms](#single-pattern-algorithms)
      - [Algorithms Using a Suffix Array](#algorithms-using-a-suffix-array)
      - [Suffix Array Generation Algorithms](#suffix-array-generation-algorithms)
      - [Approximative Algorithms](#approximative-algorithms)
    - [List of Command-Line Arguments](#list-of-command-line-arguments)

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

You can generate a random text with a length of `m` bytes by using the `-t` or `--tr` argument:

```
aas-benchmark naive -t m ...
```

#### Text From File

It is possible to load a text as a UTF-8 string from a file by using `--tf`:

```
aas-benchmark naive ... --tf text.txt
```

This would load the content of the file `text.txt` as the text.

### Specifying a Pattern Source

Below, all possible arguments for specifying a pattern source are listed.

| Pattern(s) from...         | Usage               | Parameters                           | Multiple patterns?                                                                          | Note |
| -------------------------- | ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------- | ---- |
| ...fixed position in text  | `--pt a..b`         | Range¹ `a..b` of characters in text. | No.                                                                                         |      |
| ...random position in text | `--prt m` or `-p m` | Pattern length `m`.                  | Yes, supply a range¹ for `m` or use `--pmrt m1;m2;m3` with different lengths `m_i`.         |      |
| ...CLI argument            | `--pa pattern`      | Pattern as ASCII string `pattern`.   | Yes, use `--pa` multiple times or enter multiple patterns separated by spaces after `--pa`. |      |
| ...file                    | `--pf pattern.txt`  | File `pattern.txt`                   | Yes, use `--pmf` and supply a file where each line contains one pattern.                    |      |
| Randomly generated         | `--pr m`            | Pattern length `m`.                  | Yes, supply a range¹ for `m`.                                                               |      |

¹ A range is written as `a..b` where `a` is the lower bound and `b` is the *exclusive* upper bound. You can also supply a step size `c` as in `a..b,c`.

Note that the names of those arguments all follow the same naming convention:

`--` + `p` + Multiple? (`m`) Random? (`r`) + Source

This may help you to remember the correct arguments.
### Specifying a Seed

You can set a seed to make the generation of a random text and random patterns predictable using the `-s` or `--seed` argument:

```
aas-benchmark naive ... --seed 12345
```

### Other Arguments

Here is a list of other arguments you can set:

| Argument     | Description                           |
| ------------ | ------------------------------------- |
| `--noheader` | Disables the header in the CSV output |

### List of Algorithms

Currently, these algorithms are supported:

#### Single Pattern Algorithms

| Algorithm                                       | Command-line argument name |
| ----------------------------------------------- | -------------------------- |
| Backward Nondeterministic DAWG Matching (BNDM)  | `bndm`                     |
| Backward Oracle Matching (BOM)                  | `bom`                      |
| Horspool                                        | `horspool`                 |
| Naive Approach                                  | `naive`                    |
| Knuth-Morris-Pratt (KMP)                        | `kmp` or `kmp-classic`     |
| Shift-And                                       | `shift-and`                |
| Double Window Algorithm                         | `dw`                       |
| Bit-Parallel Length Independent Matching (BLIM) | `blim`                     |

#### Algorithms Using a Suffix Array

| Algorithm        | Command-line argument name |
| ---------------- | -------------------------- |
| Pattern Matching | `sa-match`                 |

See [Suffix Array Generation Algorithms](#Suffix-Array-Generation-Algorithms) for more information on how the suffix array is generated.

#### Suffix Array Generation Algorithms

Algorithms that require a suffix array to work generate this suffix array using the SAIS algorithm by default. You can, however, select the used suffix array generation algorithm yourself by specifying the `--suffixarray` argument:

```
aas-benchmark sa-match ... --suffixarray sais
```

Currently, these algorithms are available for suffix array generation:

| Algorithm      | Command-line argument name |
| -------------- | -------------------------- |
| Naive approach | `naive`                    |
| SAIS           | `sais`                     |

#### Approximative Algorithms

| Algorithm                | Command-line argument name |
| ------------------------ | -------------------------- |
| Ukkonen's DP Algorithm   | `ukkonen`                  |
| Error Tolerant Shift-And | `et-shift-and`             |

For approximative algorithms you can set a maximum allowed error value using the `--maxerror` argument:

```
aas-benchmark ukkonen ... --maxerror 2
```

This value defaults to `0` if not set.

### List of Command-Line Arguments

You can run `aas-benchmark --help` to get a list of available arguments.
