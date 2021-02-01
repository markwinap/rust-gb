

use std::ops::Index;
use std::sync::Arc;
use std::path::Path;

pub struct BootromData(pub [u8; 0x100]);

impl BootromData {
    pub fn new() -> BootromData {
        BootromData([0; 0x100])
    }
}

impl Clone for BootromData {
    fn clone(&self) -> BootromData {
        BootromData((*self).0)
    }
}


#[derive(Clone)]
pub struct Bootrom {
    data: Arc<BootromData>,
    active: bool,
}

impl Bootrom {
    pub fn new(config: Option<Arc<BootromData>>) -> Bootrom {
        let (active, data) = match config {
            Some(config_data) => (true, config_data),
            None => (false, Arc::new(BootromData::new())),
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
