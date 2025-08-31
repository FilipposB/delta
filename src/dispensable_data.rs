use std::collections::HashMap;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct DispensableData{
    path: String,
    chunk_size: usize,
    in_memory_chunks: HashMap<u64, Vec<u8>>,
    total_chunks: u64,
    max_chunks_in_memory: u64,
    buffer: Vec<u8>,
}

impl DispensableData{
    pub fn new(path: &str, chunk_size: usize, max_chunks_in_memory: u64) -> io::Result<DispensableData> {

        let metadata = fs::metadata(path)?;
        let total_chunks = metadata.len() / chunk_size as u64;

        Ok(
        DispensableData{
            path: path.parse().unwrap(),
            in_memory_chunks: HashMap::new(),
            chunk_size,
            total_chunks,
            max_chunks_in_memory,
            buffer: vec![0; chunk_size],
        })
    }
    
    fn clear_from_memory(&mut self, chunk: u64) -> u64 {
        self.in_memory_chunks.remove(&chunk);
        self.in_memory_chunks.len() as u64
    }
    
    fn add_or_replace(&mut self, chunk: u64, chunk_data: &Vec<u8>, to_be_replaced: u64) {
        if self.max_chunks_in_memory >= self.in_memory_chunks.len() as u64 {
            if self.max_chunks_in_memory <= self.clear_from_memory(to_be_replaced) {
                return;
            }
        }
        self.in_memory_chunks.insert(chunk, chunk_data.clone());
    }

    pub fn load_chunk(&mut self, chunk: u64, keep_in_memory: bool, drop_from_memory: u64) -> io::Result<Vec<u8>> {
        match self.in_memory_chunks.get(&chunk) {
            Some(chunk) => {Ok(chunk.clone())}
            None => {
                let data = self.read_chunk(chunk);
                
                match data {
                    Ok(data) => {
                        if keep_in_memory {
                            self.add_or_replace(chunk, &data, drop_from_memory)
                        }
                        Ok(data.clone())
                    }
                    Err(e) => Err(e)
                }
                
            }
        }
    }

    fn read_chunk(&mut self, offset: u64) -> io::Result<Vec<u8>> {
        let mut file = File::open(self.path.clone())?;
        file.seek(SeekFrom::Start(offset))?;

        match file.read(&mut self.buffer) {
            Ok(size) => {
                Ok(self.buffer[0..size].to_vec())
            }
            Err(e) => Err(e)
        }
    }
    
    pub fn get_total_chunks(&self) -> u64 {
        self.total_chunks
    }

}