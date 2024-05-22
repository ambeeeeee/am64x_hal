#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    apbaddr_etm_cpu0_trcprgctlr: ApbaddrEtmCpu0Trcprgctlr,
    _reserved1: [u8; 0x04],
    apbaddr_etm_cpu0_trcstatr: ApbaddrEtmCpu0Trcstatr,
    apbaddr_etm_cpu0_trcconfigr: ApbaddrEtmCpu0Trcconfigr,
    _reserved3: [u8; 0x04],
    apbaddr_etm_cpu0_trcauxctlr: ApbaddrEtmCpu0Trcauxctlr,
    _reserved4: [u8; 0x04],
    apbaddr_etm_cpu0_trceventctl0r: ApbaddrEtmCpu0Trceventctl0r,
    apbaddr_etm_cpu0_trceventctl1r: ApbaddrEtmCpu0Trceventctl1r,
    _reserved6: [u8; 0x04],
    apbaddr_etm_cpu0_trcstallctlr: ApbaddrEtmCpu0Trcstallctlr,
    apbaddr_etm_cpu0_trctsctlr: ApbaddrEtmCpu0Trctsctlr,
    apbaddr_etm_cpu0_trcsyncpr: ApbaddrEtmCpu0Trcsyncpr,
    apbaddr_etm_cpu0_trcccctlr: ApbaddrEtmCpu0Trcccctlr,
    apbaddr_etm_cpu0_trcbbctlr: ApbaddrEtmCpu0Trcbbctlr,
    apbaddr_etm_cpu0_trctraceidr: ApbaddrEtmCpu0Trctraceidr,
    _reserved12: [u8; 0x3c],
    apbaddr_etm_cpu0_trcvictlr: ApbaddrEtmCpu0Trcvictlr,
    apbaddr_etm_cpu0_trcviiectlr: ApbaddrEtmCpu0Trcviiectlr,
    apbaddr_etm_cpu0_trcvissctlr: ApbaddrEtmCpu0Trcvissctlr,
    _reserved15: [u8; 0x74],
    apbaddr_etm_cpu0_trcseqevr0: ApbaddrEtmCpu0Trcseqevr0,
    apbaddr_etm_cpu0_trcseqevr1: ApbaddrEtmCpu0Trcseqevr1,
    apbaddr_etm_cpu0_trcseqevr2: ApbaddrEtmCpu0Trcseqevr2,
    _reserved18: [u8; 0x0c],
    apbaddr_etm_cpu0_trcseqrstevr: ApbaddrEtmCpu0Trcseqrstevr,
    apbaddr_etm_cpu0_trcseqstr: ApbaddrEtmCpu0Trcseqstr,
    apbaddr_etm_cpu0_trcextinselr: ApbaddrEtmCpu0Trcextinselr,
    _reserved21: [u8; 0x1c],
    apbaddr_etm_cpu0_trccntrldvr0: ApbaddrEtmCpu0Trccntrldvr0,
    apbaddr_etm_cpu0_trccntrldvr1: ApbaddrEtmCpu0Trccntrldvr1,
    _reserved23: [u8; 0x08],
    apbaddr_etm_cpu0_trccntctlr0: ApbaddrEtmCpu0Trccntctlr0,
    apbaddr_etm_cpu0_trccntctlr1: ApbaddrEtmCpu0Trccntctlr1,
    _reserved25: [u8; 0x08],
    apbaddr_etm_cpu0_trccntvr0: ApbaddrEtmCpu0Trccntvr0,
    apbaddr_etm_cpu0_trccntvr1: ApbaddrEtmCpu0Trccntvr1,
    _reserved27: [u8; 0x18],
    apbaddr_etm_cpu0_trcidr8: ApbaddrEtmCpu0Trcidr8,
    apbaddr_etm_cpu0_trcidr9: ApbaddrEtmCpu0Trcidr9,
    apbaddr_etm_cpu0_trcidr10: ApbaddrEtmCpu0Trcidr10,
    apbaddr_etm_cpu0_trcidr11: ApbaddrEtmCpu0Trcidr11,
    apbaddr_etm_cpu0_trcidr12: ApbaddrEtmCpu0Trcidr12,
    apbaddr_etm_cpu0_trcidr13: ApbaddrEtmCpu0Trcidr13,
    _reserved33: [u8; 0x28],
    apbaddr_etm_cpu0_trcimspec0: ApbaddrEtmCpu0Trcimspec0,
    _reserved34: [u8; 0x1c],
    apbaddr_etm_cpu0_trcidr0: ApbaddrEtmCpu0Trcidr0,
    apbaddr_etm_cpu0_trcidr1: ApbaddrEtmCpu0Trcidr1,
    apbaddr_etm_cpu0_trcidr2: ApbaddrEtmCpu0Trcidr2,
    apbaddr_etm_cpu0_trcidr3: ApbaddrEtmCpu0Trcidr3,
    apbaddr_etm_cpu0_trcidr4: ApbaddrEtmCpu0Trcidr4,
    apbaddr_etm_cpu0_trcidr5: ApbaddrEtmCpu0Trcidr5,
    _reserved40: [u8; 0x10],
    apbaddr_etm_cpu0_trcrsctlr2: ApbaddrEtmCpu0Trcrsctlr2,
    apbaddr_etm_cpu0_trcrsctlr3: ApbaddrEtmCpu0Trcrsctlr3,
    apbaddr_etm_cpu0_trcrsctlr4: ApbaddrEtmCpu0Trcrsctlr4,
    apbaddr_etm_cpu0_trcrsctlr5: ApbaddrEtmCpu0Trcrsctlr5,
    apbaddr_etm_cpu0_trcrsctlr6: ApbaddrEtmCpu0Trcrsctlr6,
    apbaddr_etm_cpu0_trcrsctlr7: ApbaddrEtmCpu0Trcrsctlr7,
    apbaddr_etm_cpu0_trcrsctlr8: ApbaddrEtmCpu0Trcrsctlr8,
    apbaddr_etm_cpu0_trcrsctlr9: ApbaddrEtmCpu0Trcrsctlr9,
    apbaddr_etm_cpu0_trcrsctlr10: ApbaddrEtmCpu0Trcrsctlr10,
    apbaddr_etm_cpu0_trcrsctlr11: ApbaddrEtmCpu0Trcrsctlr11,
    apbaddr_etm_cpu0_trcrsctlr12: ApbaddrEtmCpu0Trcrsctlr12,
    apbaddr_etm_cpu0_trcrsctlr13: ApbaddrEtmCpu0Trcrsctlr13,
    apbaddr_etm_cpu0_trcrsctlr14: ApbaddrEtmCpu0Trcrsctlr14,
    apbaddr_etm_cpu0_trcrsctlr15: ApbaddrEtmCpu0Trcrsctlr15,
    _reserved54: [u8; 0x40],
    apbaddr_etm_cpu0_trcssccr0: ApbaddrEtmCpu0Trcssccr0,
    _reserved55: [u8; 0x1c],
    apbaddr_etm_cpu0_trcsscsr0: ApbaddrEtmCpu0Trcsscsr0,
    _reserved56: [u8; 0x5c],
    apbaddr_etm_cpu0_trcoslar: ApbaddrEtmCpu0Trcoslar,
    apbaddr_etm_cpu0_trcoslsr: ApbaddrEtmCpu0Trcoslsr,
    _reserved58: [u8; 0x08],
    apbaddr_etm_cpu0_trcpdcr: ApbaddrEtmCpu0Trcpdcr,
    apbaddr_etm_cpu0_trcpdsr: ApbaddrEtmCpu0Trcpdsr,
    _reserved60: [u8; 0xe8],
    apbaddr_etm_cpu0_trcacvr0_31_0: ApbaddrEtmCpu0Trcacvr0_31_0,
    apbaddr_etm_cpu0_trcacvr0_63_32: ApbaddrEtmCpu0Trcacvr0_63_32,
    apbaddr_etm_cpu0_trcacvr1_31_0: ApbaddrEtmCpu0Trcacvr1_31_0,
    apbaddr_etm_cpu0_trcacvr1_63_32: ApbaddrEtmCpu0Trcacvr1_63_32,
    apbaddr_etm_cpu0_trcacvr2_31_0: ApbaddrEtmCpu0Trcacvr2_31_0,
    apbaddr_etm_cpu0_trcacvr2_63_32: ApbaddrEtmCpu0Trcacvr2_63_32,
    apbaddr_etm_cpu0_trcacvr3_31_0: ApbaddrEtmCpu0Trcacvr3_31_0,
    apbaddr_etm_cpu0_trcacvr3_63_32: ApbaddrEtmCpu0Trcacvr3_63_32,
    apbaddr_etm_cpu0_trcacvr4_31_0: ApbaddrEtmCpu0Trcacvr4_31_0,
    apbaddr_etm_cpu0_trcacvr4_63_32: ApbaddrEtmCpu0Trcacvr4_63_32,
    apbaddr_etm_cpu0_trcacvr5_31_0: ApbaddrEtmCpu0Trcacvr5_31_0,
    apbaddr_etm_cpu0_trcacvr5_63_32: ApbaddrEtmCpu0Trcacvr5_63_32,
    apbaddr_etm_cpu0_trcacvr6_31_0: ApbaddrEtmCpu0Trcacvr6_31_0,
    apbaddr_etm_cpu0_trcacvr6_63_32: ApbaddrEtmCpu0Trcacvr6_63_32,
    apbaddr_etm_cpu0_trcacvr7_31_0: ApbaddrEtmCpu0Trcacvr7_31_0,
    apbaddr_etm_cpu0_trcacvr7_63_32: ApbaddrEtmCpu0Trcacvr7_63_32,
    _reserved76: [u8; 0x40],
    apbaddr_etm_cpu0_trcacatr0: ApbaddrEtmCpu0Trcacatr0,
    _reserved77: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr1: ApbaddrEtmCpu0Trcacatr1,
    _reserved78: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr2: ApbaddrEtmCpu0Trcacatr2,
    _reserved79: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr3: ApbaddrEtmCpu0Trcacatr3,
    _reserved80: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr4: ApbaddrEtmCpu0Trcacatr4,
    _reserved81: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr5: ApbaddrEtmCpu0Trcacatr5,
    _reserved82: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr6: ApbaddrEtmCpu0Trcacatr6,
    _reserved83: [u8; 0x04],
    apbaddr_etm_cpu0_trcacatr7: ApbaddrEtmCpu0Trcacatr7,
    _reserved84: [u8; 0x0144],
    apbaddr_etm_cpu0_trccidcvr0: ApbaddrEtmCpu0Trccidcvr0,
    _reserved85: [u8; 0x3c],
    apbaddr_etm_cpu0_trcvmidcvr0: ApbaddrEtmCpu0Trcvmidcvr0,
    _reserved86: [u8; 0x3c],
    apbaddr_etm_cpu0_trccidcctlr0: ApbaddrEtmCpu0Trccidcctlr0,
    _reserved87: [u8; 0x0860],
    apbaddr_etm_cpu0_trcitatbidr: ApbaddrEtmCpu0Trcitatbidr,
    _reserved88: [u8; 0x04],
    apbaddr_etm_cpu0_trcitidatar: ApbaddrEtmCpu0Trcitidatar,
    _reserved89: [u8; 0x04],
    apbaddr_etm_cpu0_trcitiatbinr: ApbaddrEtmCpu0Trcitiatbinr,
    _reserved90: [u8; 0x04],
    apbaddr_etm_cpu0_trcitiatboutr: ApbaddrEtmCpu0Trcitiatboutr,
    apbaddr_etm_cpu0_trcitctrl: ApbaddrEtmCpu0Trcitctrl,
    _reserved92: [u8; 0x9c],
    apbaddr_etm_cpu0_trcclaimset: ApbaddrEtmCpu0Trcclaimset,
    apbaddr_etm_cpu0_trcclaimclr: ApbaddrEtmCpu0Trcclaimclr,
    apbaddr_etm_cpu0_trcdevaff0: ApbaddrEtmCpu0Trcdevaff0,
    apbaddr_etm_cpu0_trcdevaff1: ApbaddrEtmCpu0Trcdevaff1,
    apbaddr_etm_cpu0_trclar: ApbaddrEtmCpu0Trclar,
    apbaddr_etm_cpu0_trclsr: ApbaddrEtmCpu0Trclsr,
    apbaddr_etm_cpu0_trcauthstatus: ApbaddrEtmCpu0Trcauthstatus,
    apbaddr_etm_cpu0_trcdevarch: ApbaddrEtmCpu0Trcdevarch,
    _reserved100: [u8; 0x08],
    apbaddr_etm_cpu0_trcdevid: ApbaddrEtmCpu0Trcdevid,
    apbaddr_etm_cpu0_trcdevtype: ApbaddrEtmCpu0Trcdevtype,
    apbaddr_etm_cpu0_trcpidr4: ApbaddrEtmCpu0Trcpidr4,
    apbaddr_etm_cpu0_trcpidr5: ApbaddrEtmCpu0Trcpidr5,
    apbaddr_etm_cpu0_trcpidr6: ApbaddrEtmCpu0Trcpidr6,
    apbaddr_etm_cpu0_trcpidr7: ApbaddrEtmCpu0Trcpidr7,
    apbaddr_etm_cpu0_trcpidr0: ApbaddrEtmCpu0Trcpidr0,
    apbaddr_etm_cpu0_trcpidr1: ApbaddrEtmCpu0Trcpidr1,
    apbaddr_etm_cpu0_trcpidr2: ApbaddrEtmCpu0Trcpidr2,
    apbaddr_etm_cpu0_trcpidr3: ApbaddrEtmCpu0Trcpidr3,
    apbaddr_etm_cpu0_trccidr0: ApbaddrEtmCpu0Trccidr0,
    apbaddr_etm_cpu0_trccidr1: ApbaddrEtmCpu0Trccidr1,
    apbaddr_etm_cpu0_trccidr2: ApbaddrEtmCpu0Trccidr2,
    apbaddr_etm_cpu0_trccidr3: ApbaddrEtmCpu0Trccidr3,
}
impl RegisterBlock {
    #[doc = "0x04 - Programming Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcprgctlr(&self) -> &ApbaddrEtmCpu0Trcprgctlr {
        &self.apbaddr_etm_cpu0_trcprgctlr
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcstatr(&self) -> &ApbaddrEtmCpu0Trcstatr {
        &self.apbaddr_etm_cpu0_trcstatr
    }
    #[doc = "0x10 - Trace Configuration Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcconfigr(&self) -> &ApbaddrEtmCpu0Trcconfigr {
        &self.apbaddr_etm_cpu0_trcconfigr
    }
    #[doc = "0x18 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcauxctlr(&self) -> &ApbaddrEtmCpu0Trcauxctlr {
        &self.apbaddr_etm_cpu0_trcauxctlr
    }
    #[doc = "0x20 - Event Control 0 Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trceventctl0r(&self) -> &ApbaddrEtmCpu0Trceventctl0r {
        &self.apbaddr_etm_cpu0_trceventctl0r
    }
    #[doc = "0x24 - Event Control 1 Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trceventctl1r(&self) -> &ApbaddrEtmCpu0Trceventctl1r {
        &self.apbaddr_etm_cpu0_trceventctl1r
    }
    #[doc = "0x2c - Stall Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcstallctlr(&self) -> &ApbaddrEtmCpu0Trcstallctlr {
        &self.apbaddr_etm_cpu0_trcstallctlr
    }
    #[doc = "0x30 - Global Timestamp Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trctsctlr(&self) -> &ApbaddrEtmCpu0Trctsctlr {
        &self.apbaddr_etm_cpu0_trctsctlr
    }
    #[doc = "0x34 - Synchronization Period Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcsyncpr(&self) -> &ApbaddrEtmCpu0Trcsyncpr {
        &self.apbaddr_etm_cpu0_trcsyncpr
    }
    #[doc = "0x38 - Cycle Count Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcccctlr(&self) -> &ApbaddrEtmCpu0Trcccctlr {
        &self.apbaddr_etm_cpu0_trcccctlr
    }
    #[doc = "0x3c - Branch Broadcast Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcbbctlr(&self) -> &ApbaddrEtmCpu0Trcbbctlr {
        &self.apbaddr_etm_cpu0_trcbbctlr
    }
    #[doc = "0x40 - Trace ID Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trctraceidr(&self) -> &ApbaddrEtmCpu0Trctraceidr {
        &self.apbaddr_etm_cpu0_trctraceidr
    }
    #[doc = "0x80 - ViewInst Main Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcvictlr(&self) -> &ApbaddrEtmCpu0Trcvictlr {
        &self.apbaddr_etm_cpu0_trcvictlr
    }
    #[doc = "0x84 - ViewInst Include-Exclude Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcviiectlr(&self) -> &ApbaddrEtmCpu0Trcviiectlr {
        &self.apbaddr_etm_cpu0_trcviiectlr
    }
    #[doc = "0x88 - ViewInst Start-Stop Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcvissctlr(&self) -> &ApbaddrEtmCpu0Trcvissctlr {
        &self.apbaddr_etm_cpu0_trcvissctlr
    }
    #[doc = "0x100 - Sequencer State Transition Control Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcseqevr0(&self) -> &ApbaddrEtmCpu0Trcseqevr0 {
        &self.apbaddr_etm_cpu0_trcseqevr0
    }
    #[doc = "0x104 - Sequencer State Transition Control Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcseqevr1(&self) -> &ApbaddrEtmCpu0Trcseqevr1 {
        &self.apbaddr_etm_cpu0_trcseqevr1
    }
    #[doc = "0x108 - Sequencer State Transition Control Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcseqevr2(&self) -> &ApbaddrEtmCpu0Trcseqevr2 {
        &self.apbaddr_etm_cpu0_trcseqevr2
    }
    #[doc = "0x118 - Sequencer Reset Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcseqrstevr(&self) -> &ApbaddrEtmCpu0Trcseqrstevr {
        &self.apbaddr_etm_cpu0_trcseqrstevr
    }
    #[doc = "0x11c - Sequencer State Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcseqstr(&self) -> &ApbaddrEtmCpu0Trcseqstr {
        &self.apbaddr_etm_cpu0_trcseqstr
    }
    #[doc = "0x120 - External Input Select Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcextinselr(&self) -> &ApbaddrEtmCpu0Trcextinselr {
        &self.apbaddr_etm_cpu0_trcextinselr
    }
    #[doc = "0x140 - Counter Reload Value Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntrldvr0(&self) -> &ApbaddrEtmCpu0Trccntrldvr0 {
        &self.apbaddr_etm_cpu0_trccntrldvr0
    }
    #[doc = "0x144 - Counter Reload Value Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntrldvr1(&self) -> &ApbaddrEtmCpu0Trccntrldvr1 {
        &self.apbaddr_etm_cpu0_trccntrldvr1
    }
    #[doc = "0x150 - Counter Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntctlr0(&self) -> &ApbaddrEtmCpu0Trccntctlr0 {
        &self.apbaddr_etm_cpu0_trccntctlr0
    }
    #[doc = "0x154 - Counter Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntctlr1(&self) -> &ApbaddrEtmCpu0Trccntctlr1 {
        &self.apbaddr_etm_cpu0_trccntctlr1
    }
    #[doc = "0x160 - Counter Value Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntvr0(&self) -> &ApbaddrEtmCpu0Trccntvr0 {
        &self.apbaddr_etm_cpu0_trccntvr0
    }
    #[doc = "0x164 - Counter Value Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccntvr1(&self) -> &ApbaddrEtmCpu0Trccntvr1 {
        &self.apbaddr_etm_cpu0_trccntvr1
    }
    #[doc = "0x180 - ID Register 8"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr8(&self) -> &ApbaddrEtmCpu0Trcidr8 {
        &self.apbaddr_etm_cpu0_trcidr8
    }
    #[doc = "0x184 - ID Register 9"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr9(&self) -> &ApbaddrEtmCpu0Trcidr9 {
        &self.apbaddr_etm_cpu0_trcidr9
    }
    #[doc = "0x188 - ID Register 10"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr10(&self) -> &ApbaddrEtmCpu0Trcidr10 {
        &self.apbaddr_etm_cpu0_trcidr10
    }
    #[doc = "0x18c - ID Register 11"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr11(&self) -> &ApbaddrEtmCpu0Trcidr11 {
        &self.apbaddr_etm_cpu0_trcidr11
    }
    #[doc = "0x190 - ID Register 12"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr12(&self) -> &ApbaddrEtmCpu0Trcidr12 {
        &self.apbaddr_etm_cpu0_trcidr12
    }
    #[doc = "0x194 - ID Register 13"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr13(&self) -> &ApbaddrEtmCpu0Trcidr13 {
        &self.apbaddr_etm_cpu0_trcidr13
    }
    #[doc = "0x1c0 - Implementation Specific Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcimspec0(&self) -> &ApbaddrEtmCpu0Trcimspec0 {
        &self.apbaddr_etm_cpu0_trcimspec0
    }
    #[doc = "0x1e0 - ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr0(&self) -> &ApbaddrEtmCpu0Trcidr0 {
        &self.apbaddr_etm_cpu0_trcidr0
    }
    #[doc = "0x1e4 - ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr1(&self) -> &ApbaddrEtmCpu0Trcidr1 {
        &self.apbaddr_etm_cpu0_trcidr1
    }
    #[doc = "0x1e8 - ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr2(&self) -> &ApbaddrEtmCpu0Trcidr2 {
        &self.apbaddr_etm_cpu0_trcidr2
    }
    #[doc = "0x1ec - ID Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr3(&self) -> &ApbaddrEtmCpu0Trcidr3 {
        &self.apbaddr_etm_cpu0_trcidr3
    }
    #[doc = "0x1f0 - ID Register 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr4(&self) -> &ApbaddrEtmCpu0Trcidr4 {
        &self.apbaddr_etm_cpu0_trcidr4
    }
    #[doc = "0x1f4 - ID Register 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcidr5(&self) -> &ApbaddrEtmCpu0Trcidr5 {
        &self.apbaddr_etm_cpu0_trcidr5
    }
    #[doc = "0x208 - Resource Selection Control Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr2(&self) -> &ApbaddrEtmCpu0Trcrsctlr2 {
        &self.apbaddr_etm_cpu0_trcrsctlr2
    }
    #[doc = "0x20c - Resource Selection Control Registers 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr3(&self) -> &ApbaddrEtmCpu0Trcrsctlr3 {
        &self.apbaddr_etm_cpu0_trcrsctlr3
    }
    #[doc = "0x210 - Resource Selection Control Registers 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr4(&self) -> &ApbaddrEtmCpu0Trcrsctlr4 {
        &self.apbaddr_etm_cpu0_trcrsctlr4
    }
    #[doc = "0x214 - Resource Selection Control Registers 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr5(&self) -> &ApbaddrEtmCpu0Trcrsctlr5 {
        &self.apbaddr_etm_cpu0_trcrsctlr5
    }
    #[doc = "0x218 - Resource Selection Control Registers 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr6(&self) -> &ApbaddrEtmCpu0Trcrsctlr6 {
        &self.apbaddr_etm_cpu0_trcrsctlr6
    }
    #[doc = "0x21c - Resource Selection Control Registers 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr7(&self) -> &ApbaddrEtmCpu0Trcrsctlr7 {
        &self.apbaddr_etm_cpu0_trcrsctlr7
    }
    #[doc = "0x220 - Resource Selection Control Registers 8"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr8(&self) -> &ApbaddrEtmCpu0Trcrsctlr8 {
        &self.apbaddr_etm_cpu0_trcrsctlr8
    }
    #[doc = "0x224 - Resource Selection Control Registers 9"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr9(&self) -> &ApbaddrEtmCpu0Trcrsctlr9 {
        &self.apbaddr_etm_cpu0_trcrsctlr9
    }
    #[doc = "0x228 - Resource Selection Control Registers 10"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr10(&self) -> &ApbaddrEtmCpu0Trcrsctlr10 {
        &self.apbaddr_etm_cpu0_trcrsctlr10
    }
    #[doc = "0x22c - Resource Selection Control Registers 11"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr11(&self) -> &ApbaddrEtmCpu0Trcrsctlr11 {
        &self.apbaddr_etm_cpu0_trcrsctlr11
    }
    #[doc = "0x230 - Resource Selection Control Registers 12"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr12(&self) -> &ApbaddrEtmCpu0Trcrsctlr12 {
        &self.apbaddr_etm_cpu0_trcrsctlr12
    }
    #[doc = "0x234 - Resource Selection Control Registers 13"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr13(&self) -> &ApbaddrEtmCpu0Trcrsctlr13 {
        &self.apbaddr_etm_cpu0_trcrsctlr13
    }
    #[doc = "0x238 - Resource Selection Control Registers 14"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr14(&self) -> &ApbaddrEtmCpu0Trcrsctlr14 {
        &self.apbaddr_etm_cpu0_trcrsctlr14
    }
    #[doc = "0x23c - Resource Selection Control Registers 15"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcrsctlr15(&self) -> &ApbaddrEtmCpu0Trcrsctlr15 {
        &self.apbaddr_etm_cpu0_trcrsctlr15
    }
    #[doc = "0x280 - Single-Shot Comparator Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcssccr0(&self) -> &ApbaddrEtmCpu0Trcssccr0 {
        &self.apbaddr_etm_cpu0_trcssccr0
    }
    #[doc = "0x2a0 - Single-Shot Comparator Status Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcsscsr0(&self) -> &ApbaddrEtmCpu0Trcsscsr0 {
        &self.apbaddr_etm_cpu0_trcsscsr0
    }
    #[doc = "0x300 - OS Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcoslar(&self) -> &ApbaddrEtmCpu0Trcoslar {
        &self.apbaddr_etm_cpu0_trcoslar
    }
    #[doc = "0x304 - OS Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcoslsr(&self) -> &ApbaddrEtmCpu0Trcoslsr {
        &self.apbaddr_etm_cpu0_trcoslsr
    }
    #[doc = "0x310 - Power Down Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpdcr(&self) -> &ApbaddrEtmCpu0Trcpdcr {
        &self.apbaddr_etm_cpu0_trcpdcr
    }
    #[doc = "0x314 - Power Down Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpdsr(&self) -> &ApbaddrEtmCpu0Trcpdsr {
        &self.apbaddr_etm_cpu0_trcpdsr
    }
    #[doc = "0x400 - Address Comparator Value Registers 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr0_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr0_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr0_31_0
    }
    #[doc = "0x404 - Address Comparator Value Registers 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr0_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr0_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr0_63_32
    }
    #[doc = "0x408 - Address Comparator Value Registers 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr1_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr1_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr1_31_0
    }
    #[doc = "0x40c - Address Comparator Value Registers 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr1_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr1_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr1_63_32
    }
    #[doc = "0x410 - Address Comparator Value Registers 2 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr2_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr2_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr2_31_0
    }
    #[doc = "0x414 - Address Comparator Value Registers 2 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr2_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr2_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr2_63_32
    }
    #[doc = "0x418 - Address Comparator Value Registers 3 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr3_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr3_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr3_31_0
    }
    #[doc = "0x41c - Address Comparator Value Registers 3 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr3_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr3_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr3_63_32
    }
    #[doc = "0x420 - Address Comparator Value Registers 4 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr4_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr4_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr4_31_0
    }
    #[doc = "0x424 - Address Comparator Value Registers 4 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr4_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr4_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr4_63_32
    }
    #[doc = "0x428 - Address Comparator Value Registers 5 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr5_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr5_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr5_31_0
    }
    #[doc = "0x42c - Address Comparator Value Registers 5 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr5_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr5_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr5_63_32
    }
    #[doc = "0x430 - Address Comparator Value Registers 6 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr6_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr6_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr6_31_0
    }
    #[doc = "0x434 - Address Comparator Value Registers 6 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr6_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr6_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr6_63_32
    }
    #[doc = "0x438 - Address Comparator Value Registers 7 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr7_31_0(&self) -> &ApbaddrEtmCpu0Trcacvr7_31_0 {
        &self.apbaddr_etm_cpu0_trcacvr7_31_0
    }
    #[doc = "0x43c - Address Comparator Value Registers 7 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacvr7_63_32(&self) -> &ApbaddrEtmCpu0Trcacvr7_63_32 {
        &self.apbaddr_etm_cpu0_trcacvr7_63_32
    }
    #[doc = "0x480 - Address Comparator Access Type Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr0(&self) -> &ApbaddrEtmCpu0Trcacatr0 {
        &self.apbaddr_etm_cpu0_trcacatr0
    }
    #[doc = "0x488 - Address Comparator Access Type Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr1(&self) -> &ApbaddrEtmCpu0Trcacatr1 {
        &self.apbaddr_etm_cpu0_trcacatr1
    }
    #[doc = "0x490 - Address Comparator Access Type Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr2(&self) -> &ApbaddrEtmCpu0Trcacatr2 {
        &self.apbaddr_etm_cpu0_trcacatr2
    }
    #[doc = "0x498 - Address Comparator Access Type Registers 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr3(&self) -> &ApbaddrEtmCpu0Trcacatr3 {
        &self.apbaddr_etm_cpu0_trcacatr3
    }
    #[doc = "0x4a0 - Address Comparator Access Type Registers 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr4(&self) -> &ApbaddrEtmCpu0Trcacatr4 {
        &self.apbaddr_etm_cpu0_trcacatr4
    }
    #[doc = "0x4a8 - Address Comparator Access Type Registers 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr5(&self) -> &ApbaddrEtmCpu0Trcacatr5 {
        &self.apbaddr_etm_cpu0_trcacatr5
    }
    #[doc = "0x4b0 - Address Comparator Access Type Registers 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr6(&self) -> &ApbaddrEtmCpu0Trcacatr6 {
        &self.apbaddr_etm_cpu0_trcacatr6
    }
    #[doc = "0x4b8 - Address Comparator Access Type Registers 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcacatr7(&self) -> &ApbaddrEtmCpu0Trcacatr7 {
        &self.apbaddr_etm_cpu0_trcacatr7
    }
    #[doc = "0x600 - Context ID Comparator Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidcvr0(&self) -> &ApbaddrEtmCpu0Trccidcvr0 {
        &self.apbaddr_etm_cpu0_trccidcvr0
    }
    #[doc = "0x640 - VMID Comparator Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcvmidcvr0(&self) -> &ApbaddrEtmCpu0Trcvmidcvr0 {
        &self.apbaddr_etm_cpu0_trcvmidcvr0
    }
    #[doc = "0x680 - Context ID Comparator Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidcctlr0(&self) -> &ApbaddrEtmCpu0Trccidcctlr0 {
        &self.apbaddr_etm_cpu0_trccidcctlr0
    }
    #[doc = "0xee4 - Integration ATB Identification Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcitatbidr(&self) -> &ApbaddrEtmCpu0Trcitatbidr {
        &self.apbaddr_etm_cpu0_trcitatbidr
    }
    #[doc = "0xeec - Integration Instruction ATB Data Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcitidatar(&self) -> &ApbaddrEtmCpu0Trcitidatar {
        &self.apbaddr_etm_cpu0_trcitidatar
    }
    #[doc = "0xef4 - Integration Instruction ATB In Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcitiatbinr(&self) -> &ApbaddrEtmCpu0Trcitiatbinr {
        &self.apbaddr_etm_cpu0_trcitiatbinr
    }
    #[doc = "0xefc - Integration Instruction ATB Out Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcitiatboutr(&self) -> &ApbaddrEtmCpu0Trcitiatboutr {
        &self.apbaddr_etm_cpu0_trcitiatboutr
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcitctrl(&self) -> &ApbaddrEtmCpu0Trcitctrl {
        &self.apbaddr_etm_cpu0_trcitctrl
    }
    #[doc = "0xfa0 - Claim Tag Set Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcclaimset(&self) -> &ApbaddrEtmCpu0Trcclaimset {
        &self.apbaddr_etm_cpu0_trcclaimset
    }
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcclaimclr(&self) -> &ApbaddrEtmCpu0Trcclaimclr {
        &self.apbaddr_etm_cpu0_trcclaimclr
    }
    #[doc = "0xfa8 - Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcdevaff0(&self) -> &ApbaddrEtmCpu0Trcdevaff0 {
        &self.apbaddr_etm_cpu0_trcdevaff0
    }
    #[doc = "0xfac - Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcdevaff1(&self) -> &ApbaddrEtmCpu0Trcdevaff1 {
        &self.apbaddr_etm_cpu0_trcdevaff1
    }
    #[doc = "0xfb0 - Software Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trclar(&self) -> &ApbaddrEtmCpu0Trclar {
        &self.apbaddr_etm_cpu0_trclar
    }
    #[doc = "0xfb4 - Software Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trclsr(&self) -> &ApbaddrEtmCpu0Trclsr {
        &self.apbaddr_etm_cpu0_trclsr
    }
    #[doc = "0xfb8 - Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcauthstatus(&self) -> &ApbaddrEtmCpu0Trcauthstatus {
        &self.apbaddr_etm_cpu0_trcauthstatus
    }
    #[doc = "0xfbc - Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcdevarch(&self) -> &ApbaddrEtmCpu0Trcdevarch {
        &self.apbaddr_etm_cpu0_trcdevarch
    }
    #[doc = "0xfc8 - Device ID Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcdevid(&self) -> &ApbaddrEtmCpu0Trcdevid {
        &self.apbaddr_etm_cpu0_trcdevid
    }
    #[doc = "0xfcc - Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcdevtype(&self) -> &ApbaddrEtmCpu0Trcdevtype {
        &self.apbaddr_etm_cpu0_trcdevtype
    }
    #[doc = "0xfd0 - Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr4(&self) -> &ApbaddrEtmCpu0Trcpidr4 {
        &self.apbaddr_etm_cpu0_trcpidr4
    }
    #[doc = "0xfd4 - Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr5(&self) -> &ApbaddrEtmCpu0Trcpidr5 {
        &self.apbaddr_etm_cpu0_trcpidr5
    }
    #[doc = "0xfd8 - Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr6(&self) -> &ApbaddrEtmCpu0Trcpidr6 {
        &self.apbaddr_etm_cpu0_trcpidr6
    }
    #[doc = "0xfdc - Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr7(&self) -> &ApbaddrEtmCpu0Trcpidr7 {
        &self.apbaddr_etm_cpu0_trcpidr7
    }
    #[doc = "0xfe0 - Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr0(&self) -> &ApbaddrEtmCpu0Trcpidr0 {
        &self.apbaddr_etm_cpu0_trcpidr0
    }
    #[doc = "0xfe4 - Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr1(&self) -> &ApbaddrEtmCpu0Trcpidr1 {
        &self.apbaddr_etm_cpu0_trcpidr1
    }
    #[doc = "0xfe8 - Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr2(&self) -> &ApbaddrEtmCpu0Trcpidr2 {
        &self.apbaddr_etm_cpu0_trcpidr2
    }
    #[doc = "0xfec - Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trcpidr3(&self) -> &ApbaddrEtmCpu0Trcpidr3 {
        &self.apbaddr_etm_cpu0_trcpidr3
    }
    #[doc = "0xff0 - Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidr0(&self) -> &ApbaddrEtmCpu0Trccidr0 {
        &self.apbaddr_etm_cpu0_trccidr0
    }
    #[doc = "0xff4 - Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidr1(&self) -> &ApbaddrEtmCpu0Trccidr1 {
        &self.apbaddr_etm_cpu0_trccidr1
    }
    #[doc = "0xff8 - Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidr2(&self) -> &ApbaddrEtmCpu0Trccidr2 {
        &self.apbaddr_etm_cpu0_trccidr2
    }
    #[doc = "0xffc - Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu0_trccidr3(&self) -> &ApbaddrEtmCpu0Trccidr3 {
        &self.apbaddr_etm_cpu0_trccidr3
    }
}
#[doc = "APBADDR_ETM_CPU0_TRCPRGCTLR (rw) register accessor: Programming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcprgctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcprgctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcprgctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPRGCTLR")]
pub type ApbaddrEtmCpu0Trcprgctlr =
    crate::Reg<apbaddr_etm_cpu0_trcprgctlr::ApbaddrEtmCpu0TrcprgctlrSpec>;
