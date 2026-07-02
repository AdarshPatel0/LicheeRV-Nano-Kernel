const BLOCK_SIZE: usize = 512;

pub struct SdmmcBlockDevice {
    card: spin::Mutex<sdmmc_protocol::sdio::SdioSdmmc<sdhci_host::Sdhci>>,
    card_info: sdmmc_protocol::sdio::CardInfo,
}

impl SdmmcBlockDevice {
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
        let card = spin::Mutex::new(card);
        SdmmcBlockDevice { card, card_info }
    }
}

impl crate::drivers::block_device::BlockDevice for SdmmcBlockDevice {
    fn read(&self, block_address: usize, buffer: &mut [u8]) {
        assert!(buffer.len() % 512 == 0);
        let mut card = self.card.lock();
        let block_count = buffer.len() / 512;
        for block in 0..block_count {
            let buffer_offset = block * 512;
            let mut sub_buffer = &mut buffer[buffer_offset..buffer_offset + 512];
            let mut read_request = card.submit_read_blocks_into((block + block_address) as u32, &mut sub_buffer).unwrap();
            while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut read_request).unwrap() {}
        }
    }

    fn write(&self, block_address: usize, buffer: &[u8]) {
        assert!(buffer.len() % 512 == 0);
        let mut card = self.card.lock();
        let block_count = buffer.len() / 512;
        for block in 0..block_count {
            let buffer_offset = block * 512;
            let mut sub_buffer = &buffer[buffer_offset..buffer_offset + 512];
            let mut write_request = card.submit_write_blocks_from((block + block_address) as u32, &mut sub_buffer).unwrap();
            while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut write_request).unwrap() {}
        }
    }

    fn total_blocks(&self) -> usize {
        self.card_info.capacity_blocks.unwrap() as usize
    }

    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }
}

impl crate::drivers::block_device::BlockDevice for alloc::rc::Rc<SdmmcBlockDevice> {
    fn read(&self, block_address: usize, buffer: &mut [u8]) {
        assert!(buffer.len() % 512 == 0);
        let mut card = self.card.lock();
        let block_count = buffer.len() / 512;
        for block in 0..block_count {
            let buffer_offset = block * 512;
            let mut sub_buffer = &mut buffer[buffer_offset..buffer_offset + 512];
            let mut read_request = card.submit_read_blocks_into((block + block_address) as u32, &mut sub_buffer).unwrap();
            while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut read_request).unwrap() {}
        }
    }

    fn write(&self, block_address: usize, buffer: &[u8]) {
        assert!(buffer.len() % 512 == 0);
        let mut card = self.card.lock();
        let block_count = buffer.len() / 512;
        for block in 0..block_count {
            let buffer_offset = block * 512;
            let mut sub_buffer = &buffer[buffer_offset..buffer_offset + 512];
            let mut write_request = card.submit_write_blocks_from((block + block_address) as u32, &mut sub_buffer).unwrap();
            while let sdmmc_protocol::DataCommandPoll::Pending = card.poll_data_request(&mut write_request).unwrap() {}
        }
    }

    fn total_blocks(&self) -> usize {
        self.card_info.capacity_blocks.unwrap() as usize
    }

    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }
}
