#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    apbaddr_pmu_cpu1_pmevcntr0_el0: ApbaddrPmuCpu1Pmevcntr0El0,
    _reserved1: [u8; 0x04],
    apbaddr_pmu_cpu1_pmevcntr1_el0: ApbaddrPmuCpu1Pmevcntr1El0,
    _reserved2: [u8; 0x04],
    apbaddr_pmu_cpu1_pmevcntr2_el0: ApbaddrPmuCpu1Pmevcntr2El0,
    _reserved3: [u8; 0x04],
    apbaddr_pmu_cpu1_pmevcntr3_el0: ApbaddrPmuCpu1Pmevcntr3El0,
    _reserved4: [u8; 0x04],
    apbaddr_pmu_cpu1_pmevcntr4_el0: ApbaddrPmuCpu1Pmevcntr4El0,
    _reserved5: [u8; 0x04],
    apbaddr_pmu_cpu1_pmevcntr5_el0: ApbaddrPmuCpu1Pmevcntr5El0,
    _reserved6: [u8; 0xcc],
    apbaddr_pmu_cpu1_pmccntr_el0_31_0: ApbaddrPmuCpu1PmccntrEl0_31_0,
    apbaddr_pmu_cpu1_pmccntr_el0_63_32: ApbaddrPmuCpu1PmccntrEl0_63_32,
    _reserved8: [u8; 0x0300],
    apbaddr_pmu_cpu1_pmevtyper0_el0: ApbaddrPmuCpu1Pmevtyper0El0,
    apbaddr_pmu_cpu1_pmevtyper1_el0: ApbaddrPmuCpu1Pmevtyper1El0,
    apbaddr_pmu_cpu1_pmevtyper2_el0: ApbaddrPmuCpu1Pmevtyper2El0,
    apbaddr_pmu_cpu1_pmevtyper3_el0: ApbaddrPmuCpu1Pmevtyper3El0,
    apbaddr_pmu_cpu1_pmevtyper4_el0: ApbaddrPmuCpu1Pmevtyper4El0,
    apbaddr_pmu_cpu1_pmevtyper5_el0: ApbaddrPmuCpu1Pmevtyper5El0,
    _reserved14: [u8; 0x64],
    apbaddr_pmu_cpu1_pmccfiltr_el0: ApbaddrPmuCpu1PmccfiltrEl0,
    _reserved15: [u8; 0x0780],
    apbaddr_pmu_cpu1_pmcntenset_el0: ApbaddrPmuCpu1PmcntensetEl0,
    _reserved16: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmcntenclr_el0: ApbaddrPmuCpu1PmcntenclrEl0,
    _reserved17: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmintenset_el1: ApbaddrPmuCpu1PmintensetEl1,
    _reserved18: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmintenclr_el1: ApbaddrPmuCpu1PmintenclrEl1,
    _reserved19: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmovsclr_el0: ApbaddrPmuCpu1PmovsclrEl0,
    _reserved20: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmswinc_el0: ApbaddrPmuCpu1PmswincEl0,
    _reserved21: [u8; 0x1c],
    apbaddr_pmu_cpu1_pmovsset_el0: ApbaddrPmuCpu1PmovssetEl0,
    _reserved22: [u8; 0x013c],
    apbaddr_pmu_cpu1_pmcfgr: ApbaddrPmuCpu1Pmcfgr,
    apbaddr_pmu_cpu1_pmcr_el0: ApbaddrPmuCpu1PmcrEl0,
    _reserved24: [u8; 0x18],
    apbaddr_pmu_cpu1_pmceid0_el0: ApbaddrPmuCpu1Pmceid0El0,
    apbaddr_pmu_cpu1_pmceid1_el0: ApbaddrPmuCpu1Pmceid1El0,
    _reserved26: [u8; 0xd8],
    apbaddr_pmu_cpu1_pmitctrl: ApbaddrPmuCpu1Pmitctrl,
    _reserved27: [u8; 0xa4],
    apbaddr_pmu_cpu1_pmdevaff0: ApbaddrPmuCpu1Pmdevaff0,
    apbaddr_pmu_cpu1_pmdevaff1: ApbaddrPmuCpu1Pmdevaff1,
    apbaddr_pmu_cpu1_pmlar: ApbaddrPmuCpu1Pmlar,
    apbaddr_pmu_cpu1_pmlsr: ApbaddrPmuCpu1Pmlsr,
    apbaddr_pmu_cpu1_pmauthstatus: ApbaddrPmuCpu1Pmauthstatus,
    apbaddr_pmu_cpu1_pmdevarch: ApbaddrPmuCpu1Pmdevarch,
    _reserved33: [u8; 0x0c],
    apbaddr_pmu_cpu1_pmdevtype: ApbaddrPmuCpu1Pmdevtype,
    apbaddr_pmu_cpu1_pmpidr4: ApbaddrPmuCpu1Pmpidr4,
    apbaddr_pmu_cpu1_pmpidr5: ApbaddrPmuCpu1Pmpidr5,
    apbaddr_pmu_cpu1_pmpidr6: ApbaddrPmuCpu1Pmpidr6,
    apbaddr_pmu_cpu1_pmpidr7: ApbaddrPmuCpu1Pmpidr7,
    apbaddr_pmu_cpu1_pmpidr0: ApbaddrPmuCpu1Pmpidr0,
    apbaddr_pmu_cpu1_pmpidr1: ApbaddrPmuCpu1Pmpidr1,
    apbaddr_pmu_cpu1_pmpidr2: ApbaddrPmuCpu1Pmpidr2,
    apbaddr_pmu_cpu1_pmpidr3: ApbaddrPmuCpu1Pmpidr3,
    apbaddr_pmu_cpu1_pmcidr0: ApbaddrPmuCpu1Pmcidr0,
    apbaddr_pmu_cpu1_pmcidr1: ApbaddrPmuCpu1Pmcidr1,
    apbaddr_pmu_cpu1_pmcidr2: ApbaddrPmuCpu1Pmcidr2,
    apbaddr_pmu_cpu1_pmcidr3: ApbaddrPmuCpu1Pmcidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Performance Monitors Event Count Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr0_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr0El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr0_el0
    }
    #[doc = "0x08 - Performance Monitors Event Count Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr1_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr1El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr1_el0
    }
    #[doc = "0x10 - Performance Monitors Event Count Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr2_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr2El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr2_el0
    }
    #[doc = "0x18 - Performance Monitors Event Count Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr3_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr3El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr3_el0
    }
    #[doc = "0x20 - Performance Monitors Event Count Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr4_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr4El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr4_el0
    }
    #[doc = "0x28 - Performance Monitors Event Count Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevcntr5_el0(&self) -> &ApbaddrPmuCpu1Pmevcntr5El0 {
        &self.apbaddr_pmu_cpu1_pmevcntr5_el0
    }
    #[doc = "0xf8 - Performance Monitors Cycle Counter (low word)"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmccntr_el0_31_0(&self) -> &ApbaddrPmuCpu1PmccntrEl0_31_0 {
        &self.apbaddr_pmu_cpu1_pmccntr_el0_31_0
    }
    #[doc = "0xfc - Performance Monitors Cycle Counter (high word)"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmccntr_el0_63_32(&self) -> &ApbaddrPmuCpu1PmccntrEl0_63_32 {
        &self.apbaddr_pmu_cpu1_pmccntr_el0_63_32
    }
    #[doc = "0x400 - Performance Monitors Event Type Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper0_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper0El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper0_el0
    }
    #[doc = "0x404 - Performance Monitors Event Type Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper1_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper1El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper1_el0
    }
    #[doc = "0x408 - Performance Monitors Event Type Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper2_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper2El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper2_el0
    }
    #[doc = "0x40c - Performance Monitors Event Type Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper3_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper3El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper3_el0
    }
    #[doc = "0x410 - Performance Monitors Event Type Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper4_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper4El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper4_el0
    }
    #[doc = "0x414 - Performance Monitors Event Type Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmevtyper5_el0(&self) -> &ApbaddrPmuCpu1Pmevtyper5El0 {
        &self.apbaddr_pmu_cpu1_pmevtyper5_el0
    }
    #[doc = "0x47c - Performance Monitors Cycle Counter Filter Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmccfiltr_el0(&self) -> &ApbaddrPmuCpu1PmccfiltrEl0 {
        &self.apbaddr_pmu_cpu1_pmccfiltr_el0
    }
    #[doc = "0xc00 - Performance Monitors Count Enable Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcntenset_el0(&self) -> &ApbaddrPmuCpu1PmcntensetEl0 {
        &self.apbaddr_pmu_cpu1_pmcntenset_el0
    }
    #[doc = "0xc20 - Performance Monitors Count Enable Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcntenclr_el0(&self) -> &ApbaddrPmuCpu1PmcntenclrEl0 {
        &self.apbaddr_pmu_cpu1_pmcntenclr_el0
    }
    #[doc = "0xc40 - Performance Monitors Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmintenset_el1(&self) -> &ApbaddrPmuCpu1PmintensetEl1 {
        &self.apbaddr_pmu_cpu1_pmintenset_el1
    }
    #[doc = "0xc60 - Performance Monitors Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmintenclr_el1(&self) -> &ApbaddrPmuCpu1PmintenclrEl1 {
        &self.apbaddr_pmu_cpu1_pmintenclr_el1
    }
    #[doc = "0xc80 - Performance Monitors Overflow Flag Status Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmovsclr_el0(&self) -> &ApbaddrPmuCpu1PmovsclrEl0 {
        &self.apbaddr_pmu_cpu1_pmovsclr_el0
    }
    #[doc = "0xca0 - Performance Monitors Software Increment Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmswinc_el0(&self) -> &ApbaddrPmuCpu1PmswincEl0 {
        &self.apbaddr_pmu_cpu1_pmswinc_el0
    }
    #[doc = "0xcc0 - Performance Monitors Overflow Flag Status Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmovsset_el0(&self) -> &ApbaddrPmuCpu1PmovssetEl0 {
        &self.apbaddr_pmu_cpu1_pmovsset_el0
    }
    #[doc = "0xe00 - Performance Monitors Configuration Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcfgr(&self) -> &ApbaddrPmuCpu1Pmcfgr {
        &self.apbaddr_pmu_cpu1_pmcfgr
    }
    #[doc = "0xe04 - Performance Monitors Control Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcr_el0(&self) -> &ApbaddrPmuCpu1PmcrEl0 {
        &self.apbaddr_pmu_cpu1_pmcr_el0
    }
    #[doc = "0xe20 - Performance Monitors Common Event Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmceid0_el0(&self) -> &ApbaddrPmuCpu1Pmceid0El0 {
        &self.apbaddr_pmu_cpu1_pmceid0_el0
    }
    #[doc = "0xe24 - Performance Monitors Common Event Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmceid1_el0(&self) -> &ApbaddrPmuCpu1Pmceid1El0 {
        &self.apbaddr_pmu_cpu1_pmceid1_el0
    }
    #[doc = "0xf00 - Performance Monitors Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmitctrl(&self) -> &ApbaddrPmuCpu1Pmitctrl {
        &self.apbaddr_pmu_cpu1_pmitctrl
    }
    #[doc = "0xfa8 - Performance Monitors Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmdevaff0(&self) -> &ApbaddrPmuCpu1Pmdevaff0 {
        &self.apbaddr_pmu_cpu1_pmdevaff0
    }
    #[doc = "0xfac - Performance Monitors Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmdevaff1(&self) -> &ApbaddrPmuCpu1Pmdevaff1 {
        &self.apbaddr_pmu_cpu1_pmdevaff1
    }
    #[doc = "0xfb0 - Performance Monitors Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmlar(&self) -> &ApbaddrPmuCpu1Pmlar {
        &self.apbaddr_pmu_cpu1_pmlar
    }
    #[doc = "0xfb4 - Performance Monitors Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmlsr(&self) -> &ApbaddrPmuCpu1Pmlsr {
        &self.apbaddr_pmu_cpu1_pmlsr
    }
    #[doc = "0xfb8 - Performance Monitors Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmauthstatus(&self) -> &ApbaddrPmuCpu1Pmauthstatus {
        &self.apbaddr_pmu_cpu1_pmauthstatus
    }
    #[doc = "0xfbc - Performance Monitors Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmdevarch(&self) -> &ApbaddrPmuCpu1Pmdevarch {
        &self.apbaddr_pmu_cpu1_pmdevarch
    }
    #[doc = "0xfcc - Performance Monitors Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmdevtype(&self) -> &ApbaddrPmuCpu1Pmdevtype {
        &self.apbaddr_pmu_cpu1_pmdevtype
    }
    #[doc = "0xfd0 - Performance Monitors Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr4(&self) -> &ApbaddrPmuCpu1Pmpidr4 {
        &self.apbaddr_pmu_cpu1_pmpidr4
    }
    #[doc = "0xfd4 - Performance Monitors Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr5(&self) -> &ApbaddrPmuCpu1Pmpidr5 {
        &self.apbaddr_pmu_cpu1_pmpidr5
    }
    #[doc = "0xfd8 - Performance Monitors Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr6(&self) -> &ApbaddrPmuCpu1Pmpidr6 {
        &self.apbaddr_pmu_cpu1_pmpidr6
    }
    #[doc = "0xfdc - Performance Monitors Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr7(&self) -> &ApbaddrPmuCpu1Pmpidr7 {
        &self.apbaddr_pmu_cpu1_pmpidr7
    }
    #[doc = "0xfe0 - Performance Monitors Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr0(&self) -> &ApbaddrPmuCpu1Pmpidr0 {
        &self.apbaddr_pmu_cpu1_pmpidr0
    }
    #[doc = "0xfe4 - Performance Monitors Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr1(&self) -> &ApbaddrPmuCpu1Pmpidr1 {
        &self.apbaddr_pmu_cpu1_pmpidr1
    }
    #[doc = "0xfe8 - Performance Monitors Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr2(&self) -> &ApbaddrPmuCpu1Pmpidr2 {
        &self.apbaddr_pmu_cpu1_pmpidr2
    }
    #[doc = "0xfec - Performance Monitors Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmpidr3(&self) -> &ApbaddrPmuCpu1Pmpidr3 {
        &self.apbaddr_pmu_cpu1_pmpidr3
    }
    #[doc = "0xff0 - Performance Monitors Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcidr0(&self) -> &ApbaddrPmuCpu1Pmcidr0 {
        &self.apbaddr_pmu_cpu1_pmcidr0
    }
    #[doc = "0xff4 - Performance Monitors Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcidr1(&self) -> &ApbaddrPmuCpu1Pmcidr1 {
        &self.apbaddr_pmu_cpu1_pmcidr1
    }
    #[doc = "0xff8 - Performance Monitors Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcidr2(&self) -> &ApbaddrPmuCpu1Pmcidr2 {
        &self.apbaddr_pmu_cpu1_pmcidr2
    }
    #[doc = "0xffc - Performance Monitors Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu1_pmcidr3(&self) -> &ApbaddrPmuCpu1Pmcidr3 {
        &self.apbaddr_pmu_cpu1_pmcidr3
    }
}
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR0_EL0 (rw) register accessor: Performance Monitors Event Count Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR0_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr0El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr0_el0::ApbaddrPmuCpu1Pmevcntr0El0Spec>;
#[doc = "Performance Monitors Event Count Register 0"]
pub mod apbaddr_pmu_cpu1_pmevcntr0_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR1_EL0 (rw) register accessor: Performance Monitors Event Count Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR1_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr1El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr1_el0::ApbaddrPmuCpu1Pmevcntr1El0Spec>;
#[doc = "Performance Monitors Event Count Register 1"]
pub mod apbaddr_pmu_cpu1_pmevcntr1_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR2_EL0 (rw) register accessor: Performance Monitors Event Count Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr2_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr2_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr2_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR2_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr2El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr2_el0::ApbaddrPmuCpu1Pmevcntr2El0Spec>;
#[doc = "Performance Monitors Event Count Register 2"]
pub mod apbaddr_pmu_cpu1_pmevcntr2_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR3_EL0 (rw) register accessor: Performance Monitors Event Count Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr3_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr3_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr3_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR3_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr3El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr3_el0::ApbaddrPmuCpu1Pmevcntr3El0Spec>;
#[doc = "Performance Monitors Event Count Register 3"]
pub mod apbaddr_pmu_cpu1_pmevcntr3_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR4_EL0 (rw) register accessor: Performance Monitors Event Count Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr4_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr4_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr4_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR4_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr4El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr4_el0::ApbaddrPmuCpu1Pmevcntr4El0Spec>;
#[doc = "Performance Monitors Event Count Register 4"]
pub mod apbaddr_pmu_cpu1_pmevcntr4_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVCNTR5_EL0 (rw) register accessor: Performance Monitors Event Count Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevcntr5_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevcntr5_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevcntr5_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVCNTR5_EL0")]
pub type ApbaddrPmuCpu1Pmevcntr5El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevcntr5_el0::ApbaddrPmuCpu1Pmevcntr5El0Spec>;
#[doc = "Performance Monitors Event Count Register 5"]
pub mod apbaddr_pmu_cpu1_pmevcntr5_el0;
#[doc = "APBADDR_PMU_CPU1_PMCCNTR_EL0_31_0 (rw) register accessor: Performance Monitors Cycle Counter (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmccntr_el0_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmccntr_el0_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmccntr_el0_31_0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCCNTR_EL0_31_0")]
pub type ApbaddrPmuCpu1PmccntrEl0_31_0 =
    crate::Reg<apbaddr_pmu_cpu1_pmccntr_el0_31_0::ApbaddrPmuCpu1PmccntrEl0_31_0Spec>;
