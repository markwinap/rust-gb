mod fb_screen;
pub mod gl_screen;

use crate::gl_screen::{render, GlScreen};
use gb_core::gameboy::{GameBoy, GameBoyState, GbEvents, SCREEN_PIXELS, SCREEN_WIDTH};
use gb_core::hardware::boot_rom::{Bootrom, BootromData};
use gb_core::hardware::color_palette::Color;
use gb_core::hardware::Screen;
use log::info;
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::time::Instant;

pub enum EmulatorKeyEvent {
    GbEvent(GbEvents),
    Save,
}
fn main() {
    use std::io::Write;

    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format(|buf, record| {
            // let ts = buf.timestamp_millis();
            // writeln!(buf, "{}: {}: {}", ts, record.level(), record.args())
            let ts = buf.timestamp_millis();
            writeln!(buf, "{}", record.args())
        })
        .init();

    info!("HELLO");
    construct_cpu()
}

pub fn construct_cpu() {
    let mut gb_rom: Vec<u8> = vec![];
    File::open("C:\\roms\\pkred.gb")
        .and_then(|mut f| f.read_to_end(&mut gb_rom))
        .map_err(|_| "Could not read ROM")
        .unwrap();

    info!("STARTING");
    let gb_rom = ByteRomManager::new(gb_rom.into_boxed_slice());
    let gb_rom = gb_core::hardware::rom::Rom::from_bytes(gb_rom);

    let (sender2, receiver2) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);
    let (control_sender, control_receiver) = mpsc::channel::<EmulatorKeyEvent>();
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
        let limit_speed = true;

        let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;

        let cart = gb_rom.into_cartridge();

        let state = fs::read_to_string("C:\\roms\\pk.state").unwrap();
        let gb_state = serde_json::from_str::<GameBoyState>(&state).unwrap(); //GameBoyState
        let mut gameboy = GameBoy::create(
            sync_screen,
            cart,
            boot_room_stuff,
            Box::new(NullAudioPlayer),
            // gb_state,
        );

        // let mut gameboy = GameBoy::create_from_state(
        //     sync_screen,
        //     cart,
        //     boot_room_stuff,
        //     Box::new(NullAudioPlayer),
        //     gb_state,
        // );

        'outer: loop {
            while ticks < waitticks {
                ticks += gameboy.tick() as u32
            }

            ticks -= waitticks;

            'recv: loop {
                match control_receiver.try_recv() {
                    Ok(event) => match event {
                        EmulatorKeyEvent::GbEvent(gb_events) => match gb_events {
                            GbEvents::KeyUp(key) => gameboy.key_released(key),
                            GbEvents::KeyDown(key) => gameboy.key_pressed(key),
                        },
                        EmulatorKeyEvent::Save => {
                            info!("SAVING STATE");
                            let state = gameboy.create_state();
                            let string = serde_json::to_string(&state).unwrap();
                            let mut file = File::create("C:\\roms\\pk_intro.state").unwrap();
                            file.write_all(string.as_bytes()).unwrap();
                        }
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

    fn frame_rate(&self) -> u8 {
        60
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
    fn read_from_offset(&self, seek_offset: usize, index: usize, _bank_number: u8) -> u8 {
        let address = seek_offset + index;
        self.data[address]
    }

    fn clock(&self) -> u64 {
        self.instant.elapsed().as_micros() as u64
        //print!("rr");
        //0
    }

    fn save(&mut self, game_title: &str, bank_index: u8, bank: &[u8]) {
        info!("SAVING RAM BANK: {}", bank_index);
    }

    fn load_to_bank(&mut self, game_title: &str, bank_index: u8, bank: &mut [u8]) {
        info!("LOADING RAM BANK: {}", bank_index);
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
