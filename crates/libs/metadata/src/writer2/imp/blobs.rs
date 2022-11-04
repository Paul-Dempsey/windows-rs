use super::*;

pub struct Blobs {
    // Blobs don't need to be sorted. A map is used to collapse duplicate blobs.
    map: HashMap<Vec<u8>, u32>,
    stream: Vec<u8>,
}

impl Default for Blobs {
    fn default() -> Self {
        // The stream's first entry is always an "empty" blob consisting of a single zero byte.
        Self { map: HashMap::new(), stream: vec![0] }
    }
}

impl Blobs {
    pub fn insert(&mut self, value: Vec<u8>) -> u32 {
        if value.is_empty() {
            return 0;
        }

        match self.map.entry(value) {
            hash_map::Entry::Occupied(entry) => *entry.get(),
            hash_map::Entry::Vacant(entry) => {
                let pos = self.stream.len() as _;
                let value = entry.key();

                match value.len() {
                    0..=0x7F => self.stream.push(value.len() as _),
                    0x80..=0x3FFF => {
                        self.stream.push((0x80 | value.len() >> 8) as _);
                        self.stream.push((0xFF & value.len()) as _);
                    }
                    _ => {
                        self.stream.push((0xC0 | value.len() >> 24) as _);
                        self.stream.push((0xFF & value.len() >> 16) as _);
                        self.stream.push((0xFF & value.len() >> 8) as _);
                        self.stream.push((0xFF & value.len()) as _);
                    }
                }

                self.stream.extend_from_slice(value);
                *entry.insert(pos)
            }
        }
    }

    pub fn into_stream(mut self) -> Vec<u8> {
        self.stream.resize(round(self.stream.len(), 4), 0);
        self.stream
    }
}
