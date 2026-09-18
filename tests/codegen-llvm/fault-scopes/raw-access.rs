// On a target whose raw-pointer accesses are unwind edges, each is an invoke
// of the matching `llvm.fault.*` intrinsic whose unwind destination is the
// frame's cleanup; the same body on a funclet target is a plain access.
//@ add-minicore
//@ revisions: ntposix msvc
//@ [ntposix] compile-flags: --target=x86_64-pc-windows-ntposix -Copt-level=0 -Cpanic=unwind
//@ [ntposix] needs-llvm-components: x86
//@ [msvc] compile-flags: --target=x86_64-pc-windows-msvc -Zprecise-fault-scopes=yes -Copt-level=0 -Cpanic=unwind
//@ [msvc] needs-llvm-components: x86
//@ compile-flags: --crate-type=rlib

#![feature(no_core, lang_items, intrinsics, rustc_attrs)]
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
    // ntposix: invoke i32 @llvm.fault.load.i32.p0(ptr {{.*}}, i32 4)
    // ntposix-NEXT: to label %{{.*}} unwind label %{{.*}}
    // msvc-NOT: llvm.fault
    // msvc: load i32, ptr
    unsafe { *p }
}

// CHECK-LABEL: @write
#[no_mangle]
pub fn write(p: *mut u32, v: u32, _g: Guard) {
    // ntposix: invoke void @llvm.fault.store.i32.p0(i32 {{.*}}, ptr {{.*}}, i32 4)
    // ntposix-NEXT: to label %{{.*}} unwind label %{{.*}}
    // msvc-NOT: llvm.fault
    // msvc: store i32
    unsafe { *p = v }
}

// CHECK-LABEL: @read_pair
#[no_mangle]
pub fn read_pair(p: *const (u32, u64), _g: Guard) -> (u32, u64) {
    // ntposix: invoke i32 @llvm.fault.load.i32.p0(ptr {{.*}}, i32 8)
    // ntposix: invoke i64 @llvm.fault.load.i64.p0(ptr {{.*}}, i32 8)
    // msvc-NOT: llvm.fault
    unsafe { *p }
}

// CHECK-LABEL: @copy_out
#[no_mangle]
pub fn copy_out(src: *const [u32; 8], dst: *mut [u32; 8], _g: Guard) {
    // ntposix: invoke void @llvm.fault.memcpy.p0.p0.i64(ptr align 4 {{.*}}, ptr align 4 {{.*}}, i64 32, i1 false)
    // msvc-NOT: llvm.fault
    unsafe { copy_nonoverlapping(src, dst, 1) }
}

#[rustc_intrinsic]
#[rustc_nounwind]
pub unsafe fn volatile_store<T>(dst: *mut T, val: T);

// CHECK-LABEL: @volatile_write
#[no_mangle]
pub fn volatile_write(p: *mut u32, v: u32, _g: Guard) {
    // ntposix: invoke void @llvm.fault.store.volatile.i32.p0(i32 {{.*}}, ptr {{.*}}, i32 4)
    // msvc-NOT: llvm.fault
    // msvc: store volatile i32
    unsafe { volatile_store(p, v) }
}

// CHECK-LABEL: @through_reference
#[no_mangle]
pub fn through_reference(r: &u32, _g: Guard) -> u32 {
    // CHECK-NOT: llvm.fault
    // CHECK: load i32, ptr
    *r
}

// A raw access with nothing to clean up is still an edge: it unwinds to a pad
// that resumes, so the frame is covered rather than a gap.
// CHECK-LABEL: @no_cleanup
#[no_mangle]
pub fn no_cleanup(p: *const u32) -> u32 {
    // ntposix: invoke i32 @llvm.fault.load.i32.p0(ptr {{.*}}, i32 4)
    // ntposix-NEXT: to label %{{.*}} unwind label %[[PAD:.*]]
    // ntposix: [[PAD]]:
    // ntposix-NEXT: landingpad
    // ntposix-NEXT: cleanup
    // ntposix: resume
    // msvc-NOT: llvm.fault
    unsafe { *p }
}
