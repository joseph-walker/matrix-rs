#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

mod matrix;
mod vector;

use matrix::*;

enum Action {
    Quit,
    Print,
    NoOp,
    NewMatrix(usize, usize, Vec<f64>),
    Scale(usize, f64),
    Add(usize, usize, f64),
    Swap(usize, usize),
    Echelon,
}

// TODO Parsing needs a serious error handling pass
// Even a light breeze will cause a panic
fn read_next_action() -> Action {
    print!("> ");

    let _flush_successful = stdout().flush();

    let mut line = String::new();

    let _bytes_read = stdin().read_line(&mut line).unwrap();

    let chunks: Vec<&str> = line.trim().split_whitespace().collect();

    if chunks.len() == 0 {
        return Action::NoOp;
    }

    match chunks[0] {
        "quit" => Action::Quit,
        "print" => Action::Print,
        "matrix" => Action::NewMatrix(
            chunks[1].parse().unwrap(),
            chunks[2].parse().unwrap(),
            chunks[3..]
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect(),
        ),
        "scale" => Action::Scale(chunks[1].parse().unwrap(), chunks[2].parse().unwrap()),
        "swap" => Action::Swap(chunks[1].parse().unwrap(), chunks[2].parse().unwrap()),
        "add" => Action::Add(
            chunks[1].parse().unwrap(),
            chunks[2].parse().unwrap(),
            chunks[3].parse().unwrap(),
        ),
        "echelon" => Action::Echelon,
        _ => panic!("Got unknown action!"),
    }
}

fn repl() {
    let mut matrix: Option<Matrix> = None;

    loop {
        let action = read_next_action();

        match action {
            Action::Quit => break,
            Action::NoOp => continue,
            Action::NewMatrix(rows, cols, values) => {
                matrix = Some(Matrix::new(rows, cols, values));
                println!("Ok");
            }
            _ => {
                match matrix {
                    Some(ref mut matrix) => {
                        match action {
                            Action::Print => print!("{}", matrix),
                            Action::Scale(row, scale) => {
                                matrix.scale_row(row, scale);
                            }
                            Action::Add(src, target, scale) => {
                                let mut r = matrix.row_at(src);
                                r.scale(scale);
                                matrix.add_row_vector(target, r);
                            }
                            Action::Swap(a, b) => {
                                matrix.swap_rows(a, b);
                            }
                            Action::Echelon => {
                                gauss(matrix);
                            }
                            _ => unreachable!(),
                        }

                        println!("Ok");
                    }
                    None => println!("No matrix in memory!"),
                };
            }
        }
    }
}

/// Take a matrix and turn it into echelon form
fn gauss(matrix: &mut Matrix) -> () {
    // Begin with the leftmost nonzero column
    // Find the first non-zero column
    let mut row = 0;

    for cdx in 1..=matrix.num_cols {
        // If all rows are eliminated, we're in echelon form
        if row >= matrix.num_rows {
            break;
        }

        // Get the nth column
        let mut col = matrix.col_at(cdx);

        // Ignore the completed rows
        col.0.drain(0..row);

        // If this column contains all zeroes, it is not a pivot candidate
        if col
            .0
            .iter()
            .filter(|&v| *v != 0.0)
            .collect::<Vec<&f64>>()
            .len()
            == 0
        {
            continue;
        }

        // If the first value is a zero, swap it with the first non-zero row
        if col.0[0] == 0.0 {
            let first_nonzero = col.0.iter().position(|&v| v != 0.0).unwrap();

            matrix.swap_rows(row + 1, first_nonzero + 1);
            col = matrix.col_at(cdx);
        }

        let pivot_value = col.0[0];

        // For all the rows under this one, perform row operations to create all zeroes
        for rdx in row + 2..=matrix.num_rows {
            let scale_factor = -matrix.at(rdx, cdx) / pivot_value;

            let mut v = matrix.row_at(row + 1);
            v.scale(scale_factor);
            matrix.add_row_vector(rdx, v);
        }

        // Eliminate this row and continue
        row += 1;
    }
}

fn main() {
    // example();
    repl();
}

fn example() {
    let mut m = Matrix::new(
        3,
        4,
        vec![1, -2, 1, 0, 0, 2, -8, 8, -4, 5, 9, -9]
            .into_iter()
            .map(|i| i as f64)
            .collect(),
    );

    gauss(&mut m);
}
