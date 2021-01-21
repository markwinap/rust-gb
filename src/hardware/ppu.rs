use crate::hardware::{Screen};
use bit_set::BitSet;
use crate::hardware::color_palette::{ColorPalette, Color};

const TILE_MAP_ADDRESS_0: usize = 0x9800;
const TILE_MAP_ADDRESS_1: usize = 0x9C00;
const TILE_MAP_WIDTH: usize = 32;
const TILE_MAP_HEIGHT: usize = 32;

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILE_COUNT: usize = 384;
const TILE_BYTE_SIZE: usize = 16;

const SPRITE_COUNT: usize = 40;
const SPRITE_BYTE_SIZE: usize = 4;
const SPRITE_HEIGHT: usize = 16;
const SPRITE_WIDTH: usize = 8;

const SCREEN_HEIGHT: usize = 144;
const SCREEN_WIDTH: usize = 160;
const SCREEN_FREQUENCY: usize = 60;

pub struct Ppu<T: Screen> {
    screen: T,
    color_palette: ColorPalette,
    scanline: u8,
    large_sprites: bool,
    background_mask: BitSet,
    video_ram: VideoRam,
}


const TILE_MAP_SIZE: usize = 0x400;
const OAM_SPRITES: usize = 40;

struct VideoRam {
    tile_map1: [u8; TILE_MAP_SIZE],
    tile_map2: [u8; TILE_MAP_SIZE],
    tiles: [Tile; TILE_COUNT],

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shade {
    DARKEST,
    DARK,
    LIGHT,
    LIGHTEST,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TilePixelValue {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

impl Default for TilePixelValue {
    fn default() -> Self {
        TilePixelValue::Zero
    }
}

impl Default for Shade {
    fn default() -> Self {
        Shade::DARKEST
    }
}

impl From<u8> for Shade {
    fn from(value: u8) -> Self {
        match value {
            0 => Shade::LIGHTEST,
            1 => Shade::LIGHT,
            2 => Shade::DARK,
            3 => Shade::LIGHTEST,
            _ => Shade::DARKEST
        }
    }
}

type TileRow = [TilePixelValue; 8];

struct Tile(u8, [TileRow; 8]);

impl Tile {
    fn shade_at(&self, x: u8, y: u8, palette: &Palette) -> Shade {
        palette.shade(self.1[(y as usize % TILE_HEIGHT)][(x as usize % TILE_WIDTH)])
    }
}


impl<T: Screen> Ppu<T> {
    fn draw_pixel(&mut self, x: u8, shade: Shade, color: Color) {
        if shade != Shade::LIGHTEST {
            self.background_mask.insert(x as usize);
        } else {
            self.background_mask.remove(x as usize);
        }
        self.screen.set_pixel(x, self.scanline - 1, color);
    }
}

struct Sprite {
    sprite_num: u8,
    x: u8,
    y: u8,
    tile_number: u8,
    prioritize_sprite: bool,
    flip_x: bool,
    flip_y: bool,
    palette: Palette,
}

impl Sprite {
    pub fn new(sprite_num: u8, palette: Palette) -> Self {
        Sprite {
            sprite_num,
            x: 0,
            y: 0,
            tile_number: 0,
            prioritize_sprite: true,
            flip_x: false,
            flip_y: false,
            palette,
        }
    }

    fn is_on_scan_line<T: Screen>(&self, ppu: &Ppu<T>) -> bool {
        let y = self.y;
        ppu.scanline >= y && ppu.scanline < (y + Sprite::height(ppu))
    }
    fn height<T: Screen>(ppu: &Ppu<T>) -> u8 {
        if ppu.large_sprites { SPRITE_HEIGHT as u8 } else { SPRITE_HEIGHT as u8 / 2 }
    }
    pub fn render<T: Screen>(&self, ppu: &mut Ppu<T>) {
        if !self.is_on_scan_line(ppu) {
            return;
        }
        for i in 0..SPRITE_WIDTH {
            let mut x = i;
            let mut y = (ppu.scanline - self.y);
            if self.flip_x { x = 7 - x; }
            if self.flip_y { y = Sprite::height(ppu) - 1 - y; }

            //TODO VERIFY  (this.x + i >= Screen.WIDTH || this.x + i < 0)
            if self.x + 1 >= SCREEN_WIDTH as u8 {
                continue;
            }
            if !self.prioritize_sprite && ppu.background_mask.contains(self.x as usize + i) {
                continue;
            }

            let tile = &ppu.video_ram.tiles[self.tile_number as usize + (y as usize / TILE_HEIGHT)];


            let shade = tile.shade_at(x as u8, y, &self.palette);

            //TODO         private int spritePaletteIndex() {
            //             return palette == objectPalette0 ? 0 : 1;
            //         }
            let color = ppu.color_palette.sprite(shade, 0);
            ppu.draw_pixel(self.x + i as u8, shade, color);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Palette(u8);

impl Palette {
    pub fn shade(&self, input: TilePixelValue) -> Shade {
        match input {
            TilePixelValue::Zero => { Shade::from((self.0 >> 0) & 0x3) }
            TilePixelValue::One => { Shade::from((self.0 >> 2) & 0x3) }
            TilePixelValue::Two => { Shade::from((self.0 >> 4) & 0x3) }
            TilePixelValue::Three => { Shade::from((self.0 >> 6) & 0x3) }
        }
    }
}