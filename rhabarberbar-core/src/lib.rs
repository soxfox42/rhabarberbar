mod string;
mod structs;

use std::mem;

use structs::BdxHeader;

#[derive(Debug)]
pub struct SavFile {
    pub records: [BdxRecord; 150],
}

impl SavFile {
    pub fn from_bytes(bytes: &[u8]) -> SavFile {
        let records = (0..150)
            .map(|i| {
                let position = 0x190000 + i * 0x8000;
                let record = &bytes[position..position + 0x8000];
                BdxRecord::from_bytes(record)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        SavFile { records }
    }
}

#[derive(Debug)]
pub struct BdxRecord {
    header: BdxHeader,
}

impl BdxRecord {
    pub fn from_bytes(bytes: &[u8]) -> BdxRecord {
        let header = *bytemuck::from_bytes::<BdxHeader>(&bytes[0..mem::size_of::<BdxHeader>()]);
        BdxRecord { header }
    }

    pub fn used(&self) -> bool {
        self.header.slot_used == 0x80000001
    }

    pub fn label(&self) -> String {
        self.header
            .label
            .iter()
            .map(|label| string::decode(label))
            .collect()
    }

    pub fn contributor(&self) -> String {
        string::decode(&self.header.contributor)
    }
}
