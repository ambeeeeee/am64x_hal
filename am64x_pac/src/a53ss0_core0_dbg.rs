#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    apbaddr_dbg_cpu0_edesr: ApbaddrDbgCpu0Edesr,
    apbaddr_dbg_cpu0_edecr: ApbaddrDbgCpu0Edecr,
    _reserved2: [u8; 0x08],
    apbaddr_dbg_cpu0_edwar_31_0: ApbaddrDbgCpu0Edwar31_0,
    apbaddr_dbg_cpu0_edwar_63_32: ApbaddrDbgCpu0Edwar63_32,
    _reserved4: [u8; 0x48],
    apbaddr_dbg_cpu0_dbgdtrrx_el0: ApbaddrDbgCpu0DbgdtrrxEl0,
    apbaddr_dbg_cpu0_editr: ApbaddrDbgCpu0Editr,
    apbaddr_dbg_cpu0_edscr: ApbaddrDbgCpu0Edscr,
    apbaddr_dbg_cpu0_dbgdtrtx_el0: ApbaddrDbgCpu0DbgdtrtxEl0,
    apbaddr_dbg_cpu0_edrcr: ApbaddrDbgCpu0Edrcr,
    apbaddr_dbg_cpu0_edacr: ApbaddrDbgCpu0Edacr,
    apbaddr_dbg_cpu0_edeccr: ApbaddrDbgCpu0Edeccr,
    _reserved11: [u8; 0x04],
    apbaddr_dbg_cpu0_edpcsr_31_0: ApbaddrDbgCpu0Edpcsr31_0,
    apbaddr_dbg_cpu0_edcidsr: ApbaddrDbgCpu0Edcidsr,
    apbaddr_dbg_cpu0_edvidsr: ApbaddrDbgCpu0Edvidsr,
    apbaddr_dbg_cpu0_edpcsr_63_32: ApbaddrDbgCpu0Edpcsr63_32,
    _reserved15: [u8; 0x0250],
    apbaddr_dbg_cpu0_oslar_el1: ApbaddrDbgCpu0OslarEl1,
    _reserved16: [u8; 0x0c],
    apbaddr_dbg_cpu0_edprcr: ApbaddrDbgCpu0Edprcr,
    apbaddr_dbg_cpu0_edprsr: ApbaddrDbgCpu0Edprsr,
    _reserved18: [u8; 0xe8],
    apbaddr_dbg_cpu0_dbgbvr0_el1_31_0: ApbaddrDbgCpu0Dbgbvr0El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr0_el1_63_32: ApbaddrDbgCpu0Dbgbvr0El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr0_el1: ApbaddrDbgCpu0Dbgbcr0El1,
    _reserved21: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgbvr1_el1_31_0: ApbaddrDbgCpu0Dbgbvr1El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr1_el1_63_32: ApbaddrDbgCpu0Dbgbvr1El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr1_el1: ApbaddrDbgCpu0Dbgbcr1El1,
    _reserved24: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgbvr2_el1_31_0: ApbaddrDbgCpu0Dbgbvr2El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr2_el1_63_32: ApbaddrDbgCpu0Dbgbvr2El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr2_el1: ApbaddrDbgCpu0Dbgbcr2El1,
    _reserved27: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgbvr3_el1_31_0: ApbaddrDbgCpu0Dbgbvr3El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr3_el1_63_32: ApbaddrDbgCpu0Dbgbvr3El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr3_el1: ApbaddrDbgCpu0Dbgbcr3El1,
    _reserved30: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgbvr4_el1_31_0: ApbaddrDbgCpu0Dbgbvr4El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr4_el1_63_32: ApbaddrDbgCpu0Dbgbvr4El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr4_el1: ApbaddrDbgCpu0Dbgbcr4El1,
    _reserved33: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgbvr5_el1_31_0: ApbaddrDbgCpu0Dbgbvr5El1_31_0,
    apbaddr_dbg_cpu0_dbgbvr5_el1_63_32: ApbaddrDbgCpu0Dbgbvr5El1_63_32,
    apbaddr_dbg_cpu0_dbgbcr5_el1: ApbaddrDbgCpu0Dbgbcr5El1,
    _reserved36: [u8; 0x03a4],
    apbaddr_dbg_cpu0_dbgwvr0_el1_31_0: ApbaddrDbgCpu0Dbgwvr0El1_31_0,
    apbaddr_dbg_cpu0_dbgwvr0_el1_63_32: ApbaddrDbgCpu0Dbgwvr0El1_63_32,
    apbaddr_dbg_cpu0_dbgwcr0_el1: ApbaddrDbgCpu0Dbgwcr0El1,
    _reserved39: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgwvr1_el1_31_0: ApbaddrDbgCpu0Dbgwvr1El1_31_0,
    apbaddr_dbg_cpu0_dbgwvr1_el1_63_32: ApbaddrDbgCpu0Dbgwvr1El1_63_32,
    apbaddr_dbg_cpu0_dbgwcr1_el1: ApbaddrDbgCpu0Dbgwcr1El1,
    _reserved42: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgwvr2_el1_31_0: ApbaddrDbgCpu0Dbgwvr2El1_31_0,
    apbaddr_dbg_cpu0_dbgwvr2_el1_63_32: ApbaddrDbgCpu0Dbgwvr2El1_63_32,
    apbaddr_dbg_cpu0_dbgwcr2_el1: ApbaddrDbgCpu0Dbgwcr2El1,
    _reserved45: [u8; 0x04],
    apbaddr_dbg_cpu0_dbgwvr3_el1_31_0: ApbaddrDbgCpu0Dbgwvr3El1_31_0,
    apbaddr_dbg_cpu0_dbgwvr3_el1_63_32: ApbaddrDbgCpu0Dbgwvr3El1_63_32,
    apbaddr_dbg_cpu0_dbgwcr3_el1: ApbaddrDbgCpu0Dbgwcr3El1,
    _reserved48: [u8; 0x04c4],
    apbaddr_dbg_cpu0_midr_el1: ApbaddrDbgCpu0MidrEl1,
    _reserved49: [u8; 0x1c],
    apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0: ApbaddrDbgCpu0IdAa64pfr0El1_31_0,
    apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32: ApbaddrDbgCpu0IdAa64pfr0El1_63_32,
    apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0: ApbaddrDbgCpu0IdAa64dfr0El1_31_0,
    apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32: ApbaddrDbgCpu0IdAa64dfr0El1_63_32,
    apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0: ApbaddrDbgCpu0IdAa64isar0El1_31_0,
    apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32: ApbaddrDbgCpu0IdAa64isar0El1_63_32,
    apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0: ApbaddrDbgCpu0IdAa64mmfr0El1_31_0,
    apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32: ApbaddrDbgCpu0IdAa64mmfr0El1_63_32,
    apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0: ApbaddrDbgCpu0IdAa64pfr1El1_31_0,
    apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32: ApbaddrDbgCpu0IdAa64pfr1El1_63_32,
    apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0: ApbaddrDbgCpu0IdAa64dfr1El1_31_0,
    apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32: ApbaddrDbgCpu0IdAa64dfr1El1_63_32,
    apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0: ApbaddrDbgCpu0IdAa64isar1El1_31_0,
    apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32: ApbaddrDbgCpu0IdAa64isar1El1_63_32,
    apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0: ApbaddrDbgCpu0IdAa64mmfr1El1_31_0,
    apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32: ApbaddrDbgCpu0IdAa64mmfr1El1_63_32,
    _reserved65: [u8; 0x01a0],
    apbaddr_dbg_cpu0_editctrl: ApbaddrDbgCpu0Editctrl,
    _reserved66: [u8; 0x9c],
    apbaddr_dbg_cpu0_dbgclaimset_el1: ApbaddrDbgCpu0DbgclaimsetEl1,
    apbaddr_dbg_cpu0_dbgclaimclr_el1: ApbaddrDbgCpu0DbgclaimclrEl1,
    apbaddr_dbg_cpu0_eddevaff0: ApbaddrDbgCpu0Eddevaff0,
    apbaddr_dbg_cpu0_eddevaff1: ApbaddrDbgCpu0Eddevaff1,
    apbaddr_dbg_cpu0_edlar: ApbaddrDbgCpu0Edlar,
    apbaddr_dbg_cpu0_edlsr: ApbaddrDbgCpu0Edlsr,
    apbaddr_dbg_cpu0_dbgauthstatus_el1: ApbaddrDbgCpu0DbgauthstatusEl1,
    apbaddr_dbg_cpu0_eddevarch: ApbaddrDbgCpu0Eddevarch,
    apbaddr_dbg_cpu0_eddevid2: ApbaddrDbgCpu0Eddevid2,
    apbaddr_dbg_cpu0_eddevid1: ApbaddrDbgCpu0Eddevid1,
    apbaddr_dbg_cpu0_eddevid: ApbaddrDbgCpu0Eddevid,
    apbaddr_dbg_cpu0_eddevtype: ApbaddrDbgCpu0Eddevtype,
    apbaddr_dbg_cpu0_edpidr4: ApbaddrDbgCpu0Edpidr4,
    _reserved79: [u8; 0x0c],
    apbaddr_dbg_cpu0_edpidr0: ApbaddrDbgCpu0Edpidr0,
    apbaddr_dbg_cpu0_edpidr1: ApbaddrDbgCpu0Edpidr1,
    apbaddr_dbg_cpu0_edpidr2: ApbaddrDbgCpu0Edpidr2,
    apbaddr_dbg_cpu0_edpidr3: ApbaddrDbgCpu0Edpidr3,
    apbaddr_dbg_cpu0_edcidr0: ApbaddrDbgCpu0Edcidr0,
    apbaddr_dbg_cpu0_edcidr1: ApbaddrDbgCpu0Edcidr1,
    apbaddr_dbg_cpu0_edcidr2: ApbaddrDbgCpu0Edcidr2,
    apbaddr_dbg_cpu0_edcidr3: ApbaddrDbgCpu0Edcidr3,
}
impl RegisterBlock {
    #[doc = "0x20 - External Debug Event Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edesr(&self) -> &ApbaddrDbgCpu0Edesr {
        &self.apbaddr_dbg_cpu0_edesr
    }
    #[doc = "0x24 - External Debug Execution Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edecr(&self) -> &ApbaddrDbgCpu0Edecr {
        &self.apbaddr_dbg_cpu0_edecr
    }
    #[doc = "0x30 - External Debug Watchpoint Address Register (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edwar_31_0(&self) -> &ApbaddrDbgCpu0Edwar31_0 {
        &self.apbaddr_dbg_cpu0_edwar_31_0
    }
    #[doc = "0x34 - External Debug Watchpoint Address Register (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edwar_63_32(&self) -> &ApbaddrDbgCpu0Edwar63_32 {
        &self.apbaddr_dbg_cpu0_edwar_63_32
    }
    #[doc = "0x80 - Debug Data Transfer Register Receive"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgdtrrx_el0(&self) -> &ApbaddrDbgCpu0DbgdtrrxEl0 {
        &self.apbaddr_dbg_cpu0_dbgdtrrx_el0
    }
    #[doc = "0x84 - External Debug Instruction Transfer Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_editr(&self) -> &ApbaddrDbgCpu0Editr {
        &self.apbaddr_dbg_cpu0_editr
    }
    #[doc = "0x88 - External Debug Status and Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edscr(&self) -> &ApbaddrDbgCpu0Edscr {
        &self.apbaddr_dbg_cpu0_edscr
    }
    #[doc = "0x8c - Debug Data Transfer Register Transmit"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgdtrtx_el0(&self) -> &ApbaddrDbgCpu0DbgdtrtxEl0 {
        &self.apbaddr_dbg_cpu0_dbgdtrtx_el0
    }
    #[doc = "0x90 - External Debug Reserve Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edrcr(&self) -> &ApbaddrDbgCpu0Edrcr {
        &self.apbaddr_dbg_cpu0_edrcr
    }
    #[doc = "0x94 - External Debug Auxiliary Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edacr(&self) -> &ApbaddrDbgCpu0Edacr {
        &self.apbaddr_dbg_cpu0_edacr
    }
    #[doc = "0x98 - External Debug Exception Catch Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edeccr(&self) -> &ApbaddrDbgCpu0Edeccr {
        &self.apbaddr_dbg_cpu0_edeccr
    }
    #[doc = "0xa0 - External Debug Program Counter Sample Register (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpcsr_31_0(&self) -> &ApbaddrDbgCpu0Edpcsr31_0 {
        &self.apbaddr_dbg_cpu0_edpcsr_31_0
    }
    #[doc = "0xa4 - External Debug Context ID Sample Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edcidsr(&self) -> &ApbaddrDbgCpu0Edcidsr {
        &self.apbaddr_dbg_cpu0_edcidsr
    }
    #[doc = "0xa8 - External Debug Virtual Context Sample Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edvidsr(&self) -> &ApbaddrDbgCpu0Edvidsr {
        &self.apbaddr_dbg_cpu0_edvidsr
    }
    #[doc = "0xac - External Debug Program Counter Sample Register (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpcsr_63_32(&self) -> &ApbaddrDbgCpu0Edpcsr63_32 {
        &self.apbaddr_dbg_cpu0_edpcsr_63_32
    }
    #[doc = "0x300 - OS Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_oslar_el1(&self) -> &ApbaddrDbgCpu0OslarEl1 {
        &self.apbaddr_dbg_cpu0_oslar_el1
    }
    #[doc = "0x310 - External Debug Power/Reset Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edprcr(&self) -> &ApbaddrDbgCpu0Edprcr {
        &self.apbaddr_dbg_cpu0_edprcr
    }
    #[doc = "0x314 - External Debug Processor Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edprsr(&self) -> &ApbaddrDbgCpu0Edprsr {
        &self.apbaddr_dbg_cpu0_edprsr
    }
    #[doc = "0x400 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr0_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr0El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr0_el1_31_0
    }
    #[doc = "0x404 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr0_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr0El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr0_el1_63_32
    }
    #[doc = "0x408 - Debug Breakpoint Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr0_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr0El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr0_el1
    }
    #[doc = "0x410 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr1_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr1El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr1_el1_31_0
    }
    #[doc = "0x414 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr1_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr1El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr1_el1_63_32
    }
    #[doc = "0x418 - Debug Breakpoint Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr1_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr1El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr1_el1
    }
    #[doc = "0x420 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr2_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr2El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr2_el1_31_0
    }
    #[doc = "0x424 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr2_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr2El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr2_el1_63_32
    }
    #[doc = "0x428 - Debug Breakpoint Control Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr2_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr2El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr2_el1
    }
    #[doc = "0x430 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr3_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr3El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr3_el1_31_0
    }
    #[doc = "0x434 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr3_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr3El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr3_el1_63_32
    }
    #[doc = "0x438 - Debug Breakpoint Control Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr3_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr3El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr3_el1
    }
    #[doc = "0x440 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr4_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr4El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr4_el1_31_0
    }
    #[doc = "0x444 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr4_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr4El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr4_el1_63_32
    }
    #[doc = "0x448 - Debug Breakpoint Control Register 4"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr4_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr4El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr4_el1
    }
    #[doc = "0x450 - Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr5_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgbvr5El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgbvr5_el1_31_0
    }
    #[doc = "0x454 - Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbvr5_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgbvr5El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgbvr5_el1_63_32
    }
    #[doc = "0x458 - Debug Breakpoint Control Register 5"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgbcr5_el1(&self) -> &ApbaddrDbgCpu0Dbgbcr5El1 {
        &self.apbaddr_dbg_cpu0_dbgbcr5_el1
    }
    #[doc = "0x800 - Debug Watchpoint Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr0_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgwvr0El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgwvr0_el1_31_0
    }
    #[doc = "0x804 - Debug Watchpoint Extended Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr0_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgwvr0El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgwvr0_el1_63_32
    }
    #[doc = "0x808 - Debug Watchpoint Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwcr0_el1(&self) -> &ApbaddrDbgCpu0Dbgwcr0El1 {
        &self.apbaddr_dbg_cpu0_dbgwcr0_el1
    }
    #[doc = "0x810 - Debug Watchpoint Value Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr1_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgwvr1El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgwvr1_el1_31_0
    }
    #[doc = "0x814 - Debug Watchpoint Extended Value Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr1_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgwvr1El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgwvr1_el1_63_32
    }
    #[doc = "0x818 - Debug Watchpoint Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwcr1_el1(&self) -> &ApbaddrDbgCpu0Dbgwcr1El1 {
        &self.apbaddr_dbg_cpu0_dbgwcr1_el1
    }
    #[doc = "0x820 - Debug Watchpoint Value Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr2_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgwvr2El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgwvr2_el1_31_0
    }
    #[doc = "0x824 - Debug Watchpoint Extended Value Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr2_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgwvr2El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgwvr2_el1_63_32
    }
    #[doc = "0x828 - Debug Watchpoint Control Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwcr2_el1(&self) -> &ApbaddrDbgCpu0Dbgwcr2El1 {
        &self.apbaddr_dbg_cpu0_dbgwcr2_el1
    }
    #[doc = "0x830 - Debug Watchpoint Value Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr3_el1_31_0(&self) -> &ApbaddrDbgCpu0Dbgwvr3El1_31_0 {
        &self.apbaddr_dbg_cpu0_dbgwvr3_el1_31_0
    }
    #[doc = "0x834 - Debug Watchpoint Extended Value Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwvr3_el1_63_32(&self) -> &ApbaddrDbgCpu0Dbgwvr3El1_63_32 {
        &self.apbaddr_dbg_cpu0_dbgwvr3_el1_63_32
    }
    #[doc = "0x838 - Debug Watchpoint Control Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgwcr3_el1(&self) -> &ApbaddrDbgCpu0Dbgwcr3El1 {
        &self.apbaddr_dbg_cpu0_dbgwcr3_el1
    }
    #[doc = "0xd00 - Main ID Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_midr_el1(&self) -> &ApbaddrDbgCpu0MidrEl1 {
        &self.apbaddr_dbg_cpu0_midr_el1
    }
    #[doc = "0xd20 - Processor Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0(&self) -> &ApbaddrDbgCpu0IdAa64pfr0El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0
    }
    #[doc = "0xd24 - Processor Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64pfr0El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32
    }
    #[doc = "0xd28 - Debug Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0(&self) -> &ApbaddrDbgCpu0IdAa64dfr0El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0
    }
    #[doc = "0xd2c - Debug Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64dfr0El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32
    }
    #[doc = "0xd30 - Instruction Set Attribute Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64isar0El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0
    }
    #[doc = "0xd34 - Instruction Set Attribute Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64isar0El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32
    }
    #[doc = "0xd38 - Memory Model Feature Register 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64mmfr0El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0
    }
    #[doc = "0xd3c - Memory Model Feature Register 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64mmfr0El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32
    }
    #[doc = "0xd40 - Processor Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0(&self) -> &ApbaddrDbgCpu0IdAa64pfr1El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0
    }
    #[doc = "0xd44 - Processor Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64pfr1El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32
    }
    #[doc = "0xd48 - Auxiliary Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0(&self) -> &ApbaddrDbgCpu0IdAa64dfr1El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0
    }
    #[doc = "0xd4c - Auxiliary Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64dfr1El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32
    }
    #[doc = "0xd50 - Instruction Set Attribute Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64isar1El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0
    }
    #[doc = "0xd54 - Instruction Set Attribute Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64isar1El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32
    }
    #[doc = "0xd58 - Memory Model Feature Register 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64mmfr1El1_31_0 {
        &self.apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0
    }
    #[doc = "0xd5c - Memory Model Feature Register 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32(
        &self,
    ) -> &ApbaddrDbgCpu0IdAa64mmfr1El1_63_32 {
        &self.apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32
    }
    #[doc = "0xf00 - External Debug Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_editctrl(&self) -> &ApbaddrDbgCpu0Editctrl {
        &self.apbaddr_dbg_cpu0_editctrl
    }
    #[doc = "0xfa0 - Debug Claim Tag Set Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgclaimset_el1(&self) -> &ApbaddrDbgCpu0DbgclaimsetEl1 {
        &self.apbaddr_dbg_cpu0_dbgclaimset_el1
    }
    #[doc = "0xfa4 - Debug Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgclaimclr_el1(&self) -> &ApbaddrDbgCpu0DbgclaimclrEl1 {
        &self.apbaddr_dbg_cpu0_dbgclaimclr_el1
    }
    #[doc = "0xfa8 - External Debug Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevaff0(&self) -> &ApbaddrDbgCpu0Eddevaff0 {
        &self.apbaddr_dbg_cpu0_eddevaff0
    }
    #[doc = "0xfac - External Debug Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevaff1(&self) -> &ApbaddrDbgCpu0Eddevaff1 {
        &self.apbaddr_dbg_cpu0_eddevaff1
    }
    #[doc = "0xfb0 - External Debug Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edlar(&self) -> &ApbaddrDbgCpu0Edlar {
        &self.apbaddr_dbg_cpu0_edlar
    }
    #[doc = "0xfb4 - External Debug Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edlsr(&self) -> &ApbaddrDbgCpu0Edlsr {
        &self.apbaddr_dbg_cpu0_edlsr
    }
    #[doc = "0xfb8 - Debug Authentication Status register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_dbgauthstatus_el1(&self) -> &ApbaddrDbgCpu0DbgauthstatusEl1 {
        &self.apbaddr_dbg_cpu0_dbgauthstatus_el1
    }
    #[doc = "0xfbc - External Debug Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevarch(&self) -> &ApbaddrDbgCpu0Eddevarch {
        &self.apbaddr_dbg_cpu0_eddevarch
    }
    #[doc = "0xfc0 - External Debug Device ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevid2(&self) -> &ApbaddrDbgCpu0Eddevid2 {
        &self.apbaddr_dbg_cpu0_eddevid2
    }
    #[doc = "0xfc4 - External Debug Device ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevid1(&self) -> &ApbaddrDbgCpu0Eddevid1 {
        &self.apbaddr_dbg_cpu0_eddevid1
    }
    #[doc = "0xfc8 - External Debug Device ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevid(&self) -> &ApbaddrDbgCpu0Eddevid {
        &self.apbaddr_dbg_cpu0_eddevid
    }
    #[doc = "0xfcc - External Debug Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_eddevtype(&self) -> &ApbaddrDbgCpu0Eddevtype {
        &self.apbaddr_dbg_cpu0_eddevtype
    }
    #[doc = "0xfd0 - External Debug Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpidr4(&self) -> &ApbaddrDbgCpu0Edpidr4 {
        &self.apbaddr_dbg_cpu0_edpidr4
    }
    #[doc = "0xfe0 - External Debug Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpidr0(&self) -> &ApbaddrDbgCpu0Edpidr0 {
        &self.apbaddr_dbg_cpu0_edpidr0
    }
    #[doc = "0xfe4 - External Debug Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpidr1(&self) -> &ApbaddrDbgCpu0Edpidr1 {
        &self.apbaddr_dbg_cpu0_edpidr1
    }
    #[doc = "0xfe8 - External Debug Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpidr2(&self) -> &ApbaddrDbgCpu0Edpidr2 {
        &self.apbaddr_dbg_cpu0_edpidr2
    }
    #[doc = "0xfec - External Debug Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edpidr3(&self) -> &ApbaddrDbgCpu0Edpidr3 {
        &self.apbaddr_dbg_cpu0_edpidr3
    }
    #[doc = "0xff0 - External Debug Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edcidr0(&self) -> &ApbaddrDbgCpu0Edcidr0 {
        &self.apbaddr_dbg_cpu0_edcidr0
    }
    #[doc = "0xff4 - External Debug Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edcidr1(&self) -> &ApbaddrDbgCpu0Edcidr1 {
        &self.apbaddr_dbg_cpu0_edcidr1
    }
    #[doc = "0xff8 - External Debug Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edcidr2(&self) -> &ApbaddrDbgCpu0Edcidr2 {
        &self.apbaddr_dbg_cpu0_edcidr2
    }
    #[doc = "0xffc - External Debug Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_dbg_cpu0_edcidr3(&self) -> &ApbaddrDbgCpu0Edcidr3 {
        &self.apbaddr_dbg_cpu0_edcidr3
    }
}
#[doc = "APBADDR_DBG_CPU0_EDESR (rw) register accessor: External Debug Event Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edesr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edesr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edesr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDESR")]
pub type ApbaddrDbgCpu0Edesr = crate::Reg<apbaddr_dbg_cpu0_edesr::ApbaddrDbgCpu0EdesrSpec>;
#[doc = "External Debug Event Status Register"]
pub mod apbaddr_dbg_cpu0_edesr;
#[doc = "APBADDR_DBG_CPU0_EDECR (rw) register accessor: External Debug Execution Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edecr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDECR")]
pub type ApbaddrDbgCpu0Edecr = crate::Reg<apbaddr_dbg_cpu0_edecr::ApbaddrDbgCpu0EdecrSpec>;
#[doc = "External Debug Execution Control Register"]
pub mod apbaddr_dbg_cpu0_edecr;
#[doc = "APBADDR_DBG_CPU0_EDWAR_31_0 (rw) register accessor: External Debug Watchpoint Address Register (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edwar_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edwar_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edwar_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDWAR_31_0")]
pub type ApbaddrDbgCpu0Edwar31_0 =
    crate::Reg<apbaddr_dbg_cpu0_edwar_31_0::ApbaddrDbgCpu0Edwar31_0Spec>;
