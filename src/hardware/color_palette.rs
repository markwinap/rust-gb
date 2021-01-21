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