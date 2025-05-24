use palette::{Hsla, IntoColor, Srgba};

pub enum Pos {
    Max,
    Mid,
    Min,
}

pub struct Neighbors {
    pub size: usize,
    pub data: Vec<[u8; 4]>,
}

impl Neighbors {
    pub fn to_hsla(&self) -> Vec<[f32; 4]> {
        self.data
            .iter()
            .map(|rgba| {
                let hsla: palette::Hsla = Srgba::new(
                    rgba[0] as f32 / 255.0,
                    rgba[1] as f32 / 255.0,
                    rgba[2] as f32 / 255.0,
                    rgba[3] as f32 / 255.0,
                )
                .into_color();
                [hsla.hue.into_degrees(), hsla.saturation, hsla.lightness, hsla.alpha]
            })
            .collect()
    }

    #[inline]
    pub fn none(&self) -> [u8; 4] {
        self.data[self.size * self.size / 2]
    }

    #[inline]
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

    #[inline]
    pub fn position(&self, location: Pos) -> [u8; 4] {
        positional_value::<u8>(&self.data, location)
    }

    #[inline]
    pub fn position_hsla(&self, location: Pos) -> [u8; 4] {
        let data = self.to_hsla();
        let hsla = positional_value::<f32>(&data, location);
        let result: palette::Srgba = Hsla::new(hsla[0], hsla[1], hsla[2], hsla[3]).into_color();
        [
            (result.red * 255.0) as u8,
            (result.green * 255.0) as u8,
            (result.blue * 255.0) as u8,
            (result.alpha * 255.0) as u8,
        ]
    }

    #[inline]
    pub fn kernel(&self, kernel: &Vec<f32>) -> [u8; 4] {
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

    #[inline]
    pub fn inner(&self) -> [u8; 4] {
        [0u8, 0u8, 0u8, 0u8]
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
