pub mod sdhci;

pub trait BlockDevice {
    fn read(&self, block_address: usize, buffer: &mut [u8]);
    fn write(&self, block_address: usize, buffer: &[u8]);
    fn block_size(&self) -> usize;
    fn block_count(&self) -> usize;
}
