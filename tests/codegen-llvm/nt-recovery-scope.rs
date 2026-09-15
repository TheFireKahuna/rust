//@ only-windows
//@ only-64bit
//@ ignore-target: arm64ec-pc-windows-msvc
//@ needs-unwind
//@ compile-flags: -Copt-level=0 -Cpanic=unwind
#![crate_type = "lib"]
#![feature(core_intrinsics)]
#![allow(internal_features)]

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        core::hint::black_box(1);
    }
}

// CHECK-LABEL: define{{.*}} @scoped
// CHECK: invoke{{.*}} @__rust_nt_recovery_scope
// CHECK: unwind label
#[unsafe(no_mangle)]
pub unsafe fn scoped(body: unsafe fn(*mut u8), data: *mut u8, buffer: *mut usize) -> bool {
    let _guard = Guard;
    unsafe { core::intrinsics::experimental_nt_recovery_scope(body, data, buffer) }
}

// CHECK-LABEL: define internal{{.*}} @__rust_nt_recovery_scope
// CHECK: call ptr @llvm.frameaddress.p0
// CHECK: callbr void @llvm.experimental.nt.recovery.scope
// CHECK: to label %initial [label %recovery]
// CHECK: initial:
// CHECK: call void %{{.*}}(ptr %{{.*}})
// CHECK: ret i1 false
// CHECK: recovery:
// CHECK: ret i1 true
