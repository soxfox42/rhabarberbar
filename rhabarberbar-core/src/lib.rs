mod lzss;
mod string;
mod structs;

use std::mem;

use structs::{BdxHeader, BdxSavHeader};

#[derive(Debug)]
pub struct SavFile {
    pub records: Vec<BdxRecord>,
}

impl SavFile {
    pub fn from_bytes(bytes: &[u8]) -> SavFile {
        let records = (0..150)
            .filter_map(|i| {
                let position = 0x190000 + i * 0x8000;
                let record = &bytes[position..position + 0x8000];
                BdxRecord::from_bytes_sav(record)
            })
            .collect::<Vec<_>>();

        SavFile { records }
    }
}

#[derive(Debug)]
pub struct BdxRecord {
    header: BdxHeader,
    data: Vec<u8>,
}

impl BdxRecord {
    pub fn from_bytes_sav(bytes: &[u8]) -> Option<BdxRecord> {
        let header_size = mem::size_of::<BdxHeader>();
        let header: BdxHeader = *bytemuck::from_bytes(&bytes[0..header_size]);

        if header.slot_used != 0x80000001 {
            return None;
        }

        let sav_header_size = mem::size_of::<BdxSavHeader>();
        let sav_header: BdxSavHeader =
            *bytemuck::from_bytes(&bytes[header_size..header_size + sav_header_size]);

        let data = lzss::decompress(
            sav_header.length as usize,
            &bytes[header_size + sav_header_size..],
        );
        Some(BdxRecord { header, data })
    }

    pub fn to_bytes_bdx(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(0x8000);
        let header_bytes = bytemuck::bytes_of(&self.header);
        bytes.extend_from_slice(header_bytes);
        bytes.extend_from_slice(&self.data);
        bytes.resize(0x8000, 0);
        bytes
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
