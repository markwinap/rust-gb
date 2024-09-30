///HOLA MUNDO
use crate::hardware::Screen;

use crate::gameboy::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::hardware::color_palette::{Color, ColorPalette, ORIGINAL_GREEN};
use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};
use crate::memory::Memory;
use alloc::boxed::Box;
use arrayvec::ArrayVec;
use bitflags::bitflags;
use core::cmp::Ordering;
use num_traits::FromPrimitive;

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
const SPRITE_HEIGHT: u8 = 16;
const SPRITE_WIDTH: usize = 8;

const SCREEN_FREQUENCY: usize = 60;
const STAT_UNUSED_MASK: u8 = 0 << 7;

const ACCESS_OAM_CYCLES: isize = 21;
const ACCESS_VRAM_CYCLES: isize = 43;
const HBLANK_CYCLES: isize = 50;
const VBLANK_LINE_CYCLES: isize = 114;

const UNDEFINED_READ: u8 = 0xff;

const TILE_MAP_SIZE: usize = 0x400;
const OAM_SPRITES: usize = 40;

#[derive(Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Mode {
    AccessOam,
    AccessVram,
    HBlank,
    VBlank,
}

impl Mode {
    fn cycles(&self, scroll_x: u8) -> isize {
        let scroll_adjust = match scroll_x % 0x08 {
            5..=7 => 2,
            1..=4 => 1,
            _ => 0,
        };
        match *self {
            Mode::AccessOam => ACCESS_OAM_CYCLES,
            Mode::AccessVram => ACCESS_VRAM_CYCLES + scroll_adjust,
            Mode::HBlank => HBLANK_CYCLES - scroll_adjust,
            Mode::VBlank => VBLANK_LINE_CYCLES,
        }
    }
    fn bits(&self) -> u8 {
        match *self {
            Mode::AccessOam => 2,
            Mode::AccessVram => 3,
            Mode::HBlank => 0,
            Mode::VBlank => 1,
        }
    }

    fn minimum_cycles(&self) -> isize {
        match *self {
            Mode::AccessOam => ACCESS_OAM_MIN_CYCLES,
            Mode::AccessVram => ACCESS_VRAM_MIN_CYCLES,
            Mode::HBlank => HBLANK_MIN_CYCLES,
            Mode::VBlank => VBLANK_MIN_CYCLES,
        }
    }
}

const ACCESS_OAM_MIN_CYCLES: isize = 80;
const ACCESS_VRAM_MIN_CYCLES: isize = 172;
const HBLANK_MIN_CYCLES: isize = 204;
const VBLANK_MIN_CYCLES: isize = 456;

pub struct Ppu<T: Screen> {
    color_palette: ColorPalette,
    background_palette: Palette,
    obj_palette0: Palette,
    obj_palette1: Palette,
    scanline: u8,
    video_ram: Box<VideoRam>,
    control: Control,
    stat: Stat,
    compare_line: u8,
    scroll_x: u8,
    scroll_y: u8,
    background_priority: [bool; SCREEN_WIDTH],
    mode: Mode,
    window_x: u8,
    window_y: u8,
    cycle_counter: isize,
    pub screen: T,
    tick: bool,
    counter: u8,
    //sprites: Vec<Sprite>,
    sprites: Box<[Sprite; SPRITE_COUNT]>,
    //sprites: [Sprite; SPRITE_COUNT]
}

impl<T: Screen> Ppu<T> {
    pub fn new(screen: T) -> Ppu<T> {
        Ppu {
            color_palette: ORIGINAL_GREEN,
            background_palette: Palette(0),
            obj_palette0: Palette(0),
            obj_palette1: Palette(0),
            background_priority: [false; SCREEN_WIDTH],
            scanline: 0,
            video_ram: Box::new(VideoRam {
                tile_map0: [0; TILE_MAP_SIZE],
                tile_map1: [0; TILE_MAP_SIZE],
                //tiles: alloc::vec![Tile::new(); TILE_COUNT],
                tiles: [Tile::new(); TILE_COUNT],
            }),
            control: Control::empty(),
            stat: Stat::empty(),
            compare_line: 0,
            scroll_x: 0,
            scroll_y: 0,

            mode: Mode::HBlank,
            window_x: 0,
            window_y: 0,
            tick: false,
            counter: 0,
            cycle_counter: 0,
            //sprites: alloc::vec![Sprite::new(Palette(0)); SPRITE_COUNT],
            sprites: Box::new([Sprite::new(Palette(0)); SPRITE_COUNT]),
            //sprites: [Sprite::new(Palette(0)); SPRITE_COUNT],
            screen,
        }
    }

