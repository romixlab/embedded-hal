use crate::{DsiReadCommand, DsiWriteCommand};

pub trait DsiHostCtrlIo {
    type Error;

    fn write(&mut self, command: DsiWriteCommand) -> Result<(), Self::Error>;
    fn read(&mut self, command: DsiReadCommand, buf: &mut [u8]) -> Result<(), Self::Error>;
}