use crate::matrix::*;
use rayon::prelude::*;

pub const SIZE: usize = 3;
pub const ITER: isize = (SIZE as isize - 1) / 2;
pub const AREA: usize = SIZE * SIZE;
pub const CENTER: usize = (AREA - 1) / 2;

impl Matrix {
    pub fn convolve<F>(&mut self, f: F)
    where
        F: Fn([[u8; 4]; AREA]) -> [u8; 4] + Send + Sync,
    {
        let mut result = vec![[0u8, 0u8, 0u8, 0u8]; self.rows * self.cols];

        result.par_iter_mut().enumerate().for_each(|(index, value)| {
            let row = (index / self.cols) as isize;
            let col = (index % self.cols) as isize;

            let mut neighbors = [[0u8, 0u8, 0u8, 0u8]; AREA];
            for drow in -ITER..=ITER {
                for dcol in -ITER..=ITER {
                    let crow = (row + drow).clamp(0, self.rows as isize - 1);
                    let ccol = (col + dcol).clamp(0, self.cols as isize - 1);
                    let nindex = CENTER as isize + dcol + drow * SIZE as isize;
                    let index = crow * self.cols as isize + ccol;
                    neighbors[nindex as usize] = self.data[index as usize];
                }
            }

            *value = f(neighbors);
        });

        self.data = result;
    }
}

pub fn none(input: [[u8; 4]; AREA]) -> [u8; 4] {
    input[CENTER - 4]
}

pub fn blur(input: [[u8; 4]; AREA]) -> [u8; 4] {
    let mut sums = [0u32; 4];
    for pixel in &input {
        for channel in 0..4 {
            sums[channel] += pixel[channel] as u32;
        }
    }
    let area = AREA as u32;
    [
        ((sums[0] + area / 2) / area) as u8,
        ((sums[1] + area / 2) / area) as u8,
        ((sums[2] + area / 2) / area) as u8,
        ((sums[3] + area / 2) / area) as u8,
    ]
}

const SOBEL_HORIZONTAL: [i32; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];
const SOBEL_VERTICAL: [i32; 9] = [-1, -2, -1, 0, 0, 0, 1, 2, 1];
const PREWITT_HORIZONTAL: [i32; 9] = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
const PREWITT_VERTICAL: [i32; 9] = [-1, -1, -1, 0, 0, 0, 1, 1, 1];
const SCHARR_HORIZONTAL: [i32; 9] = [-3, 0, 3, -10, 0, 10, -3, 0, 3];
const SCHARR_VERTICAL: [i32; 9] = [-3, -10, -3, 0, 0, 0, 3, 10, 3];
const LAPLACIAN_4NEIGHBOR: [i32; 9] = [0, 1, 0, 1, -4, 1, 0, 1, 0];
const LAPLACIAN_8NEIGHBOR: [i32; 9] = [1, 1, 1, 1, -8, 1, 1, 1, 1];
const LAPLACIAN_8NEIGHBOR_ALT: [i32; 9] = [-1, -1, -1, -1, 8, -1, -1, -1, -1];

pub fn canny(input: [[u8; 4]; AREA]) -> [u8; 4] {
    let kernel: [i32; AREA] = LAPLACIAN_8NEIGHBOR_ALT;
    let mut output = [0u8; 4];

    for channel in 0..=2 {
        let mut sum: i32 = 0;
        for (j, &k) in kernel.iter().enumerate() {
            let pixel = input[j][channel] as i32;
            sum += pixel * k;
        }
        output[channel] = sum.abs().clamp(0, 255) as u8;
    }
    output[3] = 255;

    output
}
