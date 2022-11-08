use super::*;

#[derive(Default)]
pub struct Definitions<'a> {
    // Table must be sorted to ensure reproducible builds.
    map: BTreeMap<(&'a str, &'a str), Definition<'a>>,
    stream: Vec<u8>,
}

pub struct Definition<'a> {
    pub item: &'a Item,
    pub index: u32,
}

impl<'a> Definitions<'a> {
    pub fn insert(&mut self, item: &'a Item) {
        if self.map.insert(item_type_name(item), Definition { item, index: 0 }).is_some() {
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

    pub fn get(&'a self, namespace: &'a str, name: &'a str) -> Option<&'a Definition> {
        self.map.get(&(namespace, name))
    }
}
