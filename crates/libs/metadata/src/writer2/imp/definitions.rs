use super::*;

#[derive(Default)]
pub struct Definitions<'a> {
    // Table must be sorted to ensure reproducible builds.
    map: BTreeMap<(&'a str, &'a str), Record<'a>>,
    stream: Vec<u8>,
}

struct Record<'a> {
    item: &'a Item,
    index: u32,
    columns: Columns,
}

#[derive(Default)]
struct Columns {
    flags: u32,
    type_name: u32,
    type_namespace: u32,
    extends: u32,
    field_list: u32,
    method_list: u32,
}

impl<'a> Definitions<'a> {
    // Inserts a new type into the table. The type name must be unique.
    pub fn insert(&mut self, item: &'a Item) {
        let name = match item {
            Item::Struct(ty) => (ty.namespace.as_str(), ty.name.as_str()),
            Item::Enum(ty) => (ty.namespace.as_str(), ty.name.as_str()),
        };
        if self.map.insert(name, Record { item, index: 0, columns: Columns::default() }).is_some() {
            panic!("Duplicate type found");
        }
    }

    pub fn index(&mut self) {
        debug_assert!(self.stream.is_empty(), "call `index` before calling `stage`");
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    pub fn contains(&self, name: (&'a str, &'a str)) -> bool {
        self.map.contains_key(&name)
    }

    // Gives each TypeDef and index and writes the table to the stream.
    pub fn stage(&mut self) {
        debug_assert!(self.stream.is_empty(), "only call `stage` once");
        todo!("write table to stream")
        // Remember to write "<Module>" entry...
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
