#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    apbaddr_etm_cpu1_trcprgctlr: ApbaddrEtmCpu1Trcprgctlr,
    _reserved1: [u8; 0x04],
    apbaddr_etm_cpu1_trcstatr: ApbaddrEtmCpu1Trcstatr,
    apbaddr_etm_cpu1_trcconfigr: ApbaddrEtmCpu1Trcconfigr,
    _reserved3: [u8; 0x04],
    apbaddr_etm_cpu1_trcauxctlr: ApbaddrEtmCpu1Trcauxctlr,
    _reserved4: [u8; 0x04],
    apbaddr_etm_cpu1_trceventctl0r: ApbaddrEtmCpu1Trceventctl0r,
    apbaddr_etm_cpu1_trceventctl1r: ApbaddrEtmCpu1Trceventctl1r,
    _reserved6: [u8; 0x04],
    apbaddr_etm_cpu1_trcstallctlr: ApbaddrEtmCpu1Trcstallctlr,
    apbaddr_etm_cpu1_trctsctlr: ApbaddrEtmCpu1Trctsctlr,
    apbaddr_etm_cpu1_trcsyncpr: ApbaddrEtmCpu1Trcsyncpr,
    apbaddr_etm_cpu1_trcccctlr: ApbaddrEtmCpu1Trcccctlr,
    apbaddr_etm_cpu1_trcbbctlr: ApbaddrEtmCpu1Trcbbctlr,
    apbaddr_etm_cpu1_trctraceidr: ApbaddrEtmCpu1Trctraceidr,
    _reserved12: [u8; 0x3c],
    apbaddr_etm_cpu1_trcvictlr: ApbaddrEtmCpu1Trcvictlr,
    apbaddr_etm_cpu1_trcviiectlr: ApbaddrEtmCpu1Trcviiectlr,
    apbaddr_etm_cpu1_trcvissctlr: ApbaddrEtmCpu1Trcvissctlr,
    _reserved15: [u8; 0x74],
    apbaddr_etm_cpu1_trcseqevr0: ApbaddrEtmCpu1Trcseqevr0,
    apbaddr_etm_cpu1_trcseqevr1: ApbaddrEtmCpu1Trcseqevr1,
    apbaddr_etm_cpu1_trcseqevr2: ApbaddrEtmCpu1Trcseqevr2,
    _reserved18: [u8; 0x0c],
    apbaddr_etm_cpu1_trcseqrstevr: ApbaddrEtmCpu1Trcseqrstevr,
    apbaddr_etm_cpu1_trcseqstr: ApbaddrEtmCpu1Trcseqstr,
    apbaddr_etm_cpu1_trcextinselr: ApbaddrEtmCpu1Trcextinselr,
    _reserved21: [u8; 0x1c],
    apbaddr_etm_cpu1_trccntrldvr0: ApbaddrEtmCpu1Trccntrldvr0,
    apbaddr_etm_cpu1_trccntrldvr1: ApbaddrEtmCpu1Trccntrldvr1,
    _reserved23: [u8; 0x08],
    apbaddr_etm_cpu1_trccntctlr0: ApbaddrEtmCpu1Trccntctlr0,
    apbaddr_etm_cpu1_trccntctlr1: ApbaddrEtmCpu1Trccntctlr1,
    _reserved25: [u8; 0x08],
    apbaddr_etm_cpu1_trccntvr0: ApbaddrEtmCpu1Trccntvr0,
    apbaddr_etm_cpu1_trccntvr1: ApbaddrEtmCpu1Trccntvr1,
    _reserved27: [u8; 0x18],
    apbaddr_etm_cpu1_trcidr8: ApbaddrEtmCpu1Trcidr8,
    apbaddr_etm_cpu1_trcidr9: ApbaddrEtmCpu1Trcidr9,
    apbaddr_etm_cpu1_trcidr10: ApbaddrEtmCpu1Trcidr10,
    apbaddr_etm_cpu1_trcidr11: ApbaddrEtmCpu1Trcidr11,
    apbaddr_etm_cpu1_trcidr12: ApbaddrEtmCpu1Trcidr12,
    apbaddr_etm_cpu1_trcidr13: ApbaddrEtmCpu1Trcidr13,
    _reserved33: [u8; 0x28],
    apbaddr_etm_cpu1_trcimspec0: ApbaddrEtmCpu1Trcimspec0,
    _reserved34: [u8; 0x1c],
    apbaddr_etm_cpu1_trcidr0: ApbaddrEtmCpu1Trcidr0,
    apbaddr_etm_cpu1_trcidr1: ApbaddrEtmCpu1Trcidr1,
    apbaddr_etm_cpu1_trcidr2: ApbaddrEtmCpu1Trcidr2,
    apbaddr_etm_cpu1_trcidr3: ApbaddrEtmCpu1Trcidr3,
    apbaddr_etm_cpu1_trcidr4: ApbaddrEtmCpu1Trcidr4,
    apbaddr_etm_cpu1_trcidr5: ApbaddrEtmCpu1Trcidr5,
    _reserved40: [u8; 0x10],
    apbaddr_etm_cpu1_trcrsctlr2: ApbaddrEtmCpu1Trcrsctlr2,
    apbaddr_etm_cpu1_trcrsctlr3: ApbaddrEtmCpu1Trcrsctlr3,
    apbaddr_etm_cpu1_trcrsctlr4: ApbaddrEtmCpu1Trcrsctlr4,
    apbaddr_etm_cpu1_trcrsctlr5: ApbaddrEtmCpu1Trcrsctlr5,
    apbaddr_etm_cpu1_trcrsctlr6: ApbaddrEtmCpu1Trcrsctlr6,
    apbaddr_etm_cpu1_trcrsctlr7: ApbaddrEtmCpu1Trcrsctlr7,
    apbaddr_etm_cpu1_trcrsctlr8: ApbaddrEtmCpu1Trcrsctlr8,
    apbaddr_etm_cpu1_trcrsctlr9: ApbaddrEtmCpu1Trcrsctlr9,
    apbaddr_etm_cpu1_trcrsctlr10: ApbaddrEtmCpu1Trcrsctlr10,
    apbaddr_etm_cpu1_trcrsctlr11: ApbaddrEtmCpu1Trcrsctlr11,
    apbaddr_etm_cpu1_trcrsctlr12: ApbaddrEtmCpu1Trcrsctlr12,
    apbaddr_etm_cpu1_trcrsctlr13: ApbaddrEtmCpu1Trcrsctlr13,
    apbaddr_etm_cpu1_trcrsctlr14: ApbaddrEtmCpu1Trcrsctlr14,
    apbaddr_etm_cpu1_trcrsctlr15: ApbaddrEtmCpu1Trcrsctlr15,
    _reserved54: [u8; 0x40],
    apbaddr_etm_cpu1_trcssccr0: ApbaddrEtmCpu1Trcssccr0,
    _reserved55: [u8; 0x1c],
    apbaddr_etm_cpu1_trcsscsr0: ApbaddrEtmCpu1Trcsscsr0,
    _reserved56: [u8; 0x5c],
    apbaddr_etm_cpu1_trcoslar: ApbaddrEtmCpu1Trcoslar,
    apbaddr_etm_cpu1_trcoslsr: ApbaddrEtmCpu1Trcoslsr,
    _reserved58: [u8; 0x08],
    apbaddr_etm_cpu1_trcpdcr: ApbaddrEtmCpu1Trcpdcr,
    apbaddr_etm_cpu1_trcpdsr: ApbaddrEtmCpu1Trcpdsr,
    _reserved60: [u8; 0xe8],
    apbaddr_etm_cpu1_trcacvr0_31_0: ApbaddrEtmCpu1Trcacvr0_31_0,
    apbaddr_etm_cpu1_trcacvr0_63_32: ApbaddrEtmCpu1Trcacvr0_63_32,
    apbaddr_etm_cpu1_trcacvr1_31_0: ApbaddrEtmCpu1Trcacvr1_31_0,
    apbaddr_etm_cpu1_trcacvr1_63_32: ApbaddrEtmCpu1Trcacvr1_63_32,
    apbaddr_etm_cpu1_trcacvr2_31_0: ApbaddrEtmCpu1Trcacvr2_31_0,
    apbaddr_etm_cpu1_trcacvr2_63_32: ApbaddrEtmCpu1Trcacvr2_63_32,
    apbaddr_etm_cpu1_trcacvr3_31_0: ApbaddrEtmCpu1Trcacvr3_31_0,
    apbaddr_etm_cpu1_trcacvr3_63_32: ApbaddrEtmCpu1Trcacvr3_63_32,
    apbaddr_etm_cpu1_trcacvr4_31_0: ApbaddrEtmCpu1Trcacvr4_31_0,
    apbaddr_etm_cpu1_trcacvr4_63_32: ApbaddrEtmCpu1Trcacvr4_63_32,
    apbaddr_etm_cpu1_trcacvr5_31_0: ApbaddrEtmCpu1Trcacvr5_31_0,
    apbaddr_etm_cpu1_trcacvr5_63_32: ApbaddrEtmCpu1Trcacvr5_63_32,
    apbaddr_etm_cpu1_trcacvr6_31_0: ApbaddrEtmCpu1Trcacvr6_31_0,
    apbaddr_etm_cpu1_trcacvr6_63_32: ApbaddrEtmCpu1Trcacvr6_63_32,
    apbaddr_etm_cpu1_trcacvr7_31_0: ApbaddrEtmCpu1Trcacvr7_31_0,
    apbaddr_etm_cpu1_trcacvr7_63_32: ApbaddrEtmCpu1Trcacvr7_63_32,
    _reserved76: [u8; 0x40],
    apbaddr_etm_cpu1_trcacatr0: ApbaddrEtmCpu1Trcacatr0,
    _reserved77: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr1: ApbaddrEtmCpu1Trcacatr1,
    _reserved78: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr2: ApbaddrEtmCpu1Trcacatr2,
    _reserved79: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr3: ApbaddrEtmCpu1Trcacatr3,
    _reserved80: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr4: ApbaddrEtmCpu1Trcacatr4,
    _reserved81: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr5: ApbaddrEtmCpu1Trcacatr5,
    _reserved82: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr6: ApbaddrEtmCpu1Trcacatr6,
    _reserved83: [u8; 0x04],
    apbaddr_etm_cpu1_trcacatr7: ApbaddrEtmCpu1Trcacatr7,
    _reserved84: [u8; 0x0144],
    apbaddr_etm_cpu1_trccidcvr0: ApbaddrEtmCpu1Trccidcvr0,
    _reserved85: [u8; 0x3c],
    apbaddr_etm_cpu1_trcvmidcvr0: ApbaddrEtmCpu1Trcvmidcvr0,
    _reserved86: [u8; 0x3c],
    apbaddr_etm_cpu1_trccidcctlr0: ApbaddrEtmCpu1Trccidcctlr0,
    _reserved87: [u8; 0x0860],
    apbaddr_etm_cpu1_trcitatbidr: ApbaddrEtmCpu1Trcitatbidr,
    _reserved88: [u8; 0x04],
    apbaddr_etm_cpu1_trcitidatar: ApbaddrEtmCpu1Trcitidatar,
    _reserved89: [u8; 0x04],
    apbaddr_etm_cpu1_trcitiatbinr: ApbaddrEtmCpu1Trcitiatbinr,
    _reserved90: [u8; 0x04],
    apbaddr_etm_cpu1_trcitiatboutr: ApbaddrEtmCpu1Trcitiatboutr,
    apbaddr_etm_cpu1_trcitctrl: ApbaddrEtmCpu1Trcitctrl,
    _reserved92: [u8; 0x9c],
    apbaddr_etm_cpu1_trcclaimset: ApbaddrEtmCpu1Trcclaimset,
    apbaddr_etm_cpu1_trcclaimclr: ApbaddrEtmCpu1Trcclaimclr,
    apbaddr_etm_cpu1_trcdevaff0: ApbaddrEtmCpu1Trcdevaff0,
    apbaddr_etm_cpu1_trcdevaff1: ApbaddrEtmCpu1Trcdevaff1,
    apbaddr_etm_cpu1_trclar: ApbaddrEtmCpu1Trclar,
    apbaddr_etm_cpu1_trclsr: ApbaddrEtmCpu1Trclsr,
    apbaddr_etm_cpu1_trcauthstatus: ApbaddrEtmCpu1Trcauthstatus,
    apbaddr_etm_cpu1_trcdevarch: ApbaddrEtmCpu1Trcdevarch,
    _reserved100: [u8; 0x08],
    apbaddr_etm_cpu1_trcdevid: ApbaddrEtmCpu1Trcdevid,
    apbaddr_etm_cpu1_trcdevtype: ApbaddrEtmCpu1Trcdevtype,
    apbaddr_etm_cpu1_trcpidr4: ApbaddrEtmCpu1Trcpidr4,
    apbaddr_etm_cpu1_trcpidr5: ApbaddrEtmCpu1Trcpidr5,
    apbaddr_etm_cpu1_trcpidr6: ApbaddrEtmCpu1Trcpidr6,
    apbaddr_etm_cpu1_trcpidr7: ApbaddrEtmCpu1Trcpidr7,
    apbaddr_etm_cpu1_trcpidr0: ApbaddrEtmCpu1Trcpidr0,
    apbaddr_etm_cpu1_trcpidr1: ApbaddrEtmCpu1Trcpidr1,
    apbaddr_etm_cpu1_trcpidr2: ApbaddrEtmCpu1Trcpidr2,
    apbaddr_etm_cpu1_trcpidr3: ApbaddrEtmCpu1Trcpidr3,
    apbaddr_etm_cpu1_trccidr0: ApbaddrEtmCpu1Trccidr0,
    apbaddr_etm_cpu1_trccidr1: ApbaddrEtmCpu1Trccidr1,
    apbaddr_etm_cpu1_trccidr2: ApbaddrEtmCpu1Trccidr2,
    apbaddr_etm_cpu1_trccidr3: ApbaddrEtmCpu1Trccidr3,
}
impl RegisterBlock {
    #[doc = "0x04 - Programming Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcprgctlr(&self) -> &ApbaddrEtmCpu1Trcprgctlr {
        &self.apbaddr_etm_cpu1_trcprgctlr
    }
    #[doc = "0x0c - Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcstatr(&self) -> &ApbaddrEtmCpu1Trcstatr {
        &self.apbaddr_etm_cpu1_trcstatr
    }
    #[doc = "0x10 - Trace Configuration Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcconfigr(&self) -> &ApbaddrEtmCpu1Trcconfigr {
        &self.apbaddr_etm_cpu1_trcconfigr
    }
    #[doc = "0x18 - Auxiliary Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcauxctlr(&self) -> &ApbaddrEtmCpu1Trcauxctlr {
        &self.apbaddr_etm_cpu1_trcauxctlr
    }
    #[doc = "0x20 - Event Control 0 Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trceventctl0r(&self) -> &ApbaddrEtmCpu1Trceventctl0r {
        &self.apbaddr_etm_cpu1_trceventctl0r
    }
    #[doc = "0x24 - Event Control 1 Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trceventctl1r(&self) -> &ApbaddrEtmCpu1Trceventctl1r {
        &self.apbaddr_etm_cpu1_trceventctl1r
    }
    #[doc = "0x2c - Stall Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcstallctlr(&self) -> &ApbaddrEtmCpu1Trcstallctlr {
        &self.apbaddr_etm_cpu1_trcstallctlr
    }
    #[doc = "0x30 - Global Timestamp Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trctsctlr(&self) -> &ApbaddrEtmCpu1Trctsctlr {
        &self.apbaddr_etm_cpu1_trctsctlr
    }
    #[doc = "0x34 - Synchronization Period Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcsyncpr(&self) -> &ApbaddrEtmCpu1Trcsyncpr {
        &self.apbaddr_etm_cpu1_trcsyncpr
    }
    #[doc = "0x38 - Cycle Count Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcccctlr(&self) -> &ApbaddrEtmCpu1Trcccctlr {
        &self.apbaddr_etm_cpu1_trcccctlr
    }
    #[doc = "0x3c - Branch Broadcast Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcbbctlr(&self) -> &ApbaddrEtmCpu1Trcbbctlr {
        &self.apbaddr_etm_cpu1_trcbbctlr
    }
    #[doc = "0x40 - Trace ID Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trctraceidr(&self) -> &ApbaddrEtmCpu1Trctraceidr {
        &self.apbaddr_etm_cpu1_trctraceidr
    }
    #[doc = "0x80 - ViewInst Main Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcvictlr(&self) -> &ApbaddrEtmCpu1Trcvictlr {
        &self.apbaddr_etm_cpu1_trcvictlr
    }
    #[doc = "0x84 - ViewInst Include-Exclude Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcviiectlr(&self) -> &ApbaddrEtmCpu1Trcviiectlr {
        &self.apbaddr_etm_cpu1_trcviiectlr
    }
    #[doc = "0x88 - ViewInst Start-Stop Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcvissctlr(&self) -> &ApbaddrEtmCpu1Trcvissctlr {
        &self.apbaddr_etm_cpu1_trcvissctlr
    }
    #[doc = "0x100 - Sequencer State Transition Control Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcseqevr0(&self) -> &ApbaddrEtmCpu1Trcseqevr0 {
        &self.apbaddr_etm_cpu1_trcseqevr0
    }
    #[doc = "0x104 - Sequencer State Transition Control Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcseqevr1(&self) -> &ApbaddrEtmCpu1Trcseqevr1 {
        &self.apbaddr_etm_cpu1_trcseqevr1
    }
    #[doc = "0x108 - Sequencer State Transition Control Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcseqevr2(&self) -> &ApbaddrEtmCpu1Trcseqevr2 {
        &self.apbaddr_etm_cpu1_trcseqevr2
    }
    #[doc = "0x118 - Sequencer Reset Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcseqrstevr(&self) -> &ApbaddrEtmCpu1Trcseqrstevr {
        &self.apbaddr_etm_cpu1_trcseqrstevr
    }
    #[doc = "0x11c - Sequencer State Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcseqstr(&self) -> &ApbaddrEtmCpu1Trcseqstr {
        &self.apbaddr_etm_cpu1_trcseqstr
    }
    #[doc = "0x120 - External Input Select Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcextinselr(&self) -> &ApbaddrEtmCpu1Trcextinselr {
        &self.apbaddr_etm_cpu1_trcextinselr
    }
    #[doc = "0x140 - Counter Reload Value Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntrldvr0(&self) -> &ApbaddrEtmCpu1Trccntrldvr0 {
        &self.apbaddr_etm_cpu1_trccntrldvr0
    }
    #[doc = "0x144 - Counter Reload Value Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntrldvr1(&self) -> &ApbaddrEtmCpu1Trccntrldvr1 {
        &self.apbaddr_etm_cpu1_trccntrldvr1
    }
    #[doc = "0x150 - Counter Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntctlr0(&self) -> &ApbaddrEtmCpu1Trccntctlr0 {
        &self.apbaddr_etm_cpu1_trccntctlr0
    }
    #[doc = "0x154 - Counter Control Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntctlr1(&self) -> &ApbaddrEtmCpu1Trccntctlr1 {
        &self.apbaddr_etm_cpu1_trccntctlr1
    }
    #[doc = "0x160 - Counter Value Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntvr0(&self) -> &ApbaddrEtmCpu1Trccntvr0 {
        &self.apbaddr_etm_cpu1_trccntvr0
    }
    #[doc = "0x164 - Counter Value Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccntvr1(&self) -> &ApbaddrEtmCpu1Trccntvr1 {
        &self.apbaddr_etm_cpu1_trccntvr1
    }
    #[doc = "0x180 - ID Register 8"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr8(&self) -> &ApbaddrEtmCpu1Trcidr8 {
        &self.apbaddr_etm_cpu1_trcidr8
    }
    #[doc = "0x184 - ID Register 9"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr9(&self) -> &ApbaddrEtmCpu1Trcidr9 {
        &self.apbaddr_etm_cpu1_trcidr9
    }
    #[doc = "0x188 - ID Register 10"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr10(&self) -> &ApbaddrEtmCpu1Trcidr10 {
        &self.apbaddr_etm_cpu1_trcidr10
    }
    #[doc = "0x18c - ID Register 11"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr11(&self) -> &ApbaddrEtmCpu1Trcidr11 {
        &self.apbaddr_etm_cpu1_trcidr11
    }
    #[doc = "0x190 - ID Register 12"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr12(&self) -> &ApbaddrEtmCpu1Trcidr12 {
        &self.apbaddr_etm_cpu1_trcidr12
    }
    #[doc = "0x194 - ID Register 13"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr13(&self) -> &ApbaddrEtmCpu1Trcidr13 {
        &self.apbaddr_etm_cpu1_trcidr13
    }
    #[doc = "0x1c0 - Implementation Specific Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcimspec0(&self) -> &ApbaddrEtmCpu1Trcimspec0 {
        &self.apbaddr_etm_cpu1_trcimspec0
    }
    #[doc = "0x1e0 - ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr0(&self) -> &ApbaddrEtmCpu1Trcidr0 {
        &self.apbaddr_etm_cpu1_trcidr0
    }
    #[doc = "0x1e4 - ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr1(&self) -> &ApbaddrEtmCpu1Trcidr1 {
        &self.apbaddr_etm_cpu1_trcidr1
    }
    #[doc = "0x1e8 - ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr2(&self) -> &ApbaddrEtmCpu1Trcidr2 {
        &self.apbaddr_etm_cpu1_trcidr2
    }
    #[doc = "0x1ec - ID Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr3(&self) -> &ApbaddrEtmCpu1Trcidr3 {
        &self.apbaddr_etm_cpu1_trcidr3
    }
    #[doc = "0x1f0 - ID Register 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr4(&self) -> &ApbaddrEtmCpu1Trcidr4 {
        &self.apbaddr_etm_cpu1_trcidr4
    }
    #[doc = "0x1f4 - ID Register 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcidr5(&self) -> &ApbaddrEtmCpu1Trcidr5 {
        &self.apbaddr_etm_cpu1_trcidr5
    }
    #[doc = "0x208 - Resource Selection Control Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr2(&self) -> &ApbaddrEtmCpu1Trcrsctlr2 {
        &self.apbaddr_etm_cpu1_trcrsctlr2
    }
    #[doc = "0x20c - Resource Selection Control Registers 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr3(&self) -> &ApbaddrEtmCpu1Trcrsctlr3 {
        &self.apbaddr_etm_cpu1_trcrsctlr3
    }
    #[doc = "0x210 - Resource Selection Control Registers 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr4(&self) -> &ApbaddrEtmCpu1Trcrsctlr4 {
        &self.apbaddr_etm_cpu1_trcrsctlr4
    }
    #[doc = "0x214 - Resource Selection Control Registers 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr5(&self) -> &ApbaddrEtmCpu1Trcrsctlr5 {
        &self.apbaddr_etm_cpu1_trcrsctlr5
    }
    #[doc = "0x218 - Resource Selection Control Registers 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr6(&self) -> &ApbaddrEtmCpu1Trcrsctlr6 {
        &self.apbaddr_etm_cpu1_trcrsctlr6
    }
    #[doc = "0x21c - Resource Selection Control Registers 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr7(&self) -> &ApbaddrEtmCpu1Trcrsctlr7 {
        &self.apbaddr_etm_cpu1_trcrsctlr7
    }
    #[doc = "0x220 - Resource Selection Control Registers 8"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr8(&self) -> &ApbaddrEtmCpu1Trcrsctlr8 {
        &self.apbaddr_etm_cpu1_trcrsctlr8
    }
    #[doc = "0x224 - Resource Selection Control Registers 9"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr9(&self) -> &ApbaddrEtmCpu1Trcrsctlr9 {
        &self.apbaddr_etm_cpu1_trcrsctlr9
    }
    #[doc = "0x228 - Resource Selection Control Registers 10"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr10(&self) -> &ApbaddrEtmCpu1Trcrsctlr10 {
        &self.apbaddr_etm_cpu1_trcrsctlr10
    }
    #[doc = "0x22c - Resource Selection Control Registers 11"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr11(&self) -> &ApbaddrEtmCpu1Trcrsctlr11 {
        &self.apbaddr_etm_cpu1_trcrsctlr11
    }
    #[doc = "0x230 - Resource Selection Control Registers 12"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr12(&self) -> &ApbaddrEtmCpu1Trcrsctlr12 {
        &self.apbaddr_etm_cpu1_trcrsctlr12
    }
    #[doc = "0x234 - Resource Selection Control Registers 13"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr13(&self) -> &ApbaddrEtmCpu1Trcrsctlr13 {
        &self.apbaddr_etm_cpu1_trcrsctlr13
    }
    #[doc = "0x238 - Resource Selection Control Registers 14"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr14(&self) -> &ApbaddrEtmCpu1Trcrsctlr14 {
        &self.apbaddr_etm_cpu1_trcrsctlr14
    }
    #[doc = "0x23c - Resource Selection Control Registers 15"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcrsctlr15(&self) -> &ApbaddrEtmCpu1Trcrsctlr15 {
        &self.apbaddr_etm_cpu1_trcrsctlr15
    }
    #[doc = "0x280 - Single-Shot Comparator Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcssccr0(&self) -> &ApbaddrEtmCpu1Trcssccr0 {
        &self.apbaddr_etm_cpu1_trcssccr0
    }
    #[doc = "0x2a0 - Single-Shot Comparator Status Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcsscsr0(&self) -> &ApbaddrEtmCpu1Trcsscsr0 {
        &self.apbaddr_etm_cpu1_trcsscsr0
    }
    #[doc = "0x300 - OS Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcoslar(&self) -> &ApbaddrEtmCpu1Trcoslar {
        &self.apbaddr_etm_cpu1_trcoslar
    }
    #[doc = "0x304 - OS Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcoslsr(&self) -> &ApbaddrEtmCpu1Trcoslsr {
        &self.apbaddr_etm_cpu1_trcoslsr
    }
    #[doc = "0x310 - Power Down Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpdcr(&self) -> &ApbaddrEtmCpu1Trcpdcr {
        &self.apbaddr_etm_cpu1_trcpdcr
    }
    #[doc = "0x314 - Power Down Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpdsr(&self) -> &ApbaddrEtmCpu1Trcpdsr {
        &self.apbaddr_etm_cpu1_trcpdsr
    }
    #[doc = "0x400 - Address Comparator Value Registers 0 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr0_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr0_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr0_31_0
    }
    #[doc = "0x404 - Address Comparator Value Registers 0 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr0_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr0_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr0_63_32
    }
    #[doc = "0x408 - Address Comparator Value Registers 1 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr1_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr1_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr1_31_0
    }
    #[doc = "0x40c - Address Comparator Value Registers 1 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr1_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr1_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr1_63_32
    }
    #[doc = "0x410 - Address Comparator Value Registers 2 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr2_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr2_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr2_31_0
    }
    #[doc = "0x414 - Address Comparator Value Registers 2 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr2_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr2_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr2_63_32
    }
    #[doc = "0x418 - Address Comparator Value Registers 3 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr3_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr3_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr3_31_0
    }
    #[doc = "0x41c - Address Comparator Value Registers 3 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr3_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr3_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr3_63_32
    }
    #[doc = "0x420 - Address Comparator Value Registers 4 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr4_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr4_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr4_31_0
    }
    #[doc = "0x424 - Address Comparator Value Registers 4 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr4_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr4_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr4_63_32
    }
    #[doc = "0x428 - Address Comparator Value Registers 5 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr5_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr5_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr5_31_0
    }
    #[doc = "0x42c - Address Comparator Value Registers 5 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr5_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr5_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr5_63_32
    }
    #[doc = "0x430 - Address Comparator Value Registers 6 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr6_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr6_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr6_31_0
    }
    #[doc = "0x434 - Address Comparator Value Registers 6 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr6_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr6_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr6_63_32
    }
    #[doc = "0x438 - Address Comparator Value Registers 7 (low word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr7_31_0(&self) -> &ApbaddrEtmCpu1Trcacvr7_31_0 {
        &self.apbaddr_etm_cpu1_trcacvr7_31_0
    }
    #[doc = "0x43c - Address Comparator Value Registers 7 (high word)"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacvr7_63_32(&self) -> &ApbaddrEtmCpu1Trcacvr7_63_32 {
        &self.apbaddr_etm_cpu1_trcacvr7_63_32
    }
    #[doc = "0x480 - Address Comparator Access Type Registers 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr0(&self) -> &ApbaddrEtmCpu1Trcacatr0 {
        &self.apbaddr_etm_cpu1_trcacatr0
    }
    #[doc = "0x488 - Address Comparator Access Type Registers 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr1(&self) -> &ApbaddrEtmCpu1Trcacatr1 {
        &self.apbaddr_etm_cpu1_trcacatr1
    }
    #[doc = "0x490 - Address Comparator Access Type Registers 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr2(&self) -> &ApbaddrEtmCpu1Trcacatr2 {
        &self.apbaddr_etm_cpu1_trcacatr2
    }
    #[doc = "0x498 - Address Comparator Access Type Registers 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr3(&self) -> &ApbaddrEtmCpu1Trcacatr3 {
        &self.apbaddr_etm_cpu1_trcacatr3
    }
    #[doc = "0x4a0 - Address Comparator Access Type Registers 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr4(&self) -> &ApbaddrEtmCpu1Trcacatr4 {
        &self.apbaddr_etm_cpu1_trcacatr4
    }
    #[doc = "0x4a8 - Address Comparator Access Type Registers 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr5(&self) -> &ApbaddrEtmCpu1Trcacatr5 {
        &self.apbaddr_etm_cpu1_trcacatr5
    }
    #[doc = "0x4b0 - Address Comparator Access Type Registers 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr6(&self) -> &ApbaddrEtmCpu1Trcacatr6 {
        &self.apbaddr_etm_cpu1_trcacatr6
    }
    #[doc = "0x4b8 - Address Comparator Access Type Registers 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcacatr7(&self) -> &ApbaddrEtmCpu1Trcacatr7 {
        &self.apbaddr_etm_cpu1_trcacatr7
    }
    #[doc = "0x600 - Context ID Comparator Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidcvr0(&self) -> &ApbaddrEtmCpu1Trccidcvr0 {
        &self.apbaddr_etm_cpu1_trccidcvr0
    }
    #[doc = "0x640 - VMID Comparator Value Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcvmidcvr0(&self) -> &ApbaddrEtmCpu1Trcvmidcvr0 {
        &self.apbaddr_etm_cpu1_trcvmidcvr0
    }
    #[doc = "0x680 - Context ID Comparator Control Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidcctlr0(&self) -> &ApbaddrEtmCpu1Trccidcctlr0 {
        &self.apbaddr_etm_cpu1_trccidcctlr0
    }
    #[doc = "0xee4 - Integration ATB Identification Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcitatbidr(&self) -> &ApbaddrEtmCpu1Trcitatbidr {
        &self.apbaddr_etm_cpu1_trcitatbidr
    }
    #[doc = "0xeec - Integration Instruction ATB Data Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcitidatar(&self) -> &ApbaddrEtmCpu1Trcitidatar {
        &self.apbaddr_etm_cpu1_trcitidatar
    }
    #[doc = "0xef4 - Integration Instruction ATB In Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcitiatbinr(&self) -> &ApbaddrEtmCpu1Trcitiatbinr {
        &self.apbaddr_etm_cpu1_trcitiatbinr
    }
    #[doc = "0xefc - Integration Instruction ATB Out Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcitiatboutr(&self) -> &ApbaddrEtmCpu1Trcitiatboutr {
        &self.apbaddr_etm_cpu1_trcitiatboutr
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcitctrl(&self) -> &ApbaddrEtmCpu1Trcitctrl {
        &self.apbaddr_etm_cpu1_trcitctrl
    }
    #[doc = "0xfa0 - Claim Tag Set Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcclaimset(&self) -> &ApbaddrEtmCpu1Trcclaimset {
        &self.apbaddr_etm_cpu1_trcclaimset
    }
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcclaimclr(&self) -> &ApbaddrEtmCpu1Trcclaimclr {
        &self.apbaddr_etm_cpu1_trcclaimclr
    }
    #[doc = "0xfa8 - Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcdevaff0(&self) -> &ApbaddrEtmCpu1Trcdevaff0 {
        &self.apbaddr_etm_cpu1_trcdevaff0
    }
    #[doc = "0xfac - Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcdevaff1(&self) -> &ApbaddrEtmCpu1Trcdevaff1 {
        &self.apbaddr_etm_cpu1_trcdevaff1
    }
    #[doc = "0xfb0 - Software Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trclar(&self) -> &ApbaddrEtmCpu1Trclar {
        &self.apbaddr_etm_cpu1_trclar
    }
    #[doc = "0xfb4 - Software Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trclsr(&self) -> &ApbaddrEtmCpu1Trclsr {
        &self.apbaddr_etm_cpu1_trclsr
    }
    #[doc = "0xfb8 - Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcauthstatus(&self) -> &ApbaddrEtmCpu1Trcauthstatus {
        &self.apbaddr_etm_cpu1_trcauthstatus
    }
    #[doc = "0xfbc - Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcdevarch(&self) -> &ApbaddrEtmCpu1Trcdevarch {
        &self.apbaddr_etm_cpu1_trcdevarch
    }
    #[doc = "0xfc8 - Device ID Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcdevid(&self) -> &ApbaddrEtmCpu1Trcdevid {
        &self.apbaddr_etm_cpu1_trcdevid
    }
    #[doc = "0xfcc - Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcdevtype(&self) -> &ApbaddrEtmCpu1Trcdevtype {
        &self.apbaddr_etm_cpu1_trcdevtype
    }
    #[doc = "0xfd0 - Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr4(&self) -> &ApbaddrEtmCpu1Trcpidr4 {
        &self.apbaddr_etm_cpu1_trcpidr4
    }
    #[doc = "0xfd4 - Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr5(&self) -> &ApbaddrEtmCpu1Trcpidr5 {
        &self.apbaddr_etm_cpu1_trcpidr5
    }
    #[doc = "0xfd8 - Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr6(&self) -> &ApbaddrEtmCpu1Trcpidr6 {
        &self.apbaddr_etm_cpu1_trcpidr6
    }
    #[doc = "0xfdc - Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr7(&self) -> &ApbaddrEtmCpu1Trcpidr7 {
        &self.apbaddr_etm_cpu1_trcpidr7
    }
    #[doc = "0xfe0 - Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr0(&self) -> &ApbaddrEtmCpu1Trcpidr0 {
        &self.apbaddr_etm_cpu1_trcpidr0
    }
    #[doc = "0xfe4 - Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr1(&self) -> &ApbaddrEtmCpu1Trcpidr1 {
        &self.apbaddr_etm_cpu1_trcpidr1
    }
    #[doc = "0xfe8 - Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr2(&self) -> &ApbaddrEtmCpu1Trcpidr2 {
        &self.apbaddr_etm_cpu1_trcpidr2
    }
    #[doc = "0xfec - Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trcpidr3(&self) -> &ApbaddrEtmCpu1Trcpidr3 {
        &self.apbaddr_etm_cpu1_trcpidr3
    }
    #[doc = "0xff0 - Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidr0(&self) -> &ApbaddrEtmCpu1Trccidr0 {
        &self.apbaddr_etm_cpu1_trccidr0
    }
    #[doc = "0xff4 - Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidr1(&self) -> &ApbaddrEtmCpu1Trccidr1 {
        &self.apbaddr_etm_cpu1_trccidr1
    }
    #[doc = "0xff8 - Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidr2(&self) -> &ApbaddrEtmCpu1Trccidr2 {
        &self.apbaddr_etm_cpu1_trccidr2
    }
    #[doc = "0xffc - Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_etm_cpu1_trccidr3(&self) -> &ApbaddrEtmCpu1Trccidr3 {
        &self.apbaddr_etm_cpu1_trccidr3
    }
}
#[doc = "APBADDR_ETM_CPU1_TRCPRGCTLR (rw) register accessor: Programming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcprgctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcprgctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcprgctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPRGCTLR")]
pub type ApbaddrEtmCpu1Trcprgctlr =
    crate::Reg<apbaddr_etm_cpu1_trcprgctlr::ApbaddrEtmCpu1TrcprgctlrSpec>;
