#[rustfmt::skip]
/// Character map for non-ASCII characters. 0x80 is excluded as the symbol prefix.
const HIGH: [char; 127] = [
         '€', '�', '�', '”', '�', '�', '�', '�', '�', '�', '�', 'Œ', '�', '�', '�',
    '�', '�', '�', '“', '�', '•', '�', '�', '〜','�', '�', '�', 'œ', '�', '�', '�',
    '�', '¡', '¢', '£', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�',
    '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', '�', 'ß', '¿',
    'À', 'Á', 'Â', '�', 'Ä', '�', '�', 'Ç', 'È', 'É', 'Ê', 'Ë', 'Ì', 'Í', 'Î', 'Ï',
    '�', 'Ñ', 'Ò', 'Ó', 'Ô', '�', 'Ö', '�', '�', 'Ù', 'Ú', 'Û', 'Ü', '�', '�', '�', 
    'à', 'á', 'â', '�', 'ä', '�', '�', 'ç', 'è', 'é', 'ê', 'ë', 'ì', 'í', 'î', 'ï', 
    '�', 'ñ', 'ò', 'ó', 'ô', '�', 'ö', '�', '�', 'ù', 'ú', 'û', 'ü', '�', '�', '©', 
];

#[rustfmt::skip]
/// Character map for the second byte of 0x80, 0x?? sequences.
const SYMBOLS: [char; 48] = [
    '×',  '÷',  '≠',  '→', '↓', '←', '↑', '※',
    '�',  '♭',  '♪',  '±',  '℃',  '○', '●', '◎',
    '△', '▲', '▽', '▼', '□', '■', '◇', '◆',
    '☆', '★', '°',  '∞', '∴',  '…', '™',  '®',
    '♂',  '♀',  'α',  'β',  'γ',  'π',  'Σ',  '√',
    '�',  '�',  '�',  '�',  '�',  '�',  '�',  '�',
];

pub fn decode(bytes: &[u8]) -> String {
    let mut decoded = String::new();

    let mut iter = bytes.iter();
    while let Some(&byte) = iter.next() {
        match byte {
            0x00 | 0x10 => break,
            0x20..=0x7E => decoded.push(byte as char),
            0x81..=0xFF => {
                decoded.push(HIGH[(byte - 0x81) as usize]);
            }
            0x80 => {
                if let Some(byte) = iter.next().copied() {
                    match byte {
                        0x1e => decoded.push('／'),
                        0x1f => decoded.push('＼'),
                        0x22 => decoded.push('｜'),
                        0x40..=0x6F => decoded.push(SYMBOLS[(byte - 0x40) as usize]),
                        _ => {}
                    }
                } else {
                    decoded.push_str("�");
                }
            }
            _ => {
                decoded.push_str(&format!("[{byte:02X}]"));
            }
        }
    }

    decoded
}
