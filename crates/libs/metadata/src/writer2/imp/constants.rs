use super::*;

// We need a Constants heap to sort constants by primary key (HasConstant)
#[derive(Default)]
pub struct Constants(BTreeMap<HasConstant, Value>);

impl Constants {
    pub fn insert(&mut self, parent: HasConstant, value: Value) {
        self.0.insert(parent, value);
    }
}