#[doc = "Programming Control Register"]
pub mod apbaddr_etm_cpu1_trcprgctlr;
#[doc = "APBADDR_ETM_CPU1_TRCSTATR (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcstatr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcstatr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcstatr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSTATR")]
pub type ApbaddrEtmCpu1Trcstatr = crate::Reg<apbaddr_etm_cpu1_trcstatr::ApbaddrEtmCpu1TrcstatrSpec>;
#[doc = "Status Register"]
pub mod apbaddr_etm_cpu1_trcstatr;
#[doc = "APBADDR_ETM_CPU1_TRCCONFIGR (rw) register accessor: Trace Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcconfigr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcconfigr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcconfigr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCONFIGR")]
pub type ApbaddrEtmCpu1Trcconfigr =
    crate::Reg<apbaddr_etm_cpu1_trcconfigr::ApbaddrEtmCpu1TrcconfigrSpec>;
#[doc = "Trace Configuration Register"]
pub mod apbaddr_etm_cpu1_trcconfigr;
#[doc = "APBADDR_ETM_CPU1_TRCAUXCTLR (rw) register accessor: Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcauxctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcauxctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcauxctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCAUXCTLR")]
pub type ApbaddrEtmCpu1Trcauxctlr =
    crate::Reg<apbaddr_etm_cpu1_trcauxctlr::ApbaddrEtmCpu1TrcauxctlrSpec>;
