mod cpu;
mod hardware;
mod memory;
mod util;
pub mod gameboy;

extern crate phf;
extern crate num_traits;
#[macro_use]
extern crate enum_display_derive;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