#[doc = "External Debug Watchpoint Address Register (low word)"]
pub mod apbaddr_dbg_cpu0_edwar_31_0;
#[doc = "APBADDR_DBG_CPU0_EDWAR_63_32 (rw) register accessor: External Debug Watchpoint Address Register (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edwar_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edwar_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edwar_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDWAR_63_32")]
pub type ApbaddrDbgCpu0Edwar63_32 =
    crate::Reg<apbaddr_dbg_cpu0_edwar_63_32::ApbaddrDbgCpu0Edwar63_32Spec>;
#[doc = "External Debug Watchpoint Address Register (high word)"]
pub mod apbaddr_dbg_cpu0_edwar_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGDTRRX_EL0 (rw) register accessor: Debug Data Transfer Register Receive\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgdtrrx_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgdtrrx_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgdtrrx_el0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGDTRRX_EL0")]
pub type ApbaddrDbgCpu0DbgdtrrxEl0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgdtrrx_el0::ApbaddrDbgCpu0DbgdtrrxEl0Spec>;
#[doc = "Debug Data Transfer Register Receive"]
pub mod apbaddr_dbg_cpu0_dbgdtrrx_el0;
#[doc = "APBADDR_DBG_CPU0_EDITR (rw) register accessor: External Debug Instruction Transfer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_editr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_editr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_editr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDITR")]
pub type ApbaddrDbgCpu0Editr = crate::Reg<apbaddr_dbg_cpu0_editr::ApbaddrDbgCpu0EditrSpec>;
#[doc = "External Debug Instruction Transfer Register"]
pub mod apbaddr_dbg_cpu0_editr;
#[doc = "APBADDR_DBG_CPU0_EDSCR (rw) register accessor: External Debug Status and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edscr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDSCR")]
pub type ApbaddrDbgCpu0Edscr = crate::Reg<apbaddr_dbg_cpu0_edscr::ApbaddrDbgCpu0EdscrSpec>;
#[doc = "External Debug Status and Control Register"]
pub mod apbaddr_dbg_cpu0_edscr;
#[doc = "APBADDR_DBG_CPU0_DBGDTRTX_EL0 (rw) register accessor: Debug Data Transfer Register Transmit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgdtrtx_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgdtrtx_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgdtrtx_el0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGDTRTX_EL0")]
pub type ApbaddrDbgCpu0DbgdtrtxEl0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgdtrtx_el0::ApbaddrDbgCpu0DbgdtrtxEl0Spec>;
#[doc = "Debug Data Transfer Register Transmit"]
pub mod apbaddr_dbg_cpu0_dbgdtrtx_el0;
#[doc = "APBADDR_DBG_CPU0_EDRCR (rw) register accessor: External Debug Reserve Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edrcr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDRCR")]
pub type ApbaddrDbgCpu0Edrcr = crate::Reg<apbaddr_dbg_cpu0_edrcr::ApbaddrDbgCpu0EdrcrSpec>;
#[doc = "External Debug Reserve Control Register"]
pub mod apbaddr_dbg_cpu0_edrcr;
#[doc = "APBADDR_DBG_CPU0_EDACR (rw) register accessor: External Debug Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edacr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDACR")]
pub type ApbaddrDbgCpu0Edacr = crate::Reg<apbaddr_dbg_cpu0_edacr::ApbaddrDbgCpu0EdacrSpec>;
#[doc = "External Debug Auxiliary Control Register"]
pub mod apbaddr_dbg_cpu0_edacr;
#[doc = "APBADDR_DBG_CPU0_EDECCR (rw) register accessor: External Debug Exception Catch Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edeccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edeccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edeccr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDECCR")]
pub type ApbaddrDbgCpu0Edeccr = crate::Reg<apbaddr_dbg_cpu0_edeccr::ApbaddrDbgCpu0EdeccrSpec>;
#[doc = "External Debug Exception Catch Control Register"]
pub mod apbaddr_dbg_cpu0_edeccr;
#[doc = "APBADDR_DBG_CPU0_EDPCSR_31_0 (rw) register accessor: External Debug Program Counter Sample Register (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpcsr_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpcsr_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpcsr_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPCSR_31_0")]
pub type ApbaddrDbgCpu0Edpcsr31_0 =
    crate::Reg<apbaddr_dbg_cpu0_edpcsr_31_0::ApbaddrDbgCpu0Edpcsr31_0Spec>;
