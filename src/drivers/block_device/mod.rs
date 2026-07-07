use core::fmt::Display;

use alloc::string::String;

pub mod sdhci;

pub trait BlockDevice {
    const BLOCK_SIZE: usize;
    fn read(&self, block_address: usize, buffer: &mut [u8]) -> Result<(), String>;
    fn write(&self, block_address: usize, buffer: &[u8]) -> Result<(), String>;
    fn block_count(&self) -> usize;
}

#[derive(Debug)]
pub enum BlockDeviceError {}

impl Display for BlockDeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
