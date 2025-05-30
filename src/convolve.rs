use crate::colormode::*;
use crate::function::*;
use crate::matrix::*;
use crate::neighbors::*;
use clap::Parser;
use rayon::prelude::*;
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

#[derive(Parser)]
#[command(version)]
#[command(about = "A general image convolver", long_about = None)]
struct ConvolveCli {
    #[arg(default_value_t = String::from("rgba"))]
    mode: String,
    #[arg(default_value_t = String::from("input.png"))]
    input: String,
    #[arg(default_value_t = String::from("output.png"))]
    output: String,
    #[arg(short = 't', long, default_value_t = 1)]
    iteration: usize,
    #[arg(short, long ,default_value_t = String::from("3-none"))]
    function: String,
    #[arg(long, default_value_t = '*')]
    indicator: char,
}

pub fn convolve_cli<T>()
where
    T: ColorValue + 'static,
{
    let cli = ConvolveCli::parse();
    let mut matrix: Matrix<T> = Matrix::<T>::read_png(&cli.input).unwrap_or_else(|e| {
        eprintln!("Read PNG occurs error: {}", e);
        exit(1);
    });

    let function = Function::from_str(&cli.function).unwrap_or_else(|e| {
        eprintln!("Invalid function: {}", e);
        exit(1);
    });

    let start = Instant::now();
    for _ in 0..cli.iteration {
        matrix.convolve(&function);
        print!("{}", cli.indicator);
    }
    let duration = start.elapsed();
    println!("\nTime elapsed: {:?}", duration);

    matrix.write_png(&cli.output).unwrap_or_else(|e| {
        eprintln!("Write PNG occurs error: {}", e);
        exit(1);
    });
}

impl<T> Matrix<T>
where
    T: ColorValue + 'static,
{
    pub fn convolve(&mut self, kernel: &Function<T>) {
        let mut result = vec![[T::from(0u8); 4]; self.rows * self.cols];
        let size = kernel.size();
        let iter: isize = (size as isize - 1) / 2;
        let area: usize = size * size;
        let center: usize = (area - 1) / 2;

        result.par_iter_mut().enumerate().for_each(|(index, value)| {
            let row = (index / self.cols) as isize;
            let col = (index % self.cols) as isize;

            let mut neighbors = vec![[T::from(0u8); 4]; area];
            for drow in -iter..=iter {
                for dcol in -iter..=iter {
                    let crow = (row + drow).clamp(0, self.rows as isize - 1);
                    let ccol = (col + dcol).clamp(0, self.cols as isize - 1);
                    let nindex = center as isize + dcol + drow * size as isize;
                    neighbors[nindex as usize] = self.get(crow, ccol);
                }
            }

            *value = kernel.calculate(Neighbors { data: neighbors, size: size });
        });

        self.data = result;
    }
}