#[doc = "External Debug Program Counter Sample Register (low word)"]
pub mod apbaddr_dbg_cpu0_edpcsr_31_0;
#[doc = "APBADDR_DBG_CPU0_EDCIDSR (rw) register accessor: External Debug Context ID Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edcidsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edcidsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edcidsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDCIDSR")]
pub type ApbaddrDbgCpu0Edcidsr = crate::Reg<apbaddr_dbg_cpu0_edcidsr::ApbaddrDbgCpu0EdcidsrSpec>;
#[doc = "External Debug Context ID Sample Register"]
pub mod apbaddr_dbg_cpu0_edcidsr;
#[doc = "APBADDR_DBG_CPU0_EDVIDSR (rw) register accessor: External Debug Virtual Context Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edvidsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edvidsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edvidsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDVIDSR")]
pub type ApbaddrDbgCpu0Edvidsr = crate::Reg<apbaddr_dbg_cpu0_edvidsr::ApbaddrDbgCpu0EdvidsrSpec>;
#[doc = "External Debug Virtual Context Sample Register"]
pub mod apbaddr_dbg_cpu0_edvidsr;
#[doc = "APBADDR_DBG_CPU0_EDPCSR_63_32 (rw) register accessor: External Debug Program Counter Sample Register (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpcsr_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpcsr_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpcsr_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPCSR_63_32")]
pub type ApbaddrDbgCpu0Edpcsr63_32 =
    crate::Reg<apbaddr_dbg_cpu0_edpcsr_63_32::ApbaddrDbgCpu0Edpcsr63_32Spec>;
