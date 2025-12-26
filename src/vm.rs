#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::config::vm::*;

/// main memory
pub static mut M: [u8; Msz] = [0; Msz];
pub static mut Cp: u16 = 0;
pub static mut Ip: u16 = 0;

/// return stack
pub static mut R: [u16; Rsz] = [0; Rsz];
pub static mut Rp: u8 = 0;

/// data stack
pub static mut D: [i32; Dsz] = [0; Dsz];
pub static mut Dp: u8 = 0;
