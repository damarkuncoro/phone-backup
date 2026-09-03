use anyhow::Result;
use std::collections::HashSet;

/// A lossless preconditioning filter for SQLite databases.
/// Zero-fills unused freelist leaf pages to maximize compression efficiency.
#[derive(Debug, Clone, Default)]
pub struct SqliteZeroFillPreconditioner;

impl SqliteZeroFillPreconditioner {
    pub fn new() -> Self {
        Self
    }

    /// Checks whether the raw bytes start with the official SQLite 3 header magic.
    pub fn is_sqlite(data: &[u8]) -> bool {
        data.len() >= 100 && data.starts_with(b"SQLite format 3\0")
    }

    /// Optimizes SQLite database bytes returning a buffer with zeroed freelist pages.
    pub fn precondition(&self, data: &[u8]) -> Result<Vec<u8>> {
        if !Self::is_sqlite(data) {
            return Ok(data.to_vec());
        }

        let mut output = data.to_vec();
        let raw_page_size = u16::from_be_bytes([output[16], output[17]]) as usize;
        let page_size = if raw_page_size == 1 { 65536 } else { raw_page_size };

        if page_size < 512 || page_size > 65536 || output.len() < page_size {
            return Ok(output);
        }

        let mut first_trunk = u32::from_be_bytes([output[32], output[33], output[34], output[35]]) as usize;
        let total_freelist = u32::from_be_bytes([output[36], output[37], output[38], output[39]]) as usize;

        if first_trunk == 0 || total_freelist == 0 {
            return Ok(output);
        }

        let total_pages = output.len() / page_size;
        let mut visited = HashSet::new();

        while first_trunk > 0 && first_trunk <= total_pages && visited.insert(first_trunk) {
            let trunk_offset = (first_trunk - 1) * page_size;
            if trunk_offset + 8 > output.len() {
                break;
            }

            let next_trunk = u32::from_be_bytes([
                output[trunk_offset],
                output[trunk_offset + 1],
                output[trunk_offset + 2],
                output[trunk_offset + 3],
            ]) as usize;

            let leaf_count = u32::from_be_bytes([
                output[trunk_offset + 4],
                output[trunk_offset + 5],
                output[trunk_offset + 6],
                output[trunk_offset + 7],
            ]) as usize;

            let max_leaves = (page_size - 8) / 4;
            let actual_leaves = leaf_count.min(max_leaves);

            for i in 0..actual_leaves {
                let ptr_offset = trunk_offset + 8 + (i * 4);
                if ptr_offset + 4 > output.len() {
                    break;
                }
                let leaf_page = u32::from_be_bytes([
                    output[ptr_offset],
                    output[ptr_offset + 1],
                    output[ptr_offset + 2],
                    output[ptr_offset + 3],
                ]) as usize;

                if leaf_page > 0 && leaf_page <= total_pages {
                    let leaf_offset = (leaf_page - 1) * page_size;
                    let end_offset = (leaf_offset + page_size).min(output.len());
                    if leaf_offset < end_offset {
                        // Zero-fill the entire unused freelist leaf page
                        output[leaf_offset..end_offset].fill(0);
                    }
                }
            }

            first_trunk = next_trunk;
        }

        Ok(output)
    }
}
