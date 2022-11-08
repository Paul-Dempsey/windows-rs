use super::*;

#[derive(Default)]
pub struct References<'a> {
    // Table doesn't need to be sorted. A map is used to collapse duplicate records.
    map: BTreeMap<(&'a str, &'a str), Record<'a>>,
    stream: Vec<u8>,
}

struct Record<'a> {
    assembly: &'a str,
    value_type: bool,
    index: u32,
}

impl<'a> References<'a> {
    pub fn insert(&mut self, namespace: &'a str, name: &'a str) {
        self.map.insert((namespace, name), Record { assembly: "", value_type: false, index: 0 });
    }

    pub fn index(&mut self) {
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    pub fn get(&self, namespace: &str, name: &str) -> Option<u32> {
        self.map.get(&(namespace, name)).map(|record| record.index)
    }
}
