use crate::drivers::block_device::BlockDevice;

pub struct Ext4FileSystem {
    filesystem: rsext4::Ext4FileSystem,
}

impl Ext4FileSystem {
    pub fn new<T: BlockDevice>(ext4_partition: Ext4Partition<T>) -> Self {
        let mut journaling_block_device = rsext4::Jbd2Dev::initial_jbd2dev(0, ext4_partition, true);
        let filesystem = rsext4::Ext4FileSystem::mount(&mut journaling_block_device).unwrap();
        Self { filesystem }
    }
}

pub struct Ext4Partition<T: BlockDevice> {
    block_device: T,
    start_block: usize,
    block_count: usize,
    scale_factor: usize,
}

impl<T: BlockDevice + Clone> Clone for Ext4Partition<T> {
    fn clone(&self) -> Self {
        Self { block_device: self.block_device.clone(), start_block: self.start_block.clone(), block_count: self.block_count.clone(), scale_factor: self.scale_factor.clone() }
    }
}

impl<T: BlockDevice> Ext4Partition<T> {
    pub fn new(block_device: T, start_block: usize, block_count: usize) -> Self {
        let scale_factor = rsext4::BLOCK_SIZE / block_device.block_size();
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
        self.block_device.block_size() as u32
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
