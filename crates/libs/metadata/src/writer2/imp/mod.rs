mod blobs;
mod definitions;
mod references;
mod strings;
mod tables;
mod codes;
mod file;

use super::*;
use crate::imp::*;
use crate::bindings::*;
use blobs::*;
use std::collections::*;
use strings::*;
use tables::*;
use codes::*;

pub use definitions::*;
pub use references::*;

pub fn write(module: &str, items: &[Item], references: &[&str]) -> Vec<u8> {
    //
    // 1. First collect definitions and references...
    //
    let mut definitions = Definitions::default();
    let mut references = References::default();

    for item in items {
        definitions.insert(item);

        match item {
            Item::Struct(ty) => ty.fields.iter().for_each(|field| field.ty.reference(&definitions, &mut references)),
            _ => {}
        }
    }

    for item in items {
    }

    //
    // 2. Next, index definitions and references...
    //
    definitions.index();
    references.index();

    //
    // 3. Finally, prepare streams and tables...
    //
    let mut strings = Strings::default();
    let mut blobs = Blobs::default();
    let mut tables = Tables::default();

    tables.Module.push(Module { Name: strings.insert(module), Mvid: 1, ..Default::default() });
    tables.TypeDef.push(TypeDef { TypeName: strings.insert("<Module>"), ..Default::default() });
    let mscorlib = tables.AssemblyRef.push2(AssemblyRef { MajorVersion: 4, Name: strings.insert("mscorlib"), ..Default::default() });
    let value_type = tables.TypeRef.push2(TypeRef { TypeName: strings.insert("ValueType"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
    let enum_type = tables.TypeRef.push2(TypeRef { TypeName: strings.insert("Enum"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

    file::write(file::Streams {
        tables: tables.into_stream(),
        strings: strings.into_stream(),
        blobs: blobs.into_stream(),
        guids: vec![0; 16], // zero guid
    })
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
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
        self.extend_from_slice(std::slice::from_raw_parts(value as *const _ as _, std::mem::size_of::<T>()));
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

trait Push2<T> {
    fn push2(&mut self, value: T) -> u32;
}

impl<T> Push2<T> for Vec<T> {
    fn push2(&mut self, value: T) -> u32 {
        self.push(value);
        (self.len() - 1) as _
    }
}
