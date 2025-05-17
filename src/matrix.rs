use png;
use rayon::prelude::*;
use std::fs;
use std::io;
use std::io::Write;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<bool>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<bool>) -> Result<Self, String> {
        if data.len() != rows * cols {
            return Err(format!("数据长度 {} ≠ {} * {}", data.len(), rows, cols));
        }
        Ok(Self { rows, cols, data })
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        //* Disable for performance purpose
        // if row >= self.rows || col >= self.cols {
        //     return true;
        // }
        let index = row * self.cols + col;
        self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, value: bool) -> Result<(), String> {
        //* Disable for performance purpose
        // if row >= self.rows || col >= self.cols {
        //     return Err(format!("行列超出指定范围"));
        // }
        let index = row * self.cols + col;
        self.data[index] = value;
        Ok(())
    }

    pub fn write_to_text(&self, filename: &str) -> io::Result<()> {
        let bits: Vec<char> = self.data.par_iter().map(|b| if *b { '1' } else { '0' }).collect();
        let data: String = bits.iter().collect();

        let content = format!("{} {} {}", self.rows, self.cols, data);
        let mut file = fs::File::create(filename)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn write_to_binary(&self, filename: &str) -> io::Result<()> {
        let file = fs::File::create(filename)?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(&self.rows.to_be_bytes())?;
        writer.write_all(&self.cols.to_be_bytes())?;
        let bytes = {
            let mut bytes = vec![];
            let mut current_byte = 0u8;
            let mut bit_index = 0;

            for &bit in &self.data {
                if bit {
                    current_byte |= 1 << (7 - bit_index);
                }
                bit_index += 1;
                if bit_index == 8 {
                    bytes.push(current_byte);
                    current_byte = 0;
                    bit_index = 0;
                }
            }
            if bit_index != 0 {
                bytes.push(current_byte);
            }
            bytes
        };
        writer.write_all(&bytes)?;
        Ok(())
    }

    pub fn write_to_png(&self, filename: &str) -> io::Result<()> {
        let file = fs::File::create(filename)?;
        let ref mut buffer = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(buffer, self.rows as u32, self.cols as u32);
        encoder.set_color(png::ColorType::Grayscale);
        let mut writer = encoder.write_header()?;
        let data: Vec<u8> = self.data.iter().map(|&x| if x { 255 } else { 0 }).collect();
        writer.write_image_data(&data).unwrap();
        Ok(())
    }

    pub fn read_from_text(&mut self, filename: &str) -> Result<(), String> {
        let content = fs::read_to_string(filename).map_err(|e| format!("文件读取失败：{}", e))?;

        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(format!("无效格式，应有 3 个部分，实际收到 {} 个", parts.len()));
        }

        let rows = parts[0].parse::<usize>().map_err(|e| format!("无效的行数：{}", e))?;
        let cols = parts[1].parse::<usize>().map_err(|e| format!("无效的列数：{}", e))?;

        let data = parts[2];
        if data.len() != rows * cols {
            return Err(format!(
                "数据长度不匹配：预期{} ({}x{})，实际{}",
                rows * cols,
                rows,
                cols,
                data.len()
            ));
        }

        let matrix = data.par_chars().map(|x| if x == '1' { true } else { false }).collect();

        self.rows = rows;
        self.cols = cols;
        self.data = matrix;

        Ok(())
    }

    pub fn convolve<F>(&mut self, f: F)
    where
        F: Fn([bool; 9]) -> bool + Sync,
    {
        let mut data = vec![false; self.rows * self.cols];

        data.par_iter_mut().enumerate().for_each(|(index, value)| {
            let i = index / self.cols;
            let j = index % self.cols;

            let mut neighbors = [false; 9];
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    let val = if (i == 0 && dx < 0)
                        || (i == self.rows - 1 && dx > 0)
                        || (j == 0 && dy < 0)
                        || (j == self.cols - 1 && j > 0)
                    {
                        false
                    } else {
                        self.get(x as usize, y as usize)
                    };
                    neighbors[(4 + dx + 3 * dy) as usize] = val;
                }
            }
            *value = f(neighbors);
        });

        self.data = data;
    }
}
