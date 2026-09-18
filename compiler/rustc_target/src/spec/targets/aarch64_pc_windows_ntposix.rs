use crate::spec::{Arch, FramePointer, StackProbeType, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::windows_ntposix::opts();
    base.max_atomic_width = Some(128);
    base.cpu = "generic".into();
    base.features = "+v8.2a,+neon,+crc,+lse,+lse2,+rcpc,+rcpc-immo,+flagm,+dotprod,\
                     +fullfp16,+rdm,+aes,+sha2,+ccpp,+pauth"
        .into();

    // The frame pointer is required on ARM64 Windows: ETW and other services
    // fast-stack-walk through the {x29, x30} pair.
    base.frame_pointer = FramePointer::NonLeaf;
    base.stack_probes = StackProbeType::None;
    base.default_codegen_units = Some(1);

    Target {
        llvm_target: "aarch64-pc-windows-ntposix".into(),
        metadata: TargetMetadata {
            description: Some("ARM64 NT-POSIX (Windows 11 24H2+, NT syscalls only)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout:
            "e-m:w-p270:32:32-p271:32:32-p272:64:64-p:64:64-i32:32-i64:64-i128:128-n32:64-S128-Fn32"
                .into(),
        arch: Arch::AArch64,
        options: base,
    }
}
