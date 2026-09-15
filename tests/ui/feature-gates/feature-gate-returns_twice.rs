#![crate_type = "lib"]

extern "C" {
    #[returns_twice] //~ ERROR the `returns_twice` attribute is an experimental feature
    pub fn setjmp(buf: *mut u8) -> i32;
}
