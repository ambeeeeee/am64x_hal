#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    apbaddr_dbg_cpu1_edesr: ApbaddrDbgCpu1Edesr,
    apbaddr_dbg_cpu1_edecr: ApbaddrDbgCpu1Edecr,
    _reserved2: [u8; 0x08],
    apbaddr_dbg_cpu1_edwar_31_0: ApbaddrDbgCpu1Edwar31_0,
    apbaddr_dbg_cpu1_edwar_63_32: ApbaddrDbgCpu1Edwar63_32,
    _reserved4: [u8; 0x48],
    apbaddr_dbg_cpu1_dbgdtrrx_el0: ApbaddrDbgCpu1DbgdtrrxEl0,
    apbaddr_dbg_cpu1_editr: ApbaddrDbgCpu1Editr,
    apbaddr_dbg_cpu1_edscr: ApbaddrDbgCpu1Edscr,
    apbaddr_dbg_cpu1_dbgdtrtx_el0: ApbaddrDbgCpu1DbgdtrtxEl0,
    apbaddr_dbg_cpu1_edrcr: ApbaddrDbgCpu1Edrcr,
    apbaddr_dbg_cpu1_edacr: ApbaddrDbgCpu1Edacr,
    apbaddr_dbg_cpu1_edeccr: ApbaddrDbgCpu1Edeccr,
    _reserved11: [u8; 0x04],
    apbaddr_dbg_cpu1_edpcsr_31_0: ApbaddrDbgCpu1Edpcsr31_0,
    apbaddr_dbg_cpu1_edcidsr: ApbaddrDbgCpu1Edcidsr,
    apbaddr_dbg_cpu1_edvidsr: ApbaddrDbgCpu1Edvidsr,
    apbaddr_dbg_cpu1_edpcsr_63_32: ApbaddrDbgCpu1Edpcsr63_32,
    _reserved15: [u8; 0x0250],
    apbaddr_dbg_cpu1_oslar_el1: ApbaddrDbgCpu1OslarEl1,
    _reserved16: [u8; 0x0c],
    apbaddr_dbg_cpu1_edprcr: ApbaddrDbgCpu1Edprcr,
    apbaddr_dbg_cpu1_edprsr: ApbaddrDbgCpu1Edprsr,
    _reserved18: [u8; 0xe8],
    apbaddr_dbg_cpu1_dbgbvr0_el1_31_0: ApbaddrDbgCpu1Dbgbvr0El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr0_el1_63_32: ApbaddrDbgCpu1Dbgbvr0El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr0_el1: ApbaddrDbgCpu1Dbgbcr0El1,
    _reserved21: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgbvr1_el1_31_0: ApbaddrDbgCpu1Dbgbvr1El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr1_el1_63_32: ApbaddrDbgCpu1Dbgbvr1El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr1_el1: ApbaddrDbgCpu1Dbgbcr1El1,
    _reserved24: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgbvr2_el1_31_0: ApbaddrDbgCpu1Dbgbvr2El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr2_el1_63_32: ApbaddrDbgCpu1Dbgbvr2El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr2_el1: ApbaddrDbgCpu1Dbgbcr2El1,
    _reserved27: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgbvr3_el1_31_0: ApbaddrDbgCpu1Dbgbvr3El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr3_el1_63_32: ApbaddrDbgCpu1Dbgbvr3El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr3_el1: ApbaddrDbgCpu1Dbgbcr3El1,
    _reserved30: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgbvr4_el1_31_0: ApbaddrDbgCpu1Dbgbvr4El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr4_el1_63_32: ApbaddrDbgCpu1Dbgbvr4El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr4_el1: ApbaddrDbgCpu1Dbgbcr4El1,
    _reserved33: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgbvr5_el1_31_0: ApbaddrDbgCpu1Dbgbvr5El1_31_0,
    apbaddr_dbg_cpu1_dbgbvr5_el1_63_32: ApbaddrDbgCpu1Dbgbvr5El1_63_32,
    apbaddr_dbg_cpu1_dbgbcr5_el1: ApbaddrDbgCpu1Dbgbcr5El1,
    _reserved36: [u8; 0x03a4],
    apbaddr_dbg_cpu1_dbgwvr0_el1_31_0: ApbaddrDbgCpu1Dbgwvr0El1_31_0,
    apbaddr_dbg_cpu1_dbgwvr0_el1_63_32: ApbaddrDbgCpu1Dbgwvr0El1_63_32,
    apbaddr_dbg_cpu1_dbgwcr0_el1: ApbaddrDbgCpu1Dbgwcr0El1,
    _reserved39: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgwvr1_el1_31_0: ApbaddrDbgCpu1Dbgwvr1El1_31_0,
    apbaddr_dbg_cpu1_dbgwvr1_el1_63_32: ApbaddrDbgCpu1Dbgwvr1El1_63_32,
    apbaddr_dbg_cpu1_dbgwcr1_el1: ApbaddrDbgCpu1Dbgwcr1El1,
    _reserved42: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgwvr2_el1_31_0: ApbaddrDbgCpu1Dbgwvr2El1_31_0,
    apbaddr_dbg_cpu1_dbgwvr2_el1_63_32: ApbaddrDbgCpu1Dbgwvr2El1_63_32,
    apbaddr_dbg_cpu1_dbgwcr2_el1: ApbaddrDbgCpu1Dbgwcr2El1,
    _reserved45: [u8; 0x04],
    apbaddr_dbg_cpu1_dbgwvr3_el1_31_0: ApbaddrDbgCpu1Dbgwvr3El1_31_0,
    apbaddr_dbg_cpu1_dbgwvr3_el1_63_32: ApbaddrDbgCpu1Dbgwvr3El1_63_32,
    apbaddr_dbg_cpu1_dbgwcr3_el1: ApbaddrDbgCpu1Dbgwcr3El1,
    _reserved48: [u8; 0x04c4],
    apbaddr_dbg_cpu1_midr_el1: ApbaddrDbgCpu1MidrEl1,
    _reserved49: [u8; 0x1c],
    apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0: ApbaddrDbgCpu1IdAa64pfr0El1_31_0,
    apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32: ApbaddrDbgCpu1IdAa64pfr0El1_63_32,
    apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0: ApbaddrDbgCpu1IdAa64dfr0El1_31_0,
    apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32: ApbaddrDbgCpu1IdAa64dfr0El1_63_32,
    apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0: ApbaddrDbgCpu1IdAa64isar0El1_31_0,
    apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32: ApbaddrDbgCpu1IdAa64isar0El1_63_32,
    apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0: ApbaddrDbgCpu1IdAa64mmfr0El1_31_0,
    apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32: ApbaddrDbgCpu1IdAa64mmfr0El1_63_32,
    apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0: ApbaddrDbgCpu1IdAa64pfr1El1_31_0,
    apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32: ApbaddrDbgCpu1IdAa64pfr1El1_63_32,
    apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0: ApbaddrDbgCpu1IdAa64dfr1El1_31_0,
    apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32: ApbaddrDbgCpu1IdAa64dfr1El1_63_32,
    apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0: ApbaddrDbgCpu1IdAa64isar1El1_31_0,
    apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32: ApbaddrDbgCpu1IdAa64isar1El1_63_32,
    apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0: ApbaddrDbgCpu1IdAa64mmfr1El1_31_0,
    apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32: ApbaddrDbgCpu1IdAa64mmfr1El1_63_32,
    _reserved65: [u8; 0x01a0],
    apbaddr_dbg_cpu1_editctrl: ApbaddrDbgCpu1Editctrl,
    _reserved66: [u8; 0x9c],
    apbaddr_dbg_cpu1_dbgclaimset_el1: ApbaddrDbgCpu1DbgclaimsetEl1,
    apbaddr_dbg_cpu1_dbgclaimclr_el1: ApbaddrDbgCpu1DbgclaimclrEl1,
    apbaddr_dbg_cpu1_eddevaff0: ApbaddrDbgCpu1Eddevaff0,
    apbaddr_dbg_cpu1_eddevaff1: ApbaddrDbgCpu1Eddevaff1,
    apbaddr_dbg_cpu1_edlar: ApbaddrDbgCpu1Edlar,
    apbaddr_dbg_cpu1_edlsr: ApbaddrDbgCpu1Edlsr,
    apbaddr_dbg_cpu1_dbgauthstatus_el1: ApbaddrDbgCpu1DbgauthstatusEl1,
    apbaddr_dbg_cpu1_eddevarch: ApbaddrDbgCpu1Eddevarch,
    apbaddr_dbg_cpu1_eddevid2: ApbaddrDbgCpu1Eddevid2,
    apbaddr_dbg_cpu1_eddevid1: ApbaddrDbgCpu1Eddevid1,
    apbaddr_dbg_cpu1_eddevid: ApbaddrDbgCpu1Eddevid,
    apbaddr_dbg_cpu1_eddevtype: ApbaddrDbgCpu1Eddevtype,
    apbaddr_dbg_cpu1_edpidr4: ApbaddrDbgCpu1Edpidr4,
    _reserved79: [u8; 0x0c],
    apbaddr_dbg_cpu1_edpidr0: ApbaddrDbgCpu1Edpidr0,
    apbaddr_dbg_cpu1_edpidr1: ApbaddrDbgCpu1Edpidr1,
    apbaddr_dbg_cpu1_edpidr2: ApbaddrDbgCpu1Edpidr2,
    apbaddr_dbg_cpu1_edpidr3: ApbaddrDbgCpu1Edpidr3,
    apbaddr_dbg_cpu1_edcidr0: ApbaddrDbgCpu1Edcidr0,
    apbaddr_dbg_cpu1_edcidr1: ApbaddrDbgCpu1Edcidr1,
    apbaddr_dbg_cpu1_edcidr2: ApbaddrDbgCpu1Edcidr2,
    apbaddr_dbg_cpu1_edcidr3: ApbaddrDbgCpu1Edcidr3,
}
impl RegisterBlock {
    #[doc = "0x20 - External Debug Event Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edesr(&self) -> &ApbaddrDbgCpu1Edesr {
        &self.apbaddr_dbg_cpu1_edesr
    }
    #[doc = "0x24 - External Debug Execution Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edecr(&self) -> &ApbaddrDbgCpu1Edecr {
        &self.apbaddr_dbg_cpu1_edecr
    }
    #[doc = "0x30 - External Debug Watchpoint Address Register (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edwar_31_0(&self) -> &ApbaddrDbgCpu1Edwar31_0 {
        &self.apbaddr_dbg_cpu1_edwar_31_0
    }
    #[doc = "0x34 - External Debug Watchpoint Address Register (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edwar_63_32(&self) -> &ApbaddrDbgCpu1Edwar63_32 {
        &self.apbaddr_dbg_cpu1_edwar_63_32
    }
    #[doc = "0x80 - Debug Data Transfer Register Receive"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgdtrrx_el0(&self) -> &ApbaddrDbgCpu1DbgdtrrxEl0 {
        &self.apbaddr_dbg_cpu1_dbgdtrrx_el0
    }
    #[doc = "0x84 - External Debug Instruction Transfer Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_editr(&self) -> &ApbaddrDbgCpu1Editr {
        &self.apbaddr_dbg_cpu1_editr
    }
    #[doc = "0x88 - External Debug Status and Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edscr(&self) -> &ApbaddrDbgCpu1Edscr {
        &self.apbaddr_dbg_cpu1_edscr
    }
    #[doc = "0x8c - Debug Data Transfer Register Transmit"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgdtrtx_el0(&self) -> &ApbaddrDbgCpu1DbgdtrtxEl0 {
        &self.apbaddr_dbg_cpu1_dbgdtrtx_el0
    }
    #[doc = "0x90 - External Debug Reserve Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edrcr(&self) -> &ApbaddrDbgCpu1Edrcr {
        &self.apbaddr_dbg_cpu1_edrcr
    }
    #[doc = "0x94 - External Debug Auxiliary Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edacr(&self) -> &ApbaddrDbgCpu1Edacr {
        &self.apbaddr_dbg_cpu1_edacr
    }
    #[doc = "0x98 - External Debug Exception Catch Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edeccr(&self) -> &ApbaddrDbgCpu1Edeccr {
        &self.apbaddr_dbg_cpu1_edeccr
    }
    #[doc = "0xa0 - External Debug Program Counter Sample Register (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpcsr_31_0(&self) -> &ApbaddrDbgCpu1Edpcsr31_0 {
        &self.apbaddr_dbg_cpu1_edpcsr_31_0
    }
    #[doc = "0xa4 - External Debug Context ID Sample Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edcidsr(&self) -> &ApbaddrDbgCpu1Edcidsr {
        &self.apbaddr_dbg_cpu1_edcidsr
    }
    #[doc = "0xa8 - External Debug Virtual Context Sample Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edvidsr(&self) -> &ApbaddrDbgCpu1Edvidsr {
        &self.apbaddr_dbg_cpu1_edvidsr
    }
    #[doc = "0xac - External Debug Program Counter Sample Register (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpcsr_63_32(&self) -> &ApbaddrDbgCpu1Edpcsr63_32 {
        &self.apbaddr_dbg_cpu1_edpcsr_63_32
    }
    #[doc = "0x300 - OS Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_oslar_el1(&self) -> &ApbaddrDbgCpu1OslarEl1 {
        &self.apbaddr_dbg_cpu1_oslar_el1
    }
    #[doc = "0x310 - External Debug Power/Reset Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edprcr(&self) -> &ApbaddrDbgCpu1Edprcr {
        &self.apbaddr_dbg_cpu1_edprcr
    }
    #[doc = "0x314 - External Debug Processor Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edprsr(&self) -> &ApbaddrDbgCpu1Edprsr {
        &self.apbaddr_dbg_cpu1_edprsr
    }
    #[doc = "0x400 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr0_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr0El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr0_el1_31_0
    }
    #[doc = "0x404 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr0_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr0El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr0_el1_63_32
    }
    #[doc = "0x408 - Debug Breakpoint Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr0_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr0El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr0_el1
    }
    #[doc = "0x410 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr1_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr1El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr1_el1_31_0
    }
    #[doc = "0x414 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr1_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr1El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr1_el1_63_32
    }
    #[doc = "0x418 - Debug Breakpoint Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr1_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr1El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr1_el1
    }
    #[doc = "0x420 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr2_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr2El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr2_el1_31_0
    }
    #[doc = "0x424 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr2_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr2El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr2_el1_63_32
    }
    #[doc = "0x428 - Debug Breakpoint Control Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr2_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr2El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr2_el1
    }
    #[doc = "0x430 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr3_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr3El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr3_el1_31_0
    }
    #[doc = "0x434 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr3_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr3El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr3_el1_63_32
    }
    #[doc = "0x438 - Debug Breakpoint Control Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr3_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr3El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr3_el1
    }
    #[doc = "0x440 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr4_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr4El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr4_el1_31_0
    }
    #[doc = "0x444 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr4_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr4El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr4_el1_63_32
    }
    #[doc = "0x448 - Debug Breakpoint Control Register 4"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr4_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr4El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr4_el1
    }
    #[doc = "0x450 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr5_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgbvr5El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgbvr5_el1_31_0
    }
    #[doc = "0x454 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbvr5_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgbvr5El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgbvr5_el1_63_32
    }
    #[doc = "0x458 - Debug Breakpoint Control Register 5"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgbcr5_el1(&self) -> &ApbaddrDbgCpu1Dbgbcr5El1 {
        &self.apbaddr_dbg_cpu1_dbgbcr5_el1
    }
    #[doc = "0x800 - Debug Watchpoint Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr0_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgwvr0El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgwvr0_el1_31_0
    }
    #[doc = "0x804 - Debug Watchpoint Extended Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr0_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgwvr0El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgwvr0_el1_63_32
    }
    #[doc = "0x808 - Debug Watchpoint Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwcr0_el1(&self) -> &ApbaddrDbgCpu1Dbgwcr0El1 {
        &self.apbaddr_dbg_cpu1_dbgwcr0_el1
    }
    #[doc = "0x810 - Debug Watchpoint Value Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr1_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgwvr1El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgwvr1_el1_31_0
    }
    #[doc = "0x814 - Debug Watchpoint Extended Value Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr1_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgwvr1El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgwvr1_el1_63_32
    }
    #[doc = "0x818 - Debug Watchpoint Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwcr1_el1(&self) -> &ApbaddrDbgCpu1Dbgwcr1El1 {
        &self.apbaddr_dbg_cpu1_dbgwcr1_el1
    }
    #[doc = "0x820 - Debug Watchpoint Value Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr2_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgwvr2El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgwvr2_el1_31_0
    }
    #[doc = "0x824 - Debug Watchpoint Extended Value Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr2_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgwvr2El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgwvr2_el1_63_32
    }
    #[doc = "0x828 - Debug Watchpoint Control Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwcr2_el1(&self) -> &ApbaddrDbgCpu1Dbgwcr2El1 {
        &self.apbaddr_dbg_cpu1_dbgwcr2_el1
    }
    #[doc = "0x830 - Debug Watchpoint Value Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr3_el1_31_0(&self) -> &ApbaddrDbgCpu1Dbgwvr3El1_31_0 {
        &self.apbaddr_dbg_cpu1_dbgwvr3_el1_31_0
    }
    #[doc = "0x834 - Debug Watchpoint Extended Value Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwvr3_el1_63_32(&self) -> &ApbaddrDbgCpu1Dbgwvr3El1_63_32 {
        &self.apbaddr_dbg_cpu1_dbgwvr3_el1_63_32
    }
    #[doc = "0x838 - Debug Watchpoint Control Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgwcr3_el1(&self) -> &ApbaddrDbgCpu1Dbgwcr3El1 {
        &self.apbaddr_dbg_cpu1_dbgwcr3_el1
    }
    #[doc = "0xd00 - Main ID Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_midr_el1(&self) -> &ApbaddrDbgCpu1MidrEl1 {
        &self.apbaddr_dbg_cpu1_midr_el1
    }
    #[doc = "0xd20 - Processor Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0(&self) -> &ApbaddrDbgCpu1IdAa64pfr0El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0
    }
    #[doc = "0xd24 - Processor Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64pfr0El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32
    }
    #[doc = "0xd28 - Debug Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0(&self) -> &ApbaddrDbgCpu1IdAa64dfr0El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0
    }
    #[doc = "0xd2c - Debug Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64dfr0El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32
    }
    #[doc = "0xd30 - Instruction Set Attribute Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64isar0El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0
    }
    #[doc = "0xd34 - Instruction Set Attribute Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64isar0El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32
    }
    #[doc = "0xd38 - Memory Model Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64mmfr0El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0
    }
    #[doc = "0xd3c - Memory Model Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64mmfr0El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32
    }
    #[doc = "0xd40 - Processor Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0(&self) -> &ApbaddrDbgCpu1IdAa64pfr1El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0
    }
    #[doc = "0xd44 - Processor Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64pfr1El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32
    }
    #[doc = "0xd48 - Auxiliary Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0(&self) -> &ApbaddrDbgCpu1IdAa64dfr1El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0
    }
    #[doc = "0xd4c - Auxiliary Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64dfr1El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32
    }
    #[doc = "0xd50 - Instruction Set Attribute Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64isar1El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0
    }
    #[doc = "0xd54 - Instruction Set Attribute Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64isar1El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32
    }
    #[doc = "0xd58 - Memory Model Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64mmfr1El1_31_0 {
        &self.apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0
    }
    #[doc = "0xd5c - Memory Model Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu1IdAa64mmfr1El1_63_32 {
        &self.apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32
    }
    #[doc = "0xf00 - External Debug Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_editctrl(&self) -> &ApbaddrDbgCpu1Editctrl {
        &self.apbaddr_dbg_cpu1_editctrl
    }
    #[doc = "0xfa0 - Debug Claim Tag Set Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgclaimset_el1(&self) -> &ApbaddrDbgCpu1DbgclaimsetEl1 {
        &self.apbaddr_dbg_cpu1_dbgclaimset_el1
    }
    #[doc = "0xfa4 - Debug Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgclaimclr_el1(&self) -> &ApbaddrDbgCpu1DbgclaimclrEl1 {
        &self.apbaddr_dbg_cpu1_dbgclaimclr_el1
    }
    #[doc = "0xfa8 - External Debug Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevaff0(&self) -> &ApbaddrDbgCpu1Eddevaff0 {
        &self.apbaddr_dbg_cpu1_eddevaff0
    }
    #[doc = "0xfac - External Debug Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevaff1(&self) -> &ApbaddrDbgCpu1Eddevaff1 {
        &self.apbaddr_dbg_cpu1_eddevaff1
    }
    #[doc = "0xfb0 - External Debug Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edlar(&self) -> &ApbaddrDbgCpu1Edlar {
        &self.apbaddr_dbg_cpu1_edlar
    }
    #[doc = "0xfb4 - External Debug Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edlsr(&self) -> &ApbaddrDbgCpu1Edlsr {
        &self.apbaddr_dbg_cpu1_edlsr
    }
    #[doc = "0xfb8 - Debug Authentication Status register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_dbgauthstatus_el1(&self) -> &ApbaddrDbgCpu1DbgauthstatusEl1 {
        &self.apbaddr_dbg_cpu1_dbgauthstatus_el1
    }
    #[doc = "0xfbc - External Debug Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevarch(&self) -> &ApbaddrDbgCpu1Eddevarch {
        &self.apbaddr_dbg_cpu1_eddevarch
    }
    #[doc = "0xfc0 - External Debug Device ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevid2(&self) -> &ApbaddrDbgCpu1Eddevid2 {
        &self.apbaddr_dbg_cpu1_eddevid2
    }
    #[doc = "0xfc4 - External Debug Device ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevid1(&self) -> &ApbaddrDbgCpu1Eddevid1 {
        &self.apbaddr_dbg_cpu1_eddevid1
    }
    #[doc = "0xfc8 - External Debug Device ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevid(&self) -> &ApbaddrDbgCpu1Eddevid {
        &self.apbaddr_dbg_cpu1_eddevid
    }
    #[doc = "0xfcc - External Debug Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_eddevtype(&self) -> &ApbaddrDbgCpu1Eddevtype {
        &self.apbaddr_dbg_cpu1_eddevtype
    }
    #[doc = "0xfd0 - External Debug Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpidr4(&self) -> &ApbaddrDbgCpu1Edpidr4 {
        &self.apbaddr_dbg_cpu1_edpidr4
    }
    #[doc = "0xfe0 - External Debug Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpidr0(&self) -> &ApbaddrDbgCpu1Edpidr0 {
        &self.apbaddr_dbg_cpu1_edpidr0
    }
    #[doc = "0xfe4 - External Debug Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpidr1(&self) -> &ApbaddrDbgCpu1Edpidr1 {
        &self.apbaddr_dbg_cpu1_edpidr1
    }
    #[doc = "0xfe8 - External Debug Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpidr2(&self) -> &ApbaddrDbgCpu1Edpidr2 {
        &self.apbaddr_dbg_cpu1_edpidr2
    }
    #[doc = "0xfec - External Debug Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edpidr3(&self) -> &ApbaddrDbgCpu1Edpidr3 {
        &self.apbaddr_dbg_cpu1_edpidr3
    }
    #[doc = "0xff0 - External Debug Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edcidr0(&self) -> &ApbaddrDbgCpu1Edcidr0 {
        &self.apbaddr_dbg_cpu1_edcidr0
    }
    #[doc = "0xff4 - External Debug Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edcidr1(&self) -> &ApbaddrDbgCpu1Edcidr1 {
        &self.apbaddr_dbg_cpu1_edcidr1
    }
    #[doc = "0xff8 - External Debug Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edcidr2(&self) -> &ApbaddrDbgCpu1Edcidr2 {
        &self.apbaddr_dbg_cpu1_edcidr2
    }
    #[doc = "0xffc - External Debug Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu1_edcidr3(&self) -> &ApbaddrDbgCpu1Edcidr3 {
        &self.apbaddr_dbg_cpu1_edcidr3
    }
}
#[doc = "APBADDR_DBG_CPU1_EDESR (rw) register accessor: External Debug Event Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edesr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edesr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edesr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDESR")]
pub type ApbaddrDbgCpu1Edesr = crate::Reg<apbaddr_dbg_cpu1_edesr::ApbaddrDbgCpu1EdesrSpec>;
#[doc = "External Debug Event Status Register"]
pub mod apbaddr_dbg_cpu1_edesr;
#[doc = "APBADDR_DBG_CPU1_EDECR (rw) register accessor: External Debug Execution Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edecr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDECR")]
pub type ApbaddrDbgCpu1Edecr = crate::Reg<apbaddr_dbg_cpu1_edecr::ApbaddrDbgCpu1EdecrSpec>;
#[doc = "External Debug Execution Control Register"]
pub mod apbaddr_dbg_cpu1_edecr;
#[doc = "APBADDR_DBG_CPU1_EDWAR_31_0 (rw) register accessor: External Debug Watchpoint Address Register (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edwar_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edwar_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edwar_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDWAR_31_0")]
pub type ApbaddrDbgCpu1Edwar31_0 =
    crate::Reg<apbaddr_dbg_cpu1_edwar_31_0::ApbaddrDbgCpu1Edwar31_0Spec>;
