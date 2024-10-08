use crate::spec::base::apple::{ios_sim_llvm_target, opts, Arch, TargetAbi};
use crate::spec::{Target, TargetOptions};

pub(crate) fn target() -> Target {
    let arch = Arch::I386;
    // i386-apple-ios is a simulator target, even though it isn't declared
    // that way in the target name like the other ones...
    let abi = TargetAbi::Simulator;
    Target {
        // Clang automatically chooses a more specific target based on
        // IPHONEOS_DEPLOYMENT_TARGET.
        // This is required for the target to pick the right
        // MACH-O commands, so we do too.
        llvm_target: ios_sim_llvm_target(arch).into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("32-bit x86 iOS".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-p270:32:32-p271:32:32-p272:64:64-\
            i128:128-f64:32:64-f80:128-n8:16:32-S128"
            .into(),
        arch: arch.target_arch(),
        options: TargetOptions { max_atomic_width: Some(64), ..opts("ios", arch, abi) },
    }
}