#[doc = "Auxiliary Control Register"]
pub mod apbaddr_etm_cpu1_trcauxctlr;
#[doc = "APBADDR_ETM_CPU1_TRCEVENTCTL0R (rw) register accessor: Event Control 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trceventctl0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trceventctl0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trceventctl0r`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCEVENTCTL0R")]
pub type ApbaddrEtmCpu1Trceventctl0r =
    crate::Reg<apbaddr_etm_cpu1_trceventctl0r::ApbaddrEtmCpu1Trceventctl0rSpec>;
#[doc = "Event Control 0 Register"]
pub mod apbaddr_etm_cpu1_trceventctl0r;
#[doc = "APBADDR_ETM_CPU1_TRCEVENTCTL1R (rw) register accessor: Event Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trceventctl1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trceventctl1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trceventctl1r`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCEVENTCTL1R")]
pub type ApbaddrEtmCpu1Trceventctl1r =
    crate::Reg<apbaddr_etm_cpu1_trceventctl1r::ApbaddrEtmCpu1Trceventctl1rSpec>;
#[doc = "Event Control 1 Register"]
pub mod apbaddr_etm_cpu1_trceventctl1r;
#[doc = "APBADDR_ETM_CPU1_TRCSTALLCTLR (rw) register accessor: Stall Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcstallctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcstallctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcstallctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSTALLCTLR")]
pub type ApbaddrEtmCpu1Trcstallctlr =
    crate::Reg<apbaddr_etm_cpu1_trcstallctlr::ApbaddrEtmCpu1TrcstallctlrSpec>;
