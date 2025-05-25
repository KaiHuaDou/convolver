use num::*;

pub struct Matrix<T>
where
    T: Num + NumCast + Copy + Clone + Sync + Send + PartialOrd,
{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<[T; 4]>,
}

impl<T> Matrix<T>
where
    T: Num + NumCast + Copy + Clone + Sync + Send + PartialOrd + 'static,
{
    #[inline]
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows: rows, cols: cols, data: vec![[T::from(0u8).unwrap(); 4]; rows * cols] }
    }

    #[inline]
    pub fn get(&self, row: isize, col: isize) -> [T; 4] {
        unsafe {
            let index = row * self.cols as isize + col;
            *self.data.get_unchecked(index as usize)
        }
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: [T; 4]) {
        let index = row * self.cols + col;
        self.data[index] = value;
    }
}