    pub fn reset(&mut self) {
        self.control = Control::from_bits_truncate(0x91);
        self.scroll_y = 0x00;
        self.scroll_x = 0x00;
        self.compare_line = 0x00;
        self.background_palette = Palette(0xFC);
        self.obj_palette0 = Palette(0xFF);
        self.obj_palette1 = Palette(0xFF);
        self.window_x = 0x00;
        self.window_y = 0x00;
    }
    pub fn step(&mut self, cycles: isize, interrupts: &mut InterruptHandler) {
        if self.scanline == self.compare_line {
            self.stat.insert(Stat::COMPARE);
        } else {
            self.stat.remove(Stat::COMPARE);
        }

        if !self.update_lcd_stat_interrupts(interrupts) {
            return;
        }
        if cycles == 0 {
            self.draw_blank_screen();
            return;
        }
        self.cycle_counter -= cycles;

        if self.cycle_counter <= 0 {
            self.scanline = self.scanline + 1;
            if self.scanline == self.compare_line {
                self.stat.insert(Stat::COMPARE);
            } else {
                self.stat.remove(Stat::COMPARE);
            }

            self.cycle_counter = VBLANK_MIN_CYCLES;
            if self.scanline == SCREEN_HEIGHT as u8 {
                self.draw_scan_line();
                interrupts.request(InterruptLine::VBLANK, true);
            } else if self.scanline >= SCREEN_HEIGHT as u8 + 10 {
                self.draw_to_screen();
                if self.scanline != 0 && self.scanline as usize != SCREEN_HEIGHT + 10 {
                    self.scanline = 0;
                }
                self.scanline = 0;
            } else if self.scanline < SCREEN_HEIGHT as u8 {
                self.draw_scan_line();
            }
        }
    }

    #[inline]
    fn draw_to_screen(&mut self) {
        if self.counter % 2 == 0 {
            self.tick = true;
        } else {
            self.tick = false;
        }
        //self.tick = true;
        self.counter = self.counter.wrapping_add(1);
        self.screen.draw(self.tick);
    }

    fn update_lcd_stat_interrupts(&mut self, interrupts: &mut InterruptHandler) -> bool {
        if !self.control.contains(Control::LCD_ON) {
            self.cycle_counter = VBLANK_MIN_CYCLES;
            self.mode = Mode::VBlank;
            if self.scanline != 0 && self.scanline as usize != SCREEN_HEIGHT {
                self.scanline = 0;
            }
            self.scanline = 0;
            return false;
        }
        if self.scanline >= SCREEN_HEIGHT as u8 {
            self.update_current_mode_sec(
                interrupts,
                Mode::VBlank,
                self.stat.contains(Stat::VBLANK_INT),
            );
        } else if self.cycle_counter >= VBLANK_MIN_CYCLES - ACCESS_OAM_MIN_CYCLES {
            self.update_current_mode_sec(
                interrupts,
                Mode::AccessOam,
                self.stat.contains(Stat::ACCESS_OAM_INT),
            );
        } else if self.cycle_counter
            >= VBLANK_MIN_CYCLES - ACCESS_OAM_MIN_CYCLES - ACCESS_VRAM_MIN_CYCLES
        {
            self.update_current_mode_sec(interrupts, Mode::AccessVram, false);
        } else {
            self.update_current_mode_sec(
                interrupts,
                Mode::HBlank,
                self.stat.contains(Stat::HBLANK_INT),
            );
        }

        if self.stat.contains(Stat::COMPARE) && self.scanline == self.compare_line {
            interrupts.request(InterruptLine::STAT, true);
        }
        true
    }

    fn update_current_mode_sec(
        &mut self,
        interrupts: &mut InterruptHandler,
        new_mode: Mode,
        request_interrupt: bool,
    ) {
        if request_interrupt && new_mode != self.mode {
            interrupts.request(InterruptLine::STAT, true);
        }
        self.mode = new_mode;
    }

