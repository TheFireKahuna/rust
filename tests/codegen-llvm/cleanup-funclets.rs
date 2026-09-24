// On a target whose cleanups are funclets, drop glue is a `cleanuppad` the
// unwinder calls and a cross-funclet branch a `cleanupret` to the next. Each
// pad's one `i8` is its sites' phase-one clause: 0 where a search passes, 1 for
// a cleanup in a body that cannot unwind, 2 for that body's abort funclet. The
// same bodies on a landing-pad target are landing pads throughout.
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
    // ntposix-NEXT: %[[FUNCLET]] = cleanuppad within none [i8 0]
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
    // ntposix-NEXT: %[[OUTER]] = cleanuppad within none [i8 0]
    // ntposix: call void @{{.*}}drop_glue{{.*}}({{.*}}){{.*}} [ "funclet"(token %[[INNER:.*]]) ]
    // ntposix-NEXT: cleanupret from %[[INNER]] unwind label %[[OUTERBB]]
    // ntposix: %[[INNER]] = cleanuppad within none [i8 0]
    // itanium-NOT: cleanuppad
    let _outer = Guard;
    unsafe { may_unwind() }
    let _inner = Guard;
    unsafe { may_unwind() }
}

// CHECK-LABEL: @cannot_unwind
#[no_mangle]
pub extern "C" fn cannot_unwind() {
    // ntposix: %[[ABORT:.*]] = cleanuppad within none [i8 2]
    // ntposix: call void @{{.*}}panic_cannot_unwind{{.*}}(){{.*}} [ "funclet"(token %[[ABORT]]) ]
    // ntposix-NOT: landingpad
    // itanium: landingpad { ptr, i32 }
    // itanium-NEXT: filter [0 x ptr] zeroinitializer
    unsafe { may_unwind() }
}

// A body that cannot unwind gives its own cleanups clause 1, whatever they do:
// the clause is the site's, not the cleanup's chain's.
// CHECK-LABEL: @cannot_unwind_with_cleanup
#[no_mangle]
pub extern "C" fn cannot_unwind_with_cleanup() {
    // ntposix: invoke void @may_unwind()
    // ntposix-NEXT: to label %{{.*}} unwind label %[[PAD:.*]]
    // ntposix: [[PAD]]:
    // ntposix-NEXT: cleanuppad within none [i8 1]
    // ntposix-NOT: cleanuppad within none [i8 0]
    // ntposix-NOT: unwind to caller
    // itanium: landingpad { ptr, i32 }
    let _guard = Guard;
    unsafe { may_unwind() }
}

// A body that cannot unwind keeps a table with nothing in it to unwind: the
// personality is its, so a search reaching its frame meets a gap there.
// CHECK-LABEL: @sealed
// ntposix-SAME: personality ptr @rust_eh_personality
// itanium-NOT: personality ptr
#[no_mangle]
pub extern "C" fn sealed() {}
