use super::*;

#[derive(Default)]
pub struct AssemblyRef<'a> {
    // Table must be sorted to ensure reproducible builds.
    map: BTreeMap<&'a str, Record>,
    stream: Vec<u8>,
}

struct Record {
    index: u32,
    columns: Columns,
}

#[derive(Default)]
struct Columns {
    version: (u16, u16, u16, u16),
}

impl<'a> AssemblyRef<'a> {
    // Inserts a new type into the table. Duplicates are ignored.
    pub fn insert(&mut self, name: &'a str, version: (u16, u16, u16, u16)) {
        self.map.insert(name, Record { index: 0, columns: Columns { version } });
    }

    // Gives each TypeRef and index and writes the table to the stream.
    pub fn stage(&mut self) {
        debug_assert!(self.stream.is_empty(), "only call `stage` once");
        todo!("write table to stream")
    }

    // Retrieves the type's index as prepared by the `stage` function.
    pub fn get(&self, name: &'a str) -> Option<u32> {
        debug_assert!(!self.stream.is_empty(), "call `stage` before calling `get`");
        self.map.get(&name).map(|record| record.index)
    }

    // Retrieves the stream prepared by the `stage` function.
    pub fn into_stream(self) -> Vec<u8> {
        debug_assert!(!self.stream.is_empty(), "call `stage` before calling `into_stream`");
        self.stream
    }
}
