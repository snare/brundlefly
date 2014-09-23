//
// brundlefly
//
// An unholy fusion of UEFI and Rust.
//

#![no_std]
#![no_main]
#![feature(globs)]

extern crate core;
use core::iter::Iterator;
use core::str::StrSlice;
use core::fmt::String;
use core::intrinsics::transmute;

use efi::*;

pub mod efi;


//
// Entry point from EFI module
//

#[no_split_stack]
pub fn EfiMain(imageHandle: EfiHandle, systemTable: *mut EfiSystemTable) -> EfiStatus {
    let loader = Loader { imageHandle: imageHandle, systemTable: systemTable };
    loader.main()
}

//
// Loader class
//

struct Loader {
    pub imageHandle:    EfiHandle,
    pub systemTable:    *mut EfiSystemTable
}

impl Loader {
    fn main(&self) -> EfiStatus {
        let status: EfiStatus = EFI_SUCCESS;

        efi::print("rustybutts\n");

        status
    }
}
