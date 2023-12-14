# Advent of Code Challenge Submissions in Rust

![Advent of Code Logo](https://adventofcode.com/favicon.png)

This repository contains my solutions to the [Advent of Code](https://adventofcode.com/) challenges, implemented in the Rust programming language.

## Table of Contents

- [Advent of Code Challenge Submissions in Rust](#advent-of-code-challenge-submissions-in-rust)
  - [Table of Contents](#table-of-contents)
  - [About Advent of Code](#about-advent-of-code)
  - [Solutions](#solutions)
  - [Usage](#usage)
  - [Folder Structure](#folder-structure)
  - [License](#license)
  - [Usage Guide](#usage-guide)
  - [Template](#template)

## About Advent of Code

Advent of Code is a series of programming puzzles designed by [Eric Wastl](http://was.tl/). It consists of daily challenges, each focusing on a particular problem-solving aspect. The puzzles are released one per day, starting on December 1st and ending on December 25th.

For more information, visit [Advent of Code](https://adventofcode.com/).

## Solutions

This repository contains my solutions to the Advent of Code challenges, implemented in Rust. Each day's solution is organized in a separate folder with a dedicated README file explaining the problem and my approach to solving it.

## Usage

To run any specific day's solution, navigate to the respective folder and follow the instructions provided in the README file.

```bash
cd AdventOfCode2023
cargo run
```

Make sure you have Rust and Cargo installed on your system.

## Folder Structure

The repository follows a consistent folder structure:

- **days:** Folder all challenge.
  - **day_X** Solved tasks for that day
    - **task_1.rs**: This file contains the code for task 1. It is written in Rust and demonstrates the use of basic programming concepts such as variables, control structures, and functions.
    - **task_2.rs**: This file contains the code for task 2. It builds upon the concepts introduced in task 1 and introduces extra complexity.
    - **input.txt**: Input data for the challenge.
    - **README.md**: Detailed explanation of the problem and my approach.

- [src/main.rs]: Rust source file containing the solution code.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Feel free to adapt this template to suit your preferences and the specifics of your Rust implementation. Good luck with your Rust Advent of Code submissions!

## Usage Guide
Link to the [Usage Guide](USAGE.md).

## Template

This is a template for the README file for each day's solution. Feel free to use it as a starting point for your own solutions.

```rust
use std::fs::read_to_string;

fn read_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_input(lines: Vec<String>) -> Vec<String> {
    todo!()
}

pub fn run(filename: &str) -> usize {
    let lines = read_input(format!("src/days/day_X/{}", filename).as_str());
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        assert_eq!(run("input_test.txt"), 35);
    }
}

```