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
        let size = match parts[0].parse::<usize>() {
            Ok(x) => x,
            Err(e) => return Err(format!("Invalid function size: {}", e)),
        };
        if parts.len() < 2 {
            return Err("Invalid function".into());
        }
        match parts[1] {
            "none" => Ok(Function::Constant(size, Arc::new(|n| n.none()))),
            "blur" => Ok(Function::Constant(size, Arc::new(|n| n.blur()))),
            "gauss" => {
                let sigma = match parts[3].parse::<f32>() {
                    Ok(x) => x,
                    Err(_) => 1.0,
                };
                match parts[2] {
                    "blur" => Ok(match gauss_blur_function(size, sigma) {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(format!("Failed to create gauss blur function: {}", e));
                        }
                    }),
                    "sharpen" => Ok(match gauss_sharpen_function(size, sigma) {
                        Ok(x) => x,
                        Err(e) => {
                            return Err(format!("Failed to create gauss sharpen function: {}", e));
                        }
                    }),
                    _ => Err("Unknown function type".into()),
                }
            }
            _ => Ok(Function::Multiple(
                size,
                Arc::new(|n, i| n.core(&i)),
                match CANNY_KERNERLS.get(parts[1]) {
                    Some(x) => {
                        if x.len() == size * size {
                            x.to_vec()
                        } else {
                            return Err("Unknown function type".into());
                        }
                    }
                    None => return Err("Unknown function type".into()),
                },
            )),
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
        let (mut sum_r, mut sum_g, mut sum_b) = (0.0f32, 0.0f32, 0.0f32);

        for (&k, data) in kernel.iter().zip(self.data.iter()) {
            sum_r += data[0] as f32 * k;
            sum_g += data[1] as f32 * k;
            sum_b += data[2] as f32 * k;
        }

        [
            sum_r.clamp(0.0, 255.0) as u8,
            sum_g.clamp(0.0, 255.0) as u8,
            sum_b.clamp(0.0, 255.0) as u8,
            255,
        ]
    }
}

fn gauss_blur_function(size: usize, sigma: f32) -> Result<Function, String> {
    if size % 2 != 1 && size >= 3 {
        return Err("Kernel size must be an odd number, received {}".into());
    }
    if sigma <= 0.0 {
        return Err("Sigma must be a positive value".into());
    }

    let mut kernel = vec![0f32; size * size];
    let center = (size / 2) as f32;
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
    Ok(Function::Multiple(size, Arc::new(|n, i| n.core(&i)), kernel))
}

fn gauss_sharpen_function(size: usize, sigma: f32) -> Result<Function, String> {
    let (closure, mut kernel) = match gauss_blur_function(size, sigma)? {
        Function::Multiple(_, closure, x) => (closure, x),
        _ => unimplemented!(),
    };

    let center = size / 2;
    let center_idx = center * size + center;
    for i in 0..size * size {
        if i == center_idx {
            kernel[i] = 2.0 - kernel[i];
        } else {
            kernel[i] = -kernel[i];
        }
    }

    Ok(Function::Multiple(size, closure, kernel))
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
            "laplacian_m",
            vec![
                0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0, 0.0, -1.0, -2.0, 16.0, -2.0, -1.0,
                0.0, -1.0, -2.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            ],
        );
        x
    };
}
