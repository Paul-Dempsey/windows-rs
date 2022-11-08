mod blobs;
mod codes;
mod definitions;
mod file;
mod references;
mod strings;
mod tables;

use super::*;
use crate::bindings::*;
use crate::imp::*;
use crate::*;
use blobs::*;
use codes::*;
use std::collections::*;
use strings::*;

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
            Item::Struct(ty) => ty.fields.iter().for_each(|field| type_reference(&field.ty, &definitions, &mut references)),
            _ => todo!(),
        }
    }

    for item in items {}

    //
    // 2. Next, index definitions and references...
    //
    definitions.index();
    references.index();

    //
    // 3. Finally, prepare streams and tables...
    //
    let mut strings = Strings::default();
    let mut blobs = blobs::Blobs::default();
    let mut tables = tables::Tables::default();

    tables.Module.push(tables::Module { Name: strings.insert(module), Mvid: 1, ..Default::default() });
    tables.TypeDef.push(tables::TypeDef { TypeName: strings.insert("<Module>"), ..Default::default() });
    let mscorlib = tables.AssemblyRef.push2(tables::AssemblyRef { MajorVersion: 4, Name: strings.insert("mscorlib"), ..Default::default() });
    let value_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.insert("ValueType"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });
    let enum_type = tables.TypeRef.push2(tables::TypeRef { TypeName: strings.insert("Enum"), TypeNamespace: strings.insert("System"), ResolutionScope: ResolutionScope::AssemblyRef(mscorlib).encode() });

    for item in items {
        match item {
            Item::Struct(ty) => {
                let mut flags = TypeAttributes(0);
                flags.set_public();
                if ty.winrt {
                    flags.set_winrt();
                }
                tables.TypeDef.push(tables::TypeDef {
                    Flags: flags.0,
                    TypeName: strings.insert(&ty.name),
                    TypeNamespace: strings.insert(&ty.namespace),
                    Extends: TypeDefOrRef::TypeRef(value_type).encode(),
                    FieldList: tables.Field.len() as _,
                    MethodList: 0,
                });
                for field in &ty.fields {
                    let mut flags = FieldAttributes(0);
                    flags.set_public();
                    tables.Field.push(tables::Field { Flags: flags.0, Name: strings.insert(&field.name), Signature: blobs.insert(field_signature(&field.ty, &definitions, &references)) })
                }
            }
            _ => todo!(),
        }
    }

    file::write(file::Streams {
        tables: tables.into_stream(),
        strings: strings.into_stream(),
        blobs: blobs.into_stream(),
        guids: vec![0; 16], // zero guid
    })
}

pub fn item_type_name(item: &Item) -> (&str, &str) {
    match item {
        Item::Struct(ty) => (ty.namespace.as_str(), ty.name.as_str()),
        Item::Enum(ty) => (ty.namespace.as_str(), ty.name.as_str()),
    }
}

pub fn item_is_value_type(item: &Item) -> bool {
    match item {
        Item::Struct(_) => true,
        Item::Enum(_) => true,
    }
}

fn field_signature(ty: &Type, definitions: &Definitions, references: &References) -> Vec<u8> {
    let mut buffer = vec![0x6];
    buffer.append(&mut type_signature(ty, definitions, references));
    buffer
}

fn type_signature(ty: &Type, definitions: &Definitions, references: &References) -> Vec<u8> {
    match ty {
        Type::Void => todo!(),
        Type::Bool => vec![0x02],
        Type::Char => vec![0x03],
        Type::I8 => vec![0x04],
        Type::U8 => vec![0x05],
        Type::I16 => vec![0x06],
        Type::U16 => vec![0x07],
        Type::I32 => vec![0x08],
        Type::U32 => vec![0x09],
        Type::I64 => vec![0x0a],
        Type::U64 => vec![0x0b],
        Type::F32 => vec![0x0c],
        Type::F64 => vec![0x0d],
        Type::ISize => vec![0x18],
        Type::USize => vec![0x19],
        Type::String => vec![0x0e],
        Type::Named((namespace, name)) => type_code(namespace, name, definitions, references),
    }
}

fn type_code(namespace: &str, name: &str, definitions: &Definitions, references: &References) -> Vec<u8> {
    if let Some(definition) = definitions.get(namespace, name) {
        let code = TypeDefOrRef::TypeDef(definition.index);
        let mut buffer = if item_is_value_type(definition.item) { vec![0x11] } else { vec![0x12] };
        blobs::write_usize(&mut buffer, TypeDefOrRef::TypeDef(definition.index).encode());
        buffer
    } else if let Some(reference) = definitions.get(namespace, name) {
        //if
        todo!("typeref")
    } else {
        panic!("Type not found `{}.{}`", namespace, name);
    }
}

fn type_reference<'a>(ty: &'a Type, definitions: &Definitions, references: &mut References<'a>) {
    match ty {
        Type::Named((namespace, name)) => {
            if !definitions.contains(namespace, name) {
                references.insert(namespace, name);
            }
        }
        _ => {}
    }
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