#[doc = "Performance Monitors Cycle Counter (low word)"]
pub mod apbaddr_pmu_cpu1_pmccntr_el0_31_0;
#[doc = "APBADDR_PMU_CPU1_PMCCNTR_EL0_63_32 (rw) register accessor: Performance Monitors Cycle Counter (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmccntr_el0_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmccntr_el0_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmccntr_el0_63_32`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCCNTR_EL0_63_32")]
pub type ApbaddrPmuCpu1PmccntrEl0_63_32 =
    crate::Reg<apbaddr_pmu_cpu1_pmccntr_el0_63_32::ApbaddrPmuCpu1PmccntrEl0_63_32Spec>;
#[doc = "Performance Monitors Cycle Counter (high word)"]
pub mod apbaddr_pmu_cpu1_pmccntr_el0_63_32;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER0_EL0 (rw) register accessor: Performance Monitors Event Type Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER0_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper0El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper0_el0::ApbaddrPmuCpu1Pmevtyper0El0Spec>;
#[doc = "Performance Monitors Event Type Register 0"]
pub mod apbaddr_pmu_cpu1_pmevtyper0_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER1_EL0 (rw) register accessor: Performance Monitors Event Type Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER1_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper1El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper1_el0::ApbaddrPmuCpu1Pmevtyper1El0Spec>;
#[doc = "Performance Monitors Event Type Register 1"]
pub mod apbaddr_pmu_cpu1_pmevtyper1_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER2_EL0 (rw) register accessor: Performance Monitors Event Type Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper2_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper2_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper2_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER2_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper2El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper2_el0::ApbaddrPmuCpu1Pmevtyper2El0Spec>;
#[doc = "Performance Monitors Event Type Register 2"]
pub mod apbaddr_pmu_cpu1_pmevtyper2_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER3_EL0 (rw) register accessor: Performance Monitors Event Type Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper3_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper3_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper3_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER3_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper3El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper3_el0::ApbaddrPmuCpu1Pmevtyper3El0Spec>;
#[doc = "Performance Monitors Event Type Register 3"]
pub mod apbaddr_pmu_cpu1_pmevtyper3_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER4_EL0 (rw) register accessor: Performance Monitors Event Type Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper4_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper4_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper4_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER4_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper4El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper4_el0::ApbaddrPmuCpu1Pmevtyper4El0Spec>;
#[doc = "Performance Monitors Event Type Register 4"]
pub mod apbaddr_pmu_cpu1_pmevtyper4_el0;
#[doc = "APBADDR_PMU_CPU1_PMEVTYPER5_EL0 (rw) register accessor: Performance Monitors Event Type Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmevtyper5_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmevtyper5_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmevtyper5_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMEVTYPER5_EL0")]
pub type ApbaddrPmuCpu1Pmevtyper5El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmevtyper5_el0::ApbaddrPmuCpu1Pmevtyper5El0Spec>;
#[doc = "Performance Monitors Event Type Register 5"]
pub mod apbaddr_pmu_cpu1_pmevtyper5_el0;
#[doc = "APBADDR_PMU_CPU1_PMCCFILTR_EL0 (rw) register accessor: Performance Monitors Cycle Counter Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmccfiltr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmccfiltr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmccfiltr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCCFILTR_EL0")]
pub type ApbaddrPmuCpu1PmccfiltrEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmccfiltr_el0::ApbaddrPmuCpu1PmccfiltrEl0Spec>;
#[doc = "Performance Monitors Cycle Counter Filter Register"]
pub mod apbaddr_pmu_cpu1_pmccfiltr_el0;
#[doc = "APBADDR_PMU_CPU1_PMCNTENSET_EL0 (rw) register accessor: Performance Monitors Count Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcntenset_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcntenset_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcntenset_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCNTENSET_EL0")]
pub type ApbaddrPmuCpu1PmcntensetEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmcntenset_el0::ApbaddrPmuCpu1PmcntensetEl0Spec>;
#[doc = "Performance Monitors Count Enable Set Register"]
pub mod apbaddr_pmu_cpu1_pmcntenset_el0;
#[doc = "APBADDR_PMU_CPU1_PMCNTENCLR_EL0 (rw) register accessor: Performance Monitors Count Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcntenclr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcntenclr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcntenclr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCNTENCLR_EL0")]
pub type ApbaddrPmuCpu1PmcntenclrEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmcntenclr_el0::ApbaddrPmuCpu1PmcntenclrEl0Spec>;
#[doc = "Performance Monitors Count Enable Clear Register"]
pub mod apbaddr_pmu_cpu1_pmcntenclr_el0;
#[doc = "APBADDR_PMU_CPU1_PMINTENSET_EL1 (rw) register accessor: Performance Monitors Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmintenset_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmintenset_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmintenset_el1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMINTENSET_EL1")]
pub type ApbaddrPmuCpu1PmintensetEl1 =
    crate::Reg<apbaddr_pmu_cpu1_pmintenset_el1::ApbaddrPmuCpu1PmintensetEl1Spec>;