#[doc = "Stall Control Register"]
pub mod apbaddr_etm_cpu1_trcstallctlr;
#[doc = "APBADDR_ETM_CPU1_TRCTSCTLR (rw) register accessor: Global Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trctsctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trctsctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trctsctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCTSCTLR")]
pub type ApbaddrEtmCpu1Trctsctlr =
    crate::Reg<apbaddr_etm_cpu1_trctsctlr::ApbaddrEtmCpu1TrctsctlrSpec>;
#[doc = "Global Timestamp Control Register"]
pub mod apbaddr_etm_cpu1_trctsctlr;
#[doc = "APBADDR_ETM_CPU1_TRCSYNCPR (rw) register accessor: Synchronization Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcsyncpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcsyncpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcsyncpr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSYNCPR")]
pub type ApbaddrEtmCpu1Trcsyncpr =
    crate::Reg<apbaddr_etm_cpu1_trcsyncpr::ApbaddrEtmCpu1TrcsyncprSpec>;
#[doc = "Synchronization Period Register"]
pub mod apbaddr_etm_cpu1_trcsyncpr;
#[doc = "APBADDR_ETM_CPU1_TRCCCCTLR (rw) register accessor: Cycle Count Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcccctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcccctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcccctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCCCTLR")]
pub type ApbaddrEtmCpu1Trcccctlr =
    crate::Reg<apbaddr_etm_cpu1_trcccctlr::ApbaddrEtmCpu1TrcccctlrSpec>;
#[doc = "Cycle Count Control Register"]
pub mod apbaddr_etm_cpu1_trcccctlr;
#[doc = "APBADDR_ETM_CPU1_TRCBBCTLR (rw) register accessor: Branch Broadcast Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcbbctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcbbctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcbbctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCBBCTLR")]
pub type ApbaddrEtmCpu1Trcbbctlr =
    crate::Reg<apbaddr_etm_cpu1_trcbbctlr::ApbaddrEtmCpu1TrcbbctlrSpec>;
#[doc = "Branch Broadcast Control Register"]
pub mod apbaddr_etm_cpu1_trcbbctlr;
#[doc = "APBADDR_ETM_CPU1_TRCTRACEIDR (rw) register accessor: Trace ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trctraceidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trctraceidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trctraceidr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCTRACEIDR")]
pub type ApbaddrEtmCpu1Trctraceidr =
    crate::Reg<apbaddr_etm_cpu1_trctraceidr::ApbaddrEtmCpu1TrctraceidrSpec>;
#[doc = "Trace ID Register"]
pub mod apbaddr_etm_cpu1_trctraceidr;
#[doc = "APBADDR_ETM_CPU1_TRCVICTLR (rw) register accessor: ViewInst Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcvictlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcvictlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcvictlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCVICTLR")]
pub type ApbaddrEtmCpu1Trcvictlr =
    crate::Reg<apbaddr_etm_cpu1_trcvictlr::ApbaddrEtmCpu1TrcvictlrSpec>;
#[doc = "ViewInst Main Control Register"]
pub mod apbaddr_etm_cpu1_trcvictlr;
#[doc = "APBADDR_ETM_CPU1_TRCVIIECTLR (rw) register accessor: ViewInst Include-Exclude Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcviiectlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcviiectlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcviiectlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCVIIECTLR")]
pub type ApbaddrEtmCpu1Trcviiectlr =
    crate::Reg<apbaddr_etm_cpu1_trcviiectlr::ApbaddrEtmCpu1TrcviiectlrSpec>;
