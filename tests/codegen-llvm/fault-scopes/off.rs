// `-Zprecise-fault-scopes=no` turns the target's default off: a raw access is
// a plain load again.
//@ add-minicore
//@ compile-flags: --target=x86_64-pc-windows-ntposix -Zprecise-fault-scopes=no -Copt-level=0 -Cpanic=unwind
//@ compile-flags: --crate-type=rlib
//@ needs-llvm-components: x86

#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

pub struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {}
}

// CHECK-LABEL: @read
#[no_mangle]
pub fn read(p: *const u32, _g: Guard) -> u32 {
    // CHECK-NOT: llvm.fault
    // CHECK: load i32, ptr
    unsafe { *p }
}
