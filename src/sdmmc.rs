use spin::mutex;

use crate::print::println;

const SDIO_BASE_ADDRESS: usize = 0x4310000;

static CARD: spin::Mutex<core::mem::MaybeUninit<sdmmc_protocol::sdio::SdioSdmmc<sdhci_host::Sdhci>>> = mutex::Mutex::new(core::mem::MaybeUninit::zeroed());

pub fn initialize_card() -> sdmmc_protocol::sdio::CardInfo {
    let card_mutex = &mut *CARD.lock();
    let host = unsafe { sdhci_host::Sdhci::new_from_addr(SDIO_BASE_ADDRESS) };
    let mut card = sdmmc_protocol::sdio::SdioSdmmc::new(host);
    let mut scratch = sdmmc_protocol::sdio::SdioInitScratch::new();
    let mut init_request = card.submit_init(&mut scratch).unwrap();
    let card_info = loop {
        if let sdmmc_protocol::OperationPoll::Complete(card_info) = card.poll_init_request(&mut init_request).unwrap() {
            break card_info;
        }
    };
    card_mutex.write(card);
    card_info
}

pub fn read_blocks(block: u32, buffer: &mut [u8]) {
    let mut card_mutex = CARD.lock();
    let card = unsafe { &mut *card_mutex.as_mut_ptr() };
    println!("reading {} bytes from block {}", buffer.len(), block);
    let mut read_request = card.submit_read_blocks_into(block, buffer).unwrap();
    while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut read_request).unwrap() {}
}

pub fn write_blocks(block: u32, buffer: &[u8]) {
    let mut card_mutex = CARD.lock();
    let card = unsafe { &mut *card_mutex.as_mut_ptr() };
    println!("writing {} bytes to block {}", buffer.len(), block);
    let mut write_request = card.submit_write_blocks_from(block, buffer).unwrap();
    while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut write_request).unwrap() {}
}