#[doc = "ViewInst Include-Exclude Control Register"]
pub mod apbaddr_etm_cpu1_trcviiectlr;
#[doc = "APBADDR_ETM_CPU1_TRCVISSCTLR (rw) register accessor: ViewInst Start-Stop Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcvissctlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcvissctlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcvissctlr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCVISSCTLR")]
pub type ApbaddrEtmCpu1Trcvissctlr =
    crate::Reg<apbaddr_etm_cpu1_trcvissctlr::ApbaddrEtmCpu1TrcvissctlrSpec>;
#[doc = "ViewInst Start-Stop Control Register"]
pub mod apbaddr_etm_cpu1_trcvissctlr;
#[doc = "APBADDR_ETM_CPU1_TRCSEQEVR0 (rw) register accessor: Sequencer State Transition Control Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqevr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqevr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcseqevr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSEQEVR0")]
pub type ApbaddrEtmCpu1Trcseqevr0 =
    crate::Reg<apbaddr_etm_cpu1_trcseqevr0::ApbaddrEtmCpu1Trcseqevr0Spec>;
#[doc = "Sequencer State Transition Control Registers 0"]
pub mod apbaddr_etm_cpu1_trcseqevr0;
#[doc = "APBADDR_ETM_CPU1_TRCSEQEVR1 (rw) register accessor: Sequencer State Transition Control Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqevr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqevr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcseqevr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSEQEVR1")]
pub type ApbaddrEtmCpu1Trcseqevr1 =
    crate::Reg<apbaddr_etm_cpu1_trcseqevr1::ApbaddrEtmCpu1Trcseqevr1Spec>;
#[doc = "Sequencer State Transition Control Registers 1"]
pub mod apbaddr_etm_cpu1_trcseqevr1;
#[doc = "APBADDR_ETM_CPU1_TRCSEQEVR2 (rw) register accessor: Sequencer State Transition Control Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqevr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqevr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcseqevr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSEQEVR2")]
pub type ApbaddrEtmCpu1Trcseqevr2 =
    crate::Reg<apbaddr_etm_cpu1_trcseqevr2::ApbaddrEtmCpu1Trcseqevr2Spec>;
#[doc = "Sequencer State Transition Control Registers 2"]
pub mod apbaddr_etm_cpu1_trcseqevr2;
#[doc = "APBADDR_ETM_CPU1_TRCSEQRSTEVR (rw) register accessor: Sequencer Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqrstevr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqrstevr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcseqrstevr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSEQRSTEVR")]
pub type ApbaddrEtmCpu1Trcseqrstevr =
    crate::Reg<apbaddr_etm_cpu1_trcseqrstevr::ApbaddrEtmCpu1TrcseqrstevrSpec>;
#[doc = "Sequencer Reset Control Register"]
pub mod apbaddr_etm_cpu1_trcseqrstevr;
#[doc = "APBADDR_ETM_CPU1_TRCSEQSTR (rw) register accessor: Sequencer State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcseqstr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcseqstr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcseqstr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSEQSTR")]
pub type ApbaddrEtmCpu1Trcseqstr =
    crate::Reg<apbaddr_etm_cpu1_trcseqstr::ApbaddrEtmCpu1TrcseqstrSpec>;
#[doc = "Sequencer State Register"]
pub mod apbaddr_etm_cpu1_trcseqstr;
#[doc = "APBADDR_ETM_CPU1_TRCEXTINSELR (rw) register accessor: External Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcextinselr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcextinselr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcextinselr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCEXTINSELR")]
pub type ApbaddrEtmCpu1Trcextinselr =
    crate::Reg<apbaddr_etm_cpu1_trcextinselr::ApbaddrEtmCpu1TrcextinselrSpec>;
#[doc = "External Input Select Register"]
pub mod apbaddr_etm_cpu1_trcextinselr;
#[doc = "APBADDR_ETM_CPU1_TRCCNTRLDVR0 (rw) register accessor: Counter Reload Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntrldvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntrldvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntrldvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTRLDVR0")]
pub type ApbaddrEtmCpu1Trccntrldvr0 =
    crate::Reg<apbaddr_etm_cpu1_trccntrldvr0::ApbaddrEtmCpu1Trccntrldvr0Spec>;
#[doc = "Counter Reload Value Registers 0"]
pub mod apbaddr_etm_cpu1_trccntrldvr0;
#[doc = "APBADDR_ETM_CPU1_TRCCNTRLDVR1 (rw) register accessor: Counter Reload Value Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntrldvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntrldvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntrldvr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTRLDVR1")]
pub type ApbaddrEtmCpu1Trccntrldvr1 =
    crate::Reg<apbaddr_etm_cpu1_trccntrldvr1::ApbaddrEtmCpu1Trccntrldvr1Spec>;
#[doc = "Counter Reload Value Registers 1"]
pub mod apbaddr_etm_cpu1_trccntrldvr1;
#[doc = "APBADDR_ETM_CPU1_TRCCNTCTLR0 (rw) register accessor: Counter Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntctlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntctlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntctlr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTCTLR0")]
pub type ApbaddrEtmCpu1Trccntctlr0 =
    crate::Reg<apbaddr_etm_cpu1_trccntctlr0::ApbaddrEtmCpu1Trccntctlr0Spec>;
#[doc = "Counter Control Register 0"]
pub mod apbaddr_etm_cpu1_trccntctlr0;
#[doc = "APBADDR_ETM_CPU1_TRCCNTCTLR1 (rw) register accessor: Counter Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntctlr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntctlr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntctlr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTCTLR1")]
pub type ApbaddrEtmCpu1Trccntctlr1 =
    crate::Reg<apbaddr_etm_cpu1_trccntctlr1::ApbaddrEtmCpu1Trccntctlr1Spec>;
#[doc = "Counter Control Register 1"]
pub mod apbaddr_etm_cpu1_trccntctlr1;
#[doc = "APBADDR_ETM_CPU1_TRCCNTVR0 (rw) register accessor: Counter Value Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTVR0")]
pub type ApbaddrEtmCpu1Trccntvr0 =
    crate::Reg<apbaddr_etm_cpu1_trccntvr0::ApbaddrEtmCpu1Trccntvr0Spec>;
#[doc = "Counter Value Registers 0"]
pub mod apbaddr_etm_cpu1_trccntvr0;
#[doc = "APBADDR_ETM_CPU1_TRCCNTVR1 (rw) register accessor: Counter Value Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccntvr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccntvr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccntvr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCNTVR1")]
pub type ApbaddrEtmCpu1Trccntvr1 =
    crate::Reg<apbaddr_etm_cpu1_trccntvr1::ApbaddrEtmCpu1Trccntvr1Spec>;
#[doc = "Counter Value Registers 1"]
pub mod apbaddr_etm_cpu1_trccntvr1;
#[doc = "APBADDR_ETM_CPU1_TRCIDR8 (rw) register accessor: ID Register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr8`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR8")]
pub type ApbaddrEtmCpu1Trcidr8 = crate::Reg<apbaddr_etm_cpu1_trcidr8::ApbaddrEtmCpu1Trcidr8Spec>;
#[doc = "ID Register 8"]
pub mod apbaddr_etm_cpu1_trcidr8;
#[doc = "APBADDR_ETM_CPU1_TRCIDR9 (rw) register accessor: ID Register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr9`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR9")]
pub type ApbaddrEtmCpu1Trcidr9 = crate::Reg<apbaddr_etm_cpu1_trcidr9::ApbaddrEtmCpu1Trcidr9Spec>;
#[doc = "ID Register 9"]
pub mod apbaddr_etm_cpu1_trcidr9;
#[doc = "APBADDR_ETM_CPU1_TRCIDR10 (rw) register accessor: ID Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr10`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR10")]
pub type ApbaddrEtmCpu1Trcidr10 = crate::Reg<apbaddr_etm_cpu1_trcidr10::ApbaddrEtmCpu1Trcidr10Spec>;
#[doc = "ID Register 10"]
pub mod apbaddr_etm_cpu1_trcidr10;
#[doc = "APBADDR_ETM_CPU1_TRCIDR11 (rw) register accessor: ID Register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr11`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR11")]
pub type ApbaddrEtmCpu1Trcidr11 = crate::Reg<apbaddr_etm_cpu1_trcidr11::ApbaddrEtmCpu1Trcidr11Spec>;
#[doc = "ID Register 11"]
pub mod apbaddr_etm_cpu1_trcidr11;
#[doc = "APBADDR_ETM_CPU1_TRCIDR12 (rw) register accessor: ID Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr12`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR12")]
pub type ApbaddrEtmCpu1Trcidr12 = crate::Reg<apbaddr_etm_cpu1_trcidr12::ApbaddrEtmCpu1Trcidr12Spec>;
#[doc = "ID Register 12"]
pub mod apbaddr_etm_cpu1_trcidr12;
#[doc = "APBADDR_ETM_CPU1_TRCIDR13 (rw) register accessor: ID Register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr13`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR13")]
pub type ApbaddrEtmCpu1Trcidr13 = crate::Reg<apbaddr_etm_cpu1_trcidr13::ApbaddrEtmCpu1Trcidr13Spec>;
#[doc = "ID Register 13"]
pub mod apbaddr_etm_cpu1_trcidr13;
#[doc = "APBADDR_ETM_CPU1_TRCIMSPEC0 (rw) register accessor: Implementation Specific Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcimspec0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcimspec0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcimspec0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIMSPEC0")]
pub type ApbaddrEtmCpu1Trcimspec0 =
    crate::Reg<apbaddr_etm_cpu1_trcimspec0::ApbaddrEtmCpu1Trcimspec0Spec>;
