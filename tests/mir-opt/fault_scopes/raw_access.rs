// Under exact fault scopes a load or store through a raw pointer is a call
// to `read_via_copy`/`write_via_move` whose unwind edge is the cleanup in
// effect at the access; a `&`-deref stays a statement.
//@ compile-flags: -Zprecise-fault-scopes=yes -Zmir-opt-level=0
//@ skip-filecheck

pub struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {}
}

pub struct Pair {
    pub a: Guard,
    pub b: u32,
}

// EMIT_MIR raw_access.read.built.after.mir
pub fn read(p: *const u32) -> u32 {
    unsafe { *p }
}

// EMIT_MIR raw_access.write.built.after.mir
pub fn write(p: *mut u32, v: u32) {
    unsafe { *p = v }
}

// EMIT_MIR raw_access.compound.built.after.mir
pub fn compound(p: *mut u32) {
    unsafe { *p += 1 }
}

// EMIT_MIR raw_access.replace.built.after.mir
pub fn replace(p: *mut Guard, g: Guard) {
    unsafe { *p = g }
}

// EMIT_MIR raw_access.mid_statement.built.after.mir
pub fn mid_statement(p: *const u32, a: Guard) -> Pair {
    Pair { a, b: unsafe { *p } }
}

// EMIT_MIR raw_access.through_reference.built.after.mir
pub fn through_reference(r: &u32) -> u32 {
    *r
}

// EMIT_MIR raw_access.matched.built.after.mir
pub fn matched(p: *const Option<u32>) -> u32 {
    match unsafe { *p } {
        Some(v) => v,
        None => 0,
    }
}

// EMIT_MIR raw_access.tested_in_place.built.after.mir
pub fn tested_in_place(p: *const (u32, u32)) -> u32 {
    unsafe {
        match *p {
            (0, ref b) => *b,
            (a, _) => a,
        }
    }
}

// EMIT_MIR raw_access.in_const_fn.built.after.mir
pub const fn in_const_fn(p: *const u32) -> u32 {
    unsafe { *p }
}

fn main() {
    let x = 7u32;
    let _ = read(&x);
    let _ = in_const_fn(&x);
}