#[doc = "Programming Control Register"]
pub mod apbaddr_etm_cpu0_trcprgctlr;
#[doc = "APBADDR_ETM_CPU0_TRCSTATR (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcstatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcstatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcstatr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSTATR")]
pub type ApbaddrEtmCpu0Trcstatr = crate::Reg<apbaddr_etm_cpu0_trcstatr::ApbaddrEtmCpu0TrcstatrSpec>;
#[doc = "Status Register"]
pub mod apbaddr_etm_cpu0_trcstatr;
#[doc = "APBADDR_ETM_CPU0_TRCCONFIGR (rw) register accessor: Trace Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcconfigr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcconfigr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcconfigr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCONFIGR")]
pub type ApbaddrEtmCpu0Trcconfigr =
    crate::Reg<apbaddr_etm_cpu0_trcconfigr::ApbaddrEtmCpu0TrcconfigrSpec>;
#[doc = "Trace Configuration Register"]
pub mod apbaddr_etm_cpu0_trcconfigr;
#[doc = "APBADDR_ETM_CPU0_TRCAUXCTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcauxctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcauxctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcauxctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCAUXCTLR")]
pub type ApbaddrEtmCpu0Trcauxctlr =
    crate::Reg<apbaddr_etm_cpu0_trcauxctlr::ApbaddrEtmCpu0TrcauxctlrSpec>;