#[doc = "External Debug Watchpoint Address Register (low word)"]
pub mod apbaddr_dbg_cpu1_edwar_31_0;
#[doc = "APBADDR_DBG_CPU1_EDWAR_63_32 (rw) register accessor: External Debug Watchpoint Address Register (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edwar_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edwar_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edwar_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDWAR_63_32")]
pub type ApbaddrDbgCpu1Edwar63_32 =
    crate::Reg<apbaddr_dbg_cpu1_edwar_63_32::ApbaddrDbgCpu1Edwar63_32Spec>;
#[doc = "External Debug Watchpoint Address Register (high word)"]
pub mod apbaddr_dbg_cpu1_edwar_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGDTRRX_EL0 (rw) register accessor: Debug Data Transfer Register Receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgdtrrx_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgdtrrx_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgdtrrx_el0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGDTRRX_EL0")]
pub type ApbaddrDbgCpu1DbgdtrrxEl0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgdtrrx_el0::ApbaddrDbgCpu1DbgdtrrxEl0Spec>;
#[doc = "Debug Data Transfer Register Receive"]
pub mod apbaddr_dbg_cpu1_dbgdtrrx_el0;
#[doc = "APBADDR_DBG_CPU1_EDITR (rw) register accessor: External Debug Instruction Transfer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_editr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_editr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_editr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDITR")]
pub type ApbaddrDbgCpu1Editr = crate::Reg<apbaddr_dbg_cpu1_editr::ApbaddrDbgCpu1EditrSpec>;
#[doc = "External Debug Instruction Transfer Register"]
pub mod apbaddr_dbg_cpu1_editr;
#[doc = "APBADDR_DBG_CPU1_EDSCR (rw) register accessor: External Debug Status and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edscr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDSCR")]
pub type ApbaddrDbgCpu1Edscr = crate::Reg<apbaddr_dbg_cpu1_edscr::ApbaddrDbgCpu1EdscrSpec>;
#[doc = "External Debug Status and Control Register"]
pub mod apbaddr_dbg_cpu1_edscr;
#[doc = "APBADDR_DBG_CPU1_DBGDTRTX_EL0 (rw) register accessor: Debug Data Transfer Register Transmit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgdtrtx_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgdtrtx_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgdtrtx_el0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGDTRTX_EL0")]
pub type ApbaddrDbgCpu1DbgdtrtxEl0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgdtrtx_el0::ApbaddrDbgCpu1DbgdtrtxEl0Spec>;
#[doc = "Debug Data Transfer Register Transmit"]
pub mod apbaddr_dbg_cpu1_dbgdtrtx_el0;
#[doc = "APBADDR_DBG_CPU1_EDRCR (rw) register accessor: External Debug Reserve Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edrcr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDRCR")]
pub type ApbaddrDbgCpu1Edrcr = crate::Reg<apbaddr_dbg_cpu1_edrcr::ApbaddrDbgCpu1EdrcrSpec>;
#[doc = "External Debug Reserve Control Register"]
pub mod apbaddr_dbg_cpu1_edrcr;
#[doc = "APBADDR_DBG_CPU1_EDACR (rw) register accessor: External Debug Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edacr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDACR")]
pub type ApbaddrDbgCpu1Edacr = crate::Reg<apbaddr_dbg_cpu1_edacr::ApbaddrDbgCpu1EdacrSpec>;
#[doc = "External Debug Auxiliary Control Register"]
pub mod apbaddr_dbg_cpu1_edacr;
#[doc = "APBADDR_DBG_CPU1_EDECCR (rw) register accessor: External Debug Exception Catch Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edeccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edeccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edeccr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDECCR")]
pub type ApbaddrDbgCpu1Edeccr = crate::Reg<apbaddr_dbg_cpu1_edeccr::ApbaddrDbgCpu1EdeccrSpec>;
#[doc = "External Debug Exception Catch Control Register"]
pub mod apbaddr_dbg_cpu1_edeccr;
#[doc = "APBADDR_DBG_CPU1_EDPCSR_31_0 (rw) register accessor: External Debug Program Counter Sample Register (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpcsr_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpcsr_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpcsr_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPCSR_31_0")]
pub type ApbaddrDbgCpu1Edpcsr31_0 =
    crate::Reg<apbaddr_dbg_cpu1_edpcsr_31_0::ApbaddrDbgCpu1Edpcsr31_0Spec>;
