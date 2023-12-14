# Advent of Code 2023 - Usage Guide

## Introduction
Welcome to Advent of Code 2023! This Rust program is designed to solve the challenges presented in Advent of Code 2023. This README provides instructions on how to use and run the code.

## Prerequisites
Before running the code, ensure that you have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the official [Rust website](https://www.rust-lang.org/).

## Getting Started
1. Clone or download the repository to your local machine.

```bash
git clone <repository_url>
```

2. Navigate to the project directory.

```bash
cd <project_directory>
```

3. Build the project using Cargo.

```bash
cargo build --release
```

## Running the Program
To run the program, use the following command:

```bash
./target/release/<executable_name> [OPTIONS]
```

Replace `<executable_name>` with the name of the compiled executable.

## Command-Line Options
The program supports the following command-line options:

- `-d` or `--day`: Specifies the day of the Advent of Code challenge.
- `-t` or `--task`: Specifies the task number for the specified day.

## Examples
### Example 1: Running a Specific Day and Task
To run a specific day and task, use the following command:

```bash
./target/release/<executable_name> -d 1 -t 2
```

This command will execute the solution for Day 1, Task 2.

### Example 2: Running the Latest Task for a Specific Day
To run the latest task for a specific day, use the following command:

```bash
./target/release/<executable_name> -d 3
```

This command will execute the solution for Day 3, Task 2 (assuming Task 2 exists).

### Example 3: Running the Latest Day and Task
To run the latest day and task, use the following command:

```bash
./target/release/<executable_name>
```

This command will execute the solution for the last day and task available.

## Note
- Input files are assumed to be named "input.txt" and should be present in the same directory as the executable.

Now you are ready to explore and solve the Advent of Code challenges with this Rust program! Good luck!

## Usage Guide
Link to the [Usage Guide](USAGE.md).