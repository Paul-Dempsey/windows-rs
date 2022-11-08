use super::*;

#[derive(Default)]
pub struct References<'a> {
    // Table doesn't need to be sorted. A map is used to collapse duplicate records.
    map: BTreeMap<(&'a str, &'a str), Reference<'a>>,
    stream: Vec<u8>,
}

pub struct Reference<'a> {
    pub assembly: &'a str,
    pub value_type: bool,
    pub index: u32,
}

impl<'a> References<'a> {
    pub fn insert(&mut self, namespace: &'a str, name: &'a str) {
        self.map.insert((namespace, name), Reference { assembly: "", value_type: false, index: 0 });
    }

    pub fn index(&mut self) {
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    pub fn get(&'a self, namespace: &'a str, name: &'a str) -> Option<&'a Reference> {
        self.map.get(&(namespace, name))
    }
}
