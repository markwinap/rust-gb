use crate::hardware::ppu::Shade;

#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
    pub fn sprite(&self, shade: Shade, _index: u8) -> Color {
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
    },
    dark: Color {
        red: 53,
        green: 102,
        blue: 81,
    },
    light: Color {
        red: 135,
        green: 192,
        blue: 123,
    },
    lightest: Color {
        red: 224,
        green: 251,
        blue: 210,
    },
};
pub const ORIGINAL: ColorPalette = ColorPalette::FixedColorPalette {
    darkest: Color {
        red: 24,
        green: 60,
        blue: 21,
    },
    dark: Color {
        red: 54,
        green: 100,
        blue: 50,
    },
    light: Color {
        red: 138,
        green: 174,
        blue: 0,
    },
    lightest: Color {
        red: 153,
        green: 189,
        blue: 0,
    },
};

pub const MONOCHROME: ColorPalette = ColorPalette::FixedColorPalette {
    darkest: Color {
        red: 0,
        green: 0,
        blue: 0,
    },
    dark: Color {
        red: 64,
        green: 64,
        blue: 64,
    },
    light: Color {
        red: 192,
        green: 192,
        blue: 192,
    },
    lightest: Color {
        red: 255,
        green: 255,
        blue: 255,
    },
};
