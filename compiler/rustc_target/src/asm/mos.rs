use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

def_reg_class! {
    Mos MosInlineAsmRegClass {
        // Zero-page 8-bit "imaginary" registers (llvm-mos `Imag8`, constraint `r`).
        reg,
        // CPU general-purpose registers: accumulator + index (llvm-mos `GPR`,
        // constraint `R`): `a`, `x`, `y`.
        reg_gpr,
    }
}

impl MosInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        // The 6502 is an 8-bit machine; only `i8` operands are supported. 16-bit
        // operands would require the `Imag16` (`rs*`) register class, which
        // aliases `Imag8` register pairs and is intentionally not exposed here.
        match self {
            Self::reg | Self::reg_gpr => types! { _: I8; },
        }
    }
}

def_regs! {
    Mos MosInlineAsmReg MosInlineAsmRegClass {
        // GPR: accumulator and the two index registers.
        a: reg_gpr = ["a"],
        x: reg_gpr = ["x"],
        y: reg_gpr = ["y"],
        // Zero-page imaginary 8-bit registers usable as scratch. `rc0`/`rc1`
        // (`rs0`, the soft stack pointer) and `rc30`/`rc31` (`rs15`, the frame
        // pointer) are reserved by llvm-mos and excluded below.
        rc2: reg = ["rc2"],
        rc3: reg = ["rc3"],
        rc4: reg = ["rc4"],
        rc5: reg = ["rc5"],
        rc6: reg = ["rc6"],
        rc7: reg = ["rc7"],
        rc8: reg = ["rc8"],
        rc9: reg = ["rc9"],
        rc10: reg = ["rc10"],
        rc11: reg = ["rc11"],
        rc12: reg = ["rc12"],
        rc13: reg = ["rc13"],
        rc14: reg = ["rc14"],
        rc15: reg = ["rc15"],
        rc16: reg = ["rc16"],
        rc17: reg = ["rc17"],
        rc18: reg = ["rc18"],
        rc19: reg = ["rc19"],
        rc20: reg = ["rc20"],
        rc21: reg = ["rc21"],
        rc22: reg = ["rc22"],
        rc23: reg = ["rc23"],
        rc24: reg = ["rc24"],
        rc25: reg = ["rc25"],
        rc26: reg = ["rc26"],
        rc27: reg = ["rc27"],
        rc28: reg = ["rc28"],
        rc29: reg = ["rc29"],
        #error = ["rc0", "rc1", "rs0", "sp"] =>
            "rs0 (rc0:rc1) is the soft stack pointer and cannot be used as an operand for inline asm",
        #error = ["rc30", "rc31", "rs15"] =>
            "rs15 (rc30:rc31) is reserved as the frame pointer and cannot be used as an operand for inline asm",
        #error = ["s"] =>
            "the hardware stack pointer cannot be used as an operand for inline asm",
    }
}

impl MosInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
