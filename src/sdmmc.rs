use spin::mutex;

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

pub fn read_blocks(block_address: u32, buffer: &mut [u8]) {
    assert!(buffer.len() % 512 == 0);
    let mut card_mutex = CARD.lock();
    let card = unsafe { &mut *card_mutex.as_mut_ptr() };
    let block_count = buffer.len() / 512;
    for block in 0..block_count {
        let buffer_offset = block * 512;
        let mut sub_buffer = &mut buffer[buffer_offset..buffer_offset + 512];
        let mut read_request = card.submit_read_blocks_into(block as u32 + block_address, &mut sub_buffer).unwrap();
        while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut read_request).unwrap() {}
    }
}

pub fn write_blocks(block_address: u32, buffer: &[u8]) {
    assert!(buffer.len() % 512 == 0);
    let mut card_mutex = CARD.lock();
    let card = unsafe { &mut *card_mutex.as_mut_ptr() };
    let block_count = buffer.len() / 512;
    for block in 0..block_count {
        let buffer_offset = block * 512;
        let mut sub_buffer = &buffer[buffer_offset..buffer_offset + 512];
        let mut write_request = card.submit_write_blocks_from(block as u32 + block_address, &mut sub_buffer).unwrap();
        while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut write_request).unwrap() {}
    }
}