#[doc = "External Debug Program Counter Sample Register (high word)"]
pub mod apbaddr_dbg_cpu0_edpcsr_63_32;
#[doc = "APBADDR_DBG_CPU0_OSLAR_EL1 (rw) register accessor: OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_oslar_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_oslar_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_oslar_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_OSLAR_EL1")]
pub type ApbaddrDbgCpu0OslarEl1 =
    crate::Reg<apbaddr_dbg_cpu0_oslar_el1::ApbaddrDbgCpu0OslarEl1Spec>;
#[doc = "OS Lock Access Register"]
pub mod apbaddr_dbg_cpu0_oslar_el1;
#[doc = "APBADDR_DBG_CPU0_EDPRCR (rw) register accessor: External Debug Power/Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edprcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edprcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edprcr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPRCR")]
pub type ApbaddrDbgCpu0Edprcr = crate::Reg<apbaddr_dbg_cpu0_edprcr::ApbaddrDbgCpu0EdprcrSpec>;
#[doc = "External Debug Power/Reset Control Register"]
pub mod apbaddr_dbg_cpu0_edprcr;
#[doc = "APBADDR_DBG_CPU0_EDPRSR (rw) register accessor: External Debug Processor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edprsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edprsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edprsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPRSR")]
pub type ApbaddrDbgCpu0Edprsr = crate::Reg<apbaddr_dbg_cpu0_edprsr::ApbaddrDbgCpu0EdprsrSpec>;
#[doc = "External Debug Processor Status Register"]
pub mod apbaddr_dbg_cpu0_edprsr;
#[doc = "APBADDR_DBG_CPU0_DBGBVR0_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR0_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr0_el1_31_0::ApbaddrDbgCpu0Dbgbvr0El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR0_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR0_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr0_el1_63_32::ApbaddrDbgCpu0Dbgbvr0El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR0_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR0_EL1 (rw) register accessor: Debug Breakpoint Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr0_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr0_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr0_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR0_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr0El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr0_el1::ApbaddrDbgCpu0Dbgbcr0El1Spec>;
#[doc = "Debug Breakpoint Control Register 0"]
pub mod apbaddr_dbg_cpu0_dbgbcr0_el1;
#[doc = "APBADDR_DBG_CPU0_DBGBVR1_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR1_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr1_el1_31_0::ApbaddrDbgCpu0Dbgbvr1El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR1_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR1_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr1_el1_63_32::ApbaddrDbgCpu0Dbgbvr1El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR1_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR1_EL1 (rw) register accessor: Debug Breakpoint Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr1_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr1_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr1_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR1_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr1El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr1_el1::ApbaddrDbgCpu0Dbgbcr1El1Spec>;
#[doc = "Debug Breakpoint Control Register 1"]
pub mod apbaddr_dbg_cpu0_dbgbcr1_el1;
#[doc = "APBADDR_DBG_CPU0_DBGBVR2_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr2_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr2_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr2_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR2_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr2El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr2_el1_31_0::ApbaddrDbgCpu0Dbgbvr2El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr2_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR2_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr2_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr2_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr2_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR2_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr2El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr2_el1_63_32::ApbaddrDbgCpu0Dbgbvr2El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR2_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr2_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR2_EL1 (rw) register accessor: Debug Breakpoint Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr2_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr2_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr2_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR2_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr2El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr2_el1::ApbaddrDbgCpu0Dbgbcr2El1Spec>;
#[doc = "Debug Breakpoint Control Register 2"]
pub mod apbaddr_dbg_cpu0_dbgbcr2_el1;
#[doc = "APBADDR_DBG_CPU0_DBGBVR3_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr3_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr3_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr3_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR3_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr3El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr3_el1_31_0::ApbaddrDbgCpu0Dbgbvr3El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr3_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR3_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr3_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr3_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr3_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR3_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr3El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr3_el1_63_32::ApbaddrDbgCpu0Dbgbvr3El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR3_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr3_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR3_EL1 (rw) register accessor: Debug Breakpoint Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr3_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr3_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr3_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR3_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr3El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr3_el1::ApbaddrDbgCpu0Dbgbcr3El1Spec>;
#[doc = "Debug Breakpoint Control Register 3"]
pub mod apbaddr_dbg_cpu0_dbgbcr3_el1;
#[doc = "APBADDR_DBG_CPU0_DBGBVR4_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr4_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr4_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr4_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR4_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr4El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr4_el1_31_0::ApbaddrDbgCpu0Dbgbvr4El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr4_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR4_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr4_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr4_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr4_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR4_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr4El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr4_el1_63_32::ApbaddrDbgCpu0Dbgbvr4El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR4_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr4_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR4_EL1 (rw) register accessor: Debug Breakpoint Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr4_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr4_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr4_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR4_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr4El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr4_el1::ApbaddrDbgCpu0Dbgbcr4El1Spec>;
#[doc = "Debug Breakpoint Control Register 4"]
pub mod apbaddr_dbg_cpu0_dbgbcr4_el1;
#[doc = "APBADDR_DBG_CPU0_DBGBVR5_EL1_31_0 (rw) register accessor: Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr5_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr5_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr5_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR5_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgbvr5El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr5_el1_31_0::ApbaddrDbgCpu0Dbgbvr5El1_31_0Spec>;
#[doc = "Debug Breakpoint Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr5_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGBVR5_EL1_63_32 (rw) register accessor: Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbvr5_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbvr5_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbvr5_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBVR5_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgbvr5El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbvr5_el1_63_32::ApbaddrDbgCpu0Dbgbvr5El1_63_32Spec>;
#[doc = "Debug Breakpoint Extended Value Registers. Holds a virtual address or a VMID and/or a context ID for use in breakpoint matching. Forms breakpoint n together with control register DBGBCR5_EL1. Multiple uses of this register refer to ARMv8"]
pub mod apbaddr_dbg_cpu0_dbgbvr5_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGBCR5_EL1 (rw) register accessor: Debug Breakpoint Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr5_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr5_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgbcr5_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGBCR5_EL1")]
pub type ApbaddrDbgCpu0Dbgbcr5El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgbcr5_el1::ApbaddrDbgCpu0Dbgbcr5El1Spec>;
#[doc = "Debug Breakpoint Control Register 5"]
pub mod apbaddr_dbg_cpu0_dbgbcr5_el1;
#[doc = "APBADDR_DBG_CPU0_DBGWVR0_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR0_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgwvr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr0_el1_31_0::ApbaddrDbgCpu0Dbgwvr0El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 0"]
pub mod apbaddr_dbg_cpu0_dbgwvr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGWVR0_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR0_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgwvr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr0_el1_63_32::ApbaddrDbgCpu0Dbgwvr0El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 0"]
pub mod apbaddr_dbg_cpu0_dbgwvr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGWCR0_EL1 (rw) register accessor: Debug Watchpoint Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwcr0_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwcr0_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwcr0_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWCR0_EL1")]
pub type ApbaddrDbgCpu0Dbgwcr0El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwcr0_el1::ApbaddrDbgCpu0Dbgwcr0El1Spec>;
#[doc = "Debug Watchpoint Control Register 0"]
pub mod apbaddr_dbg_cpu0_dbgwcr0_el1;
#[doc = "APBADDR_DBG_CPU0_DBGWVR1_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR1_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgwvr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr1_el1_31_0::ApbaddrDbgCpu0Dbgwvr1El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 1"]
pub mod apbaddr_dbg_cpu0_dbgwvr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGWVR1_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR1_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgwvr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr1_el1_63_32::ApbaddrDbgCpu0Dbgwvr1El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 1"]
pub mod apbaddr_dbg_cpu0_dbgwvr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGWCR1_EL1 (rw) register accessor: Debug Watchpoint Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwcr1_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwcr1_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwcr1_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWCR1_EL1")]
pub type ApbaddrDbgCpu0Dbgwcr1El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwcr1_el1::ApbaddrDbgCpu0Dbgwcr1El1Spec>;
#[doc = "Debug Watchpoint Control Register 1"]
pub mod apbaddr_dbg_cpu0_dbgwcr1_el1;
#[doc = "APBADDR_DBG_CPU0_DBGWVR2_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr2_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr2_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr2_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR2_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgwvr2El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr2_el1_31_0::ApbaddrDbgCpu0Dbgwvr2El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 2"]
pub mod apbaddr_dbg_cpu0_dbgwvr2_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGWVR2_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr2_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr2_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr2_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR2_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgwvr2El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr2_el1_63_32::ApbaddrDbgCpu0Dbgwvr2El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 2"]
pub mod apbaddr_dbg_cpu0_dbgwvr2_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGWCR2_EL1 (rw) register accessor: Debug Watchpoint Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwcr2_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwcr2_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwcr2_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWCR2_EL1")]
pub type ApbaddrDbgCpu0Dbgwcr2El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwcr2_el1::ApbaddrDbgCpu0Dbgwcr2El1Spec>;
#[doc = "Debug Watchpoint Control Register 2"]
pub mod apbaddr_dbg_cpu0_dbgwcr2_el1;
#[doc = "APBADDR_DBG_CPU0_DBGWVR3_EL1_31_0 (rw) register accessor: Debug Watchpoint Value Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr3_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr3_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr3_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR3_EL1_31_0")]
pub type ApbaddrDbgCpu0Dbgwvr3El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr3_el1_31_0::ApbaddrDbgCpu0Dbgwvr3El1_31_0Spec>;
#[doc = "Debug Watchpoint Value Register 3"]
pub mod apbaddr_dbg_cpu0_dbgwvr3_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_DBGWVR3_EL1_63_32 (rw) register accessor: Debug Watchpoint Extended Value Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwvr3_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWVR3_EL1_63_32")]
pub type ApbaddrDbgCpu0Dbgwvr3El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwvr3_el1_63_32::ApbaddrDbgCpu0Dbgwvr3El1_63_32Spec>;
#[doc = "Debug Watchpoint Extended Value Register 3"]
pub mod apbaddr_dbg_cpu0_dbgwvr3_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_DBGWCR3_EL1 (rw) register accessor: Debug Watchpoint Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgwcr3_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgwcr3_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgwcr3_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGWCR3_EL1")]
pub type ApbaddrDbgCpu0Dbgwcr3El1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgwcr3_el1::ApbaddrDbgCpu0Dbgwcr3El1Spec>;
#[doc = "Debug Watchpoint Control Register 3"]
pub mod apbaddr_dbg_cpu0_dbgwcr3_el1;
#[doc = "APBADDR_DBG_CPU0_MIDR_EL1 (rw) register accessor: Main ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_midr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_midr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_midr_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_MIDR_EL1")]
pub type ApbaddrDbgCpu0MidrEl1 = crate::Reg<apbaddr_dbg_cpu0_midr_el1::ApbaddrDbgCpu0MidrEl1Spec>;
#[doc = "Main ID Register"]
pub mod apbaddr_dbg_cpu0_midr_el1;
#[doc = "APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_31_0 (rw) register accessor: Processor Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64pfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec>;
#[doc = "Processor Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_63_32 (rw) register accessor: Processor Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64pfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32::ApbaddrDbgCpu0IdAa64pfr0El1_63_32Spec>;
#[doc = "Processor Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64pfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_31_0 (rw) register accessor: Debug Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64dfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec>;
#[doc = "Debug Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_63_32 (rw) register accessor: Debug Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64dfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32::ApbaddrDbgCpu0IdAa64dfr0El1_63_32Spec>;
#[doc = "Debug Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64dfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_31_0 (rw) register accessor: Instruction Set Attribute Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64isar0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0::ApbaddrDbgCpu0IdAa64isar0El1_31_0Spec>;
#[doc = "Instruction Set Attribute Register 0 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64isar0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_63_32 (rw) register accessor: Instruction Set Attribute Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64ISAR0_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64isar0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32::ApbaddrDbgCpu0IdAa64isar0El1_63_32Spec>;
#[doc = "Instruction Set Attribute Register 0 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64isar0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64MMFR0_EL1_31_0 (rw) register accessor: Memory Model Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64MMFR0_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64mmfr0El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0::ApbaddrDbgCpu0IdAa64mmfr0El1_31_0Spec>;
#[doc = "Memory Model Feature Register 0 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64mmfr0_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64MMFR0_EL1_63_32 (rw) register accessor: Memory Model Feature Register 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64MMFR0_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64mmfr0El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32::ApbaddrDbgCpu0IdAa64mmfr0El1_63_32Spec>;
#[doc = "Memory Model Feature Register 0 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64mmfr0_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64PFR1_EL1_31_0 (rw) register accessor: Processor Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64PFR1_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64pfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0::ApbaddrDbgCpu0IdAa64pfr1El1_31_0Spec>;
#[doc = "Processor Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64pfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64PFR1_EL1_63_32 (rw) register accessor: Processor Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64PFR1_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64pfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32::ApbaddrDbgCpu0IdAa64pfr1El1_63_32Spec>;
#[doc = "Processor Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64pfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64DFR1_EL1_31_0 (rw) register accessor: Auxiliary Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64DFR1_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64dfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0::ApbaddrDbgCpu0IdAa64dfr1El1_31_0Spec>;
#[doc = "Auxiliary Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64dfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64DFR1_EL1_63_32 (rw) register accessor: Auxiliary Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64DFR1_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64dfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32::ApbaddrDbgCpu0IdAa64dfr1El1_63_32Spec>;
#[doc = "Auxiliary Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64dfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64ISAR1_EL1_31_0 (rw) register accessor: Instruction Set Attribute Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64ISAR1_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64isar1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0::ApbaddrDbgCpu0IdAa64isar1El1_31_0Spec>;
#[doc = "Instruction Set Attribute Register 1 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64isar1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64ISAR1_EL1_63_32 (rw) register accessor: Instruction Set Attribute Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64ISAR1_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64isar1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32::ApbaddrDbgCpu0IdAa64isar1El1_63_32Spec>;
#[doc = "Instruction Set Attribute Register 1 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64isar1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_ID_AA64MMFR1_EL1_31_0 (rw) register accessor: Memory Model Feature Register 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64MMFR1_EL1_31_0")]
pub type ApbaddrDbgCpu0IdAa64mmfr1El1_31_0 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0::ApbaddrDbgCpu0IdAa64mmfr1El1_31_0Spec>;
#[doc = "Memory Model Feature Register 1 (low word)"]
pub mod apbaddr_dbg_cpu0_id_aa64mmfr1_el1_31_0;
#[doc = "APBADDR_DBG_CPU0_ID_AA64MMFR1_EL1_63_32 (rw) register accessor: Memory Model Feature Register 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_ID_AA64MMFR1_EL1_63_32")]
pub type ApbaddrDbgCpu0IdAa64mmfr1El1_63_32 =
    crate::Reg<apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32::ApbaddrDbgCpu0IdAa64mmfr1El1_63_32Spec>;
