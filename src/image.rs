use crate::parse::{BmpImage, parse_bmp_file};

pub enum TermColor {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

impl TermColor {
    fn from_brightness(brightness: u32) -> Self {
        match brightness {
            0..=25   => TermColor::Level0,
            26..=51  => TermColor::Level1,
            52..=76  => TermColor::Level2,
            77..=102 => TermColor::Level3,
            103..=127=> TermColor::Level4,
            128..=153=> TermColor::Level5,
            154..=178=> TermColor::Level6,
            179..=204=> TermColor::Level7,
            205..=229=> TermColor::Level8,
            _        => TermColor::Level9,
        }
    }

    fn to_char(&self) -> &'static str {
        match self {
            TermColor::Level0 => "  ",
            TermColor::Level1 => " .",
            TermColor::Level2 => " :",
            TermColor::Level3 => "░░",
            TermColor::Level4 => "▒▒",
            TermColor::Level5 => "▓▓",
            TermColor::Level6 => "▨▨",
            TermColor::Level7 => "▩▩",
            TermColor::Level8 => "⣿⣿",
            TermColor::Level9 => "██",
        }
    }
}

impl BmpImage {
    pub fn draw_in_terminal(&self) {
        // Ограничение для вывода изображения
        if self.width > 2000 || self.height > 2000 {
            println!("ERROR: Image dimensions exceed maximum allowed size (2000x2000)");
            return;
        }

        // Начальное адресс смещения указан с 10 по 13 байт, то-есть 4 байт, то-есть 32 бит
        let pixel_offset = u32::from_le_bytes([self.data[10], self.data[11], self.data[12], self.data[13]]) as usize;
        // Bits per pixel глубина цвета RGB888. То-есть получаем из двух байт 24, то-есть 24/8 = 3
        let bpp = u16::from_le_bytes([self.data[28], self.data[29]]) as usize / 8;

        if bpp != 3 {
            println!("Supported only 24bit mode");
            return;
        }

        let width = self.width as usize;
        let height = self.height as usize;
        let row_stride = ((width * bpp + 3) / 4) * 4;

        // Шаг/Кэфициент определяющий соотношения пикселя к символу в терминале
        let get_dim_step = |dim: usize| -> usize {
            if dim <= 500 { 1 }
            else if dim <= 1000 { 2 }
            else if dim <= 1500 { 3 }
            else { 4 }
        };
        let step_x = get_dim_step(width);
        let step_y = get_dim_step(height);
        let step = ((step_x + step_y) + 1) / 2;

        for y in (0..height).rev().step_by(step) {
            let row_start = pixel_offset + (y * row_stride);

            for x in (0..width).step_by(step) {
                let pixel_index = row_start + (x * bpp);

                let b = self.data[pixel_index] as u32;
                let g = self.data[pixel_index + 1] as u32;
                let r = self.data[pixel_index + 2] as u32;

                let brightness = (r + g + b) / 3;
                let color_symbol = TermColor::from_brightness(brightness);

                print!("{}", color_symbol.to_char());
            }
            println!();
        }
    }
}
