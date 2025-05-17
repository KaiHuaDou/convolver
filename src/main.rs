#![allow(dead_code)]

mod matrix;
mod matrix_rules;

use crate::matrix::*;
use crate::matrix_rules::*;
use rand::Rng;
use std::env;

fn main() {
    // let mut matrix = match Matrix::read_from_text("initial.txt") {
    //     Ok(value) => value,
    //     Err(e) => {
    //         eprintln!("发生错误：{}", e);
    //         return;
    //     }
    // };
    let size = env::args().nth(1).unwrap().parse().unwrap();
    let iteration = env::args().nth(2).unwrap().parse().unwrap();
    let mut matrix = generate_bool_matrix(size);

    match matrix.write_to_png("initial.png") {
        Ok(()) => {}
        Err(e) => eprintln!("发生错误：{}", e),
    }

    for _ in 0..iteration {
        print!("*");
        matrix.convolve(conways_rule);
    }

    match matrix.write_to_png("result.png") {
        Ok(()) => {}
        Err(e) => eprintln!("发生错误：{}", e),
    }
}

fn generate_bool_matrix(size: usize) -> Matrix {
    let n: usize = size * size;
    let mut rng = rand::rng();
    let data = (0..n).map(|_| rng.random_bool(0.3)).collect();
    Matrix { rows: size, cols: size, data: data }
}