#[doc = "Memory Model Feature Register 1 (high word)"]
pub mod apbaddr_dbg_cpu0_id_aa64mmfr1_el1_63_32;
#[doc = "APBADDR_DBG_CPU0_EDITCTRL (rw) register accessor: External Debug Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_editctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_editctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_editctrl`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDITCTRL")]
pub type ApbaddrDbgCpu0Editctrl = crate::Reg<apbaddr_dbg_cpu0_editctrl::ApbaddrDbgCpu0EditctrlSpec>;
#[doc = "External Debug Integration mode Control Register"]
pub mod apbaddr_dbg_cpu0_editctrl;
#[doc = "APBADDR_DBG_CPU0_DBGCLAIMSET_EL1 (rw) register accessor: Debug Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgclaimset_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgclaimset_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgclaimset_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGCLAIMSET_EL1")]
pub type ApbaddrDbgCpu0DbgclaimsetEl1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgclaimset_el1::ApbaddrDbgCpu0DbgclaimsetEl1Spec>;
#[doc = "Debug Claim Tag Set Register"]
pub mod apbaddr_dbg_cpu0_dbgclaimset_el1;
#[doc = "APBADDR_DBG_CPU0_DBGCLAIMCLR_EL1 (rw) register accessor: Debug Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgclaimclr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgclaimclr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgclaimclr_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGCLAIMCLR_EL1")]
pub type ApbaddrDbgCpu0DbgclaimclrEl1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgclaimclr_el1::ApbaddrDbgCpu0DbgclaimclrEl1Spec>;
#[doc = "Debug Claim Tag Clear Register"]
pub mod apbaddr_dbg_cpu0_dbgclaimclr_el1;
#[doc = "APBADDR_DBG_CPU0_EDDEVAFF0 (rw) register accessor: External Debug Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevaff0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVAFF0")]
pub type ApbaddrDbgCpu0Eddevaff0 =
    crate::Reg<apbaddr_dbg_cpu0_eddevaff0::ApbaddrDbgCpu0Eddevaff0Spec>;