#[doc = "Implementation Specific Register 0"]
pub mod apbaddr_etm_cpu1_trcimspec0;
#[doc = "APBADDR_ETM_CPU1_TRCIDR0 (rw) register accessor: ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR0")]
pub type ApbaddrEtmCpu1Trcidr0 = crate::Reg<apbaddr_etm_cpu1_trcidr0::ApbaddrEtmCpu1Trcidr0Spec>;
#[doc = "ID Register 0"]
pub mod apbaddr_etm_cpu1_trcidr0;
#[doc = "APBADDR_ETM_CPU1_TRCIDR1 (rw) register accessor: ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR1")]
pub type ApbaddrEtmCpu1Trcidr1 = crate::Reg<apbaddr_etm_cpu1_trcidr1::ApbaddrEtmCpu1Trcidr1Spec>;
#[doc = "ID Register 1"]
pub mod apbaddr_etm_cpu1_trcidr1;
#[doc = "APBADDR_ETM_CPU1_TRCIDR2 (rw) register accessor: ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR2")]
pub type ApbaddrEtmCpu1Trcidr2 = crate::Reg<apbaddr_etm_cpu1_trcidr2::ApbaddrEtmCpu1Trcidr2Spec>;
#[doc = "ID Register 2"]
pub mod apbaddr_etm_cpu1_trcidr2;
#[doc = "APBADDR_ETM_CPU1_TRCIDR3 (rw) register accessor: ID Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR3")]
pub type ApbaddrEtmCpu1Trcidr3 = crate::Reg<apbaddr_etm_cpu1_trcidr3::ApbaddrEtmCpu1Trcidr3Spec>;
#[doc = "ID Register 3"]
pub mod apbaddr_etm_cpu1_trcidr3;
#[doc = "APBADDR_ETM_CPU1_TRCIDR4 (rw) register accessor: ID Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR4")]
pub type ApbaddrEtmCpu1Trcidr4 = crate::Reg<apbaddr_etm_cpu1_trcidr4::ApbaddrEtmCpu1Trcidr4Spec>;
#[doc = "ID Register 4"]
pub mod apbaddr_etm_cpu1_trcidr4;
#[doc = "APBADDR_ETM_CPU1_TRCIDR5 (rw) register accessor: ID Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcidr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCIDR5")]
pub type ApbaddrEtmCpu1Trcidr5 = crate::Reg<apbaddr_etm_cpu1_trcidr5::ApbaddrEtmCpu1Trcidr5Spec>;
#[doc = "ID Register 5"]
pub mod apbaddr_etm_cpu1_trcidr5;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR2 (rw) register accessor: Resource Selection Control Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR2")]
pub type ApbaddrEtmCpu1Trcrsctlr2 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr2::ApbaddrEtmCpu1Trcrsctlr2Spec>;
#[doc = "Resource Selection Control Registers 2"]
pub mod apbaddr_etm_cpu1_trcrsctlr2;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR3 (rw) register accessor: Resource Selection Control Registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR3")]
pub type ApbaddrEtmCpu1Trcrsctlr3 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr3::ApbaddrEtmCpu1Trcrsctlr3Spec>;
#[doc = "Resource Selection Control Registers 3"]
pub mod apbaddr_etm_cpu1_trcrsctlr3;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR4 (rw) register accessor: Resource Selection Control Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR4")]
pub type ApbaddrEtmCpu1Trcrsctlr4 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr4::ApbaddrEtmCpu1Trcrsctlr4Spec>;
#[doc = "Resource Selection Control Registers 4"]
pub mod apbaddr_etm_cpu1_trcrsctlr4;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR5 (rw) register accessor: Resource Selection Control Registers 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR5")]
pub type ApbaddrEtmCpu1Trcrsctlr5 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr5::ApbaddrEtmCpu1Trcrsctlr5Spec>;
#[doc = "Resource Selection Control Registers 5"]
pub mod apbaddr_etm_cpu1_trcrsctlr5;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR6 (rw) register accessor: Resource Selection Control Registers 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR6")]
pub type ApbaddrEtmCpu1Trcrsctlr6 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr6::ApbaddrEtmCpu1Trcrsctlr6Spec>;
#[doc = "Resource Selection Control Registers 6"]
pub mod apbaddr_etm_cpu1_trcrsctlr6;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR7 (rw) register accessor: Resource Selection Control Registers 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR7")]
pub type ApbaddrEtmCpu1Trcrsctlr7 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr7::ApbaddrEtmCpu1Trcrsctlr7Spec>;
#[doc = "Resource Selection Control Registers 7"]
pub mod apbaddr_etm_cpu1_trcrsctlr7;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR8 (rw) register accessor: Resource Selection Control Registers 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr8`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR8")]
pub type ApbaddrEtmCpu1Trcrsctlr8 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr8::ApbaddrEtmCpu1Trcrsctlr8Spec>;
#[doc = "Resource Selection Control Registers 8"]
pub mod apbaddr_etm_cpu1_trcrsctlr8;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR9 (rw) register accessor: Resource Selection Control Registers 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr9`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR9")]
pub type ApbaddrEtmCpu1Trcrsctlr9 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr9::ApbaddrEtmCpu1Trcrsctlr9Spec>;
#[doc = "Resource Selection Control Registers 9"]
pub mod apbaddr_etm_cpu1_trcrsctlr9;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR10 (rw) register accessor: Resource Selection Control Registers 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr10`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR10")]
pub type ApbaddrEtmCpu1Trcrsctlr10 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr10::ApbaddrEtmCpu1Trcrsctlr10Spec>;
#[doc = "Resource Selection Control Registers 10"]
pub mod apbaddr_etm_cpu1_trcrsctlr10;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR11 (rw) register accessor: Resource Selection Control Registers 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr11`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR11")]
pub type ApbaddrEtmCpu1Trcrsctlr11 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr11::ApbaddrEtmCpu1Trcrsctlr11Spec>;
#[doc = "Resource Selection Control Registers 11"]
pub mod apbaddr_etm_cpu1_trcrsctlr11;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR12 (rw) register accessor: Resource Selection Control Registers 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr12`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR12")]
pub type ApbaddrEtmCpu1Trcrsctlr12 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr12::ApbaddrEtmCpu1Trcrsctlr12Spec>;
#[doc = "Resource Selection Control Registers 12"]
pub mod apbaddr_etm_cpu1_trcrsctlr12;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR13 (rw) register accessor: Resource Selection Control Registers 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr13`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR13")]
pub type ApbaddrEtmCpu1Trcrsctlr13 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr13::ApbaddrEtmCpu1Trcrsctlr13Spec>;
#[doc = "Resource Selection Control Registers 13"]
pub mod apbaddr_etm_cpu1_trcrsctlr13;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR14 (rw) register accessor: Resource Selection Control Registers 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr14`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR14")]
pub type ApbaddrEtmCpu1Trcrsctlr14 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr14::ApbaddrEtmCpu1Trcrsctlr14Spec>;
#[doc = "Resource Selection Control Registers 14"]
pub mod apbaddr_etm_cpu1_trcrsctlr14;
#[doc = "APBADDR_ETM_CPU1_TRCRSCTLR15 (rw) register accessor: Resource Selection Control Registers 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcrsctlr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcrsctlr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcrsctlr15`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCRSCTLR15")]
pub type ApbaddrEtmCpu1Trcrsctlr15 =
    crate::Reg<apbaddr_etm_cpu1_trcrsctlr15::ApbaddrEtmCpu1Trcrsctlr15Spec>;
#[doc = "Resource Selection Control Registers 15"]
pub mod apbaddr_etm_cpu1_trcrsctlr15;
#[doc = "APBADDR_ETM_CPU1_TRCSSCCR0 (rw) register accessor: Single-Shot Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcssccr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcssccr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcssccr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSSCCR0")]
pub type ApbaddrEtmCpu1Trcssccr0 =
    crate::Reg<apbaddr_etm_cpu1_trcssccr0::ApbaddrEtmCpu1Trcssccr0Spec>;
#[doc = "Single-Shot Comparator Control Register 0"]
pub mod apbaddr_etm_cpu1_trcssccr0;
#[doc = "APBADDR_ETM_CPU1_TRCSSCSR0 (rw) register accessor: Single-Shot Comparator Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcsscsr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcsscsr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcsscsr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCSSCSR0")]
pub type ApbaddrEtmCpu1Trcsscsr0 =
    crate::Reg<apbaddr_etm_cpu1_trcsscsr0::ApbaddrEtmCpu1Trcsscsr0Spec>;