    #[inline(always)]
    fn draw_pixel(&mut self, x: u8, color: Color) {
        self.screen.set_pixel(x, self.scanline - 1, color);
    }

    pub fn get_memory_as_mut(&mut self) -> &mut impl Memory {
        &mut *self.video_ram
    }

    pub fn get_control(&self) -> u8 {
        self.control.bits
    }

    // #[unroll_for_loops]
    // pub fn draw_scan_line(&mut self) {
    //     if !self.tick {
    //         return;
    //     }

    //     //let mut background_priority = [false; SCREEN_WIDTH];
    //     if self.control.contains(Control::BG_ON) {
    //         let do_background_pixels =
    //             self.control.contains(Control::WINDOW_ON) && self.window_y <= self.scanline;

    //         let tile_map = if self.control.contains(Control::WINDOW_MAP) {
    //             &self.video_ram.tile_map1
    //         } else {
    //             &self.video_ram.tile_map0
    //         };
    //         for x in 0..SCREEN_WIDTH {
    //             if do_background_pixels {
    //                 self.draw_background_window_pixel(x as u8);
    //             } else {
    //                 self.draw_background_pixel(x as u8);
    //             }
    //         }
    //     }
    //     if self.control.contains(Control::OBJ_ON) {
    //         self.draw_sprites();
    //     }
    //     self.screen.scanline_complete(self.scanline - 1, false);
    // }

    //#[unroll_for_loops]
    pub fn draw_scan_line(&mut self) {
        if !self.tick {
            return;
        }

        if self.control.contains(Control::BG_ON) {
            if self.control.contains(Control::WINDOW_ON) && self.window_y <= self.scanline {
                let y = (self.scanline - 1) - self.window_y;
                let window_map = self.control.contains(Control::WINDOW_MAP);
                for x in 0..SCREEN_WIDTH {
                    self.draw_background_window_pixel(x as u8, y, window_map);
                }
            } else {
                let y = (self.scanline - 1).wrapping_add(self.scroll_y);
                let bg_map = self.control.contains(Control::BG_MAP);
                for x in 0..SCREEN_WIDTH {
                    self.draw_background_pixel(x as u8, y, bg_map);
                }
            }
        }
        if self.control.contains(Control::OBJ_ON) {
            self.draw_sprites();
        }
        self.screen.scanline_complete(self.scanline - 1, false);
    }

    pub fn set_control(&mut self, value: u8) {
        let new_control = Control::from_bits_truncate(value);
        if new_control.contains(Control::LCD_ON) && !self.control.contains(Control::LCD_ON) {
            self.stat.insert(Stat::COMPARE);
            self.screen.turn_on();
        }
        self.control = new_control;
    }
    pub fn set_stat(&mut self, value: u8) {
        let new_stat = Stat::from_bits_truncate(value);

        self.stat = (self.stat & Stat::COMPARE)
            | (new_stat & Stat::HBLANK_INT)
            | (new_stat & Stat::VBLANK_INT)
            | (new_stat & Stat::ACCESS_OAM_INT)
            | (new_stat & Stat::COMPARE_INT);
    }

    pub fn get_stat(&self) -> u8 {
        self.mode.bits() | self.stat.bits | STAT_UNUSED_MASK
    }

