use core::error::Error;

use alloc:: vec::Vec;

pub mod ext4;

pub trait FileSystem {
    type Error: Error;
    fn read_file(&mut self, path: &str) -> Result<Vec<u8>, FileSystemError<Self::Error>>;
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), FileSystemError<Self::Error>>;
    fn create_file(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn remove_file(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn create_directory(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn remove_directory(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
}

pub enum FileSystemError<T: Error> {
    InvalidInput(T),
    AlreadyExists(T),
    NotFound(T),
    NotDirectory(T),
    IsDirectory(T),
    BlockDeviceError(T),
    FileSystemError(T),
    PermissionError(T),
    ReadOnly(T),
    NoSpace(T),
}