#[doc = "External Debug Device Affinity Register 0"]
pub mod apbaddr_dbg_cpu0_eddevaff0;
#[doc = "APBADDR_DBG_CPU0_EDDEVAFF1 (rw) register accessor: External Debug Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevaff1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVAFF1")]
pub type ApbaddrDbgCpu0Eddevaff1 =
    crate::Reg<apbaddr_dbg_cpu0_eddevaff1::ApbaddrDbgCpu0Eddevaff1Spec>;
#[doc = "External Debug Device Affinity Register 1"]
pub mod apbaddr_dbg_cpu0_eddevaff1;
#[doc = "APBADDR_DBG_CPU0_EDLAR (rw) register accessor: External Debug Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edlar`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDLAR")]
pub type ApbaddrDbgCpu0Edlar = crate::Reg<apbaddr_dbg_cpu0_edlar::ApbaddrDbgCpu0EdlarSpec>;
#[doc = "External Debug Lock Access Register"]
pub mod apbaddr_dbg_cpu0_edlar;
#[doc = "APBADDR_DBG_CPU0_EDLSR (rw) register accessor: External Debug Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edlsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edlsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edlsr`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDLSR")]
pub type ApbaddrDbgCpu0Edlsr = crate::Reg<apbaddr_dbg_cpu0_edlsr::ApbaddrDbgCpu0EdlsrSpec>;
#[doc = "External Debug Lock Status Register"]
pub mod apbaddr_dbg_cpu0_edlsr;
#[doc = "APBADDR_DBG_CPU0_DBGAUTHSTATUS_EL1 (rw) register accessor: Debug Authentication Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgauthstatus_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgauthstatus_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_dbgauthstatus_el1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_DBGAUTHSTATUS_EL1")]
pub type ApbaddrDbgCpu0DbgauthstatusEl1 =
    crate::Reg<apbaddr_dbg_cpu0_dbgauthstatus_el1::ApbaddrDbgCpu0DbgauthstatusEl1Spec>;
