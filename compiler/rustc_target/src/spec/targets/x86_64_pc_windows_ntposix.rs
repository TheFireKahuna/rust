use crate::spec::{Arch, FramePointer, StackProbeType, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::windows_ntposix::opts();
    base.cpu = "x86-64-v3".into();
    base.features = "+avx2,+fma,+bmi1,+bmi2,+movbe,+aes,+pclmul,+adx,+rdrnd,+rdseed,+fsgsbase,\
                     +clflushopt,+prfchw,+xsavec,+xsaveopt,+cmpxchg16b,+ermsb"
        .into();
    base.plt_by_default = false;
    base.max_atomic_width = Some(128);

    // `KiUserExceptionDispatcher`/`KiUserApcDispatcher` stage frames at
    // `[rsp..]` and do not honor the SysV 128-byte red zone, so any frame live
    // across VEH or APC entry would be corrupted by it.
    base.disable_redzone = true;
    base.frame_pointer = FramePointer::NonLeaf;
    base.stack_probes = StackProbeType::None;
    base.default_codegen_units = Some(1);

    // Plain `"C"` is SysV here, as it is on the clang side; `extern "system"`
    // stays MS x64 for the ntdll boundary (pinned in `AbiMap`).
    base.windows_c_abi_sysv64 = true;

    Target {
        llvm_target: "x86_64-pc-windows-ntposix".into(),
        metadata: TargetMetadata {
            description: Some("64-bit NT-POSIX (Windows 11 24H2+, NT syscalls only)".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 64,
        data_layout:
            "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: Arch::X86_64,
        options: base,
    }
}