#[doc = "External Debug Program Counter Sample Register (low word)"]
pub mod apbaddr_dbg_cpu1_edpcsr_31_0;
#[doc = "APBADDR_DBG_CPU1_EDCIDSR (rw) register accessor: External Debug Context ID Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edcidsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDCIDSR")]
pub type ApbaddrDbgCpu1Edcidsr = crate::Reg<apbaddr_dbg_cpu1_edcidsr::ApbaddrDbgCpu1EdcidsrSpec>;
#[doc = "External Debug Context ID Sample Register"]
pub mod apbaddr_dbg_cpu1_edcidsr;
#[doc = "APBADDR_DBG_CPU1_EDVIDSR (rw) register accessor: External Debug Virtual Context Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edvidsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edvidsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edvidsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDVIDSR")]
pub type ApbaddrDbgCpu1Edvidsr = crate::Reg<apbaddr_dbg_cpu1_edvidsr::ApbaddrDbgCpu1EdvidsrSpec>;
#[doc = "External Debug Virtual Context Sample Register"]
pub mod apbaddr_dbg_cpu1_edvidsr;
#[doc = "APBADDR_DBG_CPU1_EDPCSR_63_32 (rw) register accessor: External Debug Program Counter Sample Register (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpcsr_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpcsr_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpcsr_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPCSR_63_32")]
pub type ApbaddrDbgCpu1Edpcsr63_32 =
    crate::Reg<apbaddr_dbg_cpu1_edpcsr_63_32::ApbaddrDbgCpu1Edpcsr63_32Spec>;
