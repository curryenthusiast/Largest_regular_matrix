use m4ri_rust::friendly::{BinMatrix, BinVector};
use std::io;
use vob::*;

// Returns stdin as String
pub fn get_input_string() -> String {
    io::read_to_string(io::stdin()).unwrap()
}

// Creates a n x n matrix from string
pub fn string_to_binmatrix(s: String) -> BinMatrix {
    let matrix: Vec<BinVector> = s
        .split_terminator('m')
        .skip(1)
        .map(|substring| {
            BinVector::from(Vob::from_iter(
                substring
                    .split_whitespace()
                    .map(|c| c.parse::<u8>().unwrap() == 1),
            ))
        })
        .collect();
    BinMatrix::new(matrix)
}

// print a binary matrix
pub fn print_matrix(matrix: &BinMatrix) {
    let dim = matrix.ncols();
    
    for i in 0..dim {
        print!("|");
        for j in 0..dim {
            if matrix.bit(i,j) {
                print!("1|");
            } else {
                print!("0|");
            }
        }
        println!();
    }
}
