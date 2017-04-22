use vcell::VolatileCell;

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

use core::mem;
# [ repr ( u16 ) ]
pub enum TestEnum {
    # [ doc = "Foo Is a really cool bar." ]
    TestOne,
    # [ doc = "BORING" ]
    TestTwo = 7,
}
# [ doc = "A register for testing" ]
pub struct TestRegister {
    value: VolatileCell<u16>,
}
pub struct TestRegisterRead {
    value: u16,
}
impl TestRegisterRead {
    # [ doc = "A field that represents an integer." ]
    # [ inline ]
    pub fn int_field(&self) -> u8 {
        ((self.value & 7) >> 0) as u8
    }
    # [ doc = "A field that is an enum" ]
    # [ inline ]
    pub fn enum_field(&self) -> TestEnum {
        unsafe { mem::transmute(((self.value & 56) >> 3)) }
    }
    # [ doc = "A basic field that is an enum" ]
    # [ inline ]
    pub fn bool_is_enabled(&self) -> bool {
        (((self.value & 64) >> 6)) != 0
    }
    # [ inline ]
    pub fn test_one(&self) -> bool {
        return (TestEnum::TestOne as u16) == ((self.value & 56) >> 3);
    }
    # [ inline ]
    pub fn test_two(&self) -> bool {
        return (TestEnum::TestTwo as u16) == ((self.value & 56) >> 3);
    }
}
pub struct TestRegisterWrite {
    value: u16,
}
impl TestRegisterWrite {
    # [ doc = "A field that represents an integer." ]
    # [ inline ]
    pub fn set_int_field(&mut self, tt: u8) -> &mut TestRegisterWrite {
        self.value &= !7;
        self.value |= ((tt as u16) << 0) & 7;
        self
    }
    # [ doc = "A field that is an enum" ]
    # [ inline ]
    pub fn set_enum_field(&mut self, tt: TestEnum) -> &mut TestRegisterWrite {
        self.value &= !56;
        self.value |= ((tt as u16) << 3) & 56;
        self
    }
    # [ doc = "A basic field that is an enum" ]
    # [ inline ]
    pub fn set_bool_is_enabled(&mut self, tt: bool) -> &mut TestRegisterWrite {
        self.value &= !64;
        self.value |= ((if tt { 1 } else { 0 }) << 6) & 64;
        self
    }
    # [ inline ]
    pub fn set_test_one(&mut self) -> &mut TestRegisterWrite {
        self.value &= !56;
        self.value |= ((TestEnum::TestOne as u16) << 3) & 56;
        self
    }
    # [ inline ]
    pub fn set_test_two(&mut self) -> &mut TestRegisterWrite {
        self.value &= !56;
        self.value |= ((TestEnum::TestTwo as u16) << 3) & 56;
        self
    }
}
impl Default for TestRegisterWrite {
    fn default() -> TestRegisterWrite {
        TestRegisterWrite { value: 0 }
    }
}
impl TestRegister {
    pub fn read(&self) -> TestRegisterRead {
        TestRegisterRead { value: self.value.get() }
    }
    pub fn write(&mut self, write: &TestRegisterWrite) -> &mut TestRegister {
        self.value.set(write.value);
        self
    }
}