#[doc = "External Debug Program Counter Sample Register (high word)"]
pub mod apbaddr_dbg_cpu1_edpcsr_63_32;
#[doc = "APBADDR_DBG_CPU1_OSLAR_EL1 (rw) register accessor: OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_oslar_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_oslar_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_oslar_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_OSLAR_EL1")]
pub type ApbaddrDbgCpu1OslarEl1 =
    crate::Reg<apbaddr_dbg_cpu1_oslar_el1::ApbaddrDbgCpu1OslarEl1Spec>;
#[doc = "OS Lock Access Register"]
pub mod apbaddr_dbg_cpu1_oslar_el1;
#[doc = "APBADDR_DBG_CPU1_EDPRCR (rw) register accessor: External Debug Power/Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edprcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edprcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edprcr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPRCR")]
pub type ApbaddrDbgCpu1Edprcr = crate::Reg<apbaddr_dbg_cpu1_edprcr::ApbaddrDbgCpu1EdprcrSpec>;
#[doc = "External Debug Power/Reset Control Register"]
pub mod apbaddr_dbg_cpu1_edprcr;
#[doc = "APBADDR_DBG_CPU1_EDPRSR (rw) register accessor: External Debug Processor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edprsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edprsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edprsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPRSR")]
pub type ApbaddrDbgCpu1Edprsr = crate::Reg<apbaddr_dbg_cpu1_edprsr::ApbaddrDbgCpu1EdprsrSpec>;
#[doc = "External Debug Processor Status Register"]
pub mod apbaddr_dbg_cpu1_edprsr;
#[doc = "APBADDR_DBG_CPU1_DBGBVR0_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR0_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr0_el1_31_0::ApbaddrDbgCpu1Dbgbvr0El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR0_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR0_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr0_el1_63_32::ApbaddrDbgCpu1Dbgbvr0El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR0_EL1 (rw) register accessor: Debug Breakpoint Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr0_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr0_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr0_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR0_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr0El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr0_el1::ApbaddrDbgCpu1Dbgbcr0El1Spec>;
#[doc = "Debug Breakpoint Control Register 0"]
pub mod apbaddr_dbg_cpu1_dbgbcr0_el1;
#[doc = "APBADDR_DBG_CPU1_DBGBVR1_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR1_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr1_el1_31_0::ApbaddrDbgCpu1Dbgbvr1El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR1_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR1_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr1_el1_63_32::ApbaddrDbgCpu1Dbgbvr1El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR1_EL1 (rw) register accessor: Debug Breakpoint Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr1_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr1_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr1_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR1_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr1El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr1_el1::ApbaddrDbgCpu1Dbgbcr1El1Spec>;
#[doc = "Debug Breakpoint Control Register 1"]
pub mod apbaddr_dbg_cpu1_dbgbcr1_el1;
#[doc = "APBADDR_DBG_CPU1_DBGBVR2_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr2_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr2_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr2_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR2_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr2El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr2_el1_31_0::ApbaddrDbgCpu1Dbgbvr2El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr2_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR2_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr2_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr2_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr2_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR2_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr2El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr2_el1_63_32::ApbaddrDbgCpu1Dbgbvr2El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr2_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR2_EL1 (rw) register accessor: Debug Breakpoint Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr2_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr2_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr2_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR2_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr2El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr2_el1::ApbaddrDbgCpu1Dbgbcr2El1Spec>;
#[doc = "Debug Breakpoint Control Register 2"]
pub mod apbaddr_dbg_cpu1_dbgbcr2_el1;
#[doc = "APBADDR_DBG_CPU1_DBGBVR3_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr3_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr3_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr3_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR3_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr3El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr3_el1_31_0::ApbaddrDbgCpu1Dbgbvr3El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr3_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR3_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr3_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr3_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr3_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR3_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr3El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr3_el1_63_32::ApbaddrDbgCpu1Dbgbvr3El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr3_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR3_EL1 (rw) register accessor: Debug Breakpoint Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr3_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr3_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr3_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR3_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr3El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr3_el1::ApbaddrDbgCpu1Dbgbcr3El1Spec>;
#[doc = "Debug Breakpoint Control Register 3"]
pub mod apbaddr_dbg_cpu1_dbgbcr3_el1;
#[doc = "APBADDR_DBG_CPU1_DBGBVR4_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr4_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr4_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr4_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR4_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr4El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr4_el1_31_0::ApbaddrDbgCpu1Dbgbvr4El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr4_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR4_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr4_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr4_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr4_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR4_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr4El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr4_el1_63_32::ApbaddrDbgCpu1Dbgbvr4El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr4_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR4_EL1 (rw) register accessor: Debug Breakpoint Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr4_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr4_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr4_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR4_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr4El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr4_el1::ApbaddrDbgCpu1Dbgbcr4El1Spec>;
#[doc = "Debug Breakpoint Control Register 4"]
pub mod apbaddr_dbg_cpu1_dbgbcr4_el1;
#[doc = "APBADDR_DBG_CPU1_DBGBVR5_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr5_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr5_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr5_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR5_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgbvr5El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr5_el1_31_0::ApbaddrDbgCpu1Dbgbvr5El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr5_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGBVR5_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbvr5_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbvr5_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbvr5_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBVR5_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgbvr5El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbvr5_el1_63_32::ApbaddrDbgCpu1Dbgbvr5El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu1_dbgbvr5_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGBCR5_EL1 (rw) register accessor: Debug Breakpoint Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgbcr5_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgbcr5_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgbcr5_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGBCR5_EL1")]
pub type ApbaddrDbgCpu1Dbgbcr5El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgbcr5_el1::ApbaddrDbgCpu1Dbgbcr5El1Spec>;
#[doc = "Debug Breakpoint Control Register 5"]
pub mod apbaddr_dbg_cpu1_dbgbcr5_el1;
#[doc = "APBADDR_DBG_CPU1_DBGWVR0_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR0_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgwvr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr0_el1_31_0::ApbaddrDbgCpu1Dbgwvr0El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 0"]
pub mod apbaddr_dbg_cpu1_dbgwvr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGWVR0_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR0_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgwvr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr0_el1_63_32::ApbaddrDbgCpu1Dbgwvr0El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 0"]
pub mod apbaddr_dbg_cpu1_dbgwvr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGWCR0_EL1 (rw) register accessor: Debug Watchpoint Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwcr0_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwcr0_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwcr0_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWCR0_EL1")]
pub type ApbaddrDbgCpu1Dbgwcr0El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwcr0_el1::ApbaddrDbgCpu1Dbgwcr0El1Spec>;
#[doc = "Debug Watchpoint Control Register 0"]
pub mod apbaddr_dbg_cpu1_dbgwcr0_el1;
#[doc = "APBADDR_DBG_CPU1_DBGWVR1_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR1_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgwvr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr1_el1_31_0::ApbaddrDbgCpu1Dbgwvr1El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 1"]
pub mod apbaddr_dbg_cpu1_dbgwvr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGWVR1_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR1_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgwvr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr1_el1_63_32::ApbaddrDbgCpu1Dbgwvr1El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 1"]
pub mod apbaddr_dbg_cpu1_dbgwvr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGWCR1_EL1 (rw) register accessor: Debug Watchpoint Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwcr1_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwcr1_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwcr1_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWCR1_EL1")]
pub type ApbaddrDbgCpu1Dbgwcr1El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwcr1_el1::ApbaddrDbgCpu1Dbgwcr1El1Spec>;
#[doc = "Debug Watchpoint Control Register 1"]
pub mod apbaddr_dbg_cpu1_dbgwcr1_el1;
#[doc = "APBADDR_DBG_CPU1_DBGWVR2_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr2_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr2_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr2_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR2_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgwvr2El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr2_el1_31_0::ApbaddrDbgCpu1Dbgwvr2El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 2"]
pub mod apbaddr_dbg_cpu1_dbgwvr2_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGWVR2_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr2_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr2_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr2_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR2_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgwvr2El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr2_el1_63_32::ApbaddrDbgCpu1Dbgwvr2El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 2"]
pub mod apbaddr_dbg_cpu1_dbgwvr2_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGWCR2_EL1 (rw) register accessor: Debug Watchpoint Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwcr2_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwcr2_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwcr2_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWCR2_EL1")]
pub type ApbaddrDbgCpu1Dbgwcr2El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwcr2_el1::ApbaddrDbgCpu1Dbgwcr2El1Spec>;
#[doc = "Debug Watchpoint Control Register 2"]
pub mod apbaddr_dbg_cpu1_dbgwcr2_el1;
#[doc = "APBADDR_DBG_CPU1_DBGWVR3_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr3_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr3_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr3_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR3_EL1_31_0")]
pub type ApbaddrDbgCpu1Dbgwvr3El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr3_el1_31_0::ApbaddrDbgCpu1Dbgwvr3El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 3"]
pub mod apbaddr_dbg_cpu1_dbgwvr3_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_DBGWVR3_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwvr3_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwvr3_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwvr3_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWVR3_EL1_63_32")]
pub type ApbaddrDbgCpu1Dbgwvr3El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwvr3_el1_63_32::ApbaddrDbgCpu1Dbgwvr3El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 3"]
pub mod apbaddr_dbg_cpu1_dbgwvr3_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_DBGWCR3_EL1 (rw) register accessor: Debug Watchpoint Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgwcr3_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgwcr3_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgwcr3_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGWCR3_EL1")]
pub type ApbaddrDbgCpu1Dbgwcr3El1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgwcr3_el1::ApbaddrDbgCpu1Dbgwcr3El1Spec>;
#[doc = "Debug Watchpoint Control Register 3"]
pub mod apbaddr_dbg_cpu1_dbgwcr3_el1;
#[doc = "APBADDR_DBG_CPU1_MIDR_EL1 (rw) register accessor: Main ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_midr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_midr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_midr_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_MIDR_EL1")]
pub type ApbaddrDbgCpu1MidrEl1 = crate::Reg<apbaddr_dbg_cpu1_midr_el1::ApbaddrDbgCpu1MidrEl1Spec>;
#[doc = "Main ID Register"]
pub mod apbaddr_dbg_cpu1_midr_el1;
#[doc = "APBADDR_DBG_CPU1_ID_AA64PFR0_EL1_31_0 (rw) register accessor: Processor Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64PFR0_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64pfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0::ApbaddrDbgCpu1IdAa64pfr0El1_31_0Spec>;
#[doc = "Processor Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64pfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64PFR0_EL1_63_32 (rw) register accessor: Processor Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64PFR0_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64pfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32::ApbaddrDbgCpu1IdAa64pfr0El1_63_32Spec>;
#[doc = "Processor Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64pfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_31_0 (rw) register accessor: Debug Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64dfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0::ApbaddrDbgCpu1IdAa64dfr0El1_31_0Spec>;
#[doc = "Debug Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64dfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_63_32 (rw) register accessor: Debug Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64DFR0_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64dfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32::ApbaddrDbgCpu1IdAa64dfr0El1_63_32Spec>;
#[doc = "Debug Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64dfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64ISAR0_EL1_31_0 (rw) register accessor: Instruction Set Attribute Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64ISAR0_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64isar0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0::ApbaddrDbgCpu1IdAa64isar0El1_31_0Spec>;
#[doc = "Instruction Set Attribute Register 0 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64isar0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64ISAR0_EL1_63_32 (rw) register accessor: Instruction Set Attribute Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64ISAR0_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64isar0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32::ApbaddrDbgCpu1IdAa64isar0El1_63_32Spec>;
#[doc = "Instruction Set Attribute Register 0 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64isar0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_31_0 (rw) register accessor: Memory Model Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64mmfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0::ApbaddrDbgCpu1IdAa64mmfr0El1_31_0Spec>;
#[doc = "Memory Model Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64mmfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_63_32 (rw) register accessor: Memory Model Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64MMFR0_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64mmfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32::ApbaddrDbgCpu1IdAa64mmfr0El1_63_32Spec>;
#[doc = "Memory Model Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64mmfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64PFR1_EL1_31_0 (rw) register accessor: Processor Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64PFR1_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64pfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0::ApbaddrDbgCpu1IdAa64pfr1El1_31_0Spec>;
#[doc = "Processor Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64pfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64PFR1_EL1_63_32 (rw) register accessor: Processor Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64PFR1_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64pfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32::ApbaddrDbgCpu1IdAa64pfr1El1_63_32Spec>;
#[doc = "Processor Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64pfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64DFR1_EL1_31_0 (rw) register accessor: Auxiliary Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64DFR1_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64dfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0::ApbaddrDbgCpu1IdAa64dfr1El1_31_0Spec>;
#[doc = "Auxiliary Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64dfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64DFR1_EL1_63_32 (rw) register accessor: Auxiliary Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64DFR1_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64dfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32::ApbaddrDbgCpu1IdAa64dfr1El1_63_32Spec>;
#[doc = "Auxiliary Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64dfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64ISAR1_EL1_31_0 (rw) register accessor: Instruction Set Attribute Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64ISAR1_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64isar1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0::ApbaddrDbgCpu1IdAa64isar1El1_31_0Spec>;
#[doc = "Instruction Set Attribute Register 1 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64isar1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64ISAR1_EL1_63_32 (rw) register accessor: Instruction Set Attribute Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64ISAR1_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64isar1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32::ApbaddrDbgCpu1IdAa64isar1El1_63_32Spec>;
#[doc = "Instruction Set Attribute Register 1 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64isar1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_ID_AA64MMFR1_EL1_31_0 (rw) register accessor: Memory Model Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64MMFR1_EL1_31_0")]
pub type ApbaddrDbgCpu1IdAa64mmfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0::ApbaddrDbgCpu1IdAa64mmfr1El1_31_0Spec>;
#[doc = "Memory Model Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu1_id_aa64mmfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU1_ID_AA64MMFR1_EL1_63_32 (rw) register accessor: Memory Model Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_ID_AA64MMFR1_EL1_63_32")]
pub type ApbaddrDbgCpu1IdAa64mmfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32::ApbaddrDbgCpu1IdAa64mmfr1El1_63_32Spec>;
#[doc = "Memory Model Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu1_id_aa64mmfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU1_EDITCTRL (rw) register accessor: External Debug Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_editctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_editctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_editctrl`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDITCTRL")]
pub type ApbaddrDbgCpu1Editctrl = crate::Reg<apbaddr_dbg_cpu1_editctrl::ApbaddrDbgCpu1EditctrlSpec>;
#[doc = "External Debug Integration mode Control Register"]
pub mod apbaddr_dbg_cpu1_editctrl;
#[doc = "APBADDR_DBG_CPU1_DBGCLAIMSET_EL1 (rw) register accessor: Debug Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgclaimset_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgclaimset_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgclaimset_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGCLAIMSET_EL1")]
pub type ApbaddrDbgCpu1DbgclaimsetEl1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgclaimset_el1::ApbaddrDbgCpu1DbgclaimsetEl1Spec>;
#[doc = "Debug Claim Tag Set Register"]
pub mod apbaddr_dbg_cpu1_dbgclaimset_el1;
#[doc = "APBADDR_DBG_CPU1_DBGCLAIMCLR_EL1 (rw) register accessor: Debug Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgclaimclr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgclaimclr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgclaimclr_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGCLAIMCLR_EL1")]
pub type ApbaddrDbgCpu1DbgclaimclrEl1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgclaimclr_el1::ApbaddrDbgCpu1DbgclaimclrEl1Spec>;
#[doc = "Debug Claim Tag Clear Register"]
pub mod apbaddr_dbg_cpu1_dbgclaimclr_el1;
#[doc = "APBADDR_DBG_CPU1_EDDEVAFF0 (rw) register accessor: External Debug Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevaff0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVAFF0")]
pub type ApbaddrDbgCpu1Eddevaff0 =
    crate::Reg<apbaddr_dbg_cpu1_eddevaff0::ApbaddrDbgCpu1Eddevaff0Spec>;
