use core::{error::Error, fmt::Display};

use alloc::string::String;

pub mod sdhci;

pub trait BlockDevice {
    type Error: Error;
    const BLOCK_SIZE: usize;
    fn read(&self, block_address: usize, buffer: &mut [u8]) -> Result<(), BlockDeviceError<Self::Error>>;
    fn write(&self, block_address: usize, buffer: &[u8]) -> Result<(), BlockDeviceError<Self::Error>>;
    fn block_count(&self) -> usize;
}

#[derive(Debug)]
pub enum BlockDeviceError<T> where T: Error {
    IllegalBufferLength(usize),
    ReadError(T),
    WriteError(T)
}

impl <T: Error>Display for BlockDeviceError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BlockDeviceError::IllegalBufferLength(length) => write!(f,"{}: {}",self, length),
            BlockDeviceError::ReadError(error) => write!(f,"{}: {}",self, error),
            BlockDeviceError::WriteError(error) => write!(f,"{}: {}",self, error),
        }
    }
}