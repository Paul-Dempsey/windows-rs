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
pub fn write<P: AsRef<std::path::Path>>(path: P, items: &std::collections::BTreeMap<TypeName, Item>) {
    let module = path.as_ref().file_name().expect("Missing file name").to_str().expect("Invalid file name");

    let streams = Streams::new(module, items);
    file::write(path, streams);
}

pub fn round(size: usize, round: usize) -> usize {
    let round = round - 1;
    (size + round) & !round
}

pub trait Write {
    fn write<T: Sized>(&mut self, value: &T);
}

impl Write for Vec<u8> {
    fn write<T: Sized>(&mut self, value: &T) {
        unsafe {
            self.extend_from_slice(from_raw_parts(value as *const _ as _, size_of::<T>()));
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
