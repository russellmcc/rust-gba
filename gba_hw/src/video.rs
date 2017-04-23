// use vcell::VolatileCell;

// pub struct DisplayControl {
//     bits: VolatileCell<u16>
// }

// pub struct DisplayControlR {
//     bits: u16
// }

// impl DisplayControl {
//     pub fn read(&self) -> DisplayControlR {
//         DisplayControlR {bits: self.bits.get()}
//     }
// }

// #[repr(C)]
// pub enum HBlankAccessMode {
//     NoHBlankOAMAccess,
//     HBlankOAMAccess
// }

// impl DisplayControlR {
//     pub fn bg_mode(&self) -> u8 {
//         bits & 0x7
//     }

//     pub fn current_display_frame(&self) -> u8 {
//         (bits >> 4) & 1
//     }

// }

