mod fb_screen;
pub mod gl_screen;

extern crate gb_core;
extern crate glium;

use crate::gl_screen::{render, GlScreen};
use gb_core::gameboy::{GameBoy, GbEvents, SCREEN_PIXELS, SCREEN_WIDTH};
use gb_core::hardware::boot_rom::{Bootrom, BootromData};
use gb_core::hardware::color_palette::Color;
use gb_core::hardware::rom::Rom;
use gb_core::hardware::Screen;
use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::sync::{mpsc, Arc};

fn main() {
    construct_cpu()
}

// pub fn load_rom(zip_file: &str, rom_name: &str) -> Rom<'static> {
//     println!("sd {}", 3);

//     let foo = format!("sd {}", 3);
//     let file = fs::File::open(&zip_file).unwrap();
//     let mut archive = zip::ZipArchive::new(file).unwrap();

//     let bytes = match archive.by_name(rom_name) {
//         Ok(rom_file) => {
//             rom_file.bytes()
//         }
//         Err(_) => { panic!() }
//     };
//     let data: Result<Vec<_>, _> = bytes.collect();
//     Rom::from_bytes(Box::leak(Box::new(data.unwrap())))
// }

// pub fn load_rom_from_path(path: &Path) -> Rom<> {
//     let mut gb_rom: Vec<u8> = vec![];
//     File::open(path).and_then(|mut f| f.read_to_end(&mut gb_rom)).map_err(|_| "Could not read ROM").unwrap();
//    // Rom::from_bytes(Box::leak(Box::new(gb_rom)))
// }

pub fn construct_cpu() {
    let mut gb_rom: Vec<u8> = vec![];
    File::open("C:\\roms\\sml.gb")
        .and_then(|mut f| f.read_to_end(&mut gb_rom))
        .map_err(|_| "Could not read ROM")
        .unwrap();

    let gb_rom = ByteRomManager::new(gb_rom.into_boxed_slice());
    let gb_rom = gb_core::hardware::rom::Rom::from_bytes(gb_rom);
    // let gb_rom = load_rom("test-roms/cpu_instrs.zip", "cpu_instrs/cpu_instrs.gb");
    // let boot_rom = std::path::PathBuf::from("/home/plozano/gbrom/dmg_boot.bin");

    let (sender2, receiver2) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);
    let (control_sender, control_receiver) = mpsc::channel::<GbEvents>();
    let gl_screen = GlScreen::init("foo".to_string(), receiver2);

    let sync_screen = SynScreen {
        sender: sender2,
        off_screen_buffer: RefCell::new(Box::new([0; SCREEN_PIXELS])),
    };

    let boot_room_stuff = Bootrom::new(Some(BootromData::from_bytes(include_bytes!(
        "C:\\roms\\dmg_boot.bin"
    ))));

    let cputhread = std::thread::spawn(move || {
        let periodic = timer_periodic(16);
        let mut limit_speed = true;

        let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;

        let cart = gb_rom.into_cartridge();
        let mut gameboy = GameBoy::create(
            sync_screen,
            cart,
            boot_room_stuff,
            Box::new(NullAudioPlayer),
        );

        'outer: loop {
            while ticks < waitticks {
                ticks += gameboy.tick() as u32
            }

            ticks -= waitticks;

            'recv: loop {
                match control_receiver.try_recv() {
                    Ok(event) => match event {
                        GbEvents::KeyUp(key) => gameboy.key_released(key),
                        GbEvents::KeyDown(key) => gameboy.key_pressed(key),
                    },
                    Err(TryRecvError::Empty) => break 'recv,
                    Err(TryRecvError::Disconnected) => break 'outer,
                }
            }
            if limit_speed {
                let _ = periodic.recv();
            }
        }
    });

    render(gl_screen, control_sender);
    cputhread.join().unwrap();
}

fn timer_periodic(ms: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(ms));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

pub struct SynScreen {
    sender: SyncSender<Box<[u8; SCREEN_PIXELS]>>,
    off_screen_buffer: RefCell<Box<[u8; SCREEN_PIXELS]>>,
}

impl Screen for SynScreen {
    fn turn_on(&mut self) {}

    fn turn_off(&mut self) {}

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 0] =
            color.red;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 1] =
            color.green;
        self.off_screen_buffer.get_mut()[y as usize * SCREEN_WIDTH * 3 + x as usize * 3 + 2] =
            color.blue;
    }

    fn draw(&mut self, skip: bool) {
        let stuff = self.off_screen_buffer.replace(Box::new([0; SCREEN_PIXELS]));
        self.sender.send(stuff).unwrap();
    }
}

struct ByteRomManager {
    data: Box<[u8]>,
}

impl ByteRomManager {
    fn new(data: Box<[u8]>) -> Self {
        return Self { data };
    }
}

impl gb_core::hardware::rom::RomManager for ByteRomManager {
    fn read_from_offset(&self, seek_offset: usize, index: usize) -> u8 {
        let address = seek_offset + index;
        self.data[address]
    }
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

pub struct NullAudioPlayer;

impl gb_core::hardware::sound::AudioPlayer for NullAudioPlayer {
    fn play(&mut self, _buf_left: &[u16]) {
        // Do nothing
    }

    fn samples_rate(&self) -> u32 {
        44100
    }

    fn underflowed(&self) -> bool {
        false
    }
}
