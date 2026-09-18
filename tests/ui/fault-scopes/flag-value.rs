//@ check-fail
//@ compile-flags: -Zprecise-fault-scopes=maybe

fn main() {}

//~? ERROR incorrect value `maybe` for unstable option `precise-fault-scopes`
