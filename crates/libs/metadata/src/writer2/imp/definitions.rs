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
}

impl<'a> Definitions<'a> {
    pub fn insert(&mut self, item: &'a Item) {
        if self.map.insert(item.type_name(), Record { item, index: 0 }).is_some() {
            panic!("Duplicate type found");
        }
    }

    pub fn index(&mut self) {
        debug_assert!(self.stream.is_empty(), "call `index` before calling `stage`");
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.map.contains_key(&(namespace, name))
    }

    pub fn get(&self, namespace: &str, name: &str) -> Option<(&Item, u32)> {
        self.map.get(&(namespace, name)).map(|record| (record.item, record.index))
    }
}
