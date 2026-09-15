// Checks that `#[returns_twice]` adds LLVM's `returns_twice` attribute to the
// function's declaration, whether the function is defined here or foreign.
//
//@ compile-flags: -C no-prepopulate-passes
#![crate_type = "lib"]
#![feature(returns_twice)]

pub fn bar(buf: *mut u8) -> i32 {
    unsafe { setjmp(buf) + twice() }
}

// CHECK: Function Attrs: {{.*}}returns_twice
// CHECK-NEXT: define{{.*}}i32 @twice()
#[returns_twice]
#[unsafe(no_mangle)]
pub fn twice() -> i32 {
    0
}

extern "C" {
    // CHECK: Function Attrs: {{.*}}returns_twice
    // CHECK-NEXT: declare{{.*}}i32 @setjmp(ptr
    #[returns_twice]
    pub fn setjmp(buf: *mut u8) -> i32;
}