#[doc = "Single-Shot Comparator Status Register 0"]
pub mod apbaddr_etm_cpu1_trcsscsr0;
#[doc = "APBADDR_ETM_CPU1_TRCOSLAR (rw) register accessor: OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcoslar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcoslar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcoslar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCOSLAR")]
pub type ApbaddrEtmCpu1Trcoslar = crate::Reg<apbaddr_etm_cpu1_trcoslar::ApbaddrEtmCpu1TrcoslarSpec>;
#[doc = "OS Lock Access Register"]
pub mod apbaddr_etm_cpu1_trcoslar;
#[doc = "APBADDR_ETM_CPU1_TRCOSLSR (rw) register accessor: OS Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcoslsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcoslsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcoslsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCOSLSR")]
pub type ApbaddrEtmCpu1Trcoslsr = crate::Reg<apbaddr_etm_cpu1_trcoslsr::ApbaddrEtmCpu1TrcoslsrSpec>;
#[doc = "OS Lock Status Register"]
pub mod apbaddr_etm_cpu1_trcoslsr;
#[doc = "APBADDR_ETM_CPU1_TRCPDCR (rw) register accessor: Power Down Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpdcr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPDCR")]
pub type ApbaddrEtmCpu1Trcpdcr = crate::Reg<apbaddr_etm_cpu1_trcpdcr::ApbaddrEtmCpu1TrcpdcrSpec>;
#[doc = "Power Down Control Register"]
pub mod apbaddr_etm_cpu1_trcpdcr;
#[doc = "APBADDR_ETM_CPU1_TRCPDSR (rw) register accessor: Power Down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpdsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpdsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpdsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPDSR")]
pub type ApbaddrEtmCpu1Trcpdsr = crate::Reg<apbaddr_etm_cpu1_trcpdsr::ApbaddrEtmCpu1TrcpdsrSpec>;
#[doc = "Power Down Status Register"]
pub mod apbaddr_etm_cpu1_trcpdsr;
#[doc = "APBADDR_ETM_CPU1_TRCACVR0_31_0 (rw) register accessor: Address Comparator Value Registers 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr0_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr0_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr0_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR0_31_0")]
pub type ApbaddrEtmCpu1Trcacvr0_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr0_31_0::ApbaddrEtmCpu1Trcacvr0_31_0Spec>;
#[doc = "Address Comparator Value Registers 0 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr0_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR0_63_32 (rw) register accessor: Address Comparator Value Registers 0 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr0_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr0_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr0_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR0_63_32")]
pub type ApbaddrEtmCpu1Trcacvr0_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr0_63_32::ApbaddrEtmCpu1Trcacvr0_63_32Spec>;
#[doc = "Address Comparator Value Registers 0 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr0_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR1_31_0 (rw) register accessor: Address Comparator Value Registers 1 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr1_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr1_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr1_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR1_31_0")]
pub type ApbaddrEtmCpu1Trcacvr1_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr1_31_0::ApbaddrEtmCpu1Trcacvr1_31_0Spec>;
#[doc = "Address Comparator Value Registers 1 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr1_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR1_63_32 (rw) register accessor: Address Comparator Value Registers 1 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr1_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr1_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr1_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR1_63_32")]
pub type ApbaddrEtmCpu1Trcacvr1_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr1_63_32::ApbaddrEtmCpu1Trcacvr1_63_32Spec>;
#[doc = "Address Comparator Value Registers 1 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr1_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR2_31_0 (rw) register accessor: Address Comparator Value Registers 2 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr2_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr2_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr2_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR2_31_0")]
pub type ApbaddrEtmCpu1Trcacvr2_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr2_31_0::ApbaddrEtmCpu1Trcacvr2_31_0Spec>;
#[doc = "Address Comparator Value Registers 2 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr2_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR2_63_32 (rw) register accessor: Address Comparator Value Registers 2 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr2_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr2_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr2_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR2_63_32")]
pub type ApbaddrEtmCpu1Trcacvr2_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr2_63_32::ApbaddrEtmCpu1Trcacvr2_63_32Spec>;
#[doc = "Address Comparator Value Registers 2 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr2_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR3_31_0 (rw) register accessor: Address Comparator Value Registers 3 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr3_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr3_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr3_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR3_31_0")]
pub type ApbaddrEtmCpu1Trcacvr3_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr3_31_0::ApbaddrEtmCpu1Trcacvr3_31_0Spec>;
#[doc = "Address Comparator Value Registers 3 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr3_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR3_63_32 (rw) register accessor: Address Comparator Value Registers 3 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr3_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr3_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr3_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR3_63_32")]
pub type ApbaddrEtmCpu1Trcacvr3_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr3_63_32::ApbaddrEtmCpu1Trcacvr3_63_32Spec>;
#[doc = "Address Comparator Value Registers 3 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr3_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR4_31_0 (rw) register accessor: Address Comparator Value Registers 4 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr4_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr4_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr4_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR4_31_0")]
pub type ApbaddrEtmCpu1Trcacvr4_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr4_31_0::ApbaddrEtmCpu1Trcacvr4_31_0Spec>;
#[doc = "Address Comparator Value Registers 4 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr4_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR4_63_32 (rw) register accessor: Address Comparator Value Registers 4 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr4_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr4_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr4_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR4_63_32")]
pub type ApbaddrEtmCpu1Trcacvr4_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr4_63_32::ApbaddrEtmCpu1Trcacvr4_63_32Spec>;
#[doc = "Address Comparator Value Registers 4 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr4_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR5_31_0 (rw) register accessor: Address Comparator Value Registers 5 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr5_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr5_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr5_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR5_31_0")]
pub type ApbaddrEtmCpu1Trcacvr5_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr5_31_0::ApbaddrEtmCpu1Trcacvr5_31_0Spec>;
#[doc = "Address Comparator Value Registers 5 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr5_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR5_63_32 (rw) register accessor: Address Comparator Value Registers 5 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr5_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr5_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr5_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR5_63_32")]
pub type ApbaddrEtmCpu1Trcacvr5_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr5_63_32::ApbaddrEtmCpu1Trcacvr5_63_32Spec>;
#[doc = "Address Comparator Value Registers 5 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr5_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR6_31_0 (rw) register accessor: Address Comparator Value Registers 6 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr6_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr6_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr6_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR6_31_0")]
pub type ApbaddrEtmCpu1Trcacvr6_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr6_31_0::ApbaddrEtmCpu1Trcacvr6_31_0Spec>;
#[doc = "Address Comparator Value Registers 6 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr6_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR6_63_32 (rw) register accessor: Address Comparator Value Registers 6 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr6_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr6_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr6_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR6_63_32")]
pub type ApbaddrEtmCpu1Trcacvr6_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr6_63_32::ApbaddrEtmCpu1Trcacvr6_63_32Spec>;
#[doc = "Address Comparator Value Registers 6 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr6_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACVR7_31_0 (rw) register accessor: Address Comparator Value Registers 7 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr7_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr7_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr7_31_0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR7_31_0")]
pub type ApbaddrEtmCpu1Trcacvr7_31_0 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr7_31_0::ApbaddrEtmCpu1Trcacvr7_31_0Spec>;
#[doc = "Address Comparator Value Registers 7 (low word)"]
pub mod apbaddr_etm_cpu1_trcacvr7_31_0;
#[doc = "APBADDR_ETM_CPU1_TRCACVR7_63_32 (rw) register accessor: Address Comparator Value Registers 7 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacvr7_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacvr7_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacvr7_63_32`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACVR7_63_32")]
pub type ApbaddrEtmCpu1Trcacvr7_63_32 =
    crate::Reg<apbaddr_etm_cpu1_trcacvr7_63_32::ApbaddrEtmCpu1Trcacvr7_63_32Spec>;
#[doc = "Address Comparator Value Registers 7 (high word)"]
pub mod apbaddr_etm_cpu1_trcacvr7_63_32;
#[doc = "APBADDR_ETM_CPU1_TRCACATR0 (rw) register accessor: Address Comparator Access Type Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR0")]
pub type ApbaddrEtmCpu1Trcacatr0 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr0::ApbaddrEtmCpu1Trcacatr0Spec>;
#[doc = "Address Comparator Access Type Registers 0"]
pub mod apbaddr_etm_cpu1_trcacatr0;
#[doc = "APBADDR_ETM_CPU1_TRCACATR1 (rw) register accessor: Address Comparator Access Type Registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR1")]
pub type ApbaddrEtmCpu1Trcacatr1 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr1::ApbaddrEtmCpu1Trcacatr1Spec>;
#[doc = "Address Comparator Access Type Registers 1"]
pub mod apbaddr_etm_cpu1_trcacatr1;
#[doc = "APBADDR_ETM_CPU1_TRCACATR2 (rw) register accessor: Address Comparator Access Type Registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR2")]
pub type ApbaddrEtmCpu1Trcacatr2 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr2::ApbaddrEtmCpu1Trcacatr2Spec>;
#[doc = "Address Comparator Access Type Registers 2"]
pub mod apbaddr_etm_cpu1_trcacatr2;
#[doc = "APBADDR_ETM_CPU1_TRCACATR3 (rw) register accessor: Address Comparator Access Type Registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR3")]
pub type ApbaddrEtmCpu1Trcacatr3 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr3::ApbaddrEtmCpu1Trcacatr3Spec>;
#[doc = "Address Comparator Access Type Registers 3"]
pub mod apbaddr_etm_cpu1_trcacatr3;
#[doc = "APBADDR_ETM_CPU1_TRCACATR4 (rw) register accessor: Address Comparator Access Type Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR4")]
pub type ApbaddrEtmCpu1Trcacatr4 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr4::ApbaddrEtmCpu1Trcacatr4Spec>;
#[doc = "Address Comparator Access Type Registers 4"]
pub mod apbaddr_etm_cpu1_trcacatr4;
#[doc = "APBADDR_ETM_CPU1_TRCACATR5 (rw) register accessor: Address Comparator Access Type Registers 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR5")]
pub type ApbaddrEtmCpu1Trcacatr5 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr5::ApbaddrEtmCpu1Trcacatr5Spec>;
#[doc = "Address Comparator Access Type Registers 5"]
pub mod apbaddr_etm_cpu1_trcacatr5;
#[doc = "APBADDR_ETM_CPU1_TRCACATR6 (rw) register accessor: Address Comparator Access Type Registers 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR6")]
pub type ApbaddrEtmCpu1Trcacatr6 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr6::ApbaddrEtmCpu1Trcacatr6Spec>;
#[doc = "Address Comparator Access Type Registers 6"]
pub mod apbaddr_etm_cpu1_trcacatr6;
#[doc = "APBADDR_ETM_CPU1_TRCACATR7 (rw) register accessor: Address Comparator Access Type Registers 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcacatr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcacatr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcacatr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCACATR7")]
pub type ApbaddrEtmCpu1Trcacatr7 =
    crate::Reg<apbaddr_etm_cpu1_trcacatr7::ApbaddrEtmCpu1Trcacatr7Spec>;
#[doc = "Address Comparator Access Type Registers 7"]
pub mod apbaddr_etm_cpu1_trcacatr7;
#[doc = "APBADDR_ETM_CPU1_TRCCIDCVR0 (rw) register accessor: Context ID Comparator Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidcvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidcvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidcvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDCVR0")]
pub type ApbaddrEtmCpu1Trccidcvr0 =
    crate::Reg<apbaddr_etm_cpu1_trccidcvr0::ApbaddrEtmCpu1Trccidcvr0Spec>;
#[doc = "Context ID Comparator Value Register 0"]
pub mod apbaddr_etm_cpu1_trccidcvr0;
#[doc = "APBADDR_ETM_CPU1_TRCVMIDCVR0 (rw) register accessor: VMID Comparator Value Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcvmidcvr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcvmidcvr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcvmidcvr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCVMIDCVR0")]
pub type ApbaddrEtmCpu1Trcvmidcvr0 =
    crate::Reg<apbaddr_etm_cpu1_trcvmidcvr0::ApbaddrEtmCpu1Trcvmidcvr0Spec>;
#[doc = "VMID Comparator Value Register 0"]
pub mod apbaddr_etm_cpu1_trcvmidcvr0;
#[doc = "APBADDR_ETM_CPU1_TRCCIDCCTLR0 (rw) register accessor: Context ID Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidcctlr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidcctlr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidcctlr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDCCTLR0")]
pub type ApbaddrEtmCpu1Trccidcctlr0 =
    crate::Reg<apbaddr_etm_cpu1_trccidcctlr0::ApbaddrEtmCpu1Trccidcctlr0Spec>;
#[doc = "Context ID Comparator Control Register 0"]
pub mod apbaddr_etm_cpu1_trccidcctlr0;
#[doc = "APBADDR_ETM_CPU1_TRCITATBIDR (rw) register accessor: Integration ATB Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitatbidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitatbidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcitatbidr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCITATBIDR")]
pub type ApbaddrEtmCpu1Trcitatbidr =
    crate::Reg<apbaddr_etm_cpu1_trcitatbidr::ApbaddrEtmCpu1TrcitatbidrSpec>;
#[doc = "Integration ATB Identification Register"]
pub mod apbaddr_etm_cpu1_trcitatbidr;
#[doc = "APBADDR_ETM_CPU1_TRCITIDATAR (rw) register accessor: Integration Instruction ATB Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitidatar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitidatar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcitidatar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCITIDATAR")]
pub type ApbaddrEtmCpu1Trcitidatar =
    crate::Reg<apbaddr_etm_cpu1_trcitidatar::ApbaddrEtmCpu1TrcitidatarSpec>;
