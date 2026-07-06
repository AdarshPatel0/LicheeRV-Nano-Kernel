pub mod sdhci;

pub trait BlockDevice {
    const BLOCK_SIZE: usize;
    fn read(&self, block_address: usize, buffer: &mut [u8]);
    fn write(&self, block_address: usize, buffer: &[u8]);
    fn block_count(&self) -> usize;
}
