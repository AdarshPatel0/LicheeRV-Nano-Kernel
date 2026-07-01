use crate::sdmmc::{read_blocks, write_blocks};
use rsext4::{Ext4FileSystem, Ext4Result, Ext4Timestamp, Jbd2Dev};

pub fn initialize_filesystem(start_block: u32, block_count: u32) {
    let block_device = BlockDevice { start_block, block_count };
    let mut journaling_block_device = Jbd2Dev::initial_jbd2dev(0, block_device, true);
    let filesystem = Ext4FileSystem::mount(&mut journaling_block_device).unwrap();
}

struct BlockDevice {
    start_block: u32,
    block_count: u32,
}

impl rsext4::BlockDevice for BlockDevice {
    fn write(&mut self, buffer: &[u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> Ext4Result<()> {
        let block_address = (block_id.to_u32().unwrap() * 8) + self.start_block;
        write_blocks(block_address, buffer);
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> Ext4Result<()> {
        let block_address = (block_id.to_u32().unwrap() * 8) + self.start_block;
        read_blocks(block_address, buffer);
        Ok(())
    }

    fn open(&mut self) -> Ext4Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Ext4Result<()> {
        Ok(())
    }

    fn total_blocks(&self) -> u64 {
        self.block_count as u64
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
        true
    }
}
