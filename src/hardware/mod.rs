mod ppu;
mod color_palette;

use crate::cpu::address::Cpu;
use crate::cpu::interrupt_handler::{InterruptHandler, InterruptLine};
use crate::cpu::{Interface, Step};
use crate::hardware::color_palette::Color;


pub trait Screen {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn set_pixel(&mut self, x: u8, y: u8, color: Color);
    fn draw(&mut self);
}



/////////////////////////
trait AInterface {}

struct AInterfaceImpl {}

impl AInterface for AInterfaceImpl {}

struct ACpu<'a, T: AInterface> {
    pub interface: Option<&'a mut T>
}


struct AHardware {
    pub interface: AInterfaceImpl,
    pub cpu: ACpu<'static, AInterfaceImpl>,
}

impl AHardware {
    fn new() -> Self {
        let mut inter = AInterfaceImpl {};
        let mut hrd = AHardware {
            interface: inter,
            cpu: ACpu {
                interface: None,
            },
        };
        // hrd.cpu.interface = Some((&mut hrd.interface));
        hrd
    }
}

fn test() {
    let my_string = [1, 2, 3];
    copy_if(&my_string, |a_number| true);
}

fn copy_if<'b, F>(slice: &'b [i32], pred: F) -> Vec<i32>
    where F: for<'a> Fn(&'a i32) -> bool
{
    let mut result = vec![];
    for element in slice {
        if pred(&element) {
            result.push(*element);
        }
    }
    result
}

// fn load_from_file<T>(path: String) -> T
//     where
//         T: for<'de> serde::Deserialize<'de>
// {
//     let mut file = File::open(path).unwrap();
//     serde_json::from_reader(&mut file).unwrap()
// }
// fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
//     y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
//     let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
// }