#[doc = "Performance Monitors Interrupt Enable Set Register"]
pub mod apbaddr_pmu_cpu1_pmintenset_el1;
#[doc = "APBADDR_PMU_CPU1_PMINTENCLR_EL1 (rw) register accessor: Performance Monitors Interrupt Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmintenclr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmintenclr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmintenclr_el1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMINTENCLR_EL1")]
pub type ApbaddrPmuCpu1PmintenclrEl1 =
    crate::Reg<apbaddr_pmu_cpu1_pmintenclr_el1::ApbaddrPmuCpu1PmintenclrEl1Spec>;
#[doc = "Performance Monitors Interrupt Enable Clear Register"]
pub mod apbaddr_pmu_cpu1_pmintenclr_el1;
#[doc = "APBADDR_PMU_CPU1_PMOVSCLR_EL0 (rw) register accessor: Performance Monitors Overflow Flag Status Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmovsclr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmovsclr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmovsclr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMOVSCLR_EL0")]
pub type ApbaddrPmuCpu1PmovsclrEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmovsclr_el0::ApbaddrPmuCpu1PmovsclrEl0Spec>;
#[doc = "Performance Monitors Overflow Flag Status Clear Register"]
pub mod apbaddr_pmu_cpu1_pmovsclr_el0;
#[doc = "APBADDR_PMU_CPU1_PMSWINC_EL0 (rw) register accessor: Performance Monitors Software Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmswinc_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmswinc_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmswinc_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMSWINC_EL0")]
pub type ApbaddrPmuCpu1PmswincEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmswinc_el0::ApbaddrPmuCpu1PmswincEl0Spec>;
#[doc = "Performance Monitors Software Increment Register"]
pub mod apbaddr_pmu_cpu1_pmswinc_el0;
#[doc = "APBADDR_PMU_CPU1_PMOVSSET_EL0 (rw) register accessor: Performance Monitors Overflow Flag Status Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmovsset_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmovsset_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmovsset_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMOVSSET_EL0")]
pub type ApbaddrPmuCpu1PmovssetEl0 =
    crate::Reg<apbaddr_pmu_cpu1_pmovsset_el0::ApbaddrPmuCpu1PmovssetEl0Spec>;
