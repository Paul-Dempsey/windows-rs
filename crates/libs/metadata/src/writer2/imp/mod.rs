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
use tables::*;

// TODO: do we need references?
pub fn write<P: AsRef<std::path::Path>>(path: P, _references: &crate::reader::Reader, items: &[Item]) {
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