    pub fn draw_background_window_pixel(&mut self, x: u8, y: u8, window_map: bool) {
        let adjusted_x = (((x as u16 + self.window_x as u16 - 7) + SCREEN_WIDTH as u16)
            % SCREEN_WIDTH as u16) as u8;
        let tile_map = if window_map {
            &self.video_ram.tile_map1
        } else {
            &self.video_ram.tile_map0
        };
        let tile = self.tile_at(adjusted_x, y, tile_map);
        let bit = (adjusted_x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
        let shade = tile.shade_at((y % 8) * 2, bit, &self.background_palette);
        self.background_priority[x as usize] = shade != Shade::LIGHTEST;
        self.draw_pixel(x, self.color_palette.background(shade));
    }

    pub fn draw_background_pixel(&mut self, x: u8, y: u8, bg_map: bool) {
        let adjusted_x = x.wrapping_add(self.scroll_x);
        let tile_map = if bg_map {
            &self.video_ram.tile_map1
        } else {
            &self.video_ram.tile_map0
        };
        let tile = self.tile_at(adjusted_x, y, tile_map);
        let bit = (adjusted_x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
        let shade = tile.shade_at((y % 8) * 2, bit, &self.background_palette);
        self.background_priority[x as usize] = shade != Shade::LIGHTEST;
        self.draw_pixel(x, self.color_palette.background(shade));
    }

    // pub fn draw_background_window_pixel(&mut self, x: u8) {
    //     let y = (self.scanline - 1) - self.window_y;
    //     let adjusted_x = (((x as u16 + self.window_x as u16 - 7) + SCREEN_WIDTH as u16)
    //         % SCREEN_WIDTH as u16) as u8;
    //     let tile_map = if self.control.contains(Control::WINDOW_MAP) {
    //         &self.video_ram.tile_map1
    //     } else {
    //         &self.video_ram.tile_map0
    //     };
    //     let tile = self.tile_at(adjusted_x, y, tile_map);
    //     let bit = (adjusted_x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
    //     let shade = tile.shade_at((y % 8) * 2, bit, &self.background_palette);
    //     self.background_priority[x as usize] = shade != Shade::LIGHTEST;
    //     self.draw_pixel(x, self.color_palette.background(shade));
    // }

    // pub fn draw_background_pixel(&mut self, x: u8) {
    //     let y = (self.scanline - 1).wrapping_add(self.scroll_y);
    //     let adjusted_x = x.wrapping_add(self.scroll_x);
    //     let tile_map = if self.control.contains(Control::BG_MAP) {
    //         &self.video_ram.tile_map1
    //     } else {
    //         &self.video_ram.tile_map0
    //     };
    //     let tile = self.tile_at(adjusted_x, y, tile_map);
    //     let bit = (adjusted_x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
    //     let shade = tile.shade_at((y % 8) * 2, bit, &self.background_palette);
    //     self.background_priority[x as usize] = shade != Shade::LIGHTEST;
    //     self.draw_pixel(x, self.color_palette.background(shade));
    // }

    pub fn draw_blank_screen(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.screen.set_pixel(
                    x as u8,
                    y as u8,
                    self.color_palette.background(Shade::LIGHTEST),
                )
            }
        }
    }

    fn tile_at(&self, x: u8, y: u8, tile_map: &[u8; TILE_MAP_SIZE]) -> &Tile {
        let col = x as usize / TILE_WIDTH;
        let row = y as usize / TILE_HEIGHT;
        let raw_tile_num = tile_map[row * 32 + col];
        let addr_select = self.control.contains(Control::BG_ADDR);
        let tile_num = if addr_select {
            raw_tile_num as usize
        } else {
            128 + ((raw_tile_num as i8 as i16) + 128) as usize
        };
        &self.video_ram.tiles[tile_num]
    }

    //#[unroll_for_loops]
    pub fn draw_sprites(&mut self) {
        let current_line = self.scanline - 1;
        let size = if self.control.contains(Control::OBJ_SIZE) {
            SPRITE_HEIGHT
        } else {
            SPRITE_HEIGHT / 2
        };
        let mut sprites_to_draw: ArrayVec<[(usize, &Sprite); 10]> = self
            .sprites
            .iter()
            .filter(|sprite| current_line.wrapping_sub(sprite.y) < size)
            .take(10)
            .enumerate()
            .collect();

        sprites_to_draw.sort_by(|&(a_index, a), &(b_index, b)| match a.x.cmp(&b.x) {
            Ordering::Equal => a_index.cmp(&b_index).reverse(),
            other => other.reverse(),
        });

        for (_, sprite) in sprites_to_draw {
            let palette = if sprite.flags.contains(SpriteFlags::PALETTE) {
                &self.obj_palette1
            } else {
                &self.obj_palette0
            };
            let mut tile_num = sprite.tile_number as usize;
            let mut line = if sprite.flags.contains(SpriteFlags::FLIPY) {
                size - current_line.wrapping_sub(sprite.y) - 1
            } else {
                current_line.wrapping_sub(sprite.y)
            };
            if line >= 8 {
                tile_num += 1;
                line -= 8;
            }
            line *= 2;
            let tile = self.video_ram.tiles[tile_num];

            for x in (0..TILE_WIDTH).rev() {
                let bit = if sprite.flags.contains(SpriteFlags::FLIPX) {
                    7 - x
                } else {
                    x
                } as usize;
                let target_x = sprite.x.wrapping_add(7 - x as u8);
                let shade = tile.shade_at(line, bit, &palette);
                let color = self.color_palette.sprite(shade, 0);

                if target_x < SCREEN_WIDTH as u8 && shade != Shade::LIGHTEST {
                    if !sprite.flags.contains(SpriteFlags::PRIORITY)
                        || !self.background_priority[target_x as usize]
                    {
                        self.screen.set_pixel(target_x, self.scanline - 1, color);
                    }
                }
            }
        }
    }