#[doc = "Debug Authentication Status register"]
pub mod apbaddr_dbg_cpu0_dbgauthstatus_el1;
#[doc = "APBADDR_DBG_CPU0_EDDEVARCH (rw) register accessor: External Debug Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevarch`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVARCH")]
pub type ApbaddrDbgCpu0Eddevarch =
    crate::Reg<apbaddr_dbg_cpu0_eddevarch::ApbaddrDbgCpu0EddevarchSpec>;
#[doc = "External Debug Device Architecture Register"]
pub mod apbaddr_dbg_cpu0_eddevarch;
#[doc = "APBADDR_DBG_CPU0_EDDEVID2 (rw) register accessor: External Debug Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevid2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVID2")]
pub type ApbaddrDbgCpu0Eddevid2 = crate::Reg<apbaddr_dbg_cpu0_eddevid2::ApbaddrDbgCpu0Eddevid2Spec>;
#[doc = "External Debug Device ID Register 2"]
pub mod apbaddr_dbg_cpu0_eddevid2;
#[doc = "APBADDR_DBG_CPU0_EDDEVID1 (rw) register accessor: External Debug Device ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevid1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVID1")]
pub type ApbaddrDbgCpu0Eddevid1 = crate::Reg<apbaddr_dbg_cpu0_eddevid1::ApbaddrDbgCpu0Eddevid1Spec>;
#[doc = "External Debug Device ID Register 1"]
pub mod apbaddr_dbg_cpu0_eddevid1;
#[doc = "APBADDR_DBG_CPU0_EDDEVID (rw) register accessor: External Debug Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevid`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVID")]
pub type ApbaddrDbgCpu0Eddevid = crate::Reg<apbaddr_dbg_cpu0_eddevid::ApbaddrDbgCpu0EddevidSpec>;
#[doc = "External Debug Device ID Register 0"]
pub mod apbaddr_dbg_cpu0_eddevid;
#[doc = "APBADDR_DBG_CPU0_EDDEVTYPE (rw) register accessor: External Debug Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_eddevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_eddevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_eddevtype`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDDEVTYPE")]
pub type ApbaddrDbgCpu0Eddevtype =
    crate::Reg<apbaddr_dbg_cpu0_eddevtype::ApbaddrDbgCpu0EddevtypeSpec>;