#[doc = "Performance Monitors Overflow Flag Status Set Register"]
pub mod apbaddr_pmu_cpu1_pmovsset_el0;
#[doc = "APBADDR_PMU_CPU1_PMCFGR (rw) register accessor: Performance Monitors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcfgr`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCFGR")]
pub type ApbaddrPmuCpu1Pmcfgr = crate::Reg<apbaddr_pmu_cpu1_pmcfgr::ApbaddrPmuCpu1PmcfgrSpec>;
#[doc = "Performance Monitors Configuration Register"]
pub mod apbaddr_pmu_cpu1_pmcfgr;
#[doc = "APBADDR_PMU_CPU1_PMCR_EL0 (rw) register accessor: Performance Monitors Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCR_EL0")]
pub type ApbaddrPmuCpu1PmcrEl0 = crate::Reg<apbaddr_pmu_cpu1_pmcr_el0::ApbaddrPmuCpu1PmcrEl0Spec>;
#[doc = "Performance Monitors Control Register"]
pub mod apbaddr_pmu_cpu1_pmcr_el0;
#[doc = "APBADDR_PMU_CPU1_PMCEID0_EL0 (rw) register accessor: Performance Monitors Common Event Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmceid0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmceid0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmceid0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCEID0_EL0")]
pub type ApbaddrPmuCpu1Pmceid0El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmceid0_el0::ApbaddrPmuCpu1Pmceid0El0Spec>;
#[doc = "Performance Monitors Common Event Identification Register 0"]
pub mod apbaddr_pmu_cpu1_pmceid0_el0;
#[doc = "APBADDR_PMU_CPU1_PMCEID1_EL0 (rw) register accessor: Performance Monitors Common Event Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmceid1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmceid1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmceid1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCEID1_EL0")]
pub type ApbaddrPmuCpu1Pmceid1El0 =
    crate::Reg<apbaddr_pmu_cpu1_pmceid1_el0::ApbaddrPmuCpu1Pmceid1El0Spec>;
