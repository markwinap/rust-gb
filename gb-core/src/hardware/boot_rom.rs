use alloc::vec::Vec;
use core::ops::Index;

pub struct BootromData(pub Vec<u8>);

impl BootromData {
    pub fn new() -> BootromData {
        BootromData(Vec::with_capacity(256))
    }

    pub fn from_bytes(bytes: &[u8]) -> BootromData {
        let mut x: Vec<u8> = core::iter::repeat(0).take(bytes.len()).collect();
        x.clone_from_slice(bytes);
        BootromData(x)
    }
}

// #[derive(Clone)]
pub struct Bootrom {
    data: BootromData,
    active: bool,
}

impl Bootrom {
    pub fn new(config: Option<BootromData>) -> Bootrom {
        let (active, data) = match config {
            Some(config_data) => (true, config_data),
            None => (false, BootromData::new()),
        };

        Bootrom { data, active }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

impl Index<u16> for Bootrom {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.data.0[index as usize]
    }
}
