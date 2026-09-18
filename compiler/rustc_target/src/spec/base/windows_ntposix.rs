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

        // `rust-lld` in the sysroot, not the `link.exe` that `base::msvc`
        // would otherwise have `get_linker` discover through the Visual
        // Studio install. Nothing in this toolchain comes from MSVC.
        linker_flavor: LinkerFlavor::Msvc(Lld::Yes),
        linker: Some("rust-lld".into()),

        // We take the COFF/lld-link/CodeView half of `base::msvc` and none of
        // its toolchain identity: there is no MSVC runtime, no MSVC C++ ABI and
        // no vcruntime here. Everything that genuinely needs "the MSVC linker
        // dialect" or "CodeView" is keyed on `linker_flavor` and
        // `uses_pdb_debuginfo()` respectively, so clearing this costs nothing
        // and keeps `is_like_msvc` meaning what its name says.
        is_like_msvc: false,

        // Itanium landing pads dispatched through our own `eh_personality`,
        // rather than MSVC funclets bound to `__CxxFrameHandler3` — a symbol no
        // NT-only image can resolve.
        eh_model: EhModel::Itanium,
        panic_strategy: PanicStrategy::Unwind,

        // A hardware fault is an unwind on this target: every raw-pointer
        // access is an unwind edge, so a recovered fault runs exactly the
        // live drops of the faulting frame.
        precise_fault_scopes: true,

        ..base
    }
}
