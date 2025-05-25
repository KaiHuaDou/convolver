use num::Num;

pub enum Pos {
    Max,
    Mid,
    Min,
}

pub struct Neighbors<T>
where
    T: Num + Copy + Clone + Sync + Send + PartialOrd + From<u8>,
{
    pub size: usize,
    pub data: Vec<[T; 4]>,
}

impl<T> Neighbors<T>
where
    T: Num + Copy + Clone + Sync + Send + PartialOrd + From<u8>,
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
            ((sum_r + area / 2.0) / area).into(),
            ((sum_g + area / 2.0) / area).into(),
            ((sum_b + area / 2.0) / area).into(),
            ((sum_a + area / 2.0) / area).into(),
        ]
    }

    #[inline]
    pub fn position(&self, location: Pos) -> [T; 4] {
        positional_value::<T>(&self.data, location)
    }

    #[inline]
    pub fn kernel(&self, kernel: &Vec<f32>) -> [T; 4] {
        let (mut sum_r, mut sum_g, mut sum_b) = (0.0f32, 0.0f32, 0.0f32);

        for (&k, data) in kernel.iter().zip(self.data.iter()) {
            sum_r += data[0].into() * k;
            sum_g += data[1].into() * k;
            sum_b += data[2].into() * k;
        }

        [
            sum_r.clamp(0.0, 255.0).into(),
            sum_g.clamp(0.0, 255.0).into(),
            sum_b.clamp(0.0, 255.0).into(),
            255.into(),
        ]
    }

    #[inline]
    pub fn inner(&self) -> [T; 4] {
        [T::from(0u8); 4]
    }
}

fn positional_value<T>(data: &Vec<[T; 4]>, location: Pos) -> [T; 4]
where
    T: Copy + PartialOrd + From<u8>,
{
    let mut result: [T; 4] = [T::from(0u8); 4];
    result.iter_mut().enumerate().for_each(|(i, value)| {
        let mut channels: Vec<T> = data.iter().map(|pixel| pixel[i]).collect();
        channels.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        let len = data.len();
        *value = match location {
            Pos::Min => channels[0],
            Pos::Max => channels[len - 1],
            Pos::Mid => channels[len / 2],
        };
    });
    result
}
