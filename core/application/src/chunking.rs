use fastcdc::v2020::FastCDC;
use crate::hashing::calculate_hash;

pub struct Chunk {
    pub hash: String,
    pub offset: u64,
    pub length: u32,
}

pub struct Chunker;

impl Chunker {
    pub fn chunk_data(data: &[u8]) -> Vec<(Chunk, Vec<u8>)> {
        let avg_size = 1024 * 1024;
        let min_size = avg_size / 4;
        let max_size = avg_size * 4;

        let chunker = FastCDC::new(data, min_size, avg_size, max_size);
        let mut results = Vec::new();

        for entry in chunker {
            let chunk_data = &data[entry.offset..entry.offset + entry.length];
            let hash = calculate_hash(chunk_data);

            results.push((
                Chunk {
                    hash,
                    offset: entry.offset as u64,
                    length: entry.length as u32,
                },
                chunk_data.to_vec()
            ));
        }

        results
    }
}
