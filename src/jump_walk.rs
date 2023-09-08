use m4ri_rust::friendly::BinMatrix;
use m4ri_sys::{mzd_init, mzd_submatrix};
use rayon::prelude::*;

fn get_submatrix(matrix: &BinMatrix, width: usize, height: usize, row: usize, column: usize) -> BinMatrix {
    unsafe { BinMatrix::from_mzd(mzd_submatrix(mzd_init(height as i32, width as i32), matrix.mzd.as_ptr(), row as i32, column as i32, (row + height) as i32, (column + width) as i32)) }
}

fn get_square_submatrix(matrix: &BinMatrix, dimension: usize, row: usize, column: usize) -> BinMatrix {
    get_submatrix(matrix, dimension, dimension, row, column)
}

pub fn solve(matrix: &BinMatrix) -> (usize,usize,usize) {
    let instance_size = matrix.ncols();

    // Parallel loop over all dimensions from 1 to the rank of the matrix (including the rank)
    if let Some(i) = (1..(matrix.rank() + 1)).into_par_iter().find_last(|&dimension| solve_dim(matrix, dimension, instance_size).is_some()) {
        // NOTE: this call is a duplicate; but i don't know of a sane way to get the down and left shift from the inner loops
        // This causes overhead of around 200-800ms according to my testings against our solver with instance sizes arounf 300-400
        return solve_dim(matrix, i, instance_size).unwrap();
    }
    (0,0,0)
}

fn solve_dim(matrix: &BinMatrix, dimension: usize, instance_size: usize) -> Option<(usize,usize,usize)> {
    // Loop over all submatrices with a size of instance_size x dimension
    let mut shift_left = 0;
    while shift_left < instance_size - dimension + 1 {
        let submatrix = get_submatrix(matrix, dimension, instance_size, 0, shift_left);
        let submatrix_rank = submatrix.rank();

        // If the submatrix rank is equal to the current dimension check all dimension x dimension matrices in it
        if submatrix_rank == dimension {
            let mut shift_down = 0;
            while shift_down < instance_size - dimension + 1 {
                let subsubmatrix_rank = get_square_submatrix(&submatrix, dimension, shift_down, 0).rank();
                if subsubmatrix_rank == dimension {
                    return Some((dimension, shift_down, shift_left));
                }
                shift_down += dimension - subsubmatrix_rank;
            }
            shift_left += 1;
        } else {
            shift_left += dimension - submatrix_rank;
        }
    }

    None
}
