use super::term::{mk_binders, Lower};
use crate::{
    ctx::PreludeModule,
    translation::{
        traits::resolve_opt,
        ty::{translate_ty, variant_accessor_name},
    },
    util::get_builtin,
};
use creusot_rustc::{
    hir::def_id::DefId,
    middle::ty::{subst::SubstsRef, TyCtxt},
    span::{symbol::sym, Symbol},
};
use why3::{
    exp::{BinOp, Constant, Exp, Purity, UnOp},
    QName,
};

impl<'tcx> Lower<'_, 'tcx> {
    pub(crate) fn lookup_builtin(
        &mut self,
        method: (DefId, SubstsRef<'tcx>),
        args: &mut Vec<Exp>,
    ) -> Option<Exp> {
        let mut def_id = method.0;
        let substs = method.1;
        if let Some(trait_id) = trait_id_of_method(self.ctx.tcx, def_id) {
            // We typically implement `From` but call `into`, using the blanket impl of `Into`
            // for any `From` type. So when we see an instance of `into` we check that isn't just
            // a wrapper for a builtin `From` impl.
            if self.ctx.tcx.is_diagnostic_item(sym::Into, trait_id) {
                let from_fn = self.ctx.tcx.lang_items().from_fn().unwrap();
                let from_subst = self.ctx.tcx.intern_substs(&[substs[1], substs[0]]);
                let from_impl =
                    resolve_opt(self.ctx.tcx, self.ctx.tcx.param_env(def_id), from_fn, from_subst)
                        .unwrap();
                def_id = from_impl.0;
            }
        }

        let def_id = Some(def_id);
        let builtin_attr = get_builtin(self.ctx.tcx, def_id.unwrap());

        if builtin_attr == Some(Symbol::intern("ghost_new")) {
            return Some(args.remove(0));
        // } else if builtin_attr == Some(Symbol::intern("ghost_inner")) {
        //     return Some(args.remove(0));
        } else if builtin_attr == Some(Symbol::intern("ghost_deref")) {
            let ghost_type = self.ctx.get_diagnostic_item(Symbol::intern("ghost_type")).unwrap();
            self.names.insert(ghost_type, substs);

            let variant = &self.ctx.adt_def(ghost_type).variants()[0u32.into()];
            self.ctx.translate_accessor(variant.fields[0].did);
            let acc_name = variant_accessor_name(self.ctx, ghost_type, variant, 0);

            return Some(Exp::pure_qvar(acc_name).app_to(args.remove(0)));
        } else if builtin_attr == Some(Symbol::intern("identity")) {
            return Some(args.remove(0));
        } else if def_id == self.ctx.tcx.get_diagnostic_item(sym::abort) {
            // Semi-questionable: we allow abort() & unreachable() in pearlite but
            // interpret them as `absurd` (aka prove false).
            return Some(Exp::Absurd);
        } else if def_id == self.ctx.tcx.get_diagnostic_item(sym::unreachable) {
            return Some(Exp::Absurd);
        } else if self.ctx.tcx.def_path_str(def_id.unwrap()) == "std::boxed::Box::<T>::new" {
            return Some(args.remove(0));
        }

        if let Some(builtin) = builtin_attr.and_then(|a| QName::from_string(&a.as_str())) {
            self.names.import_builtin_module(builtin.clone().module_qname());

            if let Purity::Program = self.pure {
                return Some(mk_binders(
                    Exp::pure_qvar(builtin.without_search_path()),
                    args.clone(),
                ));
            } else {
                return Some(Exp::Call(
                    box Exp::pure_qvar(builtin.without_search_path()),
                    args.clone(),
                ));
            }
        }
        None
    }
}

fn trait_id_of_method(tcx: TyCtxt, def_id: DefId) -> Option<DefId> {
    tcx.impl_of_method(def_id).and_then(|id| tcx.trait_id_of_impl(id))
}
