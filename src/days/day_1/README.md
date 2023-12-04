**Task Description:**

You are tasked with recovering calibration values from a newly-improved calibration document. The document consists of lines of text, and each line contains a specific calibration value that needs to be retrieved. The calibration value is obtained by combining the first digit and the last digit (in that order) from each line, resulting in a two-digit number.

For instance, given the lines:

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

The corresponding calibration values are 12, 38, 15, and 77. The final objective is to calculate the sum of all these calibration values.

Your task is to write a program or script that reads the calibration document, extracts the calibration values, calculates their sum, and outputs the result.

**Example:**

```
Input:
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

Output:
142
```

**Solution**
**General Approach:**

1. **Reading Input:**
   - Use the `read_to_string` function from the `std::fs` module to read the contents of the input file.
   - Convert the file content into a vector of strings, where each string represents a line from the input.

2. **Extracting Calibration Values:**
   - Iterate over each line in the vector of strings.
   - Find the index of the first digit using the `find` method with a closure that checks if a character is a digit.
   - Extract the first digit from the line using the found index.
   - Find the index of the last digit using the `rfind` method.
   - Extract the last digit from the line using the found index.
   - Create pairs of the first and last digits.

3. **Calculating Calibration Values:**
   - Transform each pair of digits into a two-digit number (u32) by multiplying the first digit by 10 and adding the last digit.
   - Collect these two-digit numbers into a vector.

4. **Calculating the Sum:**
   - Calculate the sum of all two-digit numbers in the vector using the `iter` and `sum` methods.

5. **Returning Result:**
   - Return the calculated sum as the final result.

**Methodology:**

- The approach involves a sequential processing of each line in the input.
- Key operations include finding the first and last digits, forming pairs, transforming pairs into two-digit numbers, and calculating their sum.
- The code is structured with functions to enhance readability and maintainability.
- The use of standard Rust functions and methods simplifies the implementation.
- The solution relies on Rust's strong type system and built-in methods to handle string manipulation and numeric operations.