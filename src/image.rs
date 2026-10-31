use crate::parse::{BmpImage, parse_bmp_file};
pub enum TermColor {
    Level0, // Dark
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6, // White
}

impl TermColor {
    fn to_char(&self) -> &'static str {
        match self {
            TermColor::Level0 => "  ",
            TermColor::Level1 => "░░",
            TermColor::Level2 => "▒▒",
            TermColor::Level3 => "▓▓",
            TermColor::Level4 => "██",
            TermColor::Level5 => "⣿⣿",
            TermColor::Level6 => "██",
        }
    }
}

impl BmpImage {
    pub fn draw_in_terminal(&self) {
        let pixel_offset = u32::from_le_bytes([self.data[10], self.data[11], self.data[12], self.data[13]]) as usize;
        let bpp = u16::from_le_bytes([self.data[28], self.data[29]]) as usize / 8;

        if bpp != 3 {
            println!("Supported only 24bit mode");
            return;
        }

        let width = self.width as usize;
        let height = self.height as usize;
        let row_stride = ((width * bpp + 3) / 4) * 4;

        for y in (0..height).rev() {
            let row_start = pixel_offset + (y * row_stride);

            for x in 0..width {
                let pixel_index = row_start + (x * bpp);

                let b = self.data[pixel_index] as u32;
                let g = self.data[pixel_index + 1] as u32;
                let r = self.data[pixel_index + 2] as u32;

                let brightness = (r + g + b) / 3;

                let color_symbol = match brightness {
                    0..=36   => TermColor::Level0,
                    37..=72  => TermColor::Level1,
                    73..=109 => TermColor::Level2,
                    110..=145=> TermColor::Level3,
                    146..=181=> TermColor::Level4,
                    182..=218=> TermColor::Level5,
                    _        => TermColor::Level6,
                };

                print!("{}", color_symbol.to_char());
            }
            println!();
        }
    }
}