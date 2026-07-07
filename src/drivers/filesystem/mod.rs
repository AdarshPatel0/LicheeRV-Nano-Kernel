use core::error::Error;

use alloc::{string::String, vec::Vec};

pub mod ext4;

pub trait FileSystem {
    type Error: Error;
    fn read_file(path: &str) -> Result<Vec<u8>, String>;
    fn write_file(path: &str, data: &[u8]) -> Result<(), String>;
    fn is_directory(path: &str) -> Result<bool, String>;
    fn is_file(path: &str) -> Result<bool, String>;
    fn create_file(path: &str) -> Result<(), String>;
    fn delete_file(path: &str) -> Result<(), String>;
    fn create_directory(path: &str) -> Result<(), String>;
    fn delete_directory(path: &str) -> Result<(), String>;
    fn find(path: &str) -> Result<(), String>;
}