    pub fn write_oam(&mut self, reladdr: u8, value: u8) {
        // if self.mode == Mode::AccessVram || self.mode == Mode::AccessOam {
        //     return;
        // }
        let sprite = &mut self.sprites[reladdr as usize / 4];
        match reladdr as usize % 4 {
            3 => {
                sprite.flags = SpriteFlags::from_bits_truncate(value);
            }
            2 => sprite.tile_number = value,
            1 => sprite.x = value.wrapping_sub(8),
            _ => sprite.y = value.wrapping_sub(16),
        }
    }
    pub fn read_oam(&self, reladdr: u8) -> u8 {
        // if self.mode == Mode::AccessVram || self.mode == Mode::AccessOam {
        //     return 0xff;
        // }
        let sprite = &self.sprites[reladdr as usize / 4];
        match reladdr as usize % 4 {
            3 => sprite.flags.bits(),
            2 => sprite.tile_number,
            1 => sprite.x.wrapping_add(8),
            _ => sprite.y.wrapping_add(16),
        }
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.video_ram.get_byte(address)
    }

    pub fn get_scroll_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn get_scroll_x(&self) -> u8 {
        self.scroll_x
    }

    pub fn set_scroll_y(&mut self, value: u8) {
        self.scroll_y = value;
    }

    pub fn set_scroll_x(&mut self, value: u8) {
        self.scroll_x = value;
    }

    pub fn reset_current_line(&mut self) {
        self.scanline = 0;
    }

    pub fn set_compare_line(&mut self, value: u8) {
        self.compare_line = value;
    }

    pub fn get_current_line(&self) -> u8 {
        self.scanline
    }
    pub fn get_compare_line(&self) -> u8 {
        self.compare_line
    }

    pub fn get_obj_palette0(&self) -> u8 {
        self.obj_palette0.0
    }
    pub fn get_obj_palette1(&self) -> u8 {
        self.obj_palette1.0
    }

    pub fn set_obj_palette0(&mut self, value: u8) {
        self.obj_palette0.0 = value;
    }
    pub fn set_obj_palette1(&mut self, value: u8) {
        self.obj_palette1.0 = value;
    }

    pub fn get_window_x(&self) -> u8 {
        self.window_x
    }
    pub fn get_window_y(&self) -> u8 {
        self.window_y
    }

    pub fn set_window_x(&mut self, value: u8) {
        self.window_x = value;
    }
    pub fn set_window_y(&mut self, value: u8) {
        self.window_y = value;
    }

    pub fn get_bg_palette(&self) -> u8 {
        self.background_palette.0
    }

    pub fn set_bg_palette(&mut self, value: u8) {
        self.background_palette.0 = value;
    }
}

bitflags!(
  struct Control: u8 {
    const BG_ON = 1 << 0;
    const OBJ_ON = 1 << 1;
    const OBJ_SIZE = 1 << 2;
    const BG_MAP = 1 << 3;
    const BG_ADDR = 1 << 4;
    const WINDOW_ON = 1 << 5;
    const WINDOW_MAP = 1 << 6;
    const LCD_ON = 1 << 7;
  }
);
bitflags!(
  struct Stat: u8 {
    const COMPARE = 1 << 2;
    const HBLANK_INT = 1 << 3;
    const VBLANK_INT = 1 << 4;
    const ACCESS_OAM_INT = 1 << 5;
    const COMPARE_INT = 1 << 6;
  }
);

