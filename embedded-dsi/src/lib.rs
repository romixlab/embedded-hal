#![no_std]

pub mod blocking;
pub use blocking::DsiHostCtrlIo;

#[repr(u8)]
#[derive(Debug)]
pub enum DsiReadCommand {
    /// 0x06
    DcsShort { dcs_cmd: u8 } = 6,

    // xx x101 0-7 arguments
    /// 0x04
    GenericShortP0 = 0x04,
    /// 0x14
    GenericShortP1 { arg0: u8 } = 0x14,
    /// 0x24
    GenericShortP2 { arg0: u8, arg1: u8 } = 0x24,
}

impl DsiReadCommand {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum DsiWriteCommand<'i> {
    /// 0x05
    DcsShortP0 { reg: u8 } = 0x5,
    /// 0x15
    DcsShortP1 { reg: u8, data: u8 } = 0x15,
    /// 0x39
    DcsLongWrite { dcs_cmd: u8, buf: &'i [u8] } = 0x39,

    /// 0x03
    GenericShortP0 = 0x03,
    /// 0x13
    GenericShortP1 = 0x13,
    /// 0x23
    GenericShortP2 = 0x23,
    /// 0x29
    GenericLongWrite { cmd: u8, buf: &'i [u8] } = 0x29,

    /// 0x37
    SetMaximumReturnPacketSize(u16) = 0x37,
}

impl<'i> DsiWriteCommand<'i> {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}