#[doc = "External Debug Device Type Register"]
pub mod apbaddr_dbg_cpu0_eddevtype;
#[doc = "APBADDR_DBG_CPU0_EDPIDR4 (rw) register accessor: External Debug Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpidr4`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPIDR4")]
pub type ApbaddrDbgCpu0Edpidr4 = crate::Reg<apbaddr_dbg_cpu0_edpidr4::ApbaddrDbgCpu0Edpidr4Spec>;
#[doc = "External Debug Peripheral Identification Register 4"]
pub mod apbaddr_dbg_cpu0_edpidr4;
#[doc = "APBADDR_DBG_CPU0_EDPIDR0 (rw) register accessor: External Debug Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpidr0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPIDR0")]
pub type ApbaddrDbgCpu0Edpidr0 = crate::Reg<apbaddr_dbg_cpu0_edpidr0::ApbaddrDbgCpu0Edpidr0Spec>;
#[doc = "External Debug Peripheral Identification Register 0"]
pub mod apbaddr_dbg_cpu0_edpidr0;
#[doc = "APBADDR_DBG_CPU0_EDPIDR1 (rw) register accessor: External Debug Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpidr1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPIDR1")]
pub type ApbaddrDbgCpu0Edpidr1 = crate::Reg<apbaddr_dbg_cpu0_edpidr1::ApbaddrDbgCpu0Edpidr1Spec>;
#[doc = "External Debug Peripheral Identification Register 1"]
pub mod apbaddr_dbg_cpu0_edpidr1;
#[doc = "APBADDR_DBG_CPU0_EDPIDR2 (rw) register accessor: External Debug Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpidr2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPIDR2")]
pub type ApbaddrDbgCpu0Edpidr2 = crate::Reg<apbaddr_dbg_cpu0_edpidr2::ApbaddrDbgCpu0Edpidr2Spec>;
#[doc = "External Debug Peripheral Identification Register 2"]
pub mod apbaddr_dbg_cpu0_edpidr2;
#[doc = "APBADDR_DBG_CPU0_EDPIDR3 (rw) register accessor: External Debug Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edpidr3`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDPIDR3")]
pub type ApbaddrDbgCpu0Edpidr3 = crate::Reg<apbaddr_dbg_cpu0_edpidr3::ApbaddrDbgCpu0Edpidr3Spec>;
#[doc = "External Debug Peripheral Identification Register 3"]
pub mod apbaddr_dbg_cpu0_edpidr3;
#[doc = "APBADDR_DBG_CPU0_EDCIDR0 (rw) register accessor: External Debug Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edcidr0`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDCIDR0")]
pub type ApbaddrDbgCpu0Edcidr0 = crate::Reg<apbaddr_dbg_cpu0_edcidr0::ApbaddrDbgCpu0Edcidr0Spec>;
#[doc = "External Debug Component Identification Register 0"]
pub mod apbaddr_dbg_cpu0_edcidr0;
#[doc = "APBADDR_DBG_CPU0_EDCIDR1 (rw) register accessor: External Debug Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edcidr1`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDCIDR1")]
pub type ApbaddrDbgCpu0Edcidr1 = crate::Reg<apbaddr_dbg_cpu0_edcidr1::ApbaddrDbgCpu0Edcidr1Spec>;
#[doc = "External Debug Component Identification Register 1"]
pub mod apbaddr_dbg_cpu0_edcidr1;
#[doc = "APBADDR_DBG_CPU0_EDCIDR2 (rw) register accessor: External Debug Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edcidr2`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDCIDR2")]
pub type ApbaddrDbgCpu0Edcidr2 = crate::Reg<apbaddr_dbg_cpu0_edcidr2::ApbaddrDbgCpu0Edcidr2Spec>;
#[doc = "External Debug Component Identification Register 2"]
pub mod apbaddr_dbg_cpu0_edcidr2;
#[doc = "APBADDR_DBG_CPU0_EDCIDR3 (rw) register accessor: External Debug Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_edcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_edcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_dbg_cpu0_edcidr3`]
module"]
#[doc(alias = "APBADDR_DBG_CPU0_EDCIDR3")]
pub type ApbaddrDbgCpu0Edcidr3 = crate::Reg<apbaddr_dbg_cpu0_edcidr3::ApbaddrDbgCpu0Edcidr3Spec>;
#[doc = "External Debug Component Identification Register 3"]
pub mod apbaddr_dbg_cpu0_edcidr3;