#[doc = "Auxiliary Control Register"]
pub mod apbaddr_etm_cpu0_trcauxctlr;
#[doc = "APBADDR_ETM_CPU0_TRCEVENTCTL0R (rw) register accessor: Event Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trceventctl0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trceventctl0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trceventctl0r`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCEVENTCTL0R")]
pub type ApbaddrEtmCpu0Trceventctl0r =
    crate::Reg<apbaddr_etm_cpu0_trceventctl0r::ApbaddrEtmCpu0Trceventctl0rSpec>;
#[doc = "Event Control 0 Register"]
pub mod apbaddr_etm_cpu0_trceventctl0r;
#[doc = "APBADDR_ETM_CPU0_TRCEVENTCTL1R (rw) register accessor: Event Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trceventctl1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trceventctl1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trceventctl1r`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCEVENTCTL1R")]
pub type ApbaddrEtmCpu0Trceventctl1r =
    crate::Reg<apbaddr_etm_cpu0_trceventctl1r::ApbaddrEtmCpu0Trceventctl1rSpec>;
#[doc = "Event Control 1 Register"]
pub mod apbaddr_etm_cpu0_trceventctl1r;
#[doc = "APBADDR_ETM_CPU0_TRCSTALLCTLR (rw) register accessor: Stall Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcstallctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcstallctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcstallctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSTALLCTLR")]
pub type ApbaddrEtmCpu0Trcstallctlr =
    crate::Reg<apbaddr_etm_cpu0_trcstallctlr::ApbaddrEtmCpu0TrcstallctlrSpec>;