struct VideoRam {
    tile_map0: [u8; TILE_MAP_SIZE],
    tile_map1: [u8; TILE_MAP_SIZE],
    tiles: [Tile; TILE_COUNT], // tiles: Vec<Tile>,
}

impl VideoRam {
    pub fn read_tile_map_byte(&self, address: u16) -> u8 {
        let mut offset_address: u16 = 0;
        let tile_map = if address < TILE_MAP_ADDRESS_1 as u16 {
            offset_address = address - TILE_MAP_ADDRESS_0 as u16;
            self.tile_map0
        } else {
            offset_address = address - TILE_MAP_ADDRESS_1 as u16;
            self.tile_map1
        };
        tile_map[offset_address as usize]
    }

    pub fn write_tile_map_byte(&mut self, address: u16, value: u8) {
        let offset_address;
        let tile_map = if address < TILE_MAP_ADDRESS_1 as u16 {
            offset_address = address - TILE_MAP_ADDRESS_0 as u16;
            &mut self.tile_map0
        } else {
            offset_address = address - TILE_MAP_ADDRESS_1 as u16;
            &mut self.tile_map1
        };

        tile_map[offset_address as usize] = value;
    }

    fn write_tile_byte(&mut self, address: u16, value: u8) {
        let virtual_address = address - 0x8000;
        let tile: &mut Tile = &mut self.tiles[virtual_address as usize / TILE_BYTE_SIZE];
        tile.data[virtual_address as usize % 16] = value;
    }

    fn read_tile_byte(&self, address: u16) -> u8 {
        let virtual_address = address - 0x8000;
        let tile: &Tile = &self.tiles[virtual_address as usize / TILE_BYTE_SIZE];
        tile.data[virtual_address as usize % 16]
    }
}

impl Memory for VideoRam {
    fn set_byte(&mut self, address: u16, value: u8) {
        if address >= TILE_MAP_ADDRESS_0 as u16 {
            self.write_tile_map_byte(address, value);
        } else {
            self.write_tile_byte(address, value);
        }
    }

    fn get_byte(&self, address: u16) -> u8 {
        if address >= TILE_MAP_ADDRESS_0 as u16 {
            self.read_tile_map_byte(address)
        } else {
            self.read_tile_byte(address)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shade {
    DARKEST,
    DARK,
    LIGHT,
    LIGHTEST,
}

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
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
            _ => Shade::DARKEST,
        }
    }
}

type TileRow = [TilePixelValue; 8];

#[derive(Clone, Copy)]
pub struct Tile {
    data: [u8; 16],
}

impl Tile {
    fn new() -> Tile {
        Tile { data: [0; 16] }
    }

    fn shade_at(&self, line: u8, bit: usize, palette: &Palette) -> Shade {
        use crate::util::int::IntExt;
        let data1 = self.data[(line as u16) as usize];
        let data2 = self.data[(line as u16 + 1) as usize];
        let color_value = (data2.bit(bit) << 1) | data1.bit(bit);
        palette.shade(TilePixelValue::from_u8(color_value).unwrap())
    }
}

bitflags!(
  struct SpriteFlags: u8 {
    const UNUSED_MASK = 0b_0000_1111;
    const PALETTE     = 0b_0001_0000;
    const FLIPX       = 0b_0010_0000;
    const FLIPY       = 0b_0100_0000;
    const PRIORITY    = 0b_1000_0000;
  }
);

#[derive(Copy, Clone)]
struct Sprite {
    x: u8,
    y: u8,
    tile_number: u8,
    flags: SpriteFlags,
    palette: Palette,
}

impl Sprite {
    pub fn new(palette: Palette) -> Self {
        Sprite {
            x: 0,
            y: 0,
            tile_number: 0,
            flags: SpriteFlags::empty(),
            palette,
        }
    }
}

#[derive(Copy, Clone)]
struct Palette(u8);

impl Palette {
    pub fn shade(&self, input: TilePixelValue) -> Shade {
        let offset = input as u16 * 2;
        let mask = 0b0000_0011 << offset;
        let result = (self.0 & mask) >> offset;
        match result {
            0 => Shade::LIGHTEST,
            1 => Shade::LIGHT,
            2 => Shade::DARK,
            3 => Shade::DARKEST,
            _ => Shade::LIGHTEST,
        }
    }
}
