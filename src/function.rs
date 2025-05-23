use crate::neighbors::*;
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

lazy_static! {
    pub static ref STATIC_KERNELS: HashMap<&'static str, Vec<f32>> = {
        let mut x = HashMap::with_capacity(14);
        x.insert("edge", vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]);
        x.insert("emboss", vec![-1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0]);
        x.insert("kirsch_n", vec![5.0, 5.0, 5.0, -3.0, 0.0, -3.0, -3.0, -3.0, -3.0]);
        x.insert("kirsch_nw", vec![5.0, 5.0, -3.0, 5.0, 0.0, -3.0, -3.0, -3.0, -3.0]);
        x.insert("kirsch_w", vec![5.0, -3.0, -3.0, 5.0, 0.0, -3.0, 5.0, -3.0, -3.0]);
        x.insert("kirsch_sw", vec![-3.0, -3.0, -3.0, 5.0, 0.0, -3.0, 5.0, 5.0, -3.0]);
        x.insert("kirsch_s", vec![-3.0, -3.0, -3.0, -3.0, 0.0, -3.0, 5.0, 5.0, 5.0]);
        x.insert("kirsch_se", vec![-3.0, -3.0, -3.0, -3.0, 0.0, 5.0, -3.0, 5.0, 5.0]);
        x.insert("kirsch_e", vec![-3.0, -3.0, 5.0, -3.0, 0.0, 5.0, -3.0, -3.0, 5.0]);
        x.insert("kirsch_ne", vec![-3.0, 5.0, 5.0, -3.0, 0.0, 5.0, -3.0, -3.0, -3.0]);
        x.insert("robinson_n", vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0]);
        x.insert("robinson_ne", vec![0.0, -1.0, -2.0, 1.0, 0.0, -1.0, 2.0, 1.0, 0.0]);
        x.insert("robinson_e", vec![1.0, 0.0, -1.0, 2.0, 0.0, -2.0, 1.0, 0.0, -1.0]);
        x.insert("robinson_se", vec![2.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, -1.0, -2.0]);
        x.insert("robinson_s", vec![1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0]);
        x.insert("robinson_sw", vec![0.0, 1.0, 2.0, -1.0, 0.0, 1.0, -2.0, -1.0, 0.0]);
        x.insert("robinson_w", vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]);
        x.insert("robinson_nw", vec![-2.0, -1.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 2.0]);
        x.insert("laplacian_4", vec![0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0]);
        x.insert("laplacian_8", vec![1.0, 1.0, 1.0, 1.0, -8.0, 1.0, 1.0, 1.0, 1.0]);
        x.insert("laplacian_8r", vec![-1.0, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0]);
        x.insert("prewitt_h", vec![-1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0]);
        x.insert("prewitt_v", vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        x.insert("scharr_h", vec![-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0]);
        x.insert("scharr_v", vec![-3.0, -10.0, -3.0, 0.0, 0.0, 0.0, 3.0, 10.0, 3.0]);
        x.insert("sharpen", vec![0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]);
        x.insert("sharpen+", vec![-1.0, -1.0, -1.0, -1.0, 9.0, -1.0, -1.0, -1.0, -1.0]);
        x.insert(
            "unsharp_masking",
            vec![-1.0, -2.0, -1.0, -2.0, 28.0, -2.0, -1.0, -2.0, -1.0]
                .iter()
                .map(|x| x / 16.0)
                .collect(),
        );
        x.insert(
            "laplacian_og",
            vec![
                0.0, 0.0, -1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0, 0.0, -1.0, -2.0, 16.0, -2.0, -1.0,
                0.0, -1.0, -2.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            ],
        );
        x
    };
}

impl FromStr for Function {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').map(|x| x.trim()).collect();
        if parts.len() < 2 {
            return Err("Invalid function".into());
        }
        let size =
            parts[0].parse::<usize>().map_err(|e| format!("Invalid function size: {}", e))?;
        if size % 2 != 1 && size >= 3 {
            return Err("Kernel size must be an odd number, received {}".into());
        }
        match parts[1] {
            "none" => Ok(Function::Constant(size, Arc::new(|n| n.none()))),
            "blur" => Ok(Function::Constant(size, Arc::new(|n| n.blur()))),
            "motion" => {
                if parts.len() < 4 {
                    return Err("Invalid motion function format".into());
                }
                let l = parts[2].parse::<f32>().unwrap_or(1.0);
                let theta = parts[3].parse::<f32>().unwrap_or(0.0);
                Function::motion_blur_function(size, l, theta)
                    .map_err(|e| format!("Failed to create motion blur function: {}", e))
            }
            "gauss" => {
                if parts.len() < 4 {
                    return Err("Invalid gauss function format".into());
                }
                let sigma = parts[3].parse::<f32>().unwrap_or(1.0);
                match parts[2] {
                    "blur" => Function::gauss_blur_function(size, sigma)
                        .map_err(|e| format!("Failed to create gauss blur function: {}", e)),
                    "sharpen" => Function::gauss_sharpen_function(size, sigma)
                        .map_err(|e| format!("Failed to create gauss sharpen function: {}", e)),
                    _ => Err("Unknown gauss function type".into()),
                }
            }
            "dog" => {
                if parts.len() < 4 {
                    return Err("Invalid DoG function format".into());
                }
                let sigma1 = parts[2].parse::<f32>().unwrap_or(1.0);
                let sigma2 = parts[3].parse::<f32>().unwrap_or(1.0);
                Function::generate_dog_kernel(size, sigma1, sigma2)
                    .map_err(|e| format!("Failed to creat DoG function: {}", e))
            }
            "emboss" => {
                if parts.len() < 3 {
                    return Err("Invalid emboss function format".into());
                }
                let direction = parts[2].to_string();
                Function::emboss_function(size, direction)
                    .map_err(|e| format!("Failed to creat emboss function: {}", e))
            }
            kernel_name => {
                let kernel = STATIC_KERNELS.get(kernel_name).ok_or("Unknown function type")?;
                if kernel.len() != size * size {
                    return Err("Kernel size mismatch".into());
                }
                Ok(Function::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel.to_vec()))
            }
        }
    }
}