#[doc = "Stall Control Register"]
pub mod apbaddr_etm_cpu0_trcstallctlr;
#[doc = "APBADDR_ETM_CPU0_TRCTSCTLR (rw) register accessor: Global Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trctsctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trctsctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trctsctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCTSCTLR")]
pub type ApbaddrEtmCpu0Trctsctlr =
    crate::Reg<apbaddr_etm_cpu0_trctsctlr::ApbaddrEtmCpu0TrctsctlrSpec>;
#[doc = "Global Timestamp Control Register"]
pub mod apbaddr_etm_cpu0_trctsctlr;
#[doc = "APBADDR_ETM_CPU0_TRCSYNCPR (rw) register accessor: Synchronization Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcsyncpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcsyncpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcsyncpr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSYNCPR")]
pub type ApbaddrEtmCpu0Trcsyncpr =
    crate::Reg<apbaddr_etm_cpu0_trcsyncpr::ApbaddrEtmCpu0TrcsyncprSpec>;
#[doc = "Synchronization Period Register"]
pub mod apbaddr_etm_cpu0_trcsyncpr;
#[doc = "APBADDR_ETM_CPU0_TRCCCCTLR (rw) register accessor: Cycle Count Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcccctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcccctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcccctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCCCTLR")]
pub type ApbaddrEtmCpu0Trcccctlr =
    crate::Reg<apbaddr_etm_cpu0_trcccctlr::ApbaddrEtmCpu0TrcccctlrSpec>;
#[doc = "Cycle Count Control Register"]
pub mod apbaddr_etm_cpu0_trcccctlr;
#[doc = "APBADDR_ETM_CPU0_TRCBBCTLR (rw) register accessor: Branch Broadcast Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcbbctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcbbctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcbbctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCBBCTLR")]
pub type ApbaddrEtmCpu0Trcbbctlr =
    crate::Reg<apbaddr_etm_cpu0_trcbbctlr::ApbaddrEtmCpu0TrcbbctlrSpec>;
#[doc = "Branch Broadcast Control Register"]
pub mod apbaddr_etm_cpu0_trcbbctlr;
#[doc = "APBADDR_ETM_CPU0_TRCTRACEIDR (rw) register accessor: Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trctraceidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trctraceidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trctraceidr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCTRACEIDR")]
pub type ApbaddrEtmCpu0Trctraceidr =
    crate::Reg<apbaddr_etm_cpu0_trctraceidr::ApbaddrEtmCpu0TrctraceidrSpec>;
#[doc = "Trace ID Register"]
pub mod apbaddr_etm_cpu0_trctraceidr;
#[doc = "APBADDR_ETM_CPU0_TRCVICTLR (rw) register accessor: ViewInst Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcvictlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcvictlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcvictlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCVICTLR")]
pub type ApbaddrEtmCpu0Trcvictlr =
    crate::Reg<apbaddr_etm_cpu0_trcvictlr::ApbaddrEtmCpu0TrcvictlrSpec>;
