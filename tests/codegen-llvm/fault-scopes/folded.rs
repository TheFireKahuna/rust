// A raw access the language can vouch for — through a pointer to a local —
// loses its edge under optimisation and is promoted like any other local.
//@ add-minicore
//@ compile-flags: --target=x86_64-pc-windows-ntposix -Copt-level=3 -Cpanic=unwind
//@ compile-flags: --crate-type=rlib
//@ needs-llvm-components: x86

#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: @local_round_trip
#[no_mangle]
pub fn local_round_trip(v: u32) -> u32 {
    // CHECK-NOT: llvm.fault
    // CHECK-NOT: invoke
    // CHECK: ret i32 %v
    let mut slot = 0u32;
    let p = &raw mut slot;
    unsafe {
        *p = v;
        *p
    }
}

// CHECK-LABEL: @raw_stays
#[no_mangle]
pub fn raw_stays(p: *const u32) -> u32 {
    // CHECK: call i32 @llvm.fault.load.i32.p0(ptr {{.*}}, i32 4){{$}}
    unsafe { *p }
}
