use crate::colormode::*;
use crate::matrix::*;
use clap::Parser;
use rayon::prelude::*;
use std::process::exit;

#[derive(Parser)]
#[command(version)]
#[command(about = "A general image sum tool", long_about = None)]
struct AddCli {
    #[arg()]
    mode: String,
    #[arg()]
    input1: String,
    #[arg()]
    input2: String,
    #[arg()]
    output: String,
    #[arg(short, long, default_value_t = false)]
    migrate: bool,
}

pub fn add_cli() {
    let cli = AddCli::parse();

    let a = Matrix::<Rgba>::_read_png(&cli.input1).unwrap_or_else(|e| {
        eprintln!("Read PNG 1 occurs error: {}", e);
        exit(1);
    });
    let b = Matrix::<Rgba>::_read_png(&cli.input2).unwrap_or_else(|e| {
        eprintln!("Read PNG 2 occurs error: {}", e);
        exit(1);
    });
    let matrix = Matrix::<Rgba>::add(a, b, cli.migrate).unwrap_or_else(|e| {
        eprintln!("Add matrix occurs error: {}", e);
        exit(1);
    });
    matrix.write_png(&cli.output).unwrap_or_else(|e| eprintln!("Write PNG occurs error: {}", e));
}

impl<T> Matrix<T>
where
    T: ColorValue + 'static,
{
    pub fn add(a: Matrix<T>, b: Matrix<T>, migrate: bool) -> Result<Matrix<T>, String> {
        if a.rows != b.rows || a.cols != b.cols {
            return Err("The size of two matrix should be same".into());
        }
        let mut result = Matrix::<T>::new(a.rows, a.cols);
        result.data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let mut r = [T::from(0u8); 4];
            for i in 0..4 {
                let x: f32 = a.data[index][i].into();
                let y: f32 = b.data[index][i].into();
                r[i] = T::from(((x + y) / if migrate { 2.0 } else { 1.0 }).clamp(0.0, 255.0));
            }
            *value = r;
        });
        Ok(result)
    }
}
