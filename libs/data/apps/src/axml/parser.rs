use anyhow::{bail, Result};

/// AxmlParser: Pure Rust parser for Android Binary XML (AXML / compiled AndroidManifest.xml).
pub struct AxmlParser;

impl AxmlParser {
    pub const RES_XML_TYPE: u16 = 0x0003;
    pub const RES_STRING_POOL_TYPE: u16 = 0x0001;

    /// Extract all UTF-8/UTF-16 strings present in the AXML String Pool table.
    pub fn extract_string_pool(bytes: &[u8]) -> Result<Vec<String>> {
        if bytes.len() < 8 {
            bail!("AXML data too short for XML header");
        }

        let magic = u16::from_le_bytes([bytes[0], bytes[1]]);
        if magic != Self::RES_XML_TYPE {
            bail!("Invalid AXML magic bytes: 0x{:04x}", magic);
        }

        let mut offset = 8;
        while offset + 8 <= bytes.len() {
            let chunk_type = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
            let chunk_size = u32::from_le_bytes([
                bytes[offset + 4],
                bytes[offset + 5],
                bytes[offset + 6],
                bytes[offset + 7],
            ]) as usize;

            if chunk_type == Self::RES_STRING_POOL_TYPE {
                return Self::parse_string_pool(&bytes[offset..offset + chunk_size]);
            }

            if chunk_size == 0 {
                break;
            }
            offset += chunk_size;
        }

        bail!("StringPool chunk not found in AXML")
    }

    fn parse_string_pool(chunk: &[u8]) -> Result<Vec<String>> {
        if chunk.len() < 28 {
            bail!("StringPool chunk header too short");
        }

        let string_count = u32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]) as usize;
        let flags = u32::from_le_bytes([chunk[16], chunk[17], chunk[18], chunk[19]]);
        let is_utf8 = (flags & (1 << 8)) != 0;
        let strings_start = u32::from_le_bytes([chunk[20], chunk[21], chunk[22], chunk[23]]) as usize;

        let mut string_offsets = Vec::with_capacity(string_count);
        for i in 0..string_count {
            let idx = 28 + (i * 4);
            if idx + 4 <= chunk.len() {
                let off = u32::from_le_bytes([chunk[idx], chunk[idx + 1], chunk[idx + 2], chunk[idx + 3]]) as usize;
                string_offsets.push(off);
            }
        }

        let mut strings = Vec::with_capacity(string_count);
        for off in string_offsets {
            let abs_off = strings_start + off;
            if abs_off < chunk.len() {
                let s = if is_utf8 {
                    Self::read_utf8_string(&chunk[abs_off..])
                } else {
                    Self::read_utf16_string(&chunk[abs_off..])
                };
                strings.push(s);
            } else {
                strings.push(String::new());
            }
        }

        Ok(strings)
    }

    fn read_utf8_string(slice: &[u8]) -> String {
        if slice.len() < 2 {
            return String::new();
        }
        // UTF-8 AXML string starts with char length and byte length
        let len = slice[1] as usize;
        if 2 + len <= slice.len() {
            String::from_utf8_lossy(&slice[2..2 + len]).to_string()
        } else {
            String::new()
        }
    }

    fn read_utf16_string(slice: &[u8]) -> String {
        if slice.len() < 2 {
            return String::new();
        }
        let len_chars = u16::from_le_bytes([slice[0], slice[1]]) as usize;
        let mut u16_vec = Vec::with_capacity(len_chars);
        for i in 0..len_chars {
            let idx = 2 + (i * 2);
            if idx + 2 <= slice.len() {
                u16_vec.push(u16::from_le_bytes([slice[idx], slice[idx + 1]]));
            }
        }
        String::from_utf16_lossy(&u16_vec)
    }
}
