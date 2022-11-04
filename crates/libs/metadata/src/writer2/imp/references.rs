use super::*;

#[derive(Default)]
pub struct References<'a> {
    // Table doesn't need to be sorted. A map is used to collapse duplicate records.
    map: BTreeMap<(&'a str, &'a str), Record<'a>>,
    stream: Vec<u8>,
}

#[derive(Default)]
struct Record<'a> {
    scope: ResolutionScope<'a>,
    index: u32,
    columns: Columns,
}

#[derive(Default)]
struct Columns {
    scope: u32,
    type_name: u32,
    type_namespace: u32,
}

impl<'a> References<'a> {
    // Inserts a new type into the table. Duplicates are ignored.
    pub fn insert(&mut self, name: (&'a str, &'a str), scope: ResolutionScope<'a>) {
        self.map.insert(name, Record { scope, index: 0, columns: Columns::default() });
    }

    pub fn index(&mut self) {
        debug_assert!(self.stream.is_empty(), "call `index` before calling `stage`");
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    // Gives each TypeRef and index and writes the table to the stream.
    pub fn stage(&mut self) {
        debug_assert!(self.stream.is_empty(), "only call `stage` once");
        todo!("write table to stream")
    }

    // Retrieves the type's index as prepared by the `stage` function.
    pub fn get(&self, name: (&'a str, &'a str)) -> Option<u32> {
        debug_assert!(!self.stream.is_empty(), "call `stage` before calling `get`");
        self.map.get(&name).map(|record| record.index)
    }

    // Retrieves the stream prepared by the `stage` function.
    pub fn into_stream(self) -> Vec<u8> {
        debug_assert!(!self.stream.is_empty(), "call `stage` before calling `into_stream`");
        self.stream
    }
}
