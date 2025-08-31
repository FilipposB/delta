use uuid::Uuid;
use crate::controller::request::RequestData;

pub struct Manifest {
    pub(crate) id: Uuid,
    name: String,
    chunk_index: u64,
    total_chunks: u64,
    missed_chunks: Vec<u64>,
}

impl Manifest {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        // UUID (16 bytes)
        let uuid_arr: [u8; 16] = bytes[0..16].try_into().unwrap();
        let id = Uuid::from_bytes(uuid_arr);

        // Chunk index (8 bytes)
        let chunk_index = u64::from_be_bytes(bytes[16..24].try_into().unwrap());

        // Missed chunks length (8 bytes)
        let missed_len = u64::from_be_bytes(bytes[24..32].try_into().unwrap()) as usize;

        // Missed chunks (8 bytes each)
        let mut missed_chunks = Vec::with_capacity(missed_len);
        for i in 0..missed_len {
            let start = 32 + i * 8;
            let end = start + 8;
            let chunk = u64::from_be_bytes(bytes[start..end].try_into().unwrap());
            missed_chunks.push(chunk);
        }

        Self {
            id,
            name: "".to_string(),
            chunk_index,
            missed_chunks,
            total_chunks: 0,
        }
    }
}