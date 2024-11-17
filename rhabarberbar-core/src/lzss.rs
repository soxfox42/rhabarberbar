use std::mem;

use crate::structs::{BdxHeader, BdxSavHeader};

pub fn decompress(len: usize, data: &[u8]) -> Vec<u8> {
    let mut data = data.iter().copied();
    let mut result = Vec::with_capacity(len);

    loop {
        let flags = data.next().unwrap();
        for i in (0..8).rev() {
            if result.len() >= len {
                return result;
            }

            if flags & (1 << i) == 0 {
                let byte = data.next().unwrap();
                result.push(byte);
            } else {
                let byte1 = data.next().unwrap();
                let byte2 = data.next().unwrap();

                let offset = (((byte1 & 0x0F) as usize) << 8) | byte2 as usize;
                let length = (byte1 >> 4) as usize + 3;

                for _ in 0..length {
                    let index = result.len() - 1 - offset;
                    let byte = result[index];
                    result.push(byte);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Chunk {
    Literal(u8),
    Backref { offset: usize, length: usize },
}

fn encode_chunks(buf: &mut Vec<u8>, chunks: &[Chunk]) {
    let mut code_byte = 0;
    for i in 0..8 {
        if i < chunks.len() {
            let bit = match chunks[i] {
                Chunk::Literal(_) => 0,
                Chunk::Backref { .. } => 1,
            };
            code_byte |= bit << (7 - i);
        }
    }
    buf.push(code_byte);

    for chunk in chunks.iter().copied() {
        match chunk {
            Chunk::Literal(byte) => buf.push(byte),
            Chunk::Backref { offset, length } => {
                buf.push(((length - 3) << 4) as u8 | ((offset >> 8) & 0x0F) as u8);
                buf.push(offset as u8);
            }
        }
    }
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut buf =
        Vec::with_capacity(0x8000 - mem::size_of::<BdxHeader>() - mem::size_of::<BdxSavHeader>());

    let mut in_index = 0;
    let mut chunks: Vec<Chunk> = Vec::new();

    while in_index < data.len() {
        if chunks.len() == 8 {
            encode_chunks(&mut buf, &chunks);
            chunks.clear();
        }

        let mut best_position = 0;
        let mut best_length = 0;

        let start_position = in_index.saturating_sub(4096);
        for position in start_position..in_index {
            let mut length = 0;
            while in_index + length < data.len() && length < 0x12 {
                if data[in_index + length] != data[position + length] {
                    break;
                }
                length += 1;
            }

            if length > best_length {
                best_position = position;
                best_length = length;
                if in_index + length == data.len() {
                    break;
                }
            }
        }

        if best_length < 3 {
            chunks.push(Chunk::Literal(data[in_index]));
            in_index += 1;
        } else {
            chunks.push(Chunk::Backref {
                offset: in_index - best_position - 1,
                length: best_length,
            });
            in_index += best_length;
        }
    }

    if chunks.len() > 0 {
        encode_chunks(&mut buf, &chunks);
    }

    buf
}