#[doc = "ViewInst Main Control Register"]
pub mod apbaddr_etm_cpu0_trcvictlr;
#[doc = "APBADDR_ETM_CPU0_TRCVIIECTLR (rw) register accessor: ViewInst Include-Exclude Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcviiectlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcviiectlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcviiectlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCVIIECTLR")]
pub type ApbaddrEtmCpu0Trcviiectlr =
    crate::Reg<apbaddr_etm_cpu0_trcviiectlr::ApbaddrEtmCpu0TrcviiectlrSpec>;
#[doc = "ViewInst Include-Exclude Control Register"]
pub mod apbaddr_etm_cpu0_trcviiectlr;
#[doc = "APBADDR_ETM_CPU0_TRCVISSCTLR (rw) register accessor: ViewInst Start-Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcvissctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcvissctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcvissctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCVISSCTLR")]
pub type ApbaddrEtmCpu0Trcvissctlr =
    crate::Reg<apbaddr_etm_cpu0_trcvissctlr::ApbaddrEtmCpu0TrcvissctlrSpec>;
#[doc = "ViewInst Start-Stop Control Register"]
pub mod apbaddr_etm_cpu0_trcvissctlr;
#[doc = "APBADDR_ETM_CPU0_TRCSEQEVR0 (rw) register accessor: Sequencer State Transition Control Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcseqevr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcseqevr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcseqevr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSEQEVR0")]
pub type ApbaddrEtmCpu0Trcseqevr0 =
    crate::Reg<apbaddr_etm_cpu0_trcseqevr0::ApbaddrEtmCpu0Trcseqevr0Spec>;
#[doc = "Sequencer State Transition Control Registers 0"]
pub mod apbaddr_etm_cpu0_trcseqevr0;
#[doc = "APBADDR_ETM_CPU0_TRCSEQEVR1 (rw) register accessor: Sequencer State Transition Control Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcseqevr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcseqevr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcseqevr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSEQEVR1")]
pub type ApbaddrEtmCpu0Trcseqevr1 =
    crate::Reg<apbaddr_etm_cpu0_trcseqevr1::ApbaddrEtmCpu0Trcseqevr1Spec>;
#[doc = "Sequencer State Transition Control Registers 1"]
pub mod apbaddr_etm_cpu0_trcseqevr1;
#[doc = "APBADDR_ETM_CPU0_TRCSEQEVR2 (rw) register accessor: Sequencer State Transition Control Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcseqevr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcseqevr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcseqevr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSEQEVR2")]
pub type ApbaddrEtmCpu0Trcseqevr2 =
    crate::Reg<apbaddr_etm_cpu0_trcseqevr2::ApbaddrEtmCpu0Trcseqevr2Spec>;
#[doc = "Sequencer State Transition Control Registers 2"]
pub mod apbaddr_etm_cpu0_trcseqevr2;
#[doc = "APBADDR_ETM_CPU0_TRCSEQRSTEVR (rw) register accessor: Sequencer Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcseqrstevr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcseqrstevr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcseqrstevr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSEQRSTEVR")]
pub type ApbaddrEtmCpu0Trcseqrstevr =
    crate::Reg<apbaddr_etm_cpu0_trcseqrstevr::ApbaddrEtmCpu0TrcseqrstevrSpec>;
#[doc = "Sequencer Reset Control Register"]
pub mod apbaddr_etm_cpu0_trcseqrstevr;
#[doc = "APBADDR_ETM_CPU0_TRCSEQSTR (rw) register accessor: Sequencer State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcseqstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcseqstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcseqstr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSEQSTR")]
pub type ApbaddrEtmCpu0Trcseqstr =
    crate::Reg<apbaddr_etm_cpu0_trcseqstr::ApbaddrEtmCpu0TrcseqstrSpec>;
#[doc = "Sequencer State Register"]
pub mod apbaddr_etm_cpu0_trcseqstr;
#[doc = "APBADDR_ETM_CPU0_TRCEXTINSELR (rw) register accessor: External Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcextinselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcextinselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcextinselr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCEXTINSELR")]
pub type ApbaddrEtmCpu0Trcextinselr =
    crate::Reg<apbaddr_etm_cpu0_trcextinselr::ApbaddrEtmCpu0TrcextinselrSpec>;
#[doc = "External Input Select Register"]
pub mod apbaddr_etm_cpu0_trcextinselr;
#[doc = "APBADDR_ETM_CPU0_TRCCNTRLDVR0 (rw) register accessor: Counter Reload Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntrldvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntrldvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntrldvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTRLDVR0")]
pub type ApbaddrEtmCpu0Trccntrldvr0 =
    crate::Reg<apbaddr_etm_cpu0_trccntrldvr0::ApbaddrEtmCpu0Trccntrldvr0Spec>;
#[doc = "Counter Reload Value Registers 0"]
pub mod apbaddr_etm_cpu0_trccntrldvr0;
#[doc = "APBADDR_ETM_CPU0_TRCCNTRLDVR1 (rw) register accessor: Counter Reload Value Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntrldvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntrldvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntrldvr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTRLDVR1")]
pub type ApbaddrEtmCpu0Trccntrldvr1 =
    crate::Reg<apbaddr_etm_cpu0_trccntrldvr1::ApbaddrEtmCpu0Trccntrldvr1Spec>;
#[doc = "Counter Reload Value Registers 1"]
pub mod apbaddr_etm_cpu0_trccntrldvr1;
#[doc = "APBADDR_ETM_CPU0_TRCCNTCTLR0 (rw) register accessor: Counter Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntctlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntctlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntctlr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTCTLR0")]
pub type ApbaddrEtmCpu0Trccntctlr0 =
    crate::Reg<apbaddr_etm_cpu0_trccntctlr0::ApbaddrEtmCpu0Trccntctlr0Spec>;
#[doc = "Counter Control Register 0"]
pub mod apbaddr_etm_cpu0_trccntctlr0;
#[doc = "APBADDR_ETM_CPU0_TRCCNTCTLR1 (rw) register accessor: Counter Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntctlr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTCTLR1")]
pub type ApbaddrEtmCpu0Trccntctlr1 =
    crate::Reg<apbaddr_etm_cpu0_trccntctlr1::ApbaddrEtmCpu0Trccntctlr1Spec>;
#[doc = "Counter Control Register 1"]
pub mod apbaddr_etm_cpu0_trccntctlr1;
#[doc = "APBADDR_ETM_CPU0_TRCCNTVR0 (rw) register accessor: Counter Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTVR0")]
pub type ApbaddrEtmCpu0Trccntvr0 =
    crate::Reg<apbaddr_etm_cpu0_trccntvr0::ApbaddrEtmCpu0Trccntvr0Spec>;
#[doc = "Counter Value Registers 0"]
pub mod apbaddr_etm_cpu0_trccntvr0;
#[doc = "APBADDR_ETM_CPU0_TRCCNTVR1 (rw) register accessor: Counter Value Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccntvr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCNTVR1")]
pub type ApbaddrEtmCpu0Trccntvr1 =
    crate::Reg<apbaddr_etm_cpu0_trccntvr1::ApbaddrEtmCpu0Trccntvr1Spec>;
#[doc = "Counter Value Registers 1"]
pub mod apbaddr_etm_cpu0_trccntvr1;
#[doc = "APBADDR_ETM_CPU0_TRCIDR8 (rw) register accessor: ID Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr8`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR8")]
pub type ApbaddrEtmCpu0Trcidr8 = crate::Reg<apbaddr_etm_cpu0_trcidr8::ApbaddrEtmCpu0Trcidr8Spec>;
#[doc = "ID Register 8"]
pub mod apbaddr_etm_cpu0_trcidr8;
#[doc = "APBADDR_ETM_CPU0_TRCIDR9 (rw) register accessor: ID Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr9`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR9")]
pub type ApbaddrEtmCpu0Trcidr9 = crate::Reg<apbaddr_etm_cpu0_trcidr9::ApbaddrEtmCpu0Trcidr9Spec>;
#[doc = "ID Register 9"]
pub mod apbaddr_etm_cpu0_trcidr9;
#[doc = "APBADDR_ETM_CPU0_TRCIDR10 (rw) register accessor: ID Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr10`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR10")]
pub type ApbaddrEtmCpu0Trcidr10 = crate::Reg<apbaddr_etm_cpu0_trcidr10::ApbaddrEtmCpu0Trcidr10Spec>;
#[doc = "ID Register 10"]
pub mod apbaddr_etm_cpu0_trcidr10;
#[doc = "APBADDR_ETM_CPU0_TRCIDR11 (rw) register accessor: ID Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr11`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR11")]
pub type ApbaddrEtmCpu0Trcidr11 = crate::Reg<apbaddr_etm_cpu0_trcidr11::ApbaddrEtmCpu0Trcidr11Spec>;
#[doc = "ID Register 11"]
pub mod apbaddr_etm_cpu0_trcidr11;
#[doc = "APBADDR_ETM_CPU0_TRCIDR12 (rw) register accessor: ID Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr12`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR12")]
pub type ApbaddrEtmCpu0Trcidr12 = crate::Reg<apbaddr_etm_cpu0_trcidr12::ApbaddrEtmCpu0Trcidr12Spec>;
#[doc = "ID Register 12"]
pub mod apbaddr_etm_cpu0_trcidr12;
#[doc = "APBADDR_ETM_CPU0_TRCIDR13 (rw) register accessor: ID Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr13`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR13")]
pub type ApbaddrEtmCpu0Trcidr13 = crate::Reg<apbaddr_etm_cpu0_trcidr13::ApbaddrEtmCpu0Trcidr13Spec>;
#[doc = "ID Register 13"]
pub mod apbaddr_etm_cpu0_trcidr13;
#[doc = "APBADDR_ETM_CPU0_TRCIMSPEC0 (rw) register accessor: Implementation Specific Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcimspec0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcimspec0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcimspec0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIMSPEC0")]
pub type ApbaddrEtmCpu0Trcimspec0 =
    crate::Reg<apbaddr_etm_cpu0_trcimspec0::ApbaddrEtmCpu0Trcimspec0Spec>;
