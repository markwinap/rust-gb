use crate::cpu::registers::Registers;
use crate::memory::AddressSpace;
use crate::cpu::interrupt_manager::InterruptManager;
use std::fmt::Debug;


mod opcodes;
mod arguments;



#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn mai2n() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
   // print(ref_x);
}

struct Wrapper<'wrapped, T: 'wrapped> {
    wrapped: &'wrapped T
}

struct Fii<'a> {
    x: &'a Other
}
struct Other {
    fd: i32
}

fn fjkd() {
    let x = Other{ fd: 2 };
    let fii = Fii { x: &x };

    let wrapper = Wrapper{ wrapped: &fii };

    drop(x);


}


//argument!(AF, "AF",  0, false, DataType::D16, | registers: &Registers |  registers.get_af() , | registers: &mut Registers , value: u8|  registers.set_af(value) );
//argument!(AF, "AF",  0, false, DataType::D16, | registers: &Registers |  registers.get_af() , | registers: &mut Registers , value: u8|  registers.set_af(value) );
