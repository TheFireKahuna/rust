// A move out of a raw-pointer place is still refused under exact fault
// scopes: only a copy is lowered to the read intrinsic.
//@ compile-flags: -Zprecise-fault-scopes=yes

pub struct Owned(pub u32);

pub fn take(p: *const Owned) -> Owned {
    unsafe { *p } //~ ERROR cannot move out of `*p` which is behind a raw pointer
}

fn main() {}