#[doc = "Implementation Specific Register 0"]
pub mod apbaddr_etm_cpu0_trcimspec0;
#[doc = "APBADDR_ETM_CPU0_TRCIDR0 (rw) register accessor: ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR0")]
pub type ApbaddrEtmCpu0Trcidr0 = crate::Reg<apbaddr_etm_cpu0_trcidr0::ApbaddrEtmCpu0Trcidr0Spec>;
#[doc = "ID Register 0"]
pub mod apbaddr_etm_cpu0_trcidr0;
#[doc = "APBADDR_ETM_CPU0_TRCIDR1 (rw) register accessor: ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR1")]
pub type ApbaddrEtmCpu0Trcidr1 = crate::Reg<apbaddr_etm_cpu0_trcidr1::ApbaddrEtmCpu0Trcidr1Spec>;
#[doc = "ID Register 1"]
pub mod apbaddr_etm_cpu0_trcidr1;
#[doc = "APBADDR_ETM_CPU0_TRCIDR2 (rw) register accessor: ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR2")]
pub type ApbaddrEtmCpu0Trcidr2 = crate::Reg<apbaddr_etm_cpu0_trcidr2::ApbaddrEtmCpu0Trcidr2Spec>;
#[doc = "ID Register 2"]
pub mod apbaddr_etm_cpu0_trcidr2;
#[doc = "APBADDR_ETM_CPU0_TRCIDR3 (rw) register accessor: ID Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR3")]
pub type ApbaddrEtmCpu0Trcidr3 = crate::Reg<apbaddr_etm_cpu0_trcidr3::ApbaddrEtmCpu0Trcidr3Spec>;
#[doc = "ID Register 3"]
pub mod apbaddr_etm_cpu0_trcidr3;
#[doc = "APBADDR_ETM_CPU0_TRCIDR4 (rw) register accessor: ID Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR4")]
pub type ApbaddrEtmCpu0Trcidr4 = crate::Reg<apbaddr_etm_cpu0_trcidr4::ApbaddrEtmCpu0Trcidr4Spec>;
#[doc = "ID Register 4"]
pub mod apbaddr_etm_cpu0_trcidr4;
#[doc = "APBADDR_ETM_CPU0_TRCIDR5 (rw) register accessor: ID Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcidr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCIDR5")]
pub type ApbaddrEtmCpu0Trcidr5 = crate::Reg<apbaddr_etm_cpu0_trcidr5::ApbaddrEtmCpu0Trcidr5Spec>;
#[doc = "ID Register 5"]
pub mod apbaddr_etm_cpu0_trcidr5;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR2 (rw) register accessor: Resource Selection Control Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR2")]
pub type ApbaddrEtmCpu0Trcrsctlr2 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr2::ApbaddrEtmCpu0Trcrsctlr2Spec>;
#[doc = "Resource Selection Control Registers 2"]
pub mod apbaddr_etm_cpu0_trcrsctlr2;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR3 (rw) register accessor: Resource Selection Control Registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR3")]
pub type ApbaddrEtmCpu0Trcrsctlr3 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr3::ApbaddrEtmCpu0Trcrsctlr3Spec>;
#[doc = "Resource Selection Control Registers 3"]
pub mod apbaddr_etm_cpu0_trcrsctlr3;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR4 (rw) register accessor: Resource Selection Control Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR4")]
pub type ApbaddrEtmCpu0Trcrsctlr4 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr4::ApbaddrEtmCpu0Trcrsctlr4Spec>;
#[doc = "Resource Selection Control Registers 4"]
pub mod apbaddr_etm_cpu0_trcrsctlr4;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR5 (rw) register accessor: Resource Selection Control Registers 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR5")]
pub type ApbaddrEtmCpu0Trcrsctlr5 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr5::ApbaddrEtmCpu0Trcrsctlr5Spec>;
#[doc = "Resource Selection Control Registers 5"]
pub mod apbaddr_etm_cpu0_trcrsctlr5;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR6 (rw) register accessor: Resource Selection Control Registers 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR6")]
pub type ApbaddrEtmCpu0Trcrsctlr6 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr6::ApbaddrEtmCpu0Trcrsctlr6Spec>;
#[doc = "Resource Selection Control Registers 6"]
pub mod apbaddr_etm_cpu0_trcrsctlr6;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR7 (rw) register accessor: Resource Selection Control Registers 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR7")]
pub type ApbaddrEtmCpu0Trcrsctlr7 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr7::ApbaddrEtmCpu0Trcrsctlr7Spec>;
#[doc = "Resource Selection Control Registers 7"]
pub mod apbaddr_etm_cpu0_trcrsctlr7;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR8 (rw) register accessor: Resource Selection Control Registers 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr8`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR8")]
pub type ApbaddrEtmCpu0Trcrsctlr8 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr8::ApbaddrEtmCpu0Trcrsctlr8Spec>;
#[doc = "Resource Selection Control Registers 8"]
pub mod apbaddr_etm_cpu0_trcrsctlr8;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR9 (rw) register accessor: Resource Selection Control Registers 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr9`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR9")]
pub type ApbaddrEtmCpu0Trcrsctlr9 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr9::ApbaddrEtmCpu0Trcrsctlr9Spec>;
#[doc = "Resource Selection Control Registers 9"]
pub mod apbaddr_etm_cpu0_trcrsctlr9;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR10 (rw) register accessor: Resource Selection Control Registers 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr10`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR10")]
pub type ApbaddrEtmCpu0Trcrsctlr10 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr10::ApbaddrEtmCpu0Trcrsctlr10Spec>;
#[doc = "Resource Selection Control Registers 10"]
pub mod apbaddr_etm_cpu0_trcrsctlr10;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR11 (rw) register accessor: Resource Selection Control Registers 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr11`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR11")]
pub type ApbaddrEtmCpu0Trcrsctlr11 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr11::ApbaddrEtmCpu0Trcrsctlr11Spec>;
#[doc = "Resource Selection Control Registers 11"]
pub mod apbaddr_etm_cpu0_trcrsctlr11;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR12 (rw) register accessor: Resource Selection Control Registers 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr12`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR12")]
pub type ApbaddrEtmCpu0Trcrsctlr12 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr12::ApbaddrEtmCpu0Trcrsctlr12Spec>;
#[doc = "Resource Selection Control Registers 12"]
pub mod apbaddr_etm_cpu0_trcrsctlr12;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR13 (rw) register accessor: Resource Selection Control Registers 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr13`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR13")]
pub type ApbaddrEtmCpu0Trcrsctlr13 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr13::ApbaddrEtmCpu0Trcrsctlr13Spec>;
#[doc = "Resource Selection Control Registers 13"]
pub mod apbaddr_etm_cpu0_trcrsctlr13;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR14 (rw) register accessor: Resource Selection Control Registers 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr14`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR14")]
pub type ApbaddrEtmCpu0Trcrsctlr14 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr14::ApbaddrEtmCpu0Trcrsctlr14Spec>;
#[doc = "Resource Selection Control Registers 14"]
pub mod apbaddr_etm_cpu0_trcrsctlr14;
#[doc = "APBADDR_ETM_CPU0_TRCRSCTLR15 (rw) register accessor: Resource Selection Control Registers 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcrsctlr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcrsctlr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcrsctlr15`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCRSCTLR15")]
pub type ApbaddrEtmCpu0Trcrsctlr15 =
    crate::Reg<apbaddr_etm_cpu0_trcrsctlr15::ApbaddrEtmCpu0Trcrsctlr15Spec>;
#[doc = "Resource Selection Control Registers 15"]
pub mod apbaddr_etm_cpu0_trcrsctlr15;
#[doc = "APBADDR_ETM_CPU0_TRCSSCCR0 (rw) register accessor: Single-Shot Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcssccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcssccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcssccr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSSCCR0")]
pub type ApbaddrEtmCpu0Trcssccr0 =
    crate::Reg<apbaddr_etm_cpu0_trcssccr0::ApbaddrEtmCpu0Trcssccr0Spec>;
#[doc = "Single-Shot Comparator Control Register 0"]
pub mod apbaddr_etm_cpu0_trcssccr0;
#[doc = "APBADDR_ETM_CPU0_TRCSSCSR0 (rw) register accessor: Single-Shot Comparator Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcsscsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcsscsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcsscsr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCSSCSR0")]
pub type ApbaddrEtmCpu0Trcsscsr0 =
    crate::Reg<apbaddr_etm_cpu0_trcsscsr0::ApbaddrEtmCpu0Trcsscsr0Spec>;
