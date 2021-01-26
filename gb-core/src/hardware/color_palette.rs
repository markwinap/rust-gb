use crate::hardware::ppu::Shade;

#[derive(Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


pub enum ColorPalette {
    FixedColorPalette {
        darkest: Color,
        dark: Color,
        light: Color,
        lightest: Color,
    }
}

impl ColorPalette {
    pub fn sprite(&self, shade: Shade, index: u8) -> Color {
        match self {
            ColorPalette::FixedColorPalette { .. } => {
                self.background(shade)
            }
        }
    }

    pub fn window(&self, shade: Shade) -> Color {
        match self {
            ColorPalette::FixedColorPalette { .. } => {
                self.background(shade)
            }
        }
    }

    pub fn background(&self, shade: Shade) -> Color {
        match self {
            ColorPalette::FixedColorPalette { dark, darkest, light, lightest } => {
                match shade {
                    Shade::DARKEST => { *darkest }
                    Shade::DARK => { *dark }
                    Shade::LIGHT => { *light }
                    Shade::LIGHTEST => { *lightest }
                }
            }
        }
    }
}

pub const ORIGINAL_GREEN: ColorPalette = ColorPalette::FixedColorPalette {
    darkest: Color {
        red: 4,
        green: 27,
        blue: 35
    },
    dark: Color {
        red: 53,
        green: 102,
        blue: 81
    },
    light: Color {
        red: 135,
        green: 192,
        blue: 123
    },
    lightest: Color {
        red: 224,
        green: 251,
        blue: 210
    }
};