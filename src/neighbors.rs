use num::*;

use crate::colormode::ValueLimits;

pub enum Pos {
    Max,
    Mid,
    Min,
}

pub struct Neighbors<T>
where
    T: Num + NumCast + Copy + Clone + Sync + Send + PartialOrd,
{
    pub size: usize,
    pub data: Vec<[T; 4]>,
}

impl<T> Neighbors<T>
where
    T: Num + NumCast + Copy + Clone + Sync + Send + PartialOrd + ValueLimits,
{
    #[inline]
    pub fn none(&self) -> [T; 4] {
        self.data[self.size * self.size / 2]
    }

    #[inline]
    pub fn blur(&self) -> [T; 4] {
        let (sum_r, sum_g, sum_b, sum_a) =
            self.data.iter().fold((0f32, 0f32, 0f32, 0f32), |(r, g, b, a), pixel| {
                (
                    r + <f32 as NumCast>::from(pixel[0]).unwrap(),
                    g + <f32 as NumCast>::from(pixel[1]).unwrap(),
                    b + <f32 as NumCast>::from(pixel[2]).unwrap(),
                    a + <f32 as NumCast>::from(pixel[3]).unwrap(),
                )
            });
        let area = self.data.len() as f32;
        [
            T::from((sum_r + area / 2.0) / area).unwrap(),
            T::from((sum_g + area / 2.0) / area).unwrap(),
            T::from((sum_b + area / 2.0) / area).unwrap(),
            T::from((sum_a + area / 2.0) / area).unwrap(),
        ]
    }

    #[inline]
    pub fn positional(&self, location: Pos) -> [T; 4] {
        let mut result: [T; 4] = [T::from(0u8).unwrap(); 4];
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
    pub fn kernel(&self, kernel: &Vec<f32>) -> [T; 4] {
        let (mut sum_0, mut sum_1, mut sum_2) = (0.0f32, 0.0f32, 0.0f32);

        for (&k, data) in kernel.iter().zip(self.data.iter()) {
            sum_0 += <f32 as NumCast>::from(data[0]).unwrap() * k;
            sum_1 += <f32 as NumCast>::from(data[1]).unwrap() * k;
            sum_2 += <f32 as NumCast>::from(data[2]).unwrap() * k;
        }

        [
            T::from(sum_0).unwrap().clamp(0),
            T::from(sum_1).unwrap().clamp(1),
            T::from(sum_2).unwrap().clamp(2),
            T::from(255u8).unwrap(),
        ]
    }

    #[inline]
    pub fn inner(&self) -> [T; 4] {
        [T::from(0u8).unwrap(); 4]
    }
}
