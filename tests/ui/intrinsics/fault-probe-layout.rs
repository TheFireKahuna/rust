// A probe needs a type with a scalar layout.
//@ add-minicore
//@ build-fail
//@ compile-flags: --target=x86_64-pc-windows-ntposix -Cpanic=unwind --crate-type=rlib
//@ needs-llvm-components: x86

#![feature(no_core, lang_items, intrinsics, rustc_attrs)]
#![no_core]

extern crate minicore;
use minicore::*;

#[rustc_intrinsic]
#[rustc_nounwind]
unsafe fn fault_probe_read<T>(src: *const T, dst: *mut T) -> bool;

pub unsafe fn wide(src: *const [u8; 32], dst: *mut [u8; 32]) -> bool {
    fault_probe_read(src, dst)
    //~^ ERROR `fault_probe_read` and `fault_probe_write` need a type with a scalar layout
}