#[doc = "External Debug Device Affinity Register 0"]
pub mod apbaddr_dbg_cpu1_eddevaff0;
#[doc = "APBADDR_DBG_CPU1_EDDEVAFF1 (rw) register accessor: External Debug Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevaff1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVAFF1")]
pub type ApbaddrDbgCpu1Eddevaff1 =
    crate::Reg<apbaddr_dbg_cpu1_eddevaff1::ApbaddrDbgCpu1Eddevaff1Spec>;
#[doc = "External Debug Device Affinity Register 1"]
pub mod apbaddr_dbg_cpu1_eddevaff1;
#[doc = "APBADDR_DBG_CPU1_EDLAR (rw) register accessor: External Debug Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edlar`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDLAR")]
pub type ApbaddrDbgCpu1Edlar = crate::Reg<apbaddr_dbg_cpu1_edlar::ApbaddrDbgCpu1EdlarSpec>;
#[doc = "External Debug Lock Access Register"]
pub mod apbaddr_dbg_cpu1_edlar;
#[doc = "APBADDR_DBG_CPU1_EDLSR (rw) register accessor: External Debug Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edlsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edlsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edlsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDLSR")]
pub type ApbaddrDbgCpu1Edlsr = crate::Reg<apbaddr_dbg_cpu1_edlsr::ApbaddrDbgCpu1EdlsrSpec>;
#[doc = "External Debug Lock Status Register"]
pub mod apbaddr_dbg_cpu1_edlsr;
#[doc = "APBADDR_DBG_CPU1_DBGAUTHSTATUS_EL1 (rw) register accessor: Debug Authentication Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_dbgauthstatus_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_dbgauthstatus_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_dbgauthstatus_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_DBGAUTHSTATUS_EL1")]
pub type ApbaddrDbgCpu1DbgauthstatusEl1 =
    crate::Reg<apbaddr_dbg_cpu1_dbgauthstatus_el1::ApbaddrDbgCpu1DbgauthstatusEl1Spec>;