#[doc = "Single-Shot Comparator Status Register 0"]
pub mod apbaddr_etm_cpu0_trcsscsr0;
#[doc = "APBADDR_ETM_CPU0_TRCOSLAR (rw) register accessor: OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcoslar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcoslar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcoslar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCOSLAR")]
pub type ApbaddrEtmCpu0Trcoslar = crate::Reg<apbaddr_etm_cpu0_trcoslar::ApbaddrEtmCpu0TrcoslarSpec>;
#[doc = "OS Lock Access Register"]
pub mod apbaddr_etm_cpu0_trcoslar;
#[doc = "APBADDR_ETM_CPU0_TRCOSLSR (rw) register accessor: OS Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcoslsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcoslsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcoslsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCOSLSR")]
pub type ApbaddrEtmCpu0Trcoslsr = crate::Reg<apbaddr_etm_cpu0_trcoslsr::ApbaddrEtmCpu0TrcoslsrSpec>;
#[doc = "OS Lock Status Register"]
pub mod apbaddr_etm_cpu0_trcoslsr;
#[doc = "APBADDR_ETM_CPU0_TRCPDCR (rw) register accessor: Power Down Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpdcr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPDCR")]
pub type ApbaddrEtmCpu0Trcpdcr = crate::Reg<apbaddr_etm_cpu0_trcpdcr::ApbaddrEtmCpu0TrcpdcrSpec>;
#[doc = "Power Down Control Register"]
pub mod apbaddr_etm_cpu0_trcpdcr;
#[doc = "APBADDR_ETM_CPU0_TRCPDSR (rw) register accessor: Power Down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpdsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPDSR")]
pub type ApbaddrEtmCpu0Trcpdsr = crate::Reg<apbaddr_etm_cpu0_trcpdsr::ApbaddrEtmCpu0TrcpdsrSpec>;
#[doc = "Power Down Status Register"]
pub mod apbaddr_etm_cpu0_trcpdsr;
#[doc = "APBADDR_ETM_CPU0_TRCACVR0_31_0 (rw) register accessor: Address Comparator Value Registers 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr0_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr0_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr0_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR0_31_0")]
pub type ApbaddrEtmCpu0Trcacvr0_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr0_31_0::ApbaddrEtmCpu0Trcacvr0_31_0Spec>;
#[doc = "Address Comparator Value Registers 0 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr0_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR0_63_32 (rw) register accessor: Address Comparator Value Registers 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr0_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr0_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr0_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR0_63_32")]
pub type ApbaddrEtmCpu0Trcacvr0_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr0_63_32::ApbaddrEtmCpu0Trcacvr0_63_32Spec>;
#[doc = "Address Comparator Value Registers 0 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr0_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR1_31_0 (rw) register accessor: Address Comparator Value Registers 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr1_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR1_31_0")]
pub type ApbaddrEtmCpu0Trcacvr1_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr1_31_0::ApbaddrEtmCpu0Trcacvr1_31_0Spec>;
#[doc = "Address Comparator Value Registers 1 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr1_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR1_63_32 (rw) register accessor: Address Comparator Value Registers 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr1_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR1_63_32")]
pub type ApbaddrEtmCpu0Trcacvr1_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr1_63_32::ApbaddrEtmCpu0Trcacvr1_63_32Spec>;
#[doc = "Address Comparator Value Registers 1 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr1_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR2_31_0 (rw) register accessor: Address Comparator Value Registers 2 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr2_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr2_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr2_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR2_31_0")]
pub type ApbaddrEtmCpu0Trcacvr2_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr2_31_0::ApbaddrEtmCpu0Trcacvr2_31_0Spec>;
#[doc = "Address Comparator Value Registers 2 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr2_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR2_63_32 (rw) register accessor: Address Comparator Value Registers 2 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr2_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr2_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr2_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR2_63_32")]
pub type ApbaddrEtmCpu0Trcacvr2_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr2_63_32::ApbaddrEtmCpu0Trcacvr2_63_32Spec>;
#[doc = "Address Comparator Value Registers 2 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr2_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR3_31_0 (rw) register accessor: Address Comparator Value Registers 3 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr3_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr3_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr3_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR3_31_0")]
pub type ApbaddrEtmCpu0Trcacvr3_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr3_31_0::ApbaddrEtmCpu0Trcacvr3_31_0Spec>;
#[doc = "Address Comparator Value Registers 3 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr3_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR3_63_32 (rw) register accessor: Address Comparator Value Registers 3 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr3_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr3_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr3_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR3_63_32")]
pub type ApbaddrEtmCpu0Trcacvr3_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr3_63_32::ApbaddrEtmCpu0Trcacvr3_63_32Spec>;
#[doc = "Address Comparator Value Registers 3 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr3_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR4_31_0 (rw) register accessor: Address Comparator Value Registers 4 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr4_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr4_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr4_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR4_31_0")]
pub type ApbaddrEtmCpu0Trcacvr4_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr4_31_0::ApbaddrEtmCpu0Trcacvr4_31_0Spec>;
#[doc = "Address Comparator Value Registers 4 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr4_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR4_63_32 (rw) register accessor: Address Comparator Value Registers 4 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr4_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr4_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr4_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR4_63_32")]
pub type ApbaddrEtmCpu0Trcacvr4_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr4_63_32::ApbaddrEtmCpu0Trcacvr4_63_32Spec>;
#[doc = "Address Comparator Value Registers 4 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr4_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR5_31_0 (rw) register accessor: Address Comparator Value Registers 5 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr5_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr5_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr5_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR5_31_0")]
pub type ApbaddrEtmCpu0Trcacvr5_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr5_31_0::ApbaddrEtmCpu0Trcacvr5_31_0Spec>;
#[doc = "Address Comparator Value Registers 5 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr5_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR5_63_32 (rw) register accessor: Address Comparator Value Registers 5 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr5_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr5_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr5_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR5_63_32")]
pub type ApbaddrEtmCpu0Trcacvr5_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr5_63_32::ApbaddrEtmCpu0Trcacvr5_63_32Spec>;
#[doc = "Address Comparator Value Registers 5 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr5_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR6_31_0 (rw) register accessor: Address Comparator Value Registers 6 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr6_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr6_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr6_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR6_31_0")]
pub type ApbaddrEtmCpu0Trcacvr6_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr6_31_0::ApbaddrEtmCpu0Trcacvr6_31_0Spec>;
#[doc = "Address Comparator Value Registers 6 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr6_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR6_63_32 (rw) register accessor: Address Comparator Value Registers 6 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr6_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr6_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr6_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR6_63_32")]
pub type ApbaddrEtmCpu0Trcacvr6_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr6_63_32::ApbaddrEtmCpu0Trcacvr6_63_32Spec>;
#[doc = "Address Comparator Value Registers 6 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr6_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACVR7_31_0 (rw) register accessor: Address Comparator Value Registers 7 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr7_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr7_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr7_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR7_31_0")]
pub type ApbaddrEtmCpu0Trcacvr7_31_0 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr7_31_0::ApbaddrEtmCpu0Trcacvr7_31_0Spec>;
#[doc = "Address Comparator Value Registers 7 (low word)"]
pub mod apbaddr_etm_cpu0_trcacvr7_31_0;
#[doc = "APBADDR_ETM_CPU0_TRCACVR7_63_32 (rw) register accessor: Address Comparator Value Registers 7 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr7_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr7_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacvr7_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACVR7_63_32")]
pub type ApbaddrEtmCpu0Trcacvr7_63_32 =
    crate::Reg<apbaddr_etm_cpu0_trcacvr7_63_32::ApbaddrEtmCpu0Trcacvr7_63_32Spec>;
#[doc = "Address Comparator Value Registers 7 (high word)"]
pub mod apbaddr_etm_cpu0_trcacvr7_63_32;
#[doc = "APBADDR_ETM_CPU0_TRCACATR0 (rw) register accessor: Address Comparator Access Type Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR0")]
pub type ApbaddrEtmCpu0Trcacatr0 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr0::ApbaddrEtmCpu0Trcacatr0Spec>;
#[doc = "Address Comparator Access Type Registers 0"]
pub mod apbaddr_etm_cpu0_trcacatr0;
#[doc = "APBADDR_ETM_CPU0_TRCACATR1 (rw) register accessor: Address Comparator Access Type Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR1")]
pub type ApbaddrEtmCpu0Trcacatr1 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr1::ApbaddrEtmCpu0Trcacatr1Spec>;
#[doc = "Address Comparator Access Type Registers 1"]
pub mod apbaddr_etm_cpu0_trcacatr1;
#[doc = "APBADDR_ETM_CPU0_TRCACATR2 (rw) register accessor: Address Comparator Access Type Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR2")]
pub type ApbaddrEtmCpu0Trcacatr2 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr2::ApbaddrEtmCpu0Trcacatr2Spec>;
#[doc = "Address Comparator Access Type Registers 2"]
pub mod apbaddr_etm_cpu0_trcacatr2;
#[doc = "APBADDR_ETM_CPU0_TRCACATR3 (rw) register accessor: Address Comparator Access Type Registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR3")]
pub type ApbaddrEtmCpu0Trcacatr3 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr3::ApbaddrEtmCpu0Trcacatr3Spec>;
#[doc = "Address Comparator Access Type Registers 3"]
pub mod apbaddr_etm_cpu0_trcacatr3;
#[doc = "APBADDR_ETM_CPU0_TRCACATR4 (rw) register accessor: Address Comparator Access Type Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR4")]
pub type ApbaddrEtmCpu0Trcacatr4 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr4::ApbaddrEtmCpu0Trcacatr4Spec>;
#[doc = "Address Comparator Access Type Registers 4"]
pub mod apbaddr_etm_cpu0_trcacatr4;
#[doc = "APBADDR_ETM_CPU0_TRCACATR5 (rw) register accessor: Address Comparator Access Type Registers 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR5")]
pub type ApbaddrEtmCpu0Trcacatr5 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr5::ApbaddrEtmCpu0Trcacatr5Spec>;
#[doc = "Address Comparator Access Type Registers 5"]
pub mod apbaddr_etm_cpu0_trcacatr5;
#[doc = "APBADDR_ETM_CPU0_TRCACATR6 (rw) register accessor: Address Comparator Access Type Registers 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR6")]
pub type ApbaddrEtmCpu0Trcacatr6 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr6::ApbaddrEtmCpu0Trcacatr6Spec>;
#[doc = "Address Comparator Access Type Registers 6"]
pub mod apbaddr_etm_cpu0_trcacatr6;
#[doc = "APBADDR_ETM_CPU0_TRCACATR7 (rw) register accessor: Address Comparator Access Type Registers 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacatr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacatr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcacatr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCACATR7")]
pub type ApbaddrEtmCpu0Trcacatr7 =
    crate::Reg<apbaddr_etm_cpu0_trcacatr7::ApbaddrEtmCpu0Trcacatr7Spec>;
#[doc = "Address Comparator Access Type Registers 7"]
pub mod apbaddr_etm_cpu0_trcacatr7;
#[doc = "APBADDR_ETM_CPU0_TRCCIDCVR0 (rw) register accessor: Context ID Comparator Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidcvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidcvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidcvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDCVR0")]
pub type ApbaddrEtmCpu0Trccidcvr0 =
    crate::Reg<apbaddr_etm_cpu0_trccidcvr0::ApbaddrEtmCpu0Trccidcvr0Spec>;
#[doc = "Context ID Comparator Value Register 0"]
pub mod apbaddr_etm_cpu0_trccidcvr0;
#[doc = "APBADDR_ETM_CPU0_TRCVMIDCVR0 (rw) register accessor: VMID Comparator Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcvmidcvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcvmidcvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcvmidcvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCVMIDCVR0")]
pub type ApbaddrEtmCpu0Trcvmidcvr0 =
    crate::Reg<apbaddr_etm_cpu0_trcvmidcvr0::ApbaddrEtmCpu0Trcvmidcvr0Spec>;
#[doc = "VMID Comparator Value Register 0"]
pub mod apbaddr_etm_cpu0_trcvmidcvr0;
#[doc = "APBADDR_ETM_CPU0_TRCCIDCCTLR0 (rw) register accessor: Context ID Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidcctlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidcctlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidcctlr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDCCTLR0")]
pub type ApbaddrEtmCpu0Trccidcctlr0 =
    crate::Reg<apbaddr_etm_cpu0_trccidcctlr0::ApbaddrEtmCpu0Trccidcctlr0Spec>;
#[doc = "Context ID Comparator Control Register 0"]
pub mod apbaddr_etm_cpu0_trccidcctlr0;
#[doc = "APBADDR_ETM_CPU0_TRCITATBIDR (rw) register accessor: Integration ATB Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitatbidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitatbidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcitatbidr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCITATBIDR")]
pub type ApbaddrEtmCpu0Trcitatbidr =
    crate::Reg<apbaddr_etm_cpu0_trcitatbidr::ApbaddrEtmCpu0TrcitatbidrSpec>;
#[doc = "Integration ATB Identification Register"]
pub mod apbaddr_etm_cpu0_trcitatbidr;
#[doc = "APBADDR_ETM_CPU0_TRCITIDATAR (rw) register accessor: Integration Instruction ATB Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitidatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitidatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcitidatar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCITIDATAR")]
pub type ApbaddrEtmCpu0Trcitidatar =
    crate::Reg<apbaddr_etm_cpu0_trcitidatar::ApbaddrEtmCpu0TrcitidatarSpec>;
