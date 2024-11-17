use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn compute_hmac(key: &[u8], len: usize, data: &[u8]) -> Vec<u8> {
    let mut hmac_calc = HmacSha1::new_from_slice(key).unwrap();
    hmac_calc.update(&data[0x1E8..0x1E8 + len]);
    hmac_calc.finalize().into_bytes().to_vec()
}

pub fn compute_bdx_hashes(data: &[u8]) -> (u16, u16) {
    let mut check0: u16 = 0x1EA;
    let mut check1: u16 = 0;

    for i in 12..data.len() {
        if i == 0x1E8 {
            check1 = check0;
        }
        check0 = check0.rotate_left(1);
        check0 ^= data[i] as u16;
    }

    (check0, check1)
}