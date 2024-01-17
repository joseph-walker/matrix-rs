use crate::vector::*;
use std::fmt::Display;

pub struct Matrix {
    pub num_rows: usize,
    pub num_cols: usize,
    pub rows: Box<[usize]>,
    pub values: Box<[f64]>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();

        for i in 1..=self.num_rows {
            for j in 1..=self.num_cols {
                buffer.push_str(&format!("{}\t", self.at(i, j)).to_string());
            }

            buffer.push_str("\n");
        }

        write!(f, "{}", buffer)
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, values: Vec<f64>) -> Matrix {
        if rows * cols != values.len() {
            panic!(
                "Expected {} values for {} x {} matrix, but got {} vaues.",
                rows * cols,
                rows,
                cols,
                values.len()
            );
        }

        Matrix {
            num_rows: rows,
            num_cols: cols,
            rows: (0..rows)
                .into_iter()
                .map(|i| i * cols)
                .collect::<Vec<usize>>()
                .into_boxed_slice(),
            values: values.into_boxed_slice(),
        }
    }

    pub fn at(&self, row: usize, col: usize) -> &f64 {
        let idx = self.rows[row - 1] + col - 1;

        &self.values[idx]
    }

    pub fn row_at(&self, row: usize) -> Vector {
        let idx = self.rows[row - 1];

        let row = (&self.values[idx..idx + self.num_cols])
            .to_owned()
            .into_boxed_slice();

        Vector::new_from_slice(row)
    }

    pub fn col_at(&self, col: usize) -> Vector {
        let mut col_vals = vec![];
        
        for row in 1..=self.num_rows {
            col_vals.push(*self.at(row, col));
        }

        Vector::new_from_slice(col_vals.into_boxed_slice())
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut f64 {
        let idx = self.rows[row - 1] + col - 1;

        &mut self.values[idx]
    }

    pub fn scale_row(&mut self, row: usize, scale: f64) -> () {
        let idx = self.rows[row - 1];

        for i in idx..idx + self.num_cols {
            self.values[i] *= scale;
        }
    }

    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) -> () {
        let old_a = self.rows[row_a - 1];

        self.rows[row_a - 1] = self.rows[row_b - 1];
        self.rows[row_b - 1] = old_a;
    }

    pub fn add_row_vector(&mut self, row: usize, vector: Vector) -> () {
        let offset = self.rows[row - 1];

        for i in 0..self.num_cols {
            let idx = offset + i;
            self.values[idx] += vector.0[i];
        }
    }
}
