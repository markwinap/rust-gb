use gb_core::gameboy::{SCREEN_HEIGHT, SCREEN_PIXELS, SCREEN_WIDTH};
use minifb::{Window, WindowOptions};
use std::sync::mpsc::Receiver;

pub struct FbScreen {
    //   window: Window,
    receiver: Receiver<Box<[u8; SCREEN_PIXELS]>>,
}

const NUMBER_OF_PIXELS: usize = 23040;

impl FbScreen {
    pub fn init(rom_name: String, receiver: Receiver<Box<[u8; SCREEN_PIXELS]>>) -> Self {
        Self {
            //  window: window,
            receiver,
        }
    }

    pub fn render(mut screen: FbScreen) {
        std::thread::spawn(move || {
            let mut stop = false;

            let mut window = Window::new(
                "Mario Land",
                SCREEN_WIDTH as usize,
                SCREEN_HEIGHT as usize,
                WindowOptions::default(),
            )
            .unwrap();

            while !stop {
                match screen.receiver.recv() {
                    Ok(data) => screen.recalculate_screen(&mut window, &data),
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
        });
    }

    pub fn recalculate_screen(&mut self, window: &mut Window, data: &[u8; SCREEN_PIXELS]) {
        let mut buffer = [0; NUMBER_OF_PIXELS];
        for (i, pixel) in data.chunks(4).enumerate() {
            buffer[i] = (pixel[3] as u32) << 24
                | (pixel[2] as u32) << 16
                | (pixel[1] as u32) << 8
                | (pixel[0] as u32)
        }

        window
            .update_with_buffer(&buffer, SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)
            .unwrap();
    }
}
