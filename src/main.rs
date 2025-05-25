#![allow(dead_code)]

mod convolve;
mod function;
mod matrix;
mod neighbors;
mod sum;

use crate::convolve::*;
use crate::sum::*;
use std::env::args_os;

fn main() {
    match args_os().nth(1) {
        Some(x) => {
            if x == "sum" {
                sum_mode();
            } else if x == "rgba" {
                convolve_mode::<u8>();
            } else if x == "hsla" {
                convolve_mode::<f32>();
            }
        }
        None => convolve_mode::<u8>(),
    }
}
