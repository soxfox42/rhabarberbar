mod hash;
mod lzss;
mod string;
mod structs;

use std::mem;

use structs::{BdxHeader, BdxSavHeader};

#[derive(Debug)]
pub struct SavFile {
    pub records: Vec<BdxRecord>,
    common_key: [u8; 40],
}

impl SavFile {
    pub fn from_bytes(bytes: &[u8]) -> SavFile {
        let records = (0..150)
            .filter_map(|i| {
                let position = 0x190000 + i * 0x8000;
                let record = &bytes[position..position + 0x8000];
                BdxRecord::from_sav_bytes(record)
            })
            .collect::<Vec<_>>();

        let common_key = bytes[0x7F0000..0x7F0000 + 40].try_into().unwrap();

        SavFile {
            records,
            common_key,
        }
    }

    pub fn to_song_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(150 * 0x8000);
        for (i, record) in self.records.iter().enumerate() {
            let mut key = [0; 42];
            key[0] = i as u8 + 1;
            key[41] = i as u8 + 1;
            key[1..=40].copy_from_slice(&self.common_key);

            let sav_bytes = record.to_sav_bytes(&key);
            bytes.extend_from_slice(&sav_bytes);
        }
        bytes.resize(0x8000 * 150, !0);
        bytes
    }
}

#[derive(Debug)]
pub struct BdxRecord {
    header: BdxHeader,
    data: Vec<u8>,
}

impl BdxRecord {
    pub fn from_sav_bytes(bytes: &[u8]) -> Option<BdxRecord> {
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

    pub fn from_bdx_file(path: &std::path::Path) -> BdxRecord {
        let bytes = std::fs::read(path).unwrap();
        let header_size = mem::size_of::<BdxHeader>();
        let header: BdxHeader = *bytemuck::from_bytes(&bytes[0..header_size]);
        let data = bytes[header_size..].to_vec();
        BdxRecord { header, data }
    }

    pub fn to_sav_bytes(&self, key: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(0x8000);

        let header = self.header.clone();
        let sav_header = BdxSavHeader {
            sixteen: 0x10,
            length: 0x7D60,
            zero: 0,
        };
        let compressed_data = lzss::compress(&self.data);

        let total_size =
            mem::size_of::<BdxHeader>() + mem::size_of::<BdxSavHeader>() + compressed_data.len();

        assert!(total_size <= 0x8000);

        bytes.extend_from_slice(bytemuck::bytes_of(&header));
        bytes.extend_from_slice(bytemuck::bytes_of(&sav_header));
        bytes.extend_from_slice(&compressed_data);

        let header: &mut BdxHeader =
            bytemuck::from_bytes_mut(&mut bytes[0..mem::size_of::<BdxHeader>()]);

        header.compressed_length = compressed_data.len() as u32 + 4;

        let hmac = hash::compute_hmac(key, header.compressed_length as usize, &bytes);

        let header: &mut BdxHeader =
            bytemuck::from_bytes_mut(&mut bytes[0..mem::size_of::<BdxHeader>()]);
        header.hmac_sha1.copy_from_slice(&hmac);

        let (check0, check1) = hash::compute_bdx_hashes(&bytes);

        let header: &mut BdxHeader =
            bytemuck::from_bytes_mut(&mut bytes[0..mem::size_of::<BdxHeader>()]);
        header.checksums[0] = check0;
        header.checksums[1] = check1;

        bytes.resize(0x8000, !0);
        bytes
    }

    pub fn to_bdx_bytes(&self) -> Vec<u8> {
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
