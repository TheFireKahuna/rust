use rustc_macros::{Decodable, Encodable, StableHash};
use rustc_span::def_id::DefId;
use rustc_span::{Symbol, sym};

use super::TyCtxt;

#[derive(Copy, Clone, Debug, Decodable, Encodable, StableHash)]
pub struct IntrinsicDef {
    pub name: Symbol,
    /// Whether the intrinsic has no meaningful body and all backends need to shim all calls to it.
    pub must_be_overridden: bool,
    /// Whether the intrinsic can be invoked from stable const fn (`#[rustc_intrinsic_const_stable_indirect]`).
    pub const_stable_indirect: bool,
}

impl TyCtxt<'_> {
    /// Whether `def_id` is a raw-pointer memory intrinsic: an access that can fault, and is an
    /// unwind edge under exact fault scopes.
    pub fn intrinsic_may_fault(self, def_id: DefId) -> bool {
        self.intrinsic(def_id).is_some_and(|intrinsic| {
            matches!(
                intrinsic.name,
                sym::read_via_copy
                    | sym::write_via_move
                    | sym::volatile_load
                    | sym::volatile_store
                    | sym::unaligned_volatile_load
                    | sym::unaligned_volatile_store
                    | sym::copy
                    | sym::copy_nonoverlapping
                    | sym::write_bytes
            )
        })
    }

    pub fn is_intrinsic(self, def_id: DefId, name: Symbol) -> bool {
        let Some(i) = self.intrinsic(def_id) else { return false };
        i.name == name
    }
}
