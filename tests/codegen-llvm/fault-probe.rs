// A probing access is a callbr of the probe intrinsic: the value continues at
// the default destination, a fault continues at the indirect one, and the two
// join on the outcome. Its fault destination is a landing pad of the access,
// so the function carries the personality. On a funclet target the access is a
// plain volatile one and the probe reports success.
//@ add-minicore
//@ revisions: ntposix msvc
//@ [ntposix] compile-flags: --target=x86_64-pc-windows-ntposix -Copt-level=0 -Cpanic=unwind
//@ [ntposix] needs-llvm-components: x86
//@ [msvc] compile-flags: --target=x86_64-pc-windows-msvc -Copt-level=0 -Cpanic=unwind
//@ [msvc] needs-llvm-components: x86
//@ compile-flags: --crate-type=rlib

#![feature(no_core, lang_items, intrinsics, rustc_attrs)]
#![no_core]

extern crate minicore;
use minicore::*;

#[rustc_intrinsic]
#[rustc_nounwind]
unsafe fn fault_probe_read<T>(src: *const T, dst: *mut T) -> bool;

#[rustc_intrinsic]
#[rustc_nounwind]
unsafe fn fault_probe_write<T>(dst: *mut T, src: *const T) -> bool;

// CHECK-LABEL: @read_u64
// ntposix-SAME: personality
#[no_mangle]
pub unsafe fn read_u64(src: *const u64, dst: *mut u64) -> bool {
    // ntposix: callbr i64 @llvm.fault.probe.load.i64.p0(ptr {{.*}}, i32 1)
    // ntposix-NEXT: to label %{{.*}} [label %{{.*}}]
    // msvc-NOT: callbr
    // msvc: load volatile i64, ptr {{.*}}, align 1
    fault_probe_read(src, dst)
}

// CHECK-LABEL: @write_u32
#[no_mangle]
pub unsafe fn write_u32(dst: *mut u32, src: *const u32) -> bool {
    // ntposix: callbr void @llvm.fault.probe.store.i32.p0(i32 {{.*}}, ptr {{.*}}, i32 1)
    // msvc-NOT: callbr
    // msvc: store volatile i32 {{.*}}, align 1
    fault_probe_write(dst, src)
}

// A zero-sized probe touches nothing and cannot fault.
// CHECK-LABEL: @read_unit
// CHECK-NOT: callbr
// CHECK: ret i1 true
#[no_mangle]
pub unsafe fn read_unit(src: *const (), dst: *mut ()) -> bool {
    fault_probe_read(src, dst)
}
