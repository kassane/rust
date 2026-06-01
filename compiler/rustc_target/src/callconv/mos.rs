//! LLVM-frontend specific MOS calling convention implementation.
//!
//! # Current calling convention ABI
//!
//! Inherited from Clang's `clang::DefaultABIInfo` implementation - self described
//! as
//!
//! > the default implementation for ABI specific details. This implementation
//! > provides information which results in
//! > self-consistent and sensible LLVM IR generation, but does not
//! > conform to any particular ABI.
//! >
//! > - Doxygen Documentation of `clang::DefaultABIInfo`
//!
//! This mirrors `clang/lib/CodeGen/Targets/MOS.cpp` (`MOSABIInfo`), which extends
//! `DefaultABIInfo` for the llvm-mos backend:
//!
//! * Scalars use the `DefaultABIInfo` rules: integers narrower than the 16-bit C
//!   `int` are promoted (sign/zero extended) to 16 bits.
//! * Aggregates (and unions) **larger than 32 bits** are passed and returned
//!   indirectly (by hidden pointer, *not* `byval`); aggregates of 32 bits or
//!   fewer are passed/returned **directly**. This differs from a plain
//!   `DefaultABIInfo` (which would pass all aggregates indirectly) and is what
//!   keeps small by-value structs ABI-compatible with C/clang on the 6502.

use rustc_abi::TyAbiInterface;

use crate::callconv::{ArgAbi, FnAbi};

fn classify_ret<Ty>(ret: &mut ArgAbi<'_, Ty>) {
    if ret.layout.is_aggregate() && ret.layout.size.bits() > 32 {
        ret.make_indirect();
    } else {
        ret.extend_integer_width_to(16);
    }
}

fn classify_arg<'a, Ty, C>(cx: &C, arg: &mut ArgAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if arg.layout.pass_indirectly_in_non_rustic_abis(cx) {
        arg.make_indirect();
        return;
    }
    if arg.layout.is_aggregate() && arg.layout.size.bits() > 32 {
        arg.make_indirect();
    } else {
        arg.extend_integer_width_to(16);
    }
}

pub(crate) fn compute_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
{
    if !fn_abi.ret.is_ignore() {
        classify_ret(&mut fn_abi.ret);
    }

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }
        classify_arg(cx, arg);
    }
}
