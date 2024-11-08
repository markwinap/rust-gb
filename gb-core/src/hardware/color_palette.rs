use crate::hardware::ppu::Shade;

#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    // pub cached_16: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy)]
pub enum ColorPalette {
    FixedColorPalette {
        darkest: Color,
        dark: Color,
        light: Color,
        lightest: Color,
    },
}

impl ColorPalette {
    pub fn sprite(&self, shade: Shade, index: u8) -> Color {
        match self {
            ColorPalette::FixedColorPalette { .. } => self.background(shade),
        }
    }

    pub fn window(&self, shade: Shade) -> Color {
        match self {
            ColorPalette::FixedColorPalette { .. } => self.background(shade),
        }
    }

    pub fn background(&self, shade: Shade) -> Color {
        match self {
            ColorPalette::FixedColorPalette {
                dark,
                darkest,
                light,
                lightest,
            } => match shade {
                Shade::DARKEST => *darkest,
                Shade::DARK => *dark,
                Shade::LIGHT => *light,
                Shade::LIGHTEST => *lightest,
            },
        }
    }
}

pub const ORIGINAL_GREEN: ColorPalette = ColorPalette::FixedColorPalette {
    darkest: Color {
        red: 4,
        green: 27,
        blue: 35,
        // cached_16: ((4 as u16 & 0b11111000) << 8)
        // + ((27 as u16 & 0b11111100) << 3)
        // + (35 as u16 >> 3)
    },
    dark: Color {
        red: 53,
        green: 102,
        blue: 81,
        // cached_16: ((53 as u16 & 0b11111000) << 8)
        // + ((102 as u16 & 0b11111100) << 3)
        // + (81 as u16 >> 3)
    },
    light: Color {
        red: 135,
        green: 192,
        blue: 123,
        // cached_16: ((135 as u16 & 0b11111000) << 8)
        // + ((192 as u16 & 0b11111100) << 3)
        // + (123 as u16 >> 3)
    },
    lightest: Color {
        red: 224,
        green: 251,
        blue: 210,
        // cached_16: ((224 as u16 & 0b11111000) << 8)
        // + ((251 as u16 & 0b11111100) << 3)
        // + (210 as u16 >> 3)
    },
};
