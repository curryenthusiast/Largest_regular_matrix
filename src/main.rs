use io::{get_input_string, string_to_binmatrix};
pub mod jump_walk;
pub mod io;
use std::time::Instant;

// matrix index is: (i,j) i~row, j~column 
pub fn main() {
    let timer = Instant::now();

    let input = get_input_string();

    let matrix = string_to_binmatrix(input);

    let (dim, row, col) = jump_walk::solve(&matrix, timer);

    println!("s {} {} {}\n", row, col, dim);
}

#[cfg(test)]
pub mod tests {
    use m4ri_rust::friendly::BinMatrix;
    use super::*;

    #[test]
    fn m4ri() {
        let matrix = BinMatrix::identity(3);
        assert_eq!(matrix.get_window(0, 0, 1, 1).bit(0, 0), true);
        assert_eq!(matrix.rank(), 3);
    }

    #[test]
    fn io_binmatrix() {
        let s = String::from("m 1 1 1\nm 1 0 1\nm 0 0 1\n");
        let matrix = io::string_to_binmatrix(s);
        io::print_matrix(&matrix);
        println!("The rank is: {}", matrix.rank());
    }

}