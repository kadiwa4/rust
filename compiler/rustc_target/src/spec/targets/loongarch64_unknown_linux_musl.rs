use crate::spec::{base, Target, TargetOptions};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "loongarch64-unknown-linux-musl".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("LoongArch64 Linux (LP64D ABI) with musl 1.2.5".into()),
            tier: Some(2),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "loongarch64".into(),
        options: TargetOptions {
            cpu: "generic".into(),
            features: "+f,+d".into(),
            llvm_abiname: "lp64d".into(),
            max_atomic_width: Some(64),
            crt_static_default: false,
            ..base::linux_musl::opts()
        },
    }
}
