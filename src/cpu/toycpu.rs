

use crate::cpu::cpu::{Cpu, State};


impl Cpu {

    pub fn step(&mut self) {

        match self.state {
            State::Normal => {
                let ((step, address), cycles) = self.decode();

            }
            State::HaltState => {}
            State::Stopped => {}
            State::HaltBug => {}
        };
    }

}