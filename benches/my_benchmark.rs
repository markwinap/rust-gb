use criterion::{criterion_group, criterion_main, Criterion};
use gb_core::hardware::boot_rom::{Bootrom, BootromData};
use gb_core::hardware::sound::AudioPlayer;
use gb_core::{gameboy::GameBoy, gameboy::GameBoyState, hardware::Screen};
use std::fs::{self, File};
use std::hint::black_box;
use std::io::Read;
use std::time::Instant;
extern crate gb_core;

fn fibonacci() {
    let mut gb_rom: Vec<u8> = vec![];
    File::open("C:\\roms\\pkred.gb")
        .and_then(|mut f| f.read_to_end(&mut gb_rom))
        .map_err(|_| "Could not read ROM")
        .unwrap();
    let gb_rom = ByteRomManager::new(gb_rom.into_boxed_slice());
    let gb_rom = gb_core::hardware::rom::Rom::from_bytes(gb_rom);

    let boot_room_stuff = Bootrom::new(Some(BootromData::from_bytes(include_bytes!(
        "C:\\roms\\dmg_boot.bin"
    ))));

    let state = fs::read_to_string("C:\\roms\\pk.state").unwrap();
    let gb_state = serde_json::from_str::<GameBoyState>(&state).unwrap(); //GameBoyState

    let cart = gb_rom.into_cartridge();
    let mut gameboy: GameBoy<'_, DummyScreen> = GameBoy::create_from_state(
        DummyScreen::new(),
        cart,
        boot_room_stuff,
        Box::new(NullAudioPlayer),
        gb_state,
    );

    for i in 0..500_000_00 {
        gameboy.tick();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci()));
}

struct DummyScreen {
    pub line_buffer: [u16; 160],
    pub line_complete: bool,
}
impl DummyScreen {
    pub fn new() -> Self {
        Self {
            line_buffer: [0; 160],
            line_complete: false,
        }
    }
}
impl Screen for DummyScreen {
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}

    fn set_pixel(&mut self, x: u8, y: u8, color: gb_core::hardware::color_palette::Color) {
        let encoded_color = ((color.red as u16 & 0b11111000) << 8)
            + ((color.green as u16 & 0b11111100) << 3)
            + (color.blue as u16 >> 3);

        self.line_buffer[x as usize] = encoded_color;
    }
    fn scanline_complete(&mut self, _y: u8, _skip: bool) {
        self.line_complete = true;
    }
    fn draw(&mut self, skip_next: bool) {}

    fn frame_rate(&self) -> u8 {
        30
    }
}

pub struct NullAudioPlayer;

impl AudioPlayer for NullAudioPlayer {
    fn play(&mut self, _output_buffer: &[u16]) {}

    fn samples_rate(&self) -> u32 {
        16000
    }

    fn underflowed(&self) -> bool {
        false
    }
}

struct ByteRomManager {
    data: Box<[u8]>,
    instant: Instant,
}

impl ByteRomManager {
    fn new(data: Box<[u8]>) -> Self {
        return Self {
            data,
            instant: Instant::now(),
        };
    }
}

impl gb_core::hardware::rom::RomManager for ByteRomManager {
    fn read_from_offset(&self, seek_offset: usize, index: usize, bank_number: u8) -> u8 {
        let address = seek_offset + index;
        self.data[address]
    }

    fn clock(&self) -> u64 {
        self.instant.elapsed().as_micros() as u64
        //print!("rr");
        //0
    }

    fn save(&mut self, game_title: &str, bank_index: u8, bank: &[u8]) {}

    fn load_to_bank(&mut self, game_title: &str, bank_index: u8, bank: &mut [u8]) {}
}

impl core::ops::Index<usize> for ByteRomManager {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index as usize]
    }
}
impl core::ops::Index<core::ops::Range<usize>> for ByteRomManager {
    type Output = [u8];

    fn index(&self, index: core::ops::Range<usize>) -> &Self::Output {
        return &self.data[index];
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
