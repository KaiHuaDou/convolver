use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

type KernelClosure = Arc<dyn Fn(Neighbors) -> [u8; 4] + Sync + Send>;
type KernelSingleClosure = Arc<dyn Fn(Neighbors, f32) -> [u8; 4] + Sync + Send>;
type KernelMultipleClosure = Arc<dyn Fn(Neighbors, &Vec<f32>) -> [u8; 4] + Sync + Send>;

#[derive(Clone)]
pub enum Function {
    Constant(usize, KernelClosure),
    Single(usize, KernelSingleClosure, f32),
    Multiple(usize, KernelMultipleClosure, Vec<f32>),
}

impl Function {
    pub fn calculate(&self, input: Neighbors) -> [u8; 4] {
        match self {
            Function::Constant(_, f) => f(input),
            Function::Single(_, f, x) => f(input, *x),
            Function::Multiple(_, f, x) => f(input, x),
        }
    }
}

impl FromStr for Function {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').map(|x| x.trim()).collect();
        match parts[0] {
            "none" => Ok(Function::Constant(3, Arc::new(|i| i.none()))),
            "gass_blur" => {
                let size = match parts[1].parse::<usize>() {
                    Ok(x) => x,
                    Err(e) => return Err(format!("Invalid kernel size: {}", e)),
                };
                let sigma = match parts[2].parse::<f32>() {
                    Ok(x) => x,
                    Err(e) => return Err(format!("Invalid sigma value: {}", e)),
                };
                let kernel = match gaussian_blur_kernel(size, sigma) {
                    Ok(x) => x,
                    Err(e) => return Err(format!("Failed to create Gaussian blur kernel: {}", e)),
                };
                Ok(kernel)
            }
            _ => Err("Unknown kernel type".to_string()),
        }
    }
}

pub struct Neighbors {
    pub size: usize,
    pub data: Vec<[u8; 4]>,
}

impl Neighbors {
    pub fn none(&self) -> [u8; 4] {
        self.data[self.size * self.size / 2]
    }

    pub fn blur(&self) -> [u8; 4] {
        let mut sums = [0u32; 4];
        for pixel in &self.data {
            for channel in 0..4 {
                sums[channel] += pixel[channel] as u32;
            }
        }
        let area = (self.size * self.size) as u32;
        [
            ((sums[0] + area / 2) / area) as u8,
            ((sums[1] + area / 2) / area) as u8,
            ((sums[2] + area / 2) / area) as u8,
            ((sums[3] + area / 2) / area) as u8,
        ]
    }

    pub fn move_blur(&self) -> [u8; 4] {
        let mut output = [0u8; 4];
        let iter = self.size / 2;
        for channel in 0..=2 {
            let mut sum: f32 = 0.0;
            for (i, &v) in self.data.iter().enumerate() {
                if (i + 1) / self.size == iter as usize + 1 {
                    sum += v[channel] as f32 / self.size as f32
                }
            }
            output[channel] = sum.clamp(0.0, 255.0) as u8;
        }
        output[3] = 255;

        output
    }

    pub fn core(&self, kernel: &Vec<f32>) -> [u8; 4] {
        let mut output = [0u8; 4];
        let kernel = normalize(&kernel);
        for channel in 0..=2 {
            let mut sum: f32 = 0.0;
            for (i, &k) in kernel.iter().enumerate() {
                sum += self.data[i][channel] as f32 * k;
            }
            output[channel] = sum.clamp(0.0, 255.0) as u8;
        }
        output[3] = 255;

        output
    }
}

fn gaussian_blur_kernel(size: usize, sigma: f32) -> Result<Function, String> {
    if size % 2 != 1 {
        return Err("Kernel size must be an odd number, received {}".to_string());
    }
    if sigma <= 0.0 {
        return Err("Sigma must be a positive value".to_string());
    }

    let mut kernel = vec![0f32; size * size];
    let center = (size as f32 - 1.0) / 2.0;
    let mut sum = 0.0;

    for i in 0..size {
        for j in 0..size {
            let x = i as f32 - center;
            let y = j as f32 - center;
            let exponent = -(x.powi(2) + y.powi(2)) / (2.0 * sigma.powi(2));
            let value = exponent.exp();
            kernel[i * size + j] = value;
            sum += value;
        }
    }
    kernel = kernel.iter().map(|&x| x / sum).collect();
    Ok(Function::Multiple(size, Arc::new(|n, i| n.core(i)), kernel))
}

lazy_static! {
    pub static ref CANNY_KERNERLS: HashMap<&'static str, Vec<f32>> = {
        let mut x = HashMap::new();
        x.insert("sobel_h", vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]);
        x.insert("sobel_v", vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0]);
        x.insert("prewitt_h", vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0]);
        x.insert("prewitt_v", vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        x.insert("scharr_h", vec![-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0]);
        x.insert("scharr_v", vec![-3.0, -10.0, -3.0, 0.0, 0.0, 0.0, 3.0, 10.0, 3.0]);
        x.insert("laplacian_4", vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0]);
        x.insert("laplacian_8", vec![1.0, 1.0, 1.0, 1.0, -8.0, 1.0, 1.0, 1.0, 1.0]);
        x.insert("laplacian_8r", vec![-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0]);
        x.insert(
            "laplacian_5x5",
            vec![
                0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0, 0.0, -1.0, -2.0, 16.0, -2.0, -1.0,
                0.0, -1.0, -2.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            ],
        );
        x
    };
}

pub fn normalize(kernel: &Vec<f32>) -> Vec<f32> {
    let sum: f32 = kernel.iter().sum();
    if sum == 0.0 { kernel.to_vec() } else { kernel.iter().map(|&x| x / sum).collect() }
}
