mod blobs;
mod codes;
mod file;
mod streams;
mod strings;
mod tables;

use super::*;
use blobs::*;
use codes::*;
use std::collections::*;
use std::mem::*;
use std::slice::*;
use streams::*;
use strings::*;

// TODO: do we need references?
pub fn write<P: AsRef<std::path::Path>>(path: P, winrt: bool, items: &std::collections::BTreeMap<TypeName, Item>) {
    let module = path.as_ref().file_name().expect("Missing file name").to_str().expect("Invalid file name");

    let streams = Streams::new(module, winrt, items);
    file::write(path, winrt, streams);
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

/// Returns an `ELEMENT_TYPE` (see ECMA-335) type constant for the `Type` object, typically
/// used to indicate the type of a constant or primitive type signature.
pub fn type_to_code(ty: &Type) -> Option<u16> {
    match ty {
        Type::Void => Some(0x01),
        Type::Bool => Some(0x02),
        Type::Char => Some(0x03),
        Type::I8 => Some(0x04),
        Type::U8 => Some(0x05),
        Type::I16 => Some(0x06),
        Type::U16 => Some(0x07),
        Type::I32 => Some(0x08),
        Type::U32 => Some(0x09),
        Type::I64 => Some(0x0a),
        Type::U64 => Some(0x0b),
        Type::F32 => Some(0x0c),
        Type::F64 => Some(0x0d),
        Type::ISize => Some(0x18),
        Type::USize => Some(0x19),
        Type::String => Some(0x0e),
        Type::IInspectable => Some(0x1c),
        _ => None,
    }
}
