#![allow(dead_code)]

mod imp;
pub mod reader;
pub mod writer;

use imp::*;
use std::collections::*;
use std::io::*;
use std::mem::*;
use std::ptr::*;