#[doc = "Debug Authentication Status register"]
pub mod apbaddr_dbg_cpu1_dbgauthstatus_el1;
#[doc = "APBADDR_DBG_CPU1_EDDEVARCH (rw) register accessor: External Debug Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevarch`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVARCH")]
pub type ApbaddrDbgCpu1Eddevarch =
    crate::Reg<apbaddr_dbg_cpu1_eddevarch::ApbaddrDbgCpu1EddevarchSpec>;
#[doc = "External Debug Device Architecture Register"]
pub mod apbaddr_dbg_cpu1_eddevarch;
#[doc = "APBADDR_DBG_CPU1_EDDEVID2 (rw) register accessor: External Debug Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevid2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVID2")]
pub type ApbaddrDbgCpu1Eddevid2 = crate::Reg<apbaddr_dbg_cpu1_eddevid2::ApbaddrDbgCpu1Eddevid2Spec>;
#[doc = "External Debug Device ID Register 2"]
pub mod apbaddr_dbg_cpu1_eddevid2;
#[doc = "APBADDR_DBG_CPU1_EDDEVID1 (rw) register accessor: External Debug Device ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevid1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVID1")]
pub type ApbaddrDbgCpu1Eddevid1 = crate::Reg<apbaddr_dbg_cpu1_eddevid1::ApbaddrDbgCpu1Eddevid1Spec>;
#[doc = "External Debug Device ID Register 1"]
pub mod apbaddr_dbg_cpu1_eddevid1;
#[doc = "APBADDR_DBG_CPU1_EDDEVID (rw) register accessor: External Debug Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevid`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVID")]
pub type ApbaddrDbgCpu1Eddevid = crate::Reg<apbaddr_dbg_cpu1_eddevid::ApbaddrDbgCpu1EddevidSpec>;
#[doc = "External Debug Device ID Register 0"]
pub mod apbaddr_dbg_cpu1_eddevid;
#[doc = "APBADDR_DBG_CPU1_EDDEVTYPE (rw) register accessor: External Debug Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_eddevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_eddevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_eddevtype`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDDEVTYPE")]
pub type ApbaddrDbgCpu1Eddevtype =
    crate::Reg<apbaddr_dbg_cpu1_eddevtype::ApbaddrDbgCpu1EddevtypeSpec>;
