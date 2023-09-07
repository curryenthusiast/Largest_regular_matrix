# Largest_regular_matrix

## Introduction

Welcome to the Largest Regular Submatrix Finder GitHub repository! We developed this program to quickly find the largest regular submatrix within a given matrix over the field F2. Regular submatrices are submatrices with a determinant of 1, a full rank or a bunch of equivalent characteristics.

## Usage

To use this program, follow these simple steps:

1. Clone the repository

2. Navigate to the project directory:

   ```
   cd Largest_regular_matrix
   ```

3. Run the program, providing the input matrix as a text file or through standard input. The input matrix should be formatted as shown in the examples in the "Example Input" section below.

   ```
   cat examplematrix.txt | cargo run
   ```

You can also run the program and then enter the matrix manually through the command line.

4. The program will display the result, including the dimension of the largest regular submatrix and the row and column of the top-left corner of the regular submatrix.

## Example Input

Here are examples of valid input matrices:

```plaintext
m 1 0 0 0
m 0 1 1 0
m 1 0 0 1
m 1 1 0 1
```

```plaintext
m 1 0 1
m 0 1 0
m 0 0 0
```

## Example Output

For the first example input, the result would be:

```plaintext
4 0 0
```

For the second example input, the result would be:

```plaintext
2 0 0
```

## Issues

If you encounter any issues with the program or have suggestions for improvements, please feel free to create an issue in the GitHub repository.