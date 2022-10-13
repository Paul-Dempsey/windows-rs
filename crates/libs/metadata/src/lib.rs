#![allow(dead_code)]

mod imp;
pub mod reader;
pub mod writer;

use std::io::*;
use std::mem::*;
use std::ptr::*;
use std::collections::*;
use imp::*;