#[doc = "Performance Monitors Common Event Identification Register 1"]
pub mod apbaddr_pmu_cpu1_pmceid1_el0;
#[doc = "APBADDR_PMU_CPU1_PMITCTRL (rw) register accessor: Performance Monitors Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmitctrl`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMITCTRL")]
pub type ApbaddrPmuCpu1Pmitctrl = crate::Reg<apbaddr_pmu_cpu1_pmitctrl::ApbaddrPmuCpu1PmitctrlSpec>;
#[doc = "Performance Monitors Integration mode Control Register"]
pub mod apbaddr_pmu_cpu1_pmitctrl;
#[doc = "APBADDR_PMU_CPU1_PMDEVAFF0 (rw) register accessor: Performance Monitors Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmdevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmdevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmdevaff0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMDEVAFF0")]
pub type ApbaddrPmuCpu1Pmdevaff0 =
    crate::Reg<apbaddr_pmu_cpu1_pmdevaff0::ApbaddrPmuCpu1Pmdevaff0Spec>;
#[doc = "Performance Monitors Device Affinity Register 0"]
pub mod apbaddr_pmu_cpu1_pmdevaff0;
#[doc = "APBADDR_PMU_CPU1_PMDEVAFF1 (rw) register accessor: Performance Monitors Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmdevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmdevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmdevaff1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMDEVAFF1")]
pub type ApbaddrPmuCpu1Pmdevaff1 =
    crate::Reg<apbaddr_pmu_cpu1_pmdevaff1::ApbaddrPmuCpu1Pmdevaff1Spec>;
