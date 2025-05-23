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
            if x == "s" {
                sum_mode();
            } else {
                convolve_mode();
            }
        }
        None => convolve_mode(),
    }
}
