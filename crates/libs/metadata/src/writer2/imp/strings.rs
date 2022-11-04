use super::*;

pub struct Strings<'a> {
    // Strings don't need to be sorted. A map is used to collapse duplicate strings.
    map: HashMap<&'a str, usize>,
    stream: Vec<u8>,
}

impl<'a> Default for Strings<'a> {
    fn default() -> Self {
        // The stream's first entry is always an empty string e.g. '\0'.
        Self { map: HashMap::new(), stream: vec![0] }
    }
}

impl<'a> Strings<'a> {
    pub fn insert(&mut self, value: &'a str) -> u32 {
        if value.is_empty() {
            return 0;
        }

        let pos = self.stream.len();
        let mut insert = false;

        let pos = *self.map.entry(value).or_insert_with(|| {
            insert = true;
            pos
        });

        if insert {
            self.stream.extend_from_slice(value.as_bytes());
            self.stream.push(0); // terminator
        }

        pos as _
    }

    pub fn into_stream(mut self) -> Vec<u8> {
        self.stream.resize(round(self.stream.len(), 4), 0);
        self.stream
    }
}
