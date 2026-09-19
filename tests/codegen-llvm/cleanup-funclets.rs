// On a target whose cleanups are funclets, drop glue is a `cleanuppad` the
// unwinder calls, a cross-funclet branch a `cleanupret` to the next, and the
// same body on a landing-pad target is landing pads throughout.
//@ add-minicore
//@ revisions: ntposix itanium
//@ [ntposix] compile-flags: --target=x86_64-pc-windows-ntposix -Copt-level=0 -Cpanic=unwind
//@ [ntposix] needs-llvm-components: x86
//@ [itanium] compile-flags: --target=x86_64-pc-windows-gnu -Copt-level=0 -Cpanic=unwind
//@ [itanium] needs-llvm-components: x86
//@ compile-flags: --crate-type=rlib

#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

pub struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {}
}

extern "Rust" {
    fn may_unwind();
}

// CHECK-LABEL: @one_scope
#[no_mangle]
pub fn one_scope() {
    // ntposix: invoke void @may_unwind()
    // ntposix-NEXT: to label %{{.*}} unwind label %[[PAD:.*]]
    // ntposix: call void @{{.*}}drop_glue{{.*}}({{.*}}){{.*}} [ "funclet"(token %[[FUNCLET:.*]]) ]
    // ntposix-NEXT: cleanupret from %[[FUNCLET]] unwind to caller
    // ntposix: [[PAD]]:
    // ntposix-NEXT: %[[FUNCLET]] = cleanuppad within none []
    // ntposix-NOT: landingpad
    // itanium-NOT: cleanuppad
    // itanium: landingpad { ptr, i32 }
    // itanium-NEXT: cleanup
    let _guard = Guard;
    unsafe { may_unwind() }
}

// CHECK-LABEL: @two_scopes
#[no_mangle]
pub fn two_scopes() {
    // The outer scope's funclet, then the inner's, which continues into the
    // outer's.
    // ntposix: call void @{{.*}}drop_glue{{.*}}({{.*}}){{.*}} [ "funclet"(token %[[OUTER:.*]]) ]
    // ntposix-NEXT: cleanupret from %[[OUTER]] unwind to caller
    // ntposix: [[OUTERBB:.*]]:
    // ntposix-NEXT: %[[OUTER]] = cleanuppad within none []
    // ntposix: call void @{{.*}}drop_glue{{.*}}({{.*}}){{.*}} [ "funclet"(token %[[INNER:.*]]) ]
    // ntposix-NEXT: cleanupret from %[[INNER]] unwind label %[[OUTERBB]]
    // ntposix: %[[INNER]] = cleanuppad within none []
    // itanium-NOT: cleanuppad
    let _outer = Guard;
    unsafe { may_unwind() }
    let _inner = Guard;
    unsafe { may_unwind() }
}
