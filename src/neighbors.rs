use crate::colormode::*;
use fast_math::*;

pub enum Pos {
    Max,
    Mid,
    Min,
}

pub struct Neighbors<T>
where
    T: ColorValue,
{
    pub size: usize,
    pub data: Vec<[T; 4]>,
}

impl<T> Neighbors<T>
where
    T: ColorValue,
{
    #[inline]
    pub fn none(&self) -> [T; 4] {
        self.data[self.size * self.size / 2]
    }

    #[inline]
    pub fn blur(&self) -> [T; 4] {
        let (sum_r, sum_g, sum_b, sum_a) =
            self.data.iter().fold((0f32, 0f32, 0f32, 0f32), |(r, g, b, a), pixel| {
                (r + pixel[0].into(), g + pixel[1].into(), b + pixel[2].into(), a + pixel[3].into())
            });
        let area = self.data.len() as f32;
        [
            T::from((sum_r + area / 2.0) / area),
            T::from((sum_g + area / 2.0) / area),
            T::from((sum_b + area / 2.0) / area),
            T::from((sum_a + area / 2.0) / area),
        ]
    }

    #[inline]
    pub fn positional(&self, location: Pos) -> [T; 4] {
        let mut result: [T; 4] = [T::from(0u8); 4];
        result.iter_mut().enumerate().for_each(|(i, value)| {
            let mut channels: Vec<T> = self.data.iter().map(|pixel| pixel[i]).collect();
            channels.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

            let len = self.data.len();
            *value = match location {
                Pos::Min => channels[0],
                Pos::Max => channels[len - 1],
                Pos::Mid => channels[len / 2],
            };
        });
        result
    }

    #[inline]
    pub fn leave(&self, channel: usize) -> [T; 4] {
        let mut result = [T::from(0u8), T::from(0u8), T::from(0u8), T::from(255u8)];
        result[channel] = self.data[self.size * self.size / 2][channel];
        result
    }

    #[inline]
    pub fn kernel(&self, kernel: &Vec<f32>) -> [T; 4] {
        let (mut sum_0, mut sum_1, mut sum_2) = (0.0f32, 0.0f32, 0.0f32);
        for (&k, data) in kernel.iter().zip(self.data.iter()) {
            sum_0 += data[0].into() * k;
            sum_1 += data[1].into() * k;
            sum_2 += data[2].into() * k;
        }
        [T::from(sum_0).clamp(0), T::from(sum_1).clamp(1), T::from(sum_2).clamp(2), T::from(255u8)]
    }

    #[inline]
    pub fn bilateral_filter(&self, kernel_sigma: &Vec<f32>) -> [T; 4] {
        let center = self.size / 2;
        let area = self.size * self.size;
        let center_pixel = self.data[center * self.size + center];
        let mut result = [0.0f32; 4];
        let mut total_weight = 0.0;
        let sigma_factor = 2.0 * kernel_sigma[area] * kernel_sigma[area];
        let color_factor = -1.0 / (2.0 * sigma_factor);

        for x in 0..self.size {
            for y in 0..self.size {
                let idx = y * self.size + x;
                let pixel = self.data[idx];
                let space_weight = kernel_sigma[idx];

                let diff0 = (pixel[0] - center_pixel[0]).into();
                let diff1 = (pixel[1] - center_pixel[1]).into();
                let diff2 = (pixel[2] - center_pixel[2]).into();
                let diff3 = (pixel[3] - center_pixel[3]).into();
                let range_weight = (exp_raw(diff0 * diff0 * color_factor)
                    + exp_raw(diff1 * diff1 * color_factor)
                    + exp_raw(diff2 * diff2 * color_factor)
                    + exp_raw(diff3 * diff3 * color_factor))
                    / 4.0;

                let weight = space_weight * range_weight;
                total_weight += weight;

                result[0] = result[0] + pixel[0].into() * weight;
                result[1] = result[1] + pixel[1].into() * weight;
                result[2] = result[2] + pixel[2].into() * weight;
                result[3] = result[3] + pixel[3].into() * weight;
            }
        }

        result[0] = result[0] / total_weight;
        result[1] = result[1] / total_weight;
        result[2] = result[2] / total_weight;
        result[3] = result[3] / total_weight;
        [T::from(result[0]), T::from(result[1]), T::from(result[2]), T::from(result[3])]
    }
}
