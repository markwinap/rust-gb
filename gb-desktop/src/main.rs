mod fb_screen;
pub mod gl_screen;

use crate::gl_screen::{render, GlScreen};
use gb_core::gameboy::{GameBoy, GameBoyState, GbEvents, SCREEN_PIXELS, SCREEN_WIDTH};
use gb_core::hardware::boot_rom::{Bootrom, BootromData};
use gb_core::hardware::color_palette::Color;
use gb_core::hardware::Screen;
use log::info;
use std::cell::{Cell, RefCell};
use std::env;
use std::fs;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

const CYCLES_PER_FRAME: u32 = 70224;

pub enum EmulatorKeyEvent {
    GbEvent(GbEvents),
    Save,
}
fn main() {
    println!("{}", std::env::current_dir().unwrap().display());
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

    let args: Vec<String> = env::args().collect();
    let room_path = &args[1];

    construct_cpu(room_path);
}

pub fn construct_cpu(room_path: &str) {
    //read room
    info!("STARTING");
    let gb_rom: Vec<u8> = fs::read(room_path).expect("Failed to read ROM file");
    // move vec to heap allocated boxed slice
    let gb_rom = gb_rom.into_boxed_slice();

    let gb_rom = ByteRomManager {
        data: gb_rom,
        instant: Instant::now(),
    };
    info!("CONTROLS: Z = A, X = B, Enter = Start, Space = Select, Arrow keys = D-Pad");

    let gb_rom = gb_core::hardware::rom::Rom::from_bytes(gb_rom);
    // block threads waiting for the lock to become available
    let save_lock: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    // Synchronous FIFO Queue (fix size) for screen updates of (160x144 pixels) * 3 rgb = 69120 bytes
    let (tx_screen, rx_screen) = mpsc::sync_channel::<Box<[u8; SCREEN_PIXELS]>>(1);
    // FIFO Queue for key events
    let (tx_control, rx_control) = mpsc::channel::<EmulatorKeyEvent>();
    // Initialize OpenGL screen
    let gl_screen = GlScreen::init(gb_rom.title.to_string(), rx_screen);

    let sync_screen = SyncScreen {
        sender: tx_screen,
        off_screen_buffer: RefCell::new(Box::new([0; SCREEN_PIXELS])), // turn time borrow checker with RefCell
        check: save_lock.clone(),
    };

    let boot_room_stuff = Bootrom::new(None);

    let cpu_thread = thread::spawn(move || {
        let periodic = periodic_timer(16);
        let limit_speed = true;
        // let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let wait_ticks = CYCLES_PER_FRAME;

        let mut ticks = 0;

        let cart = gb_rom.into_cartridge();

        //  let state = fs::read_to_string("C:\\roms\\f_test2.state").unwrap();
        // let gb_state = serde_json::from_str::<GameBoyState>(&state).unwrap(); //GameBoyState
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
            while ticks < wait_ticks {
                ticks += gameboy.tick() as u32
            }

            ticks -= wait_ticks;

            let mut check: std::sync::MutexGuard<'_, bool> = save_lock.lock().unwrap();
            if *(check) == true {
                println!("SAVING");
                let state = gameboy.create_state();
                let json_state = serde_json::to_string(&state).unwrap();
                fs::write("test_fail.state", json_state).unwrap();
                *check = false;
            }

            'recv: loop {
                match rx_control.try_recv() {
                    Ok(event) => match event {
                        EmulatorKeyEvent::GbEvent(gb_events) => match gb_events {
                            GbEvents::KeyUp(key) => gameboy.key_released(key),
                            GbEvents::KeyDown(key) => gameboy.key_pressed(key),
                        },
                        EmulatorKeyEvent::Save => {
                            info!("SAVING STATE");
                            // Store GameBoy state to JSON for debuging {cpu_state, ppu_state, hard_ware_state, state}
                            let state = gameboy.create_state();
                            let json_state = serde_json::to_string(&state).unwrap();
                            fs::write("f_test2.state", json_state).unwrap();
                        }
                    },
                    Err(TryRecvError::Empty) => break 'recv,
                    Err(TryRecvError::Disconnected) => break 'outer,
                }
            }
            if limit_speed {
                let _ = periodic.recv();
                //sleep(16);
            }
        }
    });

    render(gl_screen, tx_control);
    cpu_thread.join().unwrap();
}

// Create a periodic timer that sends a message every ms milliseconds
fn periodic_timer(ms: u64) -> Receiver<()> {
    let (tx, rx) = mpsc::sync_channel(1);
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(ms));
        tx.send(()).unwrap();
    });
    rx
}
fn sleep(ms: u64) {
    thread::sleep(std::time::Duration::from_millis(ms));
}

pub struct SyncScreen {
    sender: SyncSender<Box<[u8; SCREEN_PIXELS]>>,
    off_screen_buffer: RefCell<Box<[u8; SCREEN_PIXELS]>>,
    check: Arc<Mutex<bool>>,
}

impl Screen for SyncScreen {
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
    data: Box<[u8]>,  // Game rom data
    instant: Instant, // Time
}

impl gb_core::hardware::rom::RomManager for ByteRomManager {
    fn read_from_offset(&self, seek_offset: usize, index: usize, _bank_number: u8) -> u8 {
        let address = seek_offset + index;
        let result = self.data[address];
        result
    }

    fn clock(&self) -> u64 {
        self.instant.elapsed().as_micros() as u64
    }

    fn save(&mut self, _game_title: &str, bank_index: u8, _bank: &[u8]) {
        info!("SAVING RAM BANK: {}", bank_index);
    }

    fn load_to_bank(&mut self, _game_title: &str, bank_index: u8, _bank: &mut [u8]) {
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
