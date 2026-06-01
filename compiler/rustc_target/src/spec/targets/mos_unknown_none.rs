use crate::spec::{Arch, PanicStrategy, Target, TargetMetadata, TargetOptions, cvs};

pub(crate) fn target() -> Target {
    // These options match the llvm-mos backend's expectations and mirror the
    // configuration used by the rust-mos fork. The `llvm_args` tune the MOS
    // backend's code generation for the very small 6502 register file; see
    // llvm-mos for details.
    //
    // `requires_lto` is set because the llvm-mos toolchain relies on whole-program
    // LTO to fit code into the 6502 address space, and atomics are limited to the
    // 8-bit, non-CAS operations the architecture can emulate.
    let options = TargetOptions {
        c_int_width: 16,
        cpu: "mos6502".into(),
        executables: true,
        singlethread: true,
        atomic_cas: false,
        min_atomic_width: Some(8),
        max_atomic_width: Some(8),
        disable_redzone: true,
        panic_strategy: PanicStrategy::Abort,
        linker: Some("mos-clang".into()),
        no_default_libraries: false,
        requires_lto: true,
        supports_stack_protector: false,
        trap_unreachable: false,
        llvm_args: cvs![
            "--force-precise-rotation-cost",
            "--jump-inst-cost=6",
            "--force-loop-cold-block",
            "--phi-node-folding-threshold=0",
            "--two-entry-phi-node-folding-threshold=0",
            "--align-large-globals=false",
            "--disable-spill-hoist",
        ],
        ..Default::default()
    };

    Target {
        llvm_target: "mos-unknown-none".into(),
        metadata: TargetMetadata {
            description: Some("8-bit 6502".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 16,
        data_layout: "e-m:e-p:16:8-p1:8:8-i16:8-i32:8-i64:8-f32:8-f64:8-a:8-Fi8-n8".into(),
        arch: Arch::Mos,
        options,
    }
}
