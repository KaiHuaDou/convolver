use crate::matrix::*;
use clap::Parser;
use rayon::prelude::*;
use std::process::exit;

#[derive(Parser)]
#[command(version)]
#[command(about = "A general image sum tool", long_about = None)]
struct SumCli {
    #[arg()]
    mode: Option<String>,
    #[arg()]
    input1: String,
    #[arg()]
    input2: String,
    #[arg()]
    output: String,
}

pub fn sum_mode() {
    let cli = SumCli::parse();

    let a = match Matrix::read_from_png(&cli.input1) {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Read PNG 1 occurs error: {}", e);
            exit(1);
        }
    };
    let b = match Matrix::read_from_png(&cli.input2) {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Read PNG 2 occurs error: {}", e);
            exit(1);
        }
    };
    let matrix = match Matrix::add(a, b) {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Add matrix occurs error: {}", e);
            exit(1);
        }
    };
    match matrix.write_to_png(&cli.output) {
        Ok(()) => {}
        Err(e) => eprintln!("Write PNG occurs error: {}", e),
    }
}

impl Matrix {
    pub fn add(a: Matrix, b: Matrix) -> Result<Matrix, String> {
        if a.rows != b.rows || a.rows != b.rows {
            return Err("The size of two matrix should be same".to_string());
        }
        let mut result = Matrix::new(a.rows, a.cols);
        result.data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let mut r = [0u8; 4];
            for i in 0..4 {
                let x = a.data[index][i] as f32;
                let y = b.data[index][i] as f32;
                r[i] = ((x + y) / 2.0) as u8;
            }
            *value = r;
        });
        Ok(result)
    }
}
