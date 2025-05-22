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

    fn gauss_blur_function(size: usize, sigma: f32) -> Result<Function, String> {
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
        let (closure, mut kernel) = match Function::gauss_blur_function(size, sigma)? {
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

    fn motion_blur_function(size: usize, l: f32, theta: f32) -> Result<Function, String> {
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

        let points = Function::bresenham_line(x0, y0, x1, y1);

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

        Ok(Function::Multiple(size, Arc::new(|n, i| n.core(&i)), kernel))
    }

    fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let (mut x, mut y) = (x0, y0);
        let max_steps = dx.max(-dy);
        let mut points = Vec::with_capacity(max_steps as usize + 1); // 预分配容量

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
    }
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
            kernel_name => {
                let kernel = CANNY_KERNELS.get(kernel_name).ok_or("Unknown function type")?;
                if kernel.len() != size * size {
                    return Err("Kernel size mismatch".into());
                }
                Ok(Function::Multiple(size, Arc::new(|n, i| n.core(&i)), kernel.to_vec()))
            }
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
        let (sum_r, sum_g, sum_b, sum_a) =
            self.data.iter().fold((0usize, 0usize, 0usize, 0usize), |(r, g, b, a), pixel| {
                (
                    r + pixel[0] as usize,
                    g + pixel[1] as usize,
                    b + pixel[2] as usize,
                    a + pixel[3] as usize,
                )
            });
        let area = self.data.len() as usize;
        [
            ((sum_r + area / 2) / area) as u8,
            ((sum_g + area / 2) / area) as u8,
            ((sum_b + area / 2) / area) as u8,
            ((sum_a + area / 2) / area) as u8,
        ]
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

lazy_static! {
    pub static ref CANNY_KERNELS: HashMap<&'static str, Vec<f32>> = {
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
