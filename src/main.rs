#![allow(dead_code)]
#![feature(trait_alias)]

mod add;
mod colormode;
mod convolve;
mod function;
mod io;
mod matrix;
mod neighbors;

use crate::add::*;
use crate::colormode::*;
use crate::convolve::*;
use std::env::args_os;

fn main() {
    match args_os().nth(1) {
        Some(x) if x == "add" => add_cli(),
        Some(x) if x == "rgba" => convolve_cli::<Rgba>(),
        Some(x) if x == "hsla" => convolve_cli::<Hsla>(),
        Some(x) if x == "luva" => convolve_cli::<Luva>(),
        _ => {
            println!("Warning: unknown mode, fallback to RGBA");
            convolve_cli::<Rgba>();
        }
    }
}
