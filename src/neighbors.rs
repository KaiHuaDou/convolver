pub struct Neighbors {
    pub size: usize,
    pub data: Vec<[u8; 4]>,
}

impl Neighbors {
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
    pub fn median(&self) -> [u8; 4] {
        let mut values = self.data.clone();
        values.sort_unstable();
        values[values.len() / 2]
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
}
