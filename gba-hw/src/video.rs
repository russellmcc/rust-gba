/// `LayerSet` represents a collection of display layers
bitflags! {
    pub flags LayerSet: u16 {
        const BG0   = 1 << 0,
        const BG1   = 1 << 1,
        const BG2   = 1 << 2,
        const BG3   = 1 << 3,
        const OBJ   = 1 << 4,
    }
}

/// `WindowSet` represents a collection of display layers
bitflags! {
    pub flags WindowSet: u16 {
        const WINDOW0   = 1 << 0,
        const WINDOW1   = 1 << 1,
        const OBJ_WINDOW   = 1 << 2,
    }
}

pub use ::gen::video::*;

register!(
    /// `display_control` controls high-level settings for the LCD
    /// display controller.
    pub display_control: DisplayControl => 0x400_0000);
