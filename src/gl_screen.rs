use gb_core::hardware::Screen;
use gb_core::hardware::color_palette::Color;

use std::sync::{Arc, RwLock, Mutex};
use gb_core::gameboy::{SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_PIXELS, GbEvents};
use glium::glutin::event_loop::EventLoop;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError, Sender};
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;
use glium::glutin::event::{Event, WindowEvent, VirtualKeyCode};
use std::time::Duration;
use gb_core::hardware::input::Button;
use glium::glutin::event::WindowEvent::KeyboardInput;


pub struct GlScreen {
    rw_lock: Arc<RwLock<bool>>,
    turned_on: AtomicBool,
    render_options: RenderOptions,
    receiver: Receiver<Box<[u8; SCREEN_PIXELS]>>,
    event_loop:  EventLoop<()>,
    display: glium::Display,
}

unsafe impl Send for GlScreen {}

impl GlScreen {

    pub fn init(rom_name: String, receiver: Receiver<Box<[u8; SCREEN_PIXELS]>>) -> Self {
        let window_builder = create_window_builder(&rom_name);
        let context_builder = glium::glutin::ContextBuilder::new();
        let mut event_loop = glium::glutin::event_loop::EventLoop::new();
        let display = glium::backend::glutin::Display::new(window_builder, context_builder, &event_loop).unwrap();
        set_window_size(display.gl_window().window(), 2);
        let render_options = <RenderOptions as Default>::default();
        Self {
            rw_lock: Arc::new(Default::default()),
            turned_on: AtomicBool::new(true),
            render_options,
            event_loop,
            receiver,
            display,
        }
    }

    fn index(x: u8, y: u8) -> usize {
        3 * ((y as usize * SCREEN_WIDTH) + x as usize)
    }

}

pub fn render(mut screen: GlScreen, sender: Sender<GbEvents>) {
    use glium::glutin::event::{Event, WindowEvent, KeyboardInput};
    use glium::glutin::event::ElementState::{Pressed, Released};
    use glium::glutin::event::VirtualKeyCode;

    let mut even_loop = screen.event_loop;
    let mut display = screen.display;

    let receiver = screen.receiver;
    even_loop.run_return(move |event, _evtarget, controlflow| {
        let mut stop = false;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => stop = true,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput { state: Pressed, virtual_keycode: Some(VirtualKeyCode::Escape), .. }
                    => stop = true,
                    KeyboardInput { state: Pressed, virtual_keycode: Some(glutinkey), .. } => {
                        if let Some(key) = glium_key_to_button(glutinkey) {
                            let _ = sender.send(GbEvents::KeyDown(key));
                        }
                    }
                    KeyboardInput { state: Released, virtual_keycode: Some(glutinkey), .. } => {
                        if let Some(key) = glium_key_to_button(glutinkey) {
                            let _ = sender.send(GbEvents::KeyUp(key));
                        }
                    }
                    _ => (),
                },
                _ => ()
            },
            Event::MainEventsCleared => {

                // the returned read_guard also implements `Deref`
                match receiver.recv() {
                    Ok(data) => recalculate_screen(&mut display, &data, &Default::default()),
                    Err(..) => stop = true, // Remote end has hung-up
                }
            }
            _ => {}
        }
        if stop {
            *controlflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });
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
        .with_title("Rust-GB ".to_owned() + romname);
}

#[cfg(target_os = "linux")]
fn create_window_builder(romname: &str) -> glium::glutin::window::WindowBuilder {
   
    return glium::glutin::window::WindowBuilder::new()
        
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )))
        .with_title("Rust-GB ".to_owned() + romname);
}


fn set_window_size(window: &glium::glutin::window::Window, scale: u32) {
    use glium::glutin::dpi::{LogicalSize, PhysicalSize};

    let dpi = window.scale_factor();

    let physical_size = PhysicalSize::<u32>::from((SCREEN_WIDTH as u32 * scale, SCREEN_HEIGHT as u32 * scale));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}


fn recalculate_screen(display: &glium::Display,
                      datavec: &[u8; SCREEN_PIXELS],
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


    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32)
        .unwrap();


    texture.write(
        glium::Rect {
            left: 0,
            bottom: 0,
            width: SCREEN_WIDTH as u32,
            height: SCREEN_HEIGHT as u32,
        },
        rawimage2d);

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


fn glium_key_to_button(key: glium::glutin::event::VirtualKeyCode) -> Option<Button> {
    use glium::glutin::event::VirtualKeyCode;
    match key {
        VirtualKeyCode::Z => Some(Button::A),
        VirtualKeyCode::X => Some(Button::B),
        VirtualKeyCode::Up => Some(Button::UP),
        VirtualKeyCode::Down => Some(Button::DOWN),
        VirtualKeyCode::Left => Some(Button::LEFT),
        VirtualKeyCode::Right => Some(Button::RIGHT),
        VirtualKeyCode::Space => Some(Button::SELECT),
        VirtualKeyCode::Return => Some(Button::START),
        _ => None,
    }
}