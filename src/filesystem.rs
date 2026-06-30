use crate::sdmmc::{read_blocks, write_blocks};
use core::mem::MaybeUninit;
use rsext4::{Ext4FileSystem, Ext4Result, Ext4Timestamp, Jbd2Dev};
use spin::Mutex;

pub static FILESYSTEM: Mutex<MaybeUninit<Ext4FileSystem>> = Mutex::new(MaybeUninit::uninit());

pub fn initialize_filesystem(start_sector_lba: u32, sector_count_lba: u32) {
    let mut filesystem_mutex = FILESYSTEM.lock();
    let block_device = BlockDevice { start_sector_lba, sector_count_lba };
    let mut journaling_block_device = Jbd2Dev::initial_jbd2dev(0, block_device, true);
    let filesystem = Ext4FileSystem::mount(&mut journaling_block_device).unwrap();
    filesystem_mutex.write(filesystem);
}

struct BlockDevice {
    start_sector_lba: u32,
    sector_count_lba: u32,
}

impl rsext4::BlockDevice for BlockDevice {
    fn write(&mut self, buffer: &[u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> Ext4Result<()> {
        let sector = block_id.to_u32().unwrap() * 4 + self.start_sector_lba;
        write_blocks(sector, buffer);
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> Ext4Result<()> {
        let sector = block_id.to_u32().unwrap() * 4 + self.start_sector_lba;
        read_blocks(sector, buffer);
        Ok(())
    }

    fn open(&mut self) -> Ext4Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Ext4Result<()> {
        Ok(())
    }

    fn total_blocks(&self) -> u64 {
        self.sector_count_lba as u64 / 4
    }

    fn current_time(&self) -> Ext4Result<Ext4Timestamp> {
        Ext4Result::Ok(Ext4Timestamp::new(0, 0))
    }

    fn block_size(&self) -> u32 {
        512
    }

    fn flush(&mut self) -> Ext4Result<()> {
        Ok(())
    }

    fn is_open(&self) -> bool {
        true
    }

    fn is_readonly(&self) -> bool {
        false
    }
}
