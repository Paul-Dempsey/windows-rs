use super::*;

#[derive(Default)]
pub struct References<'a> {
    // Table doesn't need to be sorted. A map is used to collapse duplicate records.
    map: BTreeMap<(&'a str, &'a str), Record<'a>>,
    stream: Vec<u8>,
}

struct Record<'a> {
    assembly: &'a str,
    index: u32,
}

impl<'a> References<'a> {
    pub fn insert(&mut self, name: (&'a str, &'a str)) {
        self.map.insert(name, Record { assembly: "", index: 0 });
    }

    pub fn index(&mut self) {
        for (index, record) in self.map.values_mut().enumerate() {
            record.index = (index + 1) as _;
        }
    }

    pub fn get(&self, name: (&'a str, &'a str)) -> Option<u32> {
        self.map.get(&name).map(|record| record.index)
    }

}
