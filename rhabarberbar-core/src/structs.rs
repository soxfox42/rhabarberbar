use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub(crate) struct InstrumentHeader {
    volume: u16,
    voice: u16,
    stereo: i8,
    master_pro: u8,
    clone_id: i8,
    amateur_beginner: u8,
    zeros: [i32; 2],
}

#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C, packed)]
pub(crate) struct BdxHeader {
    checksums: [u16; 2],
    magic: [u8; 8],
    identifier: [u8; 4],
    pub slot_used: u32,
    compressed_length: u32,
    hmac_sha1: [u8; 20],
    karaoke_stuff: [u8; 20],
    redundant: [u8; 8],
    pub label: [[u8; 32]; 3],
    zero_a: u16,
    meter: u8,
    lines_plus_6: u8,
    zero_b: u8,
    line_plus: u8,
    melody: u8,
    karaoke: u8,
    day: u8,
    month: u8,
    year: u16,
    unofficial: u8,
    unknown: u16,
    volume: u8,
    record_id: u32,
    date: [u8; 8],
    zero_c: u32,
    instrument_headers: [InstrumentHeader; 8],
    short_unknown: u16,
    name_length: u16,
    pub contributor: [u8; 128],
}