#[doc = "External Debug Device Type Register"]
pub mod apbaddr_dbg_cpu1_eddevtype;
#[doc = "APBADDR_DBG_CPU1_EDPIDR4 (rw) register accessor: External Debug Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpidr4`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPIDR4")]
pub type ApbaddrDbgCpu1Edpidr4 = crate::Reg<apbaddr_dbg_cpu1_edpidr4::ApbaddrDbgCpu1Edpidr4Spec>;
#[doc = "External Debug Peripheral Identification Register 4"]
pub mod apbaddr_dbg_cpu1_edpidr4;
#[doc = "APBADDR_DBG_CPU1_EDPIDR0 (rw) register accessor: External Debug Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpidr0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPIDR0")]
pub type ApbaddrDbgCpu1Edpidr0 = crate::Reg<apbaddr_dbg_cpu1_edpidr0::ApbaddrDbgCpu1Edpidr0Spec>;
#[doc = "External Debug Peripheral Identification Register 0"]
pub mod apbaddr_dbg_cpu1_edpidr0;
#[doc = "APBADDR_DBG_CPU1_EDPIDR1 (rw) register accessor: External Debug Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpidr1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPIDR1")]
pub type ApbaddrDbgCpu1Edpidr1 = crate::Reg<apbaddr_dbg_cpu1_edpidr1::ApbaddrDbgCpu1Edpidr1Spec>;
#[doc = "External Debug Peripheral Identification Register 1"]
pub mod apbaddr_dbg_cpu1_edpidr1;
#[doc = "APBADDR_DBG_CPU1_EDPIDR2 (rw) register accessor: External Debug Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpidr2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPIDR2")]
pub type ApbaddrDbgCpu1Edpidr2 = crate::Reg<apbaddr_dbg_cpu1_edpidr2::ApbaddrDbgCpu1Edpidr2Spec>;
#[doc = "External Debug Peripheral Identification Register 2"]
pub mod apbaddr_dbg_cpu1_edpidr2;
#[doc = "APBADDR_DBG_CPU1_EDPIDR3 (rw) register accessor: External Debug Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edpidr3`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDPIDR3")]
pub type ApbaddrDbgCpu1Edpidr3 = crate::Reg<apbaddr_dbg_cpu1_edpidr3::ApbaddrDbgCpu1Edpidr3Spec>;
#[doc = "External Debug Peripheral Identification Register 3"]
pub mod apbaddr_dbg_cpu1_edpidr3;
#[doc = "APBADDR_DBG_CPU1_EDCIDR0 (rw) register accessor: External Debug Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edcidr0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDCIDR0")]
pub type ApbaddrDbgCpu1Edcidr0 = crate::Reg<apbaddr_dbg_cpu1_edcidr0::ApbaddrDbgCpu1Edcidr0Spec>;
#[doc = "External Debug Component Identification Register 0"]
pub mod apbaddr_dbg_cpu1_edcidr0;
#[doc = "APBADDR_DBG_CPU1_EDCIDR1 (rw) register accessor: External Debug Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edcidr1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDCIDR1")]
pub type ApbaddrDbgCpu1Edcidr1 = crate::Reg<apbaddr_dbg_cpu1_edcidr1::ApbaddrDbgCpu1Edcidr1Spec>;
#[doc = "External Debug Component Identification Register 1"]
pub mod apbaddr_dbg_cpu1_edcidr1;
#[doc = "APBADDR_DBG_CPU1_EDCIDR2 (rw) register accessor: External Debug Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edcidr2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDCIDR2")]
pub type ApbaddrDbgCpu1Edcidr2 = crate::Reg<apbaddr_dbg_cpu1_edcidr2::ApbaddrDbgCpu1Edcidr2Spec>;
#[doc = "External Debug Component Identification Register 2"]
pub mod apbaddr_dbg_cpu1_edcidr2;
#[doc = "APBADDR_DBG_CPU1_EDCIDR3 (rw) register accessor: External Debug Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu1_edcidr3`]
module"]
#[doc(alias = "APBADDR_DBG_CPU1_EDCIDR3")]
pub type ApbaddrDbgCpu1Edcidr3 = crate::Reg<apbaddr_dbg_cpu1_edcidr3::ApbaddrDbgCpu1Edcidr3Spec>;
#[doc = "External Debug Component Identification Register 3"]
pub mod apbaddr_dbg_cpu1_edcidr3;
