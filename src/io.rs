use crate::colormode::*;
use crate::matrix::*;
use num::*;
use palette::IntoColor;
use png;
use std::any::TypeId;
use std::cmp::PartialOrd;
use std::fs;
use std::io;

impl<T> Matrix<T>
where
    T: Num + NumCast + Copy + Clone + Sync + Send + PartialOrd + 'static,
{
    pub fn read_png(filename: &str) -> io::Result<Self> {
        if TypeId::of::<T>() == TypeId::of::<Rgba>() {
            let m = Matrix::<Rgba>::_read_png(filename)?;
            let ptr = Box::into_raw(Box::new(m)) as *mut Matrix<T>;
            Ok(unsafe { *Box::from_raw(ptr) })
        } else if TypeId::of::<T>() == TypeId::of::<Hsla>() {
            let m = Matrix::<Hsla>::_read_png(filename)?;
            let ptr = Box::into_raw(Box::new(m)) as *mut Matrix<T>;
            Ok(unsafe { *Box::from_raw(ptr) })
        } else if TypeId::of::<T>() == TypeId::of::<Luva>() {
            let m = Matrix::<Luva>::_read_png(filename)?;
            let ptr = Box::into_raw(Box::new(m)) as *mut Matrix<T>;
            Ok(unsafe { *Box::from_raw(ptr) })
        } else {
            unreachable!()
        }
    }

    pub fn write_png(&self, filename: &str) -> std::io::Result<()> {
        if TypeId::of::<T>() == TypeId::of::<Rgba>() {
            let ptr = self as *const Matrix<T> as *const Matrix<Rgba>;
            let m = unsafe { &*ptr };
            m._write_png(filename)
        } else if TypeId::of::<T>() == TypeId::of::<Hsla>() {
            let ptr = self as *const Matrix<T> as *const Matrix<Hsla>;
            let m = unsafe { &*ptr };
            m._write_png(filename)
        } else if TypeId::of::<T>() == TypeId::of::<Luva>() {
            let ptr = self as *const Matrix<T> as *const Matrix<Luva>;
            let m = unsafe { &*ptr };
            m._write_png(filename)
        } else {
            unreachable!()
        }
    }
}

impl Matrix<Rgba> {
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
            .map(|chunk| [Rgba(chunk[0]), Rgba(chunk[1]), Rgba(chunk[2]), Rgba(chunk[3])])
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

impl Matrix<Hsla> {
    pub fn _read_png(filename: &str) -> io::Result<Self> {
        let matrixu8 = Matrix::<Rgba>::_read_png(filename)?;
        let dataf32 = matrixu8
            .data
            .iter()
            .map(|x| {
                let color: palette::Hsla = palette::Srgba::new(
                    x[0].0 as f32 / 255.0,
                    x[1].0 as f32 / 255.0,
                    x[2].0 as f32 / 255.0,
                    x[3].0 as f32 / 255.0,
                )
                .into_color();
                [
                    Hsla(color.hue.into_degrees()),
                    Hsla(color.saturation),
                    Hsla(color.lightness),
                    Hsla(color.alpha),
                ]
            })
            .collect();
        Ok(Matrix { rows: matrixu8.rows, cols: matrixu8.cols, data: dataf32 })
    }

    pub fn _write_png(&self, filename: &str) -> io::Result<()> {
        let datau8 = self
            .data
            .iter()
            .map(|x| {
                let color: palette::Srgba =
                    palette::Hsla::new(x[0].0, x[1].0, x[2].0, x[3].0).into_color();
                [
                    Rgba((color.red * 255.0) as u8),
                    Rgba((color.green * 255.0) as u8),
                    Rgba((color.blue * 255.0) as u8),
                    Rgba((color.alpha * 255.0) as u8),
                ]
            })
            .collect();
        let matrix_rgba = Matrix::<Rgba> { rows: self.rows, cols: self.cols, data: datau8 };
        matrix_rgba._write_png(filename)?;
        Ok(())
    }
}

impl Matrix<Luva> {
    pub fn _read_png(filename: &str) -> io::Result<Self> {
        let matrix_rgba = Matrix::<Rgba>::_read_png(filename)?;
        let dataf32 = matrix_rgba
            .data
            .iter()
            .map(|x| {
                let color: palette::Luva = palette::Srgba::new(
                    x[0].0 as f32 / 255.0,
                    x[1].0 as f32 / 255.0,
                    x[2].0 as f32 / 255.0,
                    x[3].0 as f32 / 255.0,
                )
                .into_color();
                [Luva(color.l), Luva(color.u), Luva(color.v), Luva(color.alpha)]
            })
            .collect();
        Ok(Matrix { rows: matrix_rgba.rows, cols: matrix_rgba.cols, data: dataf32 })
    }

    pub fn _write_png(&self, filename: &str) -> io::Result<()> {
        let datau8 = self
            .data
            .iter()
            .map(|x| {
                let color: palette::Srgba =
                    palette::Luva::new(x[0].0, x[1].0, x[2].0, x[3].0).into_color();
                [
                    Rgba((color.red * 255.0) as u8),
                    Rgba((color.green * 255.0) as u8),
                    Rgba((color.blue * 255.0) as u8),
                    Rgba((color.alpha * 255.0) as u8),
                ]
            })
            .collect();
        let matrix_rgba = Matrix::<Rgba> { rows: self.rows, cols: self.cols, data: datau8 };
        matrix_rgba._write_png(filename)?;
        Ok(())
    }
}
