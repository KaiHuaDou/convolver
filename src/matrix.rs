use png;
use std::fs;
use std::io;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<[u8; 4]>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows: rows, cols: cols, data: vec![[0u8; 4]; rows * cols] }
    }

    #[inline]
    pub fn get(&self, row: isize, col: isize) -> [u8; 4] {
        unsafe {
            let index = row * self.cols as isize + col;
            *self.data.get_unchecked(index as usize)
        }
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: [u8; 4]) {
        let index = row * self.cols + col;
        self.data[index] = value;
    }

    pub fn read_from_png(filename: &str) -> io::Result<Self> {
        let file = fs::File::open(filename)?;
        let decoder = png::Decoder::new(file);
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
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect();

        Ok(Matrix { rows: height, cols: width, data })
    }

    pub fn write_to_png(self, filename: &str) -> io::Result<()> {
        let file = fs::File::create(filename)?;
        let ref mut buffer = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(buffer, self.cols as u32, self.rows as u32);
        encoder.set_color(png::ColorType::Rgba);
        let mut writer = encoder.write_header()?;
        let result: Vec<u8> = self.data.into_iter().flatten().collect();
        writer.write_image_data(&result).unwrap();
        Ok(())
    }
}
