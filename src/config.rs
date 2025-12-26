//! shared config

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

pub mod vm {
    /// @ref M size, bytes
    pub const Msz: usize = 0x10000;
    /// @ref R size, addresses
    pub const Rsz: usize = 0x100;
    /// @ef D size, cells
    pub const Dsz: usize = 0x10;
}
