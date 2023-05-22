#![allow(unknown_lints)]
#![allow(clippy)]
#[rustfmt::skip]
pub mod api;


pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("descriptor.bin");
