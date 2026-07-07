use alloc::{string::String, sync::Arc};
use sdhci_host::Sdhci;
use sdmmc_protocol::sdio::{CardInfo, SdioSdmmc};
use spin::Mutex;

use crate::drivers::block_device::{
    BlockDevice,
    BlockDeviceError::{self, ReadError},
};

const BLOCK_SIZE: usize = 512;

#[derive(Clone)]
pub struct SdhciBlockDevice {
    card: Arc<Mutex<SdioSdmmc<Sdhci>>>,
    card_info: CardInfo,
}

impl SdhciBlockDevice {
    pub fn new(base_address: usize) -> Self {
        let host = unsafe { sdhci_host::Sdhci::new_from_addr(base_address) };
        let mut card = sdmmc_protocol::sdio::SdioSdmmc::new(host);
        let mut scratch = sdmmc_protocol::sdio::SdioInitScratch::new();
        let mut init_request = card.submit_init(&mut scratch).unwrap();
        let card_info = loop {
            if let sdmmc_protocol::OperationPoll::Complete(card_info) = card.poll_init_request(&mut init_request).unwrap() {
                break card_info;
            }
        };
        let card = Arc::new(Mutex::new(card));
        Self { card, card_info }
    }
}

impl BlockDevice for SdhciBlockDevice {
    const BLOCK_SIZE: usize = 512;

    fn read(&self, block_address: usize, buffer: &mut [u8]) -> Result<(), BlockDeviceError<sdmmc_protocol::Error>> {
        if buffer.len() % BLOCK_SIZE != 0 {
            return Err(BlockDeviceError::IllegalBufferLength(buffer.len()));
        }
        let mut card = self.card.lock();
        let block_count = buffer.len() / BLOCK_SIZE;
        for block in 0..block_count {
            let buffer_offset = block * BLOCK_SIZE;
            let mut sub_buffer = &mut buffer[buffer_offset..buffer_offset + BLOCK_SIZE];
            let mut read_request = match card.submit_read_blocks_into((block + block_address) as u32, &mut sub_buffer) {
                Ok(read_request) => read_request,
                Err(error) => {
                    return Err(ReadError(error));
                }
            };
            loop {
                match card.poll_data_request(&mut read_request) {
                    Ok(poll) => {
                        if let sdmmc_protocol::DataCommandPoll::Complete(_) = poll {
                            break;
                        }
                    }
                    Err(error) => return Err(ReadError(error)),
                }
            }
        }
        return Ok(());
    }

    fn write(&self, block_address: usize, buffer: &[u8]) -> Result<(), BlockDeviceError<sdmmc_protocol::Error>> {
        assert!(buffer.len() % BLOCK_SIZE == 0);
        let mut card = self.card.lock();
        let block_count = buffer.len() / BLOCK_SIZE;
        for block in 0..block_count {
            let buffer_offset = block * BLOCK_SIZE;
            let sub_buffer = &buffer[buffer_offset..buffer_offset + BLOCK_SIZE];
            let mut write_request = card.submit_write_blocks_from((block + block_address) as u32, &sub_buffer).unwrap();
            while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut write_request).unwrap() {}
        }
        return Ok(());
    }

    fn block_count(&self) -> usize {
        self.card_info.capacity_blocks.unwrap() as usize
    }

    type Error = sdmmc_protocol::Error;
}
