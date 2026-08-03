use crate::spec::{
    EhModel, Env, LinkerFlavor, Lld, Os, PanicStrategy, TargetOptions, base, cvs,
};

pub(crate) fn opts() -> TargetOptions {
    let base = base::msvc::opts();

    // libnt links no MSVC runtime at all: the import set is ntdll plus
    // libunwind, which supplies `_Unwind_Resume` for the Itanium model below.
    // The `/NODEFAULTLIB:` set mirrors clang's NT-POSIX toolchain driver.
    let pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Msvc(Lld::No),
        &[
            "/NOLOGO",
            "/NODEFAULTLIB:msvcrt",
            "/NODEFAULTLIB:msvcrtd",
            "/NODEFAULTLIB:vcruntime",
            "/NODEFAULTLIB:vcruntimed",
            "/NODEFAULTLIB:ucrt",
            "/NODEFAULTLIB:ucrtd",
            "/NODEFAULTLIB:libcmt",
            "/NODEFAULTLIB:libcmtd",
            "/NODEFAULTLIB:oldnames",
        ],
    );

    TargetOptions {
        os: Os::Windows,
        env: Env::Ntposix,
        vendor: "pc".into(),
        dynamic_linking: true,
        dll_prefix: "".into(),
        dll_suffix: ".dll".into(),
        exe_suffix: ".exe".into(),
        staticlib_prefix: "".into(),
        staticlib_suffix: ".lib".into(),
        families: cvs!["windows"],
        crt_static_allows_dylibs: false,
        crt_static_respected: false,
        requires_uwtable: true,
        no_default_libraries: true,
        has_thread_local: true,
        pre_link_args,

        // The whole point of the target: keep the MSVC linker, CodeView and
        // the MSVC link-argument dialect (`is_like_msvc` stays set, inherited
        // from `base::msvc`), but emit Itanium landing pads dispatched through
        // our own `eh_personality` rather than MSVC funclets bound to
        // `__CxxFrameHandler3`, which no NT-only image can resolve.
        eh_model: EhModel::Itanium,
        panic_strategy: PanicStrategy::Unwind,

        ..base
    }
}
