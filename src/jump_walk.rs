use m4ri_rust::friendly::BinMatrix;
use m4ri_sys::{mzd_init, mzd_submatrix};
use rayon::prelude::*;

use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use std::time::Duration;

fn get_submatrix(matrix: &BinMatrix, width: usize, height: usize, row: usize, column: usize) -> BinMatrix {
    unsafe { BinMatrix::from_mzd(mzd_submatrix(mzd_init(height as i32, width as i32), matrix.mzd.as_ptr(), row as i32, column as i32, (row + height) as i32, (column + width) as i32)) }
}

fn get_square_submatrix(matrix: &BinMatrix, dimension: usize, row: usize, column: usize) -> BinMatrix {
    get_submatrix(matrix, dimension, dimension, row, column)
}

static mut BEST_SOLUTION: (usize,usize,usize) = (0,0,0);

pub fn solve(matrix: &BinMatrix, timer: Instant) -> (usize,usize,usize) {
    // Create channel for the worker thread and the receiver which sets a timeout
    let (send, recv) = mpsc::channel();

    // Clone the matrix for usage in thread
    let matrix_clone = matrix.clone();
    thread::spawn(move || {
        let instance_size = matrix_clone.ncols();

        if let Some(result) = solve_wrapper(&matrix_clone, instance_size) {
            send.send(result).expect("failed to send");
        }
    });

    match recv.recv_timeout(Duration::from_micros(19000000 - timer.elapsed().as_micros() as u64)) {
        Ok(result) => {
            result
        },
        Err(_) => {
            unsafe {BEST_SOLUTION}
        }
    }
}

fn solve_wrapper(matrix: &BinMatrix, instance_size: usize) -> Option<(usize,usize,usize)> {
    // Parallel loop over all dimensions from 1 to the rank of the matrix (including the rank)
    if let Some(i) = (1..(matrix.rank() + 1)).into_par_iter().find_last(|&dimension| solve_dim(matrix, dimension, instance_size).is_some()) {
        // NOTE: this call is a duplicate; but i don't know of a sane way to get the down and left shift from the inner loops
        // This causes overhead of around 200-800ms according to my testings against our solver with instance sizes arounf 300-400
        return Some(solve_dim(matrix, i, instance_size).unwrap());
    }
    None
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
                    unsafe {
                        if dimension > BEST_SOLUTION.1 {
                            BEST_SOLUTION = (dimension, shift_down, shift_left);
                        }
                    }
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
