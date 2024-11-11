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