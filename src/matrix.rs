use num::*;
use palette::IntoColor;
use png;
use std::any::TypeId;
use std::fs;
use std::io;

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

    pub fn read_png(filename: &str) -> io::Result<Self> {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let m = Matrix::<u8>::_read_png(filename)?;
            let ptr = Box::into_raw(Box::new(m)) as *mut Matrix<T>;
            Ok(unsafe { *Box::from_raw(ptr) })
        } else if TypeId::of::<T>() == TypeId::of::<f32>() {
            let m = Matrix::<f32>::_read_png(filename)?;
            let ptr = Box::into_raw(Box::new(m)) as *mut Matrix<T>;
            Ok(unsafe { *Box::from_raw(ptr) })
        } else {
            unreachable!()
        }
    }

    pub fn write_png(&self, filename: &str) -> std::io::Result<()> {
        if TypeId::of::<T>() == TypeId::of::<u8>() {
            let ptr = self as *const Matrix<T> as *const Matrix<u8>;
            let m = unsafe { &*ptr };
            m._write_png(filename)
        } else if TypeId::of::<T>() == TypeId::of::<f32>() {
            let ptr = self as *const Matrix<T> as *const Matrix<f32>;
            let m = unsafe { &*ptr };
            m._write_png(filename)
        } else {
            unreachable!()
        }
    }
}

impl Matrix<u8> {
    pub fn _read_png(filename: &str) -> io::Result<Self> {
        let file = fs::File::open(filename)?;
        let mut decoder = png::Decoder::new(file);
        decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
        let mut reader = decoder.read_info().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to read PNG info: {}", e))
        })?;
        let info = reader.info();

        if info.color_type != png::ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "PNG format must be RGBA with 8-bit depth",
            ));
        }

        let width = info.width as usize;
        let height = info.height as usize;

        let mut buffer = vec![0; reader.output_buffer_size()];
        let frame_info = reader.next_frame(&mut buffer).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to read PNG frame: {}", e))
        })?;
        let data_bytes = &buffer[..frame_info.buffer_size()];

        let data = data_bytes
            .chunks_exact(4)
            .map(|chunk| [chunk[0].into(), chunk[1].into(), chunk[2].into(), chunk[3].into()])
            .collect();

        Ok(Matrix { rows: height, cols: width, data: data })
    }

    pub fn _write_png(&self, filename: &str) -> io::Result<()> {
        let file = fs::File::create(filename)?;
        let ref mut buffer = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(buffer, self.cols as u32, self.rows as u32);
        encoder.set_color(png::ColorType::Rgba);
        let mut writer = encoder.write_header()?;
        let result: &[u8] = unsafe {
            std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4)
        };
        writer.write_image_data(result)?;
        Ok(())
    }
}

impl Matrix<f32> {
    pub fn _read_png(filename: &str) -> io::Result<Self> {
        let matrixu8 = Matrix::<u8>::_read_png(filename)?;
        let dataf32 = matrixu8
            .data
            .iter()
            .map(|x| {
                let color: palette::Hsla = palette::Srgba::new(
                    x[0] as f32 / 255.0,
                    x[1] as f32 / 255.0,
                    x[2] as f32 / 255.0,
                    x[3] as f32 / 255.0,
                )
                .into_color();
                [color.hue.into_degrees(), color.saturation, color.lightness, color.alpha]
            })
            .collect();
        Ok(Matrix { rows: matrixu8.rows, cols: matrixu8.cols, data: dataf32 })
    }

    pub fn _write_png(&self, filename: &str) -> io::Result<()> {
        let datau8 = self
            .data
            .iter()
            .map(|x| {
                let color: palette::Srgba = palette::Hsla::new(x[0], x[1], x[2], x[3]).into_color();
                [
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                    (color.alpha * 255.0) as u8,
                ]
            })
            .collect();
        let matrixu8 = Matrix::<u8> { rows: self.rows, cols: self.cols, data: datau8 };
        matrixu8._write_png(filename)?;
        Ok(())
    }
}
