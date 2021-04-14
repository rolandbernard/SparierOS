// SPDX-License-Identifier: MIT

//! Abstraction of CPU specific code

#[cfg(feature = "arch_riscv64gc")]
#[path = "_arch/riscv64gc/boot.rs"]
mod boot;

#[cfg(feature = "arch_riscv32i")]
#[path = "_arch/riscv32i/boot.rs"]
mod boot;

pub use boot::*;
