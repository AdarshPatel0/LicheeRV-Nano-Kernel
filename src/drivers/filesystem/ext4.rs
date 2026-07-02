use crate::drivers::block_device::BlockDevice;

pub struct Ext4FileSystem {
    file_system: rsext4::Ext4FileSystem,
}

impl Ext4FileSystem {
    pub fn new<T: crate::drivers::block_device::BlockDevice>(block_device: Ext4Partition<T>) -> Self {
        let mut journaling_block_device = rsext4::Jbd2Dev::initial_jbd2dev(0, block_device, true);
        let file_system = rsext4::Ext4FileSystem::mount(&mut journaling_block_device).unwrap();
        Self { file_system }
    }
}

pub struct Ext4Partition<T: BlockDevice> {
    block_device: T,
    start_block: usize,
    block_count: usize,
    block_size: usize,
}

impl<T: BlockDevice> Ext4Partition<T> {
    pub fn new(start_block: usize, block_count: usize, block_device: T) -> Self {
        let block_size = block_device.block_size();
        Self { block_device, start_block, block_count, block_size }
    }
}

impl<T: BlockDevice> rsext4::BlockDevice for Ext4Partition<T> {
    fn write(&mut self, buffer: &[u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> rsext4::Ext4Result<()> {
        let block_address = (block_id.as_usize().unwrap() * (rsext4::BLOCK_SIZE / self.block_size)) + self.start_block;
        self.block_device.write(block_address, buffer);
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8], block_id: rsext4::bmalloc::AbsoluteBN, _count: u32) -> rsext4::Ext4Result<()> {
        let block_address = (block_id.as_usize().unwrap() * (rsext4::BLOCK_SIZE / self.block_size)) + self.start_block;
        self.block_device.read(block_address, buffer);
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
        rsext4::Ext4Result::Ok(rsext4::Ext4Timestamp::new(0, 0))
    }

    fn block_size(&self) -> u32 {
        self.block_size as u32
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
