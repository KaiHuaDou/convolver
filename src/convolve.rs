use crate::function::*;
use crate::matrix::*;
use clap::Parser;
use rayon::prelude::*;
use std::process::exit;
use std::str::FromStr;
use std::time::Instant;

#[derive(Parser)]
#[command(version)]
#[command(about = "A general image convolver", long_about = None)]
struct ConvolveCli {
    #[arg()]
    mode: Option<String>,
    #[arg(default_value_t = String::from("initial.png"))]
    input: String,
    #[arg(default_value_t = String::from("result.png"))]
    output: String,
    #[arg(short = 't', long, default_value_t = 1)]
    iteration: usize,
    #[arg(short, long ,default_value_t = String::from("3-none"))]
    function: String,
    #[arg(long, default_value_t = '*')]
    indicator: char,
}

pub fn convolve_mode() {
    let cli = ConvolveCli::parse();
    let mut matrix = Matrix::read_from_png(&cli.input).unwrap_or_else(|e| {
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
    println!("Time elapsed: {:?}", duration);

    matrix.write_to_png(&cli.output).unwrap_or_else(|e| {
        eprintln!("Write PNG occurs error: {}", e);
        exit(1);
    });
}

impl Matrix {
    pub fn convolve(&mut self, kernel: &Function) {
        let mut result = vec![[0u8, 0u8, 0u8, 0u8]; self.rows * self.cols];
        let size = match kernel {
            Function::Constant(x, _) => *x,
            Function::Single(x, _, _) => *x,
            Function::Multiple(x, _, _) => *x,
        };
        let iter: isize = (size as isize - 1) / 2;
        let area: usize = size * size;
        let center: usize = (area - 1) / 2;

        result.par_iter_mut().enumerate().for_each(|(index, value)| {
            let row = (index / self.cols) as isize;
            let col = (index % self.cols) as isize;

            let mut neighbors = vec![[0u8; 4]; area];
            for drow in -iter..=iter {
                for dcol in -iter..=iter {
                    let crow = (row + drow).clamp(0, self.rows as isize - 1);
                    let ccol = (col + dcol).clamp(0, self.cols as isize - 1);
                    let nindex = center as isize + dcol + drow * size as isize;
                    let index = crow * self.cols as isize + ccol;
                    unsafe {
                        neighbors[nindex as usize] = *self.data.get_unchecked(index as usize)
                    };
                }
            }

            *value = kernel.calculate(Neighbors { data: neighbors, size: size });
        });

        self.data = result;
    }
}
