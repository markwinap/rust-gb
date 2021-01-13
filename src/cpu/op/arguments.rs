use crate::cpu::registers::Registers;
use crate::memory::AddressSpace;
use crate::cpu::interrupt_manager::InterruptManager;

pub trait Op {
    fn reads_memory(&self) -> bool { false }
    fn writes_memory(&self) -> bool { false }
    fn operand_length(&self) -> u8 { 0 }
    fn execute(&self, registers: &Registers, address_space: &dyn AddressSpace, args: &[u8], context: u16) -> u16 { context }
    fn switch_interrupts(&self, interrupt_manager: &mut InterruptManager) {}
    fn proceed(&self, registers: &Registers) {}
    fn force_finish_cycle(&self) -> bool { true }
}

pub enum DataType {
    D8(u8),
    D16(u16),
    R8(u16),
}

pub trait Argument {
    fn get_operand_length() -> u8;
    fn is_memory() -> bool;

    fn read(registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8]) -> Option<DataType>;
    fn write(registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType);
    fn get_label() -> &'static str;
}


macro_rules! argument {
    ($name:ident, $label: expr, $operand_length: expr, $is_memory: expr,  $read: expr, $write: expr) => {
        pub struct $name;

        impl Argument for $name {
            fn get_operand_length() -> u8 {
                $operand_length
            }

            fn is_memory() -> bool {
                $is_memory
            }

            fn read(registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8]) -> Option<DataType> {
               $read(registers, address_space, args)
            }

            fn write(registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType) {
               $write(registers, address_space, args, value);
            }
            fn get_label() -> &'static str {
                $label
            }
        }
    }
}


argument!(A, "A",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_a())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_a(val); };
});

argument!(B, "B",  0, false,  | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_b())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_b(val); };
});
argument!(C, "C",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_c())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_c(val); };
});
argument!(D, "D",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_d())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_d(val); };
});
argument!(E, "E",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_e())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_a(val); };
});
argument!(H, "H",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_h())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_h(val); };
});
argument!(L, "L",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(registers.get_l())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { registers.set_l(val); };
});
/////////////


argument!(AF, "AF",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_af())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_af(val); };
});
argument!(BC, "BC",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_bc())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_bc(val); };
});
argument!(DE, "DE",  0, false,| registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_de())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_de(val); };
});
argument!(HL, "HL",  0, false,| registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_hl())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_hl(val); };
});
argument!(SP, "SP",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_sp())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_sp(val); };
});
argument!(PC, "PC",  0, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(registers.get_pc())) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D16(val) = value { registers.set_pc(val); };
});

////////////////
argument!(D8, "d8",  1, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D8(args[0])) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
      unimplemented!()
});

argument!(D16, "d16",  2, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(((args[0] as u16) << 8) | args[1] as u16)) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
      unimplemented!()
});

//TODO Review!!!! I THINK ITS RIGHT! :)
argument!(R8, "r8",  2, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  {
    Some(DataType::R8(args[0] as i8 as i16 as u16))
} ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
      unimplemented!()
});

argument!(A16, "a16",  2, false, | registers: &Registers, address_space: &dyn AddressSpace, args: &[u8] |  Some(DataType::D16(((args[0] as u16) << 8) | args[1] as u16)) ,
| registers: &mut Registers, address_space: &dyn AddressSpace, args: &[u8], value: DataType | {
      unimplemented!()
});

//////////////////////////////
argument!(_BC, "(BC)",  0, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
  address_space.get_byte(registers.get_bc()).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(registers.get_bc(), val); };
});

argument!(_DE, "(DE)",  0, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
  address_space.get_byte(registers.get_de()).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(registers.get_de(), val); };
});


argument!(_HL, "(HL)",  0, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
  address_space.get_byte(registers.get_hl()).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(registers.get_hl(), val); };
});

argument!(_A8, "(A8)",  1, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
  address_space.get_byte(0xFF00 | args[0] as u16).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(0xFF00 | args[0] as u16, val); };
});

argument!(_A16, "(A16)",  2, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
 address_space.get_byte(((args[0] as u16) << 8) | args[1] as u16).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(((args[0] as u16) << 8) | args[1] as u16, val); };
});

argument!(_C, "(C)",  1, true, | registers: &Registers, address_space: &mut dyn AddressSpace, args: &[u8] |
  address_space.get_byte(0xFF00 | registers.get_c() as u16).map(DataType::D8),
| registers: &mut Registers, address_space: &mut dyn AddressSpace, args: &[u8], value: DataType | {
     if let DataType::D8(val) = value { address_space.set_byte(0xFF00 | registers.get_c() as u16, val); };
});