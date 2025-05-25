use crate::matrix::*;
use clap::Parser;
use num::Num;
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
    #[arg(short, long, default_value_t = false)]
    migrate: bool,
}

pub fn sum_mode() {
    let cli = SumCli::parse();

    let a = Matrix::<u8>::read_from_png(&cli.input1).unwrap_or_else(|e| {
        eprintln!("Read PNG 1 occurs error: {}", e);
        exit(1);
    });
    let b = Matrix::<u8>::read_from_png(&cli.input2).unwrap_or_else(|e| {
        eprintln!("Read PNG 2 occurs error: {}", e);
        exit(1);
    });
    let matrix = Matrix::<u8>::add(a, b, cli.migrate).unwrap_or_else(|e| {
        eprintln!("Add matrix occurs error: {}", e);
        exit(1);
    });
    matrix.write_to_png(&cli.output).unwrap_or_else(|e| eprintln!("Write PNG occurs error: {}", e));
}

impl<T> Matrix<T>
where
    T: Num + Copy + Clone + Sync + Send + PartialOrd + From<u8>,
{
    pub fn add(a: Matrix<T>, b: Matrix<T>, migrate: bool) -> Result<Matrix<T>, String> {
        if a.rows != b.rows || a.cols != b.cols {
            return Err("The size of two matrix should be same".into());
        }
        let mut result = Matrix::new(a.rows, a.cols);
        result.data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let mut r = [0u8; 4];
            for i in 0..4 {
                let x: f32 = a.data[index][i].into();
                let y: f32 = b.data[index][i].into();
                r[i] = ((x + y) / if migrate { 2.0 } else { 1.0 }).clamp(0.0, 255.0).into();
            }
            *value = r;
        });
        Ok(result)
    }
}
