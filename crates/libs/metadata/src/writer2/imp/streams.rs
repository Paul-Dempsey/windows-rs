use super::*;

#[derive(Default)]
pub struct Streams {
    pub tables: Vec<u8>,
    pub guids: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
}

impl Streams {
    pub fn new(module: &str, _items: &[Item]) -> Self {
        let mut tables = Tables::default();
        let mut strings = Strings::new();
        let mut blobs = Blobs::new();

        tables.Module.push(Module {
            Generation: 0,
            Name: strings.insert(module),
            Mvid: 1,
            ..Default::default()
        });

        tables.TypeDef.push(TypeDef {
            TypeName : strings.insert("<Module>"),
            ..Default::default()
        });

        Self {
            tables: tables.into_stream(),
            guids: vec![16; 0], // zero guid
            strings: strings.into_stream(),
            blobs: blobs.into_stream(),
        }
    }

    pub fn len(&self) -> usize {
        self.tables.len() + self.guids.len() + self.strings.len() + self.blobs.len()
    }
}



fn write_index(buffer: &mut Vec<u8>, index: usize, len: usize) {
    if len < (1 << 16) {
        buffer.write(&(index as u16 + 1))
    } else {
        buffer.write(&(index as u32 + 1))
    }
}

fn write_coded_index(buffer: &mut Vec<u8>, value: usize, size: usize) {
    if size == 2 {
        buffer.write(&(value as u16))
    } else {
        buffer.write(&(value as u32))
    }
}