#[doc = "Integration Instruction ATB Data Register"]
pub mod apbaddr_etm_cpu0_trcitidatar;
#[doc = "APBADDR_ETM_CPU0_TRCITIATBINR (rw) register accessor: Integration Instruction ATB In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitiatbinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitiatbinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcitiatbinr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCITIATBINR")]
pub type ApbaddrEtmCpu0Trcitiatbinr =
    crate::Reg<apbaddr_etm_cpu0_trcitiatbinr::ApbaddrEtmCpu0TrcitiatbinrSpec>;
#[doc = "Integration Instruction ATB In Register"]
pub mod apbaddr_etm_cpu0_trcitiatbinr;
#[doc = "APBADDR_ETM_CPU0_TRCITIATBOUTR (rw) register accessor: Integration Instruction ATB Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitiatboutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitiatboutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcitiatboutr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCITIATBOUTR")]
pub type ApbaddrEtmCpu0Trcitiatboutr =
    crate::Reg<apbaddr_etm_cpu0_trcitiatboutr::ApbaddrEtmCpu0TrcitiatboutrSpec>;
#[doc = "Integration Instruction ATB Out Register"]
pub mod apbaddr_etm_cpu0_trcitiatboutr;
#[doc = "APBADDR_ETM_CPU0_TRCITCTRL (rw) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcitctrl`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCITCTRL")]
pub type ApbaddrEtmCpu0Trcitctrl =
    crate::Reg<apbaddr_etm_cpu0_trcitctrl::ApbaddrEtmCpu0TrcitctrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod apbaddr_etm_cpu0_trcitctrl;
#[doc = "APBADDR_ETM_CPU0_TRCCLAIMSET (rw) register accessor: Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcclaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcclaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcclaimset`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCLAIMSET")]
pub type ApbaddrEtmCpu0Trcclaimset =
    crate::Reg<apbaddr_etm_cpu0_trcclaimset::ApbaddrEtmCpu0TrcclaimsetSpec>;
#[doc = "Claim Tag Set Register"]
pub mod apbaddr_etm_cpu0_trcclaimset;
#[doc = "APBADDR_ETM_CPU0_TRCCLAIMCLR (rw) register accessor: Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcclaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcclaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcclaimclr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCLAIMCLR")]
pub type ApbaddrEtmCpu0Trcclaimclr =
    crate::Reg<apbaddr_etm_cpu0_trcclaimclr::ApbaddrEtmCpu0TrcclaimclrSpec>;
#[doc = "Claim Tag Clear Register"]
pub mod apbaddr_etm_cpu0_trcclaimclr;
#[doc = "APBADDR_ETM_CPU0_TRCDEVAFF0 (rw) register accessor: Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcdevaff0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCDEVAFF0")]
pub type ApbaddrEtmCpu0Trcdevaff0 =
    crate::Reg<apbaddr_etm_cpu0_trcdevaff0::ApbaddrEtmCpu0Trcdevaff0Spec>;
#[doc = "Device Affinity Register 0"]
pub mod apbaddr_etm_cpu0_trcdevaff0;
#[doc = "APBADDR_ETM_CPU0_TRCDEVAFF1 (rw) register accessor: Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcdevaff1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCDEVAFF1")]
pub type ApbaddrEtmCpu0Trcdevaff1 =
    crate::Reg<apbaddr_etm_cpu0_trcdevaff1::ApbaddrEtmCpu0Trcdevaff1Spec>;
#[doc = "Device Affinity Register 1"]
pub mod apbaddr_etm_cpu0_trcdevaff1;
#[doc = "APBADDR_ETM_CPU0_TRCLAR (rw) register accessor: Software Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trclar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trclar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trclar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCLAR")]
pub type ApbaddrEtmCpu0Trclar = crate::Reg<apbaddr_etm_cpu0_trclar::ApbaddrEtmCpu0TrclarSpec>;
#[doc = "Software Lock Access Register"]
pub mod apbaddr_etm_cpu0_trclar;
#[doc = "APBADDR_ETM_CPU0_TRCLSR (rw) register accessor: Software Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trclsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trclsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trclsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCLSR")]
pub type ApbaddrEtmCpu0Trclsr = crate::Reg<apbaddr_etm_cpu0_trclsr::ApbaddrEtmCpu0TrclsrSpec>;
#[doc = "Software Lock Status Register"]
pub mod apbaddr_etm_cpu0_trclsr;
#[doc = "APBADDR_ETM_CPU0_TRCAUTHSTATUS (rw) register accessor: Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcauthstatus`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCAUTHSTATUS")]
pub type ApbaddrEtmCpu0Trcauthstatus =
    crate::Reg<apbaddr_etm_cpu0_trcauthstatus::ApbaddrEtmCpu0TrcauthstatusSpec>;
#[doc = "Authentication Status Register"]
pub mod apbaddr_etm_cpu0_trcauthstatus;
#[doc = "APBADDR_ETM_CPU0_TRCDEVARCH (rw) register accessor: Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcdevarch`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCDEVARCH")]
pub type ApbaddrEtmCpu0Trcdevarch =
    crate::Reg<apbaddr_etm_cpu0_trcdevarch::ApbaddrEtmCpu0TrcdevarchSpec>;
#[doc = "Device Architecture Register"]
pub mod apbaddr_etm_cpu0_trcdevarch;
#[doc = "APBADDR_ETM_CPU0_TRCDEVID (rw) register accessor: Device ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcdevid`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCDEVID")]
pub type ApbaddrEtmCpu0Trcdevid = crate::Reg<apbaddr_etm_cpu0_trcdevid::ApbaddrEtmCpu0TrcdevidSpec>;
#[doc = "Device ID Register"]
pub mod apbaddr_etm_cpu0_trcdevid;
#[doc = "APBADDR_ETM_CPU0_TRCDEVTYPE (rw) register accessor: Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcdevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcdevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcdevtype`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCDEVTYPE")]
pub type ApbaddrEtmCpu0Trcdevtype =
    crate::Reg<apbaddr_etm_cpu0_trcdevtype::ApbaddrEtmCpu0TrcdevtypeSpec>;
#[doc = "Device Type Register"]
pub mod apbaddr_etm_cpu0_trcdevtype;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR4 (rw) register accessor: Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR4")]
pub type ApbaddrEtmCpu0Trcpidr4 = crate::Reg<apbaddr_etm_cpu0_trcpidr4::ApbaddrEtmCpu0Trcpidr4Spec>;
#[doc = "Peripheral Identification Register 4"]
pub mod apbaddr_etm_cpu0_trcpidr4;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR5 (rw) register accessor: Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR5")]
pub type ApbaddrEtmCpu0Trcpidr5 = crate::Reg<apbaddr_etm_cpu0_trcpidr5::ApbaddrEtmCpu0Trcpidr5Spec>;
#[doc = "Peripheral Identification Register 5"]
pub mod apbaddr_etm_cpu0_trcpidr5;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR6 (rw) register accessor: Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR6")]
pub type ApbaddrEtmCpu0Trcpidr6 = crate::Reg<apbaddr_etm_cpu0_trcpidr6::ApbaddrEtmCpu0Trcpidr6Spec>;
#[doc = "Peripheral Identification Register 6"]
pub mod apbaddr_etm_cpu0_trcpidr6;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR7 (rw) register accessor: Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR7")]
pub type ApbaddrEtmCpu0Trcpidr7 = crate::Reg<apbaddr_etm_cpu0_trcpidr7::ApbaddrEtmCpu0Trcpidr7Spec>;
#[doc = "Peripheral Identification Register 7"]
pub mod apbaddr_etm_cpu0_trcpidr7;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR0 (rw) register accessor: Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR0")]
pub type ApbaddrEtmCpu0Trcpidr0 = crate::Reg<apbaddr_etm_cpu0_trcpidr0::ApbaddrEtmCpu0Trcpidr0Spec>;
#[doc = "Peripheral Identification Register 0"]
pub mod apbaddr_etm_cpu0_trcpidr0;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR1 (rw) register accessor: Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR1")]
pub type ApbaddrEtmCpu0Trcpidr1 = crate::Reg<apbaddr_etm_cpu0_trcpidr1::ApbaddrEtmCpu0Trcpidr1Spec>;
#[doc = "Peripheral Identification Register 1"]
pub mod apbaddr_etm_cpu0_trcpidr1;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR2 (rw) register accessor: Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR2")]
pub type ApbaddrEtmCpu0Trcpidr2 = crate::Reg<apbaddr_etm_cpu0_trcpidr2::ApbaddrEtmCpu0Trcpidr2Spec>;
#[doc = "Peripheral Identification Register 2"]
pub mod apbaddr_etm_cpu0_trcpidr2;
#[doc = "APBADDR_ETM_CPU0_TRCPIDR3 (rw) register accessor: Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trcpidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCPIDR3")]
pub type ApbaddrEtmCpu0Trcpidr3 = crate::Reg<apbaddr_etm_cpu0_trcpidr3::ApbaddrEtmCpu0Trcpidr3Spec>;
#[doc = "Peripheral Identification Register 3"]
pub mod apbaddr_etm_cpu0_trcpidr3;
#[doc = "APBADDR_ETM_CPU0_TRCCIDR0 (rw) register accessor: Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDR0")]
pub type ApbaddrEtmCpu0Trccidr0 = crate::Reg<apbaddr_etm_cpu0_trccidr0::ApbaddrEtmCpu0Trccidr0Spec>;
#[doc = "Component Identification Register 0"]
pub mod apbaddr_etm_cpu0_trccidr0;
#[doc = "APBADDR_ETM_CPU0_TRCCIDR1 (rw) register accessor: Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDR1")]
pub type ApbaddrEtmCpu0Trccidr1 = crate::Reg<apbaddr_etm_cpu0_trccidr1::ApbaddrEtmCpu0Trccidr1Spec>;
#[doc = "Component Identification Register 1"]
pub mod apbaddr_etm_cpu0_trccidr1;
#[doc = "APBADDR_ETM_CPU0_TRCCIDR2 (rw) register accessor: Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDR2")]
pub type ApbaddrEtmCpu0Trccidr2 = crate::Reg<apbaddr_etm_cpu0_trccidr2::ApbaddrEtmCpu0Trccidr2Spec>;
#[doc = "Component Identification Register 2"]
pub mod apbaddr_etm_cpu0_trccidr2;
#[doc = "APBADDR_ETM_CPU0_TRCCIDR3 (rw) register accessor: Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu0_trccidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU0_TRCCIDR3")]
pub type ApbaddrEtmCpu0Trccidr3 = crate::Reg<apbaddr_etm_cpu0_trccidr3::ApbaddrEtmCpu0Trccidr3Spec>;
#[doc = "Component Identification Register 3"]
pub mod apbaddr_etm_cpu0_trccidr3;
