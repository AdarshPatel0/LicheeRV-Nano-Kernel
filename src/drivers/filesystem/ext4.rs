use core::{
    error::{self, Error},
    fmt::Display,
};

use crate::drivers::{
    block_device::BlockDevice,
    filesystem::{FileSystem, FileSystemError},
};

pub struct Ext4FileSystem<T: BlockDevice> {
    filesystem: rsext4::Ext4FileSystem,
    jbd2_dev: rsext4::Jbd2Dev<Ext4Partition<T>>,
}

impl<T: BlockDevice> Ext4FileSystem<T> {
    pub fn new(partition: Ext4Partition<T>) -> Self {
        let mut jbd2_dev = rsext4::Jbd2Dev::initial_jbd2dev(0, partition, true);
        let filesystem = rsext4::Ext4FileSystem::mount(&mut jbd2_dev).unwrap();
        Self { filesystem, jbd2_dev }
    }
}

#[derive(Clone)]
pub struct Ext4Partition<T: BlockDevice> {
    block_device: T,
    start_block: usize,
    block_count: usize,
    scale_factor: usize,
}

impl<T: BlockDevice> Ext4Partition<T> {
    pub fn new(block_device: T, start_block: usize, block_count: usize) -> Self {
        let scale_factor = rsext4::BLOCK_SIZE / T::BLOCK_SIZE;
        Self { block_device, start_block, block_count, scale_factor }
    }
}

impl<T: BlockDevice> rsext4::BlockDevice for Ext4Partition<T> {
    fn read(&mut self, buffer: &mut [u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> rsext4::Ext4Result<()> {
        let block_address = (block_id.as_usize().unwrap() * self.scale_factor) + self.start_block;
        self.block_device.read(block_address as usize, buffer);
        Ok(())
    }

    fn write(&mut self, buffer: &[u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> rsext4::Ext4Result<()> {
        let block_address = (block_id.as_usize().unwrap() * self.scale_factor) + self.start_block;
        self.block_device.write(block_address as usize, buffer);
        Ok(())
    }

    fn open(&mut self) -> rsext4::Ext4Result<()> {
        Ok(())
    }

    fn close(&mut self) -> rsext4::Ext4Result<()> {
        Ok(())
    }

    fn total_blocks(&self) -> u64 {
        self.block_count as u64
    }

    fn current_time(&self) -> rsext4::Ext4Result<rsext4::Ext4Timestamp> {
        Ok(rsext4::Ext4Timestamp::new(0, 0))
    }

    fn block_size(&self) -> u32 {
        T::BLOCK_SIZE as u32
    }

    fn flush(&mut self) -> rsext4::Ext4Result<()> {
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_readonly(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct Ext4Error {
    pub error: rsext4::Ext4Error,
}

impl Ext4Error {
    fn new(error: rsext4::Ext4Error) -> FileSystemError<Ext4Error> {
        match error.code {
            rsext4::Errno::EINVAL => FileSystemError::InvalidInput(Ext4Error { error }),
            rsext4::Errno::ENOENT => FileSystemError::NotFound(Ext4Error { error }),
            rsext4::Errno::EEXIST => FileSystemError::AlreadyExists(Ext4Error { error }),
            rsext4::Errno::ENOTDIR => FileSystemError::NotDirectory(Ext4Error { error }),
            rsext4::Errno::EISDIR => FileSystemError::IsDirectory(Ext4Error { error }),
            rsext4::Errno::EIO => FileSystemError::IO(Ext4Error { error }),
            rsext4::Errno::ENOSPC => FileSystemError::NoSpace(Ext4Error { error }),
            rsext4::Errno::EROFS => FileSystemError::ReadOnly(Ext4Error { error }),
            rsext4::Errno::EACCES => FileSystemError::PermissionDenied(Ext4Error { error }),
            rsext4::Errno::EOPNOTSUPP => FileSystemError::Unsupported(Ext4Error { error }),
            _ => FileSystemError::FileSystem(Ext4Error { error }),
        }
    }
}

impl Display for Ext4Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for Ext4Error {}

impl<T: BlockDevice> FileSystem for Ext4FileSystem<T> {
    type Error = Ext4Error;

    fn read_file(&mut self, path: &str) -> Result<alloc::vec::Vec<u8>, super::FileSystemError<Self::Error>> {
        match rsext4::file::read_file(&mut self.jbd2_dev, &mut self.filesystem, path) {
            Ok(data) => Ok(data),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::file::write_file(&mut self.jbd2_dev, &mut self.filesystem, path, 0, data) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn mkfile(&mut self, path: &str) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::file::mkfile(&mut self.jbd2_dev, &mut self.filesystem, path, None, None) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn rmfile(&mut self, path: &str) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::file::delete_file(&mut self.filesystem, &mut self.jbd2_dev, path) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn mkdir(&mut self, path: &str) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::dir::mkdir(&mut self.jbd2_dev, &mut self.filesystem, path) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn rmdir(&mut self, path: &str) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::file::delete_dir(&mut self.filesystem, &mut self.jbd2_dev, path) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }

    fn mv(&mut self, source_path: &str, destination_path: &str) -> Result<(), super::FileSystemError<Self::Error>> {
        match rsext4::file::mv(&mut self.filesystem, &mut self.jbd2_dev, source_path, destination_path) {
            Ok(_) => Ok(()),
            Err(error) => Err(Ext4Error::new(error)),
        }
    }
}
