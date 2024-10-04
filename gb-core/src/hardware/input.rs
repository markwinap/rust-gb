use crate::hardware::interrupt_handler::{InterruptHandler, InterruptLine};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Button {
    A,
    B,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    START,
    SELECT,
}

// impl Display for Button {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), ::core::fmt::Error> {
//         match *self {
//             Button::A => f.write_str("A"),
//             Button::B => f.write_str("B"),
//             Button::UP => f.write_str("UP"),
//             Button::DOWN => f.write_str("DOWN"),
//             Button::LEFT => f.write_str("LEFT"),
//             Button::RIGHT => f.write_str("RIGHT"),
//             Button::START => f.write_str("START"),
//             Button::SELECT => f.write_str("SELECT"),
//         }
//     }
// }

pub trait Controller {
    fn is_pressed(&self, button: Button) -> bool;

    fn tick(&self);

    fn any_pressed(&self) -> bool {
        let mut any = false;
        if self.is_pressed(Button::A) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::B) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::DOWN) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::UP) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::LEFT) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::RIGHT) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::SELECT) {
            if !any {
                any = true;
            }
        }
        if self.is_pressed(Button::START) {
            if !any {
                any = true;
            }
        }
        //Button::iter().any(|button| { self.is_pressed(button) })
        any
    }
}

pub struct InputController {
    pub register: P1,
    pressed_directional: P1,
    pressed_button: P1,
}

impl InputController {
    pub fn new() -> InputController {
        Self {
            register: P1::INITIAL_STATE,
            pressed_button: P1::empty(),
            pressed_directional: P1::empty(),
        }
    }
    pub fn update_state(&mut self, interrupts: &mut InterruptHandler) {
        let mut register = self.register.clone();
        if self.register.contains(P1::SELECT_BUTTON) {
            register |= self.pressed_button;
        } else if self.register.contains(P1::SELECT_DIRECTIONAL) {
            register |= self.pressed_directional;
        }
        if register != self.register {
            self.register = register;
            interrupts.request(InterruptLine::JOYPAD, true);
        }
    }

    pub fn write_register(&mut self, value: u8) {
        self.register = P1::from_bits_truncate(!value);
        self.register &= P1::WRITABLE;
    }

    pub fn read_register(&self) -> u8 {
        !self.register.bits()
    }

    pub fn key_pressed(&mut self, button: Button) {
        self.pressed_directional.insert(P1::directional(&button));
        self.pressed_button.insert(P1::button(&button));
    }

    pub fn key_released(&mut self, button: Button) {
        self.pressed_directional.remove(P1::directional(&button));
        self.pressed_button.remove(P1::button(&button));
    }
}

bitflags::bitflags!(
  /// P1 register
  ///
  /// Bits are inverted in get_register/set_register, so in P1
  /// a set bit is 1 as usual.
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct P1: u8 {
    const P10                = 1 << 0; // P10: →, A
    const P11                = 1 << 1; // P11: ←, B
    const P12                = 1 << 2; // P12: ↑, Select
    const P13                = 1 << 3; // P13: ↓, Start
    const SELECT_DIRECTIONAL = 1 << 4; // P14: Select dpad
    const SELECT_BUTTON      = 1 << 5; // P15: Select buttons

    /// Only select bits are writable
    const WRITABLE =
      P1::SELECT_DIRECTIONAL.bits() | P1::SELECT_BUTTON.bits();

    /// DMG: initial state 0xCF
    /// See docs/accuracy/joypad.markdown
    const INITIAL_STATE = P1::WRITABLE.bits();
  }
);

impl P1 {
    fn directional(key: &Button) -> P1 {
        match *key {
            Button::RIGHT => P1::P10,
            Button::LEFT => P1::P11,
            Button::UP => P1::P12,
            Button::DOWN => P1::P13,
            _ => P1::empty(),
        }
    }
    fn button(key: &Button) -> P1 {
        match *key {
            Button::A => P1::P10,
            Button::B => P1::P11,
            Button::SELECT => P1::P12,
            Button::START => P1::P13,
            _ => P1::empty(),
        }
    }
}
