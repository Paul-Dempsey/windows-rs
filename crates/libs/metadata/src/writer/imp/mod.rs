mod strings;
mod blobs;
mod tables;
mod codes;
mod file;

use crate::imp::*;
use super::*;
use strings::*;
use blobs::*;
use std::collections::*;
use crate::reader;
use std::slice::*;
use std::mem::*;
use tables::*;
use codes::*;

pub fn write<P: AsRef<std::path::Path>>(path: P, references: &[P], items: &[Item]) {
    let references: Vec<reader::File> = references.iter().map(|path|reader::File::new(path).expect("Invalid winmd file")).collect();
    let references = reader::Reader::new(&references);
    let mut tables =  Tables::new(&references);

    tables.Module.push(Module { Name: tables.strings.insert(path.as_ref().file_name().expect("Missing file name").to_str().expect("Invalid file name")), Mvid: 1, ..Default::default() });
    tables.TypeDef.push(TypeDef { TypeName: tables.strings.insert("<Module>"), ..Default::default() });
    let mscorlib = tables.AssemblyRef.push2(AssemblyRef { MajorVersion: 4, Name: tables.strings.insert("mscorlib"), ..Default::default() });
    let value_type = tables.TypeRef.push2(TypeRef { TypeName: tables.strings.insert("ValueType"), TypeNamespace: tables.strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
    let enum_type = tables.TypeRef.push2(TypeRef { TypeName: tables.strings.insert("Enum"), TypeNamespace: tables.strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

    
    for (index, item) in items.iter().enumerate() {
        let index = index + 1;
        match item {
            Item::Struct(s) => {
                tables.type_def_index.insert((&s.name.0, &s.name.1), (index as u32, true));
            }
            Item::Enum(e) => {
                tables.type_def_index.insert((&e.name.0, &e.name.1), (index as u32, true));
            }
        }
    }

    for item in items {
        match item {
            Item::Struct(s) => {
                for f in &s.fields {
                    tables.reference(&f.ty, &references);
                }
            }
            Item::Enum(e) => {}
        }
    }

    // TODO: first fill in the TypeRef table by walking the items and resolving type refs

    // then walk the items and fill in the definitions

    for item in items {
        match item {
            Item::Struct(s) => {
                let mut flags = TypeAttributes(0);
                flags.set_public();
                if s.winrt {
                    flags.set_winrt();
                }
                tables.TypeDef.push(TypeDef {
                    Flags: flags.0,
                    TypeNamespace: tables.strings.insert(&s.name.0),
                    TypeName: tables.strings.insert(&s.name.1),
                    Extends: TypeDefOrRef::TypeRef(value_type).encode(),
                    FieldList: tables.Field.len() as _,
                    MethodList: 0,
                });
                for f in &s.fields {
                    let mut flags = FieldAttributes(0);
                    flags.set_public();
                    let signature = tables.field_sig(&f.ty);
                    tables.Field.push(tables::Field { Flags: flags.0, Name: tables.strings.insert(&f.name), Signature: tables.blobs.insert(&signature) })
                }
            }
            Item::Enum(e) => {
                
            }
        }
    }

    file::write(path,  tables.into_streams());
}

pub struct Streams {
    pub tables: Vec<u8>,
    pub strings: Vec<u8>,
    pub blobs: Vec<u8>,
    pub guids: Vec<u8>,
}

impl Streams {
    pub fn len(&self) -> usize {
        self.tables.len() + self.guids.len() + self.strings.len() + self.blobs.len()
    }
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub trait Push2<T> {
    fn push2(&mut self, value: T) -> u32;
}

impl<T> Push2<T> for Vec<T> {
    fn push2(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1) as _
    }
}

pub trait Write {
    unsafe fn write_header<T: Sized>(&mut self, value: &T);
    fn write_u8(&mut self, value: u8);
    fn write_u16(&mut self, value: u16);
    fn write_u32(&mut self, value: u32);
    fn write_u64(&mut self, value: u64);
    fn write_code(&mut self, value: u32, size: usize);
    fn write_index(&mut self, index: u32, len: usize);
}

impl Write for Vec<u8> {
    unsafe fn write_header<T: Sized>(&mut self, value: &T) {
        self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
    }

    fn write_u8(&mut self, value: u8) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u16(&mut self, value: u16) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u32(&mut self, value: u32) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64(&mut self, value: u64) {
        self.extend_from_slice(&value.to_le_bytes());
    }

    fn write_code(&mut self, value: u32, size: usize) {
        if size == 2 {
            self.write_u16(value as _);
        } else {
            self.write_u32(value);
        }
    }

    fn write_index(&mut self, index: u32, len: usize) {
        if len < (1 << 16) {
            self.write_u16(index as u16 + 1);
        } else {
            self.write_u32(index + 1);
        }
    }
}