impl Function {
    #[inline]
    pub fn calculate(&self, input: Neighbors) -> [u8; 4] {
        match self {
            Self::Constant(_, f) => f(input),
            Self::Single(_, f, x) => f(input, *x),
            Self::Multiple(_, f, x) => f(input, x),
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        match self {
            Self::Constant(x, _) => *x,
            Self::Single(x, _, _) => *x,
            Self::Multiple(x, _, _) => *x,
        }
    }

    #[inline]
    pub fn vector(self) -> Option<Vec<f32>> {
        match self {
            Self::Multiple(_, _, vector) => Some(vector),
            _ => None,
        }
    }

    fn gauss_blur_function(size: usize, sigma: f32) -> Result<Self, String> {
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
        Ok(Self::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel))
    }

    fn gauss_sharpen_function(size: usize, sigma: f32) -> Result<Self, String> {
        let mut kernel = Self::gauss_blur_function(size, sigma)?.vector().unwrap();
        let center = size / 2;
        let center_idx = center * size + center;
        for i in 0..size * size {
            if i == center_idx {
                kernel[i] = 2.0 - kernel[i];
            } else {
                kernel[i] = -kernel[i];
            }
        }

        Ok(Self::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel))
    }

    pub fn generate_dog_kernel(size: usize, sigma1: f32, sigma2: f32) -> Result<Self, String> {
        let gauss1 = Self::gauss_blur_function(size, sigma1)?.vector().unwrap();
        let gauss2 = Self::gauss_blur_function(size, sigma2)?.vector().unwrap();
        let kernel = gauss1.into_iter().zip(gauss2.into_iter()).map(|(a, b)| a - b).collect();
        Ok(Self::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel))
    }

    fn motion_blur_function(size: usize, l: f32, theta: f32) -> Result<Self, String> {
        if l <= 0.0 || size < l as usize {
            return Err("l should be positive and less than or equal to the kernel size".into());
        }
        let mut kernel = vec![0.0; size * size];

        let center = (size / 2) as i32;
        let (x0, y0) = (center, center);

        let theta = theta.to_radians();
        let dx = l * theta.cos();
        let dy = -l * theta.sin();
        let x1 = (x0 as f32 + dx).round() as i32;
        let y1 = (y0 as f32 + dy).round() as i32;

        let points = {
            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx + dy;
            let (mut x, mut y) = (x0, y0);
            let max_steps = dx.max(-dy);
            let mut points = Vec::with_capacity(max_steps as usize + 1);

            loop {
                points.push((x, y));
                if x == x1 && y == y1 {
                    break;
                }
                let e2 = 2 * err;
                if e2 >= dy {
                    err += dy;
                    x += sx;
                }
                if e2 <= dx {
                    err += dx;
                    y += sy;
                }
            }

            points
        };

        let valid_points: Vec<_> = points
            .into_iter()
            .filter_map(|(x, y)| {
                if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .collect();

        let sum = valid_points.len() as f32;
        let weight = 1.0 / sum;

        for (x, y) in valid_points {
            kernel[y * size + x] = weight;
        }

        Ok(Self::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel))
    }

    fn emboss_function(size: usize, direction: String) -> Result<Function, String> {
        let center = size / 2;
        let (dx, dy): (isize, isize) = match direction.to_lowercase().as_str() {
            "n" => (0, -1),
            "s" => (0, 1),
            "e" => (1, 0),
            "w" => (-1, 0),
            "nw" => (-1, -1),
            "ne" => (1, -1),
            "sw" => (-1, 1),
            "se" => (1, 1),
            _ => panic!("Invalid direction string"),
        };

        let sum_abs = dx.abs() + dy.abs();
        let max_abs = sum_abs * (center as isize);
        let max_abs = if max_abs == 0 { 1 } else { max_abs };

        let mut kernel = Vec::with_capacity(size * size);

        for i in 0..size {
            for j in 0..size {
                let rel_x = j as isize - center as isize;
                let rel_y = i as isize - center as isize;
                let dot_product = rel_x * dx + rel_y * dy;
                let weight = -(dot_product as f32) / (max_abs as f32);
                kernel.push(weight);
            }
        }

        Ok(Self::Multiple(size, Arc::new(|n, i| n.kernel(&i)), kernel))
    }
}
