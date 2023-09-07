# Largest_regular_matrix

## Introduction

Welcome to the Largest Regular Submatrix Finder GitHub repository! This program was developed collaboratively by four students to find the largest regular submatrix within a given matrix over the field F2. Regular submatrices are submatrices with a determinant of 1, a full rank or a bunch of equivalent characteristics.

## Usage

To use this program, follow these simple steps:

1. Clone the repository

2. Navigate to the project directory:

   ```
   cd largest-regular-submatrix-finder
   ```

3. Run the program, providing the input matrix as a text file or through standard input. The input matrix should be formatted as shown in the examples in the "Example Input" section below.

   ```
   python largest_regular_submatrix_finder.py input_matrix.txt
   ```

   or

   ```
   python largest_regular_submatrix_finder.py
   ```

   You can then enter the matrix manually through the command line.

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

## Contributing

If you'd like to contribute to this project, please follow these guidelines:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b feature/your-feature`.
3. Make your changes and commit them with clear and concise commit messages.
4. Push your changes to your fork: `git push origin feature/your-feature`.
5. Create a pull request from your fork's branch to the `main` branch of this repository.

## Issues

If you encounter any issues with the program or have suggestions for improvements, please feel free to create an issue in the GitHub repository.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.