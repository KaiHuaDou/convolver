#![allow(dead_code)]

mod convolve;
mod matrix;

use crate::convolve::*;
use crate::matrix::*;
use std::env;

fn main() {
    let iteration = env::args().nth(1).unwrap().parse().unwrap();
    let mut matrix = Matrix::read_from_png("initial.png").unwrap();
    for _ in 0..iteration {
        matrix.convolve(canny);
        print!("*");
    }
    match matrix.write_to_png("result.png") {
        Ok(()) => {}
        Err(e) => eprintln!("发生错误：{}", e),
    }
}
