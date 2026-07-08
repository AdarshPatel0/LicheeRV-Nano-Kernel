use core::error::Error;

use alloc::vec::Vec;

pub mod ext4;

pub trait FileSystem {
    type Error: Error;
    fn read_file(&mut self, path: &str) -> Result<Vec<u8>, FileSystemError<Self::Error>>;
    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), FileSystemError<Self::Error>>;
    fn mkfile(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn rmfile(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn mkdir(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn rmdir(&mut self, path: &str) -> Result<(), FileSystemError<Self::Error>>;
    fn mv(&mut self, source_path: &str, destination_path: &str) -> Result<(), FileSystemError<Self::Error>>;
}

#[derive(Debug)]
pub enum FileSystemError<T: Error> {
    InvalidInput(T),
    AlreadyExists(T),
    NotFound(T),
    NotDirectory(T),
    IsDirectory(T),
    IO(T),
    PermissionDenied(T),
    ReadOnly(T),
    NoSpace(T),
    Unsupported(T),
    FileSystem(T)
}
