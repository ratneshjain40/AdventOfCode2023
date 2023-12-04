**Task Description:**

Task Two Summary:
Building upon Task One, the updated task involves considering spelled-out digits (one, two, three, etc.) as valid "digits." The goal remains the same: extract calibration values by identifying the real first and last digits on each line, combining them to form two-digit numbers. The objective is to calculate the sum of all these revised calibration values from the given document.

**Solution**
**General Approach:**

1. **Text Parsing**
- Catch the digit parsing and try to parse sequentially, do the same in reverse for the last digit
- Be carefull not the reverve the string, we just need to iterate in reverse