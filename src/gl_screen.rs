use gb_core::hardware::Screen;
use gb_core::hardware::color_palette::Color;

use std::sync::{Arc, RwLock, Mutex};
use gb_core::gameboy::{SCREEN_HEIGHT, SCREEN_WIDTH};
use glium::glutin::event_loop::EventLoop;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError};
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::glutin::event::{Event, WindowEvent};
use std::time::Duration;


pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;

// #[derive(Sync, Send)]
pub struct GlScreen<'a> {
    rw_lock: Arc<RwLock<bool>>,
    turned_on: AtomicBool,
    pixel_buffer: RefCell<[u8; SCREEN_PIXELS]>,
    off_screen_buffer: RefCell<[u8; SCREEN_PIXELS]>,
    render_options: RenderOptions,
    receiver: Receiver<()>,
    sender: SyncSender<()>,
    event_loop: &'a mut EventLoop<()>,
    //  texture: glium::texture::texture2d::Texture2d,
    display: glium::Display,
}

// vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
// let (sender2, receiver2) = mpsc::sync_channel(1);
unsafe impl<'a> Send for GlScreen<'a> {}

impl<'a> GlScreen<'a> {
    fn switch_buffer(&mut self) {
        self.pixel_buffer.swap(&self.off_screen_buffer)
    }

    pub fn init(rom_name: String, event_loop: &'a mut glium::glutin::event_loop::EventLoop<()>) -> Self {
        let window_builder = create_window_builder(&rom_name);
        let context_builder = glium::glutin::ContextBuilder::new();
        let display = glium::backend::glutin::Display::new(window_builder, context_builder, event_loop).unwrap();
        set_window_size(display.gl_window().window(), 2);
        let (sender2, receiver2) = mpsc::sync_channel(1);
        let render_options = <RenderOptions as Default>::default();
        Self {
            rw_lock: Arc::new(Default::default()),
            turned_on: AtomicBool::new(true),
            pixel_buffer: RefCell::new([0; SCREEN_WIDTH * SCREEN_HEIGHT * 3]),
            off_screen_buffer: RefCell::new([0; SCREEN_WIDTH * SCREEN_HEIGHT * 3]),
            render_options,
            event_loop,
            sender: sender2,
            receiver: receiver2,
            //   texture,
            display,
        }
    }

    //  private int index(int x, int y) {
    //         return 3 * ((y * Screen.WIDTH) + x);
    //     }

    fn index(x: u8, y: u8) -> usize {
        3 * ((y as usize * SCREEN_WIDTH) + x as usize)
    }
}

pub fn render(mut screen: GlScreen) {
    let even_loop = screen.event_loop;
    let mut display = screen.display;
    //  let mut texture = screen.texture;
    let mut pixel_buffer = screen.pixel_buffer;
   // let mut rw_lock = screen.rw_lock;
    let mut receiver = screen.receiver;
    even_loop.run_return(move |event, _evtarget, controlflow| {
        let mut stop = false;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                => stop = true,
                _ => ()
            },
            Event::MainEventsCleared => {

                // the returned read_guard also implements `Deref`
                match receiver.recv() {
                    Ok(_) => recalculate_screen(&mut display, pixel_buffer.get_mut(), &Default::default()),
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => {}
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });
    // std::thread::spawn(move || {
    //     while screen.turned_on.load(Ordering::SeqCst) {
    //         let lock = screen.rw_lock.clone();
    //         if let Ok(read_guard) = lock.read() {
    //             // the returned read_guard also implements `Deref`
    //             println!("Read value: {}", *read_guard);
    //             recalculate_screen(&mut screen.display, &mut screen.texture, screen.pixel_buffer.get_mut(), &Default::default())
    //         };
    //     }
    // })
}

impl<'a> Screen for GlScreen<'a> {
    fn turn_on(&mut self) {
        *self.turned_on.get_mut() = true;
    }

    fn turn_off(&mut self) {
        *self.turned_on.get_mut() = false;
    }

    fn set_pixel(&mut self, x: u8, y: u8, color: Color) {
        let index = GlScreen::index(x, y);
        self.off_screen_buffer.get_mut()[index] = color.red;
        self.off_screen_buffer.get_mut()[index + 1] = color.green;
        self.off_screen_buffer.get_mut()[index + 2] = color.blue;
    }

    fn draw(&mut self) {
        //  let lock = self.rw_lock.clone();
        // if let Ok(_) = lock.write() {
        //     self.switch_buffer();
        // };
        self.switch_buffer();
        self.sender.send(()).unwrap();
    }
}

#[derive(Default)]
struct RenderOptions {
    pub linear_interpolation: bool,
}


#[cfg(target_os = "windows")]
fn create_window_builder(romname: &str) -> glium::glutin::window::WindowBuilder {
    use glium::glutin::platform::windows::WindowBuilderExtWindows;
    return glium::glutin::window::WindowBuilder::new()
        .with_drag_and_drop(false)
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )))
        .with_title("RBoy - ".to_owned() + romname);
}

fn set_window_size(window: &glium::glutin::window::Window, scale: u32) {
    use glium::glutin::dpi::{LogicalSize, PhysicalSize};

    let dpi = window.scale_factor();

    let physical_size = PhysicalSize::<u32>::from((SCREEN_WIDTH as u32 * scale, SCREEN_HEIGHT as u32 * scale));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}


fn recalculate_screen(display: &glium::Display,
                      //  texture: &mut glium::texture::texture2d::Texture2d,
                      datavec: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
                      renderoptions: &RenderOptions)
{
    use glium::Surface;

    let interpolation_type = if renderoptions.linear_interpolation {
        glium::uniforms::MagnifySamplerFilter::Linear
    } else {
        glium::uniforms::MagnifySamplerFilter::Nearest
    };

    let rawimage2d = glium::texture::RawImage2d {
        data: std::borrow::Cow::Borrowed(datavec),
        width: SCREEN_WIDTH as u32,
        height: SCREEN_HEIGHT as u32,
        format: glium::texture::ClientFormat::U8U8U8,
    };


    // let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
    //     display,
    //     glium::texture::UncompressedFloatFormat::U8U8U8,
    //     glium::texture::MipmapsOption::NoMipmap,
    //     SCREEN_WIDTH as u32,
    //     SCREEN_HEIGHT as u32)
    //     .unwrap();

    let mut texture = glium::texture::texture2d::Texture2d::new(display, rawimage2d)
        .unwrap();

    // texture.write(
    //     glium::Rect {
    //         left: 0,
    //         bottom: 0,
    //         width: SCREEN_WIDTH as u32,
    //         height: SCREEN_HEIGHT as u32,
    //     },
    //     rawimage2d);

    // We use a custom BlitTarget to transform OpenGL coordinates to row-column coordinates
    let target = display.draw();
    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(
        &target,
        &glium::BlitTarget {
            left: 0,
            bottom: target_h,
            width: target_w as i32,
            height: -(target_h as i32),
        },
        interpolation_type);
    target.finish().unwrap();
}