#[doc = "Integration Instruction ATB Data Register"]
pub mod apbaddr_etm_cpu1_trcitidatar;
#[doc = "APBADDR_ETM_CPU1_TRCITIATBINR (rw) register accessor: Integration Instruction ATB In Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitiatbinr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitiatbinr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcitiatbinr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCITIATBINR")]
pub type ApbaddrEtmCpu1Trcitiatbinr =
    crate::Reg<apbaddr_etm_cpu1_trcitiatbinr::ApbaddrEtmCpu1TrcitiatbinrSpec>;
#[doc = "Integration Instruction ATB In Register"]
pub mod apbaddr_etm_cpu1_trcitiatbinr;
#[doc = "APBADDR_ETM_CPU1_TRCITIATBOUTR (rw) register accessor: Integration Instruction ATB Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitiatboutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitiatboutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcitiatboutr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCITIATBOUTR")]
pub type ApbaddrEtmCpu1Trcitiatboutr =
    crate::Reg<apbaddr_etm_cpu1_trcitiatboutr::ApbaddrEtmCpu1TrcitiatboutrSpec>;
#[doc = "Integration Instruction ATB Out Register"]
pub mod apbaddr_etm_cpu1_trcitiatboutr;
#[doc = "APBADDR_ETM_CPU1_TRCITCTRL (rw) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcitctrl`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCITCTRL")]
pub type ApbaddrEtmCpu1Trcitctrl =
    crate::Reg<apbaddr_etm_cpu1_trcitctrl::ApbaddrEtmCpu1TrcitctrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod apbaddr_etm_cpu1_trcitctrl;
#[doc = "APBADDR_ETM_CPU1_TRCCLAIMSET (rw) register accessor: Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcclaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcclaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcclaimset`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCLAIMSET")]
pub type ApbaddrEtmCpu1Trcclaimset =
    crate::Reg<apbaddr_etm_cpu1_trcclaimset::ApbaddrEtmCpu1TrcclaimsetSpec>;
#[doc = "Claim Tag Set Register"]
pub mod apbaddr_etm_cpu1_trcclaimset;
#[doc = "APBADDR_ETM_CPU1_TRCCLAIMCLR (rw) register accessor: Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcclaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcclaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcclaimclr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCLAIMCLR")]
pub type ApbaddrEtmCpu1Trcclaimclr =
    crate::Reg<apbaddr_etm_cpu1_trcclaimclr::ApbaddrEtmCpu1TrcclaimclrSpec>;
#[doc = "Claim Tag Clear Register"]
pub mod apbaddr_etm_cpu1_trcclaimclr;
#[doc = "APBADDR_ETM_CPU1_TRCDEVAFF0 (rw) register accessor: Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcdevaff0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCDEVAFF0")]
pub type ApbaddrEtmCpu1Trcdevaff0 =
    crate::Reg<apbaddr_etm_cpu1_trcdevaff0::ApbaddrEtmCpu1Trcdevaff0Spec>;
#[doc = "Device Affinity Register 0"]
pub mod apbaddr_etm_cpu1_trcdevaff0;
#[doc = "APBADDR_ETM_CPU1_TRCDEVAFF1 (rw) register accessor: Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcdevaff1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCDEVAFF1")]
pub type ApbaddrEtmCpu1Trcdevaff1 =
    crate::Reg<apbaddr_etm_cpu1_trcdevaff1::ApbaddrEtmCpu1Trcdevaff1Spec>;
#[doc = "Device Affinity Register 1"]
pub mod apbaddr_etm_cpu1_trcdevaff1;
#[doc = "APBADDR_ETM_CPU1_TRCLAR (rw) register accessor: Software Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trclar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trclar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trclar`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCLAR")]
pub type ApbaddrEtmCpu1Trclar = crate::Reg<apbaddr_etm_cpu1_trclar::ApbaddrEtmCpu1TrclarSpec>;
#[doc = "Software Lock Access Register"]
pub mod apbaddr_etm_cpu1_trclar;
#[doc = "APBADDR_ETM_CPU1_TRCLSR (rw) register accessor: Software Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trclsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trclsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trclsr`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCLSR")]
pub type ApbaddrEtmCpu1Trclsr = crate::Reg<apbaddr_etm_cpu1_trclsr::ApbaddrEtmCpu1TrclsrSpec>;
#[doc = "Software Lock Status Register"]
pub mod apbaddr_etm_cpu1_trclsr;
#[doc = "APBADDR_ETM_CPU1_TRCAUTHSTATUS (rw) register accessor: Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcauthstatus`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCAUTHSTATUS")]
pub type ApbaddrEtmCpu1Trcauthstatus =
    crate::Reg<apbaddr_etm_cpu1_trcauthstatus::ApbaddrEtmCpu1TrcauthstatusSpec>;
#[doc = "Authentication Status Register"]
pub mod apbaddr_etm_cpu1_trcauthstatus;
#[doc = "APBADDR_ETM_CPU1_TRCDEVARCH (rw) register accessor: Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcdevarch`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCDEVARCH")]
pub type ApbaddrEtmCpu1Trcdevarch =
    crate::Reg<apbaddr_etm_cpu1_trcdevarch::ApbaddrEtmCpu1TrcdevarchSpec>;
#[doc = "Device Architecture Register"]
pub mod apbaddr_etm_cpu1_trcdevarch;
#[doc = "APBADDR_ETM_CPU1_TRCDEVID (rw) register accessor: Device ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcdevid`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCDEVID")]
pub type ApbaddrEtmCpu1Trcdevid = crate::Reg<apbaddr_etm_cpu1_trcdevid::ApbaddrEtmCpu1TrcdevidSpec>;
#[doc = "Device ID Register"]
pub mod apbaddr_etm_cpu1_trcdevid;
#[doc = "APBADDR_ETM_CPU1_TRCDEVTYPE (rw) register accessor: Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcdevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcdevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcdevtype`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCDEVTYPE")]
pub type ApbaddrEtmCpu1Trcdevtype =
    crate::Reg<apbaddr_etm_cpu1_trcdevtype::ApbaddrEtmCpu1TrcdevtypeSpec>;
#[doc = "Device Type Register"]
pub mod apbaddr_etm_cpu1_trcdevtype;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR4 (rw) register accessor: Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr4`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR4")]
pub type ApbaddrEtmCpu1Trcpidr4 = crate::Reg<apbaddr_etm_cpu1_trcpidr4::ApbaddrEtmCpu1Trcpidr4Spec>;
#[doc = "Peripheral Identification Register 4"]
pub mod apbaddr_etm_cpu1_trcpidr4;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR5 (rw) register accessor: Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr5`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR5")]
pub type ApbaddrEtmCpu1Trcpidr5 = crate::Reg<apbaddr_etm_cpu1_trcpidr5::ApbaddrEtmCpu1Trcpidr5Spec>;
#[doc = "Peripheral Identification Register 5"]
pub mod apbaddr_etm_cpu1_trcpidr5;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR6 (rw) register accessor: Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr6`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR6")]
pub type ApbaddrEtmCpu1Trcpidr6 = crate::Reg<apbaddr_etm_cpu1_trcpidr6::ApbaddrEtmCpu1Trcpidr6Spec>;
#[doc = "Peripheral Identification Register 6"]
pub mod apbaddr_etm_cpu1_trcpidr6;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR7 (rw) register accessor: Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr7`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR7")]
pub type ApbaddrEtmCpu1Trcpidr7 = crate::Reg<apbaddr_etm_cpu1_trcpidr7::ApbaddrEtmCpu1Trcpidr7Spec>;
#[doc = "Peripheral Identification Register 7"]
pub mod apbaddr_etm_cpu1_trcpidr7;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR0 (rw) register accessor: Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR0")]
pub type ApbaddrEtmCpu1Trcpidr0 = crate::Reg<apbaddr_etm_cpu1_trcpidr0::ApbaddrEtmCpu1Trcpidr0Spec>;
#[doc = "Peripheral Identification Register 0"]
pub mod apbaddr_etm_cpu1_trcpidr0;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR1 (rw) register accessor: Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR1")]
pub type ApbaddrEtmCpu1Trcpidr1 = crate::Reg<apbaddr_etm_cpu1_trcpidr1::ApbaddrEtmCpu1Trcpidr1Spec>;
#[doc = "Peripheral Identification Register 1"]
pub mod apbaddr_etm_cpu1_trcpidr1;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR2 (rw) register accessor: Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR2")]
pub type ApbaddrEtmCpu1Trcpidr2 = crate::Reg<apbaddr_etm_cpu1_trcpidr2::ApbaddrEtmCpu1Trcpidr2Spec>;
#[doc = "Peripheral Identification Register 2"]
pub mod apbaddr_etm_cpu1_trcpidr2;
#[doc = "APBADDR_ETM_CPU1_TRCPIDR3 (rw) register accessor: Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trcpidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCPIDR3")]
pub type ApbaddrEtmCpu1Trcpidr3 = crate::Reg<apbaddr_etm_cpu1_trcpidr3::ApbaddrEtmCpu1Trcpidr3Spec>;
#[doc = "Peripheral Identification Register 3"]
pub mod apbaddr_etm_cpu1_trcpidr3;
#[doc = "APBADDR_ETM_CPU1_TRCCIDR0 (rw) register accessor: Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidr0`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDR0")]
pub type ApbaddrEtmCpu1Trccidr0 = crate::Reg<apbaddr_etm_cpu1_trccidr0::ApbaddrEtmCpu1Trccidr0Spec>;
#[doc = "Component Identification Register 0"]
pub mod apbaddr_etm_cpu1_trccidr0;
#[doc = "APBADDR_ETM_CPU1_TRCCIDR1 (rw) register accessor: Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidr1`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDR1")]
pub type ApbaddrEtmCpu1Trccidr1 = crate::Reg<apbaddr_etm_cpu1_trccidr1::ApbaddrEtmCpu1Trccidr1Spec>;
#[doc = "Component Identification Register 1"]
pub mod apbaddr_etm_cpu1_trccidr1;
#[doc = "APBADDR_ETM_CPU1_TRCCIDR2 (rw) register accessor: Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidr2`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDR2")]
pub type ApbaddrEtmCpu1Trccidr2 = crate::Reg<apbaddr_etm_cpu1_trccidr2::ApbaddrEtmCpu1Trccidr2Spec>;
#[doc = "Component Identification Register 2"]
pub mod apbaddr_etm_cpu1_trccidr2;
#[doc = "APBADDR_ETM_CPU1_TRCCIDR3 (rw) register accessor: Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_etm_cpu1_trccidr3`]
module"]
#[doc(alias = "APBADDR_ETM_CPU1_TRCCIDR3")]
pub type ApbaddrEtmCpu1Trccidr3 = crate::Reg<apbaddr_etm_cpu1_trccidr3::ApbaddrEtmCpu1Trccidr3Spec>;
#[doc = "Component Identification Register 3"]
pub mod apbaddr_etm_cpu1_trccidr3;
