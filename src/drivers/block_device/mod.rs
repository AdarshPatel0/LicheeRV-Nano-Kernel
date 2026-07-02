pub mod sdmmc;

pub trait BlockDevice {
    fn read(&self, block_address: usize, buffer: &mut [u8]);
    fn write(&self, block_address: usize, buffer: &[u8]);
    fn total_blocks(&self) -> usize;
    fn block_size(&self) -> usize;
}