#[doc = "Performance Monitors Device Affinity Register 1"]
pub mod apbaddr_pmu_cpu1_pmdevaff1;
#[doc = "APBADDR_PMU_CPU1_PMLAR (rw) register accessor: Performance Monitors Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmlar`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMLAR")]
pub type ApbaddrPmuCpu1Pmlar = crate::Reg<apbaddr_pmu_cpu1_pmlar::ApbaddrPmuCpu1PmlarSpec>;
#[doc = "Performance Monitors Lock Access Register"]
pub mod apbaddr_pmu_cpu1_pmlar;
#[doc = "APBADDR_PMU_CPU1_PMLSR (rw) register accessor: Performance Monitors Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmlsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmlsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmlsr`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMLSR")]
pub type ApbaddrPmuCpu1Pmlsr = crate::Reg<apbaddr_pmu_cpu1_pmlsr::ApbaddrPmuCpu1PmlsrSpec>;
#[doc = "Performance Monitors Lock Status Register"]
pub mod apbaddr_pmu_cpu1_pmlsr;
#[doc = "APBADDR_PMU_CPU1_PMAUTHSTATUS (rw) register accessor: Performance Monitors Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmauthstatus`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMAUTHSTATUS")]
pub type ApbaddrPmuCpu1Pmauthstatus =
    crate::Reg<apbaddr_pmu_cpu1_pmauthstatus::ApbaddrPmuCpu1PmauthstatusSpec>;
