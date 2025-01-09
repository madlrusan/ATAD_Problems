# Problem Explanation

## Function extraLongFactorials:

This function calculates the factorial of a large number n.
It initializes a vector result with a single element 1 to represent the initial factorial value.
It iterates from 2 to n, multiplying the current factorial value by each number in this range using the multiply function.
After calculating the factorial, it converts the result vector to a string and prints it. The digits are stored in reverse order, so they are reversed before converting to a string.

## Function multiply:

This function multiplies the current large number (stored in the result vector) by a given multiplier.
It uses a carry variable of type u64 to handle intermediate calculations and carry over values.
For each digit in the result vector, it calculates the product of the digit and the multiplier, adds the carry, and updates the digit with the last digit of the product.
The remaining part of the product is stored in the carry.
After processing all digits, it handles any remaining carry by adding new digits to the result vector.

## Function main:

This is the entry point of the program.
It reads input from the standard input.
It initializes a stdin_iterator to read lines from the standard input.
It reads the integer n from the input.
It calls the extraLongFactorials function with the integer n to calculate and print the factorial of n.
