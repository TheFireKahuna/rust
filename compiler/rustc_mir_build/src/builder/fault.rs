//! Raw-pointer accesses as unwind edges. Under exact fault scopes a load or
//! store through a raw pointer is a call to `read_via_copy`/`write_via_move`
//! whose unwind edge is the cleanup in effect at the access, so a hardware
//! fault there runs exactly the live drops. Places only borrowed or projected
//! are untouched: the access is what faults, not the address.

use rustc_hir::attrs::lang_items::LangItem;
use rustc_middle::mir::*;
use rustc_middle::ty::Ty;
use rustc_span::Spanned;

use crate::builder::{BlockAnd, BlockAndExtension, Builder};

impl<'a, 'tcx> Builder<'a, 'tcx> {
    /// Whether an access to `place` goes through a raw pointer, and so is an
    /// unwind edge under exact fault scopes.
    pub(crate) fn place_derefs_raw(&self, place: Place<'tcx>) -> bool {
        self.tcx.sess.precise_fault_scopes()
            && place.iter_projections().any(|(base, elem)| {
                elem == ProjectionElem::Deref && base.ty(&self.local_decls, self.tcx).ty.is_raw_ptr()
            })
    }

    /// Reads `place` into `destination` through the read intrinsic, whose
    /// unwind edge is the cleanup in effect here.
    pub(crate) fn fault_read_into(
        &mut self,
        block: BasicBlock,
        source_info: SourceInfo,
        destination: Place<'tcx>,
        place: Place<'tcx>,
    ) -> BasicBlock {
        let ty = place.ty(&self.local_decls, self.tcx).ty;
        let ptr = self.temp(Ty::new_imm_ptr(self.tcx, ty), source_info.span);
        self.cfg.push_assign(block, source_info, ptr, Rvalue::RawPtr(RawPtrKind::Const, place));
        let args = [Operand::Move(ptr)];
        self.fault_call(block, source_info, LangItem::ReadViaCopy, ty, args, destination)
    }

    /// Writes `value` to `place` through the write intrinsic, the same way.
    pub(crate) fn fault_write(
        &mut self,
        block: BasicBlock,
        source_info: SourceInfo,
        place: Place<'tcx>,
        value: Operand<'tcx>,
    ) -> BasicBlock {
        let ty = place.ty(&self.local_decls, self.tcx).ty;
        let ptr = self.temp(Ty::new_mut_ptr(self.tcx, ty), source_info.span);
        self.cfg.push_assign(block, source_info, ptr, Rvalue::RawPtr(RawPtrKind::Mut, place));
        let unit = self.get_unit_temp();
        let args = [Operand::Move(ptr), value];
        self.fault_call(block, source_info, LangItem::WriteViaMove, ty, args, unit)
    }

    /// Reads `place` into a fresh temporary when it goes through a raw pointer,
    /// so the load is the edge and a later use of the value is not; otherwise
    /// `place` itself.
    pub(crate) fn fault_read_copy(
        &mut self,
        block: BasicBlock,
        source_info: SourceInfo,
        place: Place<'tcx>,
    ) -> BlockAnd<Place<'tcx>> {
        if !self.place_derefs_raw(place) {
            return block.and(place);
        }
        let ty = place.ty(&self.local_decls, self.tcx).ty;
        let temp = self.temp(ty, source_info.span);
        let block = self.fault_read_into(block, source_info, temp, place);
        block.and(temp)
    }

    fn fault_call<const N: usize>(
        &mut self,
        block: BasicBlock,
        source_info: SourceInfo,
        item: LangItem,
        ty: Ty<'tcx>,
        args: [Operand<'tcx>; N],
        destination: Place<'tcx>,
    ) -> BasicBlock {
        let span = source_info.span;
        let def_id = self.tcx.require_lang_item(item, span);
        let func = Operand::function_handle(self.tcx, def_id, &[ty.into()], span);
        let args = args.into_iter().map(|node| Spanned { node, span }).collect();
        let next = self.cfg.start_new_block();
        self.cfg.terminate(
            block,
            source_info,
            TerminatorKind::Call {
                func,
                args,
                destination,
                target: Some(next),
                unwind: UnwindAction::Continue,
                call_source: CallSource::Misc,
                fn_span: span,
            },
        );
        self.diverge_from(block);
        next
    }
}