#[doc = "Performance Monitors Authentication Status Register"]
pub mod apbaddr_pmu_cpu1_pmauthstatus;
#[doc = "APBADDR_PMU_CPU1_PMDEVARCH (rw) register accessor: Performance Monitors Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmdevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmdevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmdevarch`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMDEVARCH")]
pub type ApbaddrPmuCpu1Pmdevarch =
    crate::Reg<apbaddr_pmu_cpu1_pmdevarch::ApbaddrPmuCpu1PmdevarchSpec>;
#[doc = "Performance Monitors Device Architecture Register"]
pub mod apbaddr_pmu_cpu1_pmdevarch;
#[doc = "APBADDR_PMU_CPU1_PMDEVTYPE (rw) register accessor: Performance Monitors Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmdevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmdevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmdevtype`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMDEVTYPE")]
pub type ApbaddrPmuCpu1Pmdevtype =
    crate::Reg<apbaddr_pmu_cpu1_pmdevtype::ApbaddrPmuCpu1PmdevtypeSpec>;
#[doc = "Performance Monitors Device Type Register"]
pub mod apbaddr_pmu_cpu1_pmdevtype;
#[doc = "APBADDR_PMU_CPU1_PMPIDR4 (rw) register accessor: Performance Monitors Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr4`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR4")]
pub type ApbaddrPmuCpu1Pmpidr4 = crate::Reg<apbaddr_pmu_cpu1_pmpidr4::ApbaddrPmuCpu1Pmpidr4Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 4"]
pub mod apbaddr_pmu_cpu1_pmpidr4;
#[doc = "APBADDR_PMU_CPU1_PMPIDR5 (rw) register accessor: Performance Monitors Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr5`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR5")]
pub type ApbaddrPmuCpu1Pmpidr5 = crate::Reg<apbaddr_pmu_cpu1_pmpidr5::ApbaddrPmuCpu1Pmpidr5Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 5"]
pub mod apbaddr_pmu_cpu1_pmpidr5;
#[doc = "APBADDR_PMU_CPU1_PMPIDR6 (rw) register accessor: Performance Monitors Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr6`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR6")]
pub type ApbaddrPmuCpu1Pmpidr6 = crate::Reg<apbaddr_pmu_cpu1_pmpidr6::ApbaddrPmuCpu1Pmpidr6Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 6"]
pub mod apbaddr_pmu_cpu1_pmpidr6;
#[doc = "APBADDR_PMU_CPU1_PMPIDR7 (rw) register accessor: Performance Monitors Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr7`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR7")]
pub type ApbaddrPmuCpu1Pmpidr7 = crate::Reg<apbaddr_pmu_cpu1_pmpidr7::ApbaddrPmuCpu1Pmpidr7Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 7"]
pub mod apbaddr_pmu_cpu1_pmpidr7;
#[doc = "APBADDR_PMU_CPU1_PMPIDR0 (rw) register accessor: Performance Monitors Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR0")]
pub type ApbaddrPmuCpu1Pmpidr0 = crate::Reg<apbaddr_pmu_cpu1_pmpidr0::ApbaddrPmuCpu1Pmpidr0Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 0"]
pub mod apbaddr_pmu_cpu1_pmpidr0;
#[doc = "APBADDR_PMU_CPU1_PMPIDR1 (rw) register accessor: Performance Monitors Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR1")]
pub type ApbaddrPmuCpu1Pmpidr1 = crate::Reg<apbaddr_pmu_cpu1_pmpidr1::ApbaddrPmuCpu1Pmpidr1Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 1"]
pub mod apbaddr_pmu_cpu1_pmpidr1;
#[doc = "APBADDR_PMU_CPU1_PMPIDR2 (rw) register accessor: Performance Monitors Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr2`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR2")]
pub type ApbaddrPmuCpu1Pmpidr2 = crate::Reg<apbaddr_pmu_cpu1_pmpidr2::ApbaddrPmuCpu1Pmpidr2Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 2"]
pub mod apbaddr_pmu_cpu1_pmpidr2;
#[doc = "APBADDR_PMU_CPU1_PMPIDR3 (rw) register accessor: Performance Monitors Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmpidr3`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMPIDR3")]
pub type ApbaddrPmuCpu1Pmpidr3 = crate::Reg<apbaddr_pmu_cpu1_pmpidr3::ApbaddrPmuCpu1Pmpidr3Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 3"]
pub mod apbaddr_pmu_cpu1_pmpidr3;
#[doc = "APBADDR_PMU_CPU1_PMCIDR0 (rw) register accessor: Performance Monitors Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcidr0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCIDR0")]
pub type ApbaddrPmuCpu1Pmcidr0 = crate::Reg<apbaddr_pmu_cpu1_pmcidr0::ApbaddrPmuCpu1Pmcidr0Spec>;
#[doc = "Performance Monitors Component Identification Register 0"]
pub mod apbaddr_pmu_cpu1_pmcidr0;
#[doc = "APBADDR_PMU_CPU1_PMCIDR1 (rw) register accessor: Performance Monitors Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcidr1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCIDR1")]
pub type ApbaddrPmuCpu1Pmcidr1 = crate::Reg<apbaddr_pmu_cpu1_pmcidr1::ApbaddrPmuCpu1Pmcidr1Spec>;
#[doc = "Performance Monitors Component Identification Register 1"]
pub mod apbaddr_pmu_cpu1_pmcidr1;
#[doc = "APBADDR_PMU_CPU1_PMCIDR2 (rw) register accessor: Performance Monitors Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcidr2`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCIDR2")]
pub type ApbaddrPmuCpu1Pmcidr2 = crate::Reg<apbaddr_pmu_cpu1_pmcidr2::ApbaddrPmuCpu1Pmcidr2Spec>;
#[doc = "Performance Monitors Component Identification Register 2"]
pub mod apbaddr_pmu_cpu1_pmcidr2;
#[doc = "APBADDR_PMU_CPU1_PMCIDR3 (rw) register accessor: Performance Monitors Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu1_pmcidr3`]
module"]
#[doc(alias = "APBADDR_PMU_CPU1_PMCIDR3")]
pub type ApbaddrPmuCpu1Pmcidr3 = crate::Reg<apbaddr_pmu_cpu1_pmcidr3::ApbaddrPmuCpu1Pmcidr3Spec>;
#[doc = "Performance Monitors Component Identification Register 3"]
pub mod apbaddr_pmu_cpu1_pmcidr3;
