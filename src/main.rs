#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

mod matrix;
mod vector;

use matrix::*;

enum Action {
    Quit,
    Print,
    NewMatrix(usize, usize, Vec<f64>),
    Scale(usize, f64),
    Add(usize, usize, f64),
}

fn read_next_action() -> Action {
    print!("> ");

    let _flush_successful = stdout().flush();

    let mut line = String::new();

    let _bytes_read = stdin().read_line(&mut line).unwrap();

    let chunks: Vec<&str> = line.trim().split_whitespace().collect();

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
        "add" => Action::Add(
            chunks[1].parse().unwrap(),
            chunks[2].parse().unwrap(),
            chunks[3].parse().unwrap(),
        ),
        _ => panic!("Got unknown action!"),
    }
}

fn repl() {
    let mut matrix: Option<Matrix> = None;

    loop {
        let action = read_next_action();

        match action {
            Action::Quit => break,
            Action::NewMatrix(rows, cols, values) => {
                matrix = Some(Matrix::new(rows, cols, values));
            }
            _ => {
                match matrix {
                    Some(ref mut matrix) => match action {
                        Action::Print => print!("{}", matrix),
                        Action::Scale(row, scale) => {
                            matrix.scale_row(row, scale);
                        }
                        Action::Add(src, target, scale) => {
                            let mut r = matrix.row_at(src);
                            r.scale(scale);
                            matrix.add_row_vector(target, r);
                        }
                        _ => unreachable!(),
                    },
                    None => println!("No matrix in memory!"),
                };
            }
        }

        println!("Ok");
    }
}

fn main() {
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

    // The following matrix operations solve this system of linear equations:
    //   x - 2y +  z = 0
    //       2y - 8z = 8
    // -4x + 5y + 9z = -9

    println!("{}", m);

    let mut r = m.row_at(1);
    r.scale(4.0);
    m.add_row_vector(3, r);

    m.scale_row(2, 0.5);

    let mut r = m.row_at(2);
    r.scale(3.0);
    m.add_row_vector(3, r);

    let mut r = m.row_at(3);
    r.scale(4.0);
    m.add_row_vector(2, r);

    let mut r = m.row_at(2);
    r.scale(2.0);
    m.add_row_vector(1, r);

    let mut r = m.row_at(3);
    r.scale(-1.0);
    m.add_row_vector(1, r);

    println!("{}", m);
}
