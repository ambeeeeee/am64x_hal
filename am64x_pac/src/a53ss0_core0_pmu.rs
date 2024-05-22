#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    apbaddr_pmu_cpu0_pmevcntr0_el0: ApbaddrPmuCpu0Pmevcntr0El0,
    _reserved1: [u8; 0x04],
    apbaddr_pmu_cpu0_pmevcntr1_el0: ApbaddrPmuCpu0Pmevcntr1El0,
    _reserved2: [u8; 0x04],
    apbaddr_pmu_cpu0_pmevcntr2_el0: ApbaddrPmuCpu0Pmevcntr2El0,
    _reserved3: [u8; 0x04],
    apbaddr_pmu_cpu0_pmevcntr3_el0: ApbaddrPmuCpu0Pmevcntr3El0,
    _reserved4: [u8; 0x04],
    apbaddr_pmu_cpu0_pmevcntr4_el0: ApbaddrPmuCpu0Pmevcntr4El0,
    _reserved5: [u8; 0x04],
    apbaddr_pmu_cpu0_pmevcntr5_el0: ApbaddrPmuCpu0Pmevcntr5El0,
    _reserved6: [u8; 0xcc],
    apbaddr_pmu_cpu0_pmccntr_el0_31_0: ApbaddrPmuCpu0PmccntrEl0_31_0,
    apbaddr_pmu_cpu0_pmccntr_el0_63_32: ApbaddrPmuCpu0PmccntrEl0_63_32,
    _reserved8: [u8; 0x0300],
    apbaddr_pmu_cpu0_pmevtyper0_el0: ApbaddrPmuCpu0Pmevtyper0El0,
    apbaddr_pmu_cpu0_pmevtyper1_el0: ApbaddrPmuCpu0Pmevtyper1El0,
    apbaddr_pmu_cpu0_pmevtyper2_el0: ApbaddrPmuCpu0Pmevtyper2El0,
    apbaddr_pmu_cpu0_pmevtyper3_el0: ApbaddrPmuCpu0Pmevtyper3El0,
    apbaddr_pmu_cpu0_pmevtyper4_el0: ApbaddrPmuCpu0Pmevtyper4El0,
    apbaddr_pmu_cpu0_pmevtyper5_el0: ApbaddrPmuCpu0Pmevtyper5El0,
    _reserved14: [u8; 0x64],
    apbaddr_pmu_cpu0_pmccfiltr_el0: ApbaddrPmuCpu0PmccfiltrEl0,
    _reserved15: [u8; 0x0780],
    apbaddr_pmu_cpu0_pmcntenset_el0: ApbaddrPmuCpu0PmcntensetEl0,
    _reserved16: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmcntenclr_el0: ApbaddrPmuCpu0PmcntenclrEl0,
    _reserved17: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmintenset_el1: ApbaddrPmuCpu0PmintensetEl1,
    _reserved18: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmintenclr_el1: ApbaddrPmuCpu0PmintenclrEl1,
    _reserved19: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmovsclr_el0: ApbaddrPmuCpu0PmovsclrEl0,
    _reserved20: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmswinc_el0: ApbaddrPmuCpu0PmswincEl0,
    _reserved21: [u8; 0x1c],
    apbaddr_pmu_cpu0_pmovsset_el0: ApbaddrPmuCpu0PmovssetEl0,
    _reserved22: [u8; 0x013c],
    apbaddr_pmu_cpu0_pmcfgr: ApbaddrPmuCpu0Pmcfgr,
    apbaddr_pmu_cpu0_pmcr_el0: ApbaddrPmuCpu0PmcrEl0,
    _reserved24: [u8; 0x18],
    apbaddr_pmu_cpu0_pmceid0_el0: ApbaddrPmuCpu0Pmceid0El0,
    apbaddr_pmu_cpu0_pmceid1_el0: ApbaddrPmuCpu0Pmceid1El0,
    _reserved26: [u8; 0xd8],
    apbaddr_pmu_cpu0_pmitctrl: ApbaddrPmuCpu0Pmitctrl,
    _reserved27: [u8; 0xa4],
    apbaddr_pmu_cpu0_pmdevaff0: ApbaddrPmuCpu0Pmdevaff0,
    apbaddr_pmu_cpu0_pmdevaff1: ApbaddrPmuCpu0Pmdevaff1,
    apbaddr_pmu_cpu0_pmlar: ApbaddrPmuCpu0Pmlar,
    apbaddr_pmu_cpu0_pmlsr: ApbaddrPmuCpu0Pmlsr,
    apbaddr_pmu_cpu0_pmauthstatus: ApbaddrPmuCpu0Pmauthstatus,
    apbaddr_pmu_cpu0_pmdevarch: ApbaddrPmuCpu0Pmdevarch,
    _reserved33: [u8; 0x0c],
    apbaddr_pmu_cpu0_pmdevtype: ApbaddrPmuCpu0Pmdevtype,
    apbaddr_pmu_cpu0_pmpidr4: ApbaddrPmuCpu0Pmpidr4,
    apbaddr_pmu_cpu0_pmpidr5: ApbaddrPmuCpu0Pmpidr5,
    apbaddr_pmu_cpu0_pmpidr6: ApbaddrPmuCpu0Pmpidr6,
    apbaddr_pmu_cpu0_pmpidr7: ApbaddrPmuCpu0Pmpidr7,
    apbaddr_pmu_cpu0_pmpidr0: ApbaddrPmuCpu0Pmpidr0,
    apbaddr_pmu_cpu0_pmpidr1: ApbaddrPmuCpu0Pmpidr1,
    apbaddr_pmu_cpu0_pmpidr2: ApbaddrPmuCpu0Pmpidr2,
    apbaddr_pmu_cpu0_pmpidr3: ApbaddrPmuCpu0Pmpidr3,
    apbaddr_pmu_cpu0_pmcidr0: ApbaddrPmuCpu0Pmcidr0,
    apbaddr_pmu_cpu0_pmcidr1: ApbaddrPmuCpu0Pmcidr1,
    apbaddr_pmu_cpu0_pmcidr2: ApbaddrPmuCpu0Pmcidr2,
    apbaddr_pmu_cpu0_pmcidr3: ApbaddrPmuCpu0Pmcidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Performance Monitors Event Count Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr0_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr0El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr0_el0
    }
    #[doc = "0x08 - Performance Monitors Event Count Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr1_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr1El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr1_el0
    }
    #[doc = "0x10 - Performance Monitors Event Count Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr2_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr2El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr2_el0
    }
    #[doc = "0x18 - Performance Monitors Event Count Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr3_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr3El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr3_el0
    }
    #[doc = "0x20 - Performance Monitors Event Count Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr4_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr4El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr4_el0
    }
    #[doc = "0x28 - Performance Monitors Event Count Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevcntr5_el0(&self) -> &ApbaddrPmuCpu0Pmevcntr5El0 {
        &self.apbaddr_pmu_cpu0_pmevcntr5_el0
    }
    #[doc = "0xf8 - Performance Monitors Cycle Counter (low word)"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmccntr_el0_31_0(&self) -> &ApbaddrPmuCpu0PmccntrEl0_31_0 {
        &self.apbaddr_pmu_cpu0_pmccntr_el0_31_0
    }
    #[doc = "0xfc - Performance Monitors Cycle Counter (high word)"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmccntr_el0_63_32(&self) -> &ApbaddrPmuCpu0PmccntrEl0_63_32 {
        &self.apbaddr_pmu_cpu0_pmccntr_el0_63_32
    }
    #[doc = "0x400 - Performance Monitors Event Type Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper0_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper0El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper0_el0
    }
    #[doc = "0x404 - Performance Monitors Event Type Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper1_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper1El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper1_el0
    }
    #[doc = "0x408 - Performance Monitors Event Type Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper2_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper2El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper2_el0
    }
    #[doc = "0x40c - Performance Monitors Event Type Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper3_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper3El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper3_el0
    }
    #[doc = "0x410 - Performance Monitors Event Type Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper4_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper4El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper4_el0
    }
    #[doc = "0x414 - Performance Monitors Event Type Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmevtyper5_el0(&self) -> &ApbaddrPmuCpu0Pmevtyper5El0 {
        &self.apbaddr_pmu_cpu0_pmevtyper5_el0
    }
    #[doc = "0x47c - Performance Monitors Cycle Counter Filter Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmccfiltr_el0(&self) -> &ApbaddrPmuCpu0PmccfiltrEl0 {
        &self.apbaddr_pmu_cpu0_pmccfiltr_el0
    }
    #[doc = "0xc00 - Performance Monitors Count Enable Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcntenset_el0(&self) -> &ApbaddrPmuCpu0PmcntensetEl0 {
        &self.apbaddr_pmu_cpu0_pmcntenset_el0
    }
    #[doc = "0xc20 - Performance Monitors Count Enable Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcntenclr_el0(&self) -> &ApbaddrPmuCpu0PmcntenclrEl0 {
        &self.apbaddr_pmu_cpu0_pmcntenclr_el0
    }
    #[doc = "0xc40 - Performance Monitors Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmintenset_el1(&self) -> &ApbaddrPmuCpu0PmintensetEl1 {
        &self.apbaddr_pmu_cpu0_pmintenset_el1
    }
    #[doc = "0xc60 - Performance Monitors Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmintenclr_el1(&self) -> &ApbaddrPmuCpu0PmintenclrEl1 {
        &self.apbaddr_pmu_cpu0_pmintenclr_el1
    }
    #[doc = "0xc80 - Performance Monitors Overflow Flag Status Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmovsclr_el0(&self) -> &ApbaddrPmuCpu0PmovsclrEl0 {
        &self.apbaddr_pmu_cpu0_pmovsclr_el0
    }
    #[doc = "0xca0 - Performance Monitors Software Increment Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmswinc_el0(&self) -> &ApbaddrPmuCpu0PmswincEl0 {
        &self.apbaddr_pmu_cpu0_pmswinc_el0
    }
    #[doc = "0xcc0 - Performance Monitors Overflow Flag Status Set Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmovsset_el0(&self) -> &ApbaddrPmuCpu0PmovssetEl0 {
        &self.apbaddr_pmu_cpu0_pmovsset_el0
    }
    #[doc = "0xe00 - Performance Monitors Configuration Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcfgr(&self) -> &ApbaddrPmuCpu0Pmcfgr {
        &self.apbaddr_pmu_cpu0_pmcfgr
    }
    #[doc = "0xe04 - Performance Monitors Control Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcr_el0(&self) -> &ApbaddrPmuCpu0PmcrEl0 {
        &self.apbaddr_pmu_cpu0_pmcr_el0
    }
    #[doc = "0xe20 - Performance Monitors Common Event Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmceid0_el0(&self) -> &ApbaddrPmuCpu0Pmceid0El0 {
        &self.apbaddr_pmu_cpu0_pmceid0_el0
    }
    #[doc = "0xe24 - Performance Monitors Common Event Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmceid1_el0(&self) -> &ApbaddrPmuCpu0Pmceid1El0 {
        &self.apbaddr_pmu_cpu0_pmceid1_el0
    }
    #[doc = "0xf00 - Performance Monitors Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmitctrl(&self) -> &ApbaddrPmuCpu0Pmitctrl {
        &self.apbaddr_pmu_cpu0_pmitctrl
    }
    #[doc = "0xfa8 - Performance Monitors Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmdevaff0(&self) -> &ApbaddrPmuCpu0Pmdevaff0 {
        &self.apbaddr_pmu_cpu0_pmdevaff0
    }
    #[doc = "0xfac - Performance Monitors Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmdevaff1(&self) -> &ApbaddrPmuCpu0Pmdevaff1 {
        &self.apbaddr_pmu_cpu0_pmdevaff1
    }
    #[doc = "0xfb0 - Performance Monitors Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmlar(&self) -> &ApbaddrPmuCpu0Pmlar {
        &self.apbaddr_pmu_cpu0_pmlar
    }
    #[doc = "0xfb4 - Performance Monitors Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmlsr(&self) -> &ApbaddrPmuCpu0Pmlsr {
        &self.apbaddr_pmu_cpu0_pmlsr
    }
    #[doc = "0xfb8 - Performance Monitors Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmauthstatus(&self) -> &ApbaddrPmuCpu0Pmauthstatus {
        &self.apbaddr_pmu_cpu0_pmauthstatus
    }
    #[doc = "0xfbc - Performance Monitors Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmdevarch(&self) -> &ApbaddrPmuCpu0Pmdevarch {
        &self.apbaddr_pmu_cpu0_pmdevarch
    }
    #[doc = "0xfcc - Performance Monitors Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmdevtype(&self) -> &ApbaddrPmuCpu0Pmdevtype {
        &self.apbaddr_pmu_cpu0_pmdevtype
    }
    #[doc = "0xfd0 - Performance Monitors Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr4(&self) -> &ApbaddrPmuCpu0Pmpidr4 {
        &self.apbaddr_pmu_cpu0_pmpidr4
    }
    #[doc = "0xfd4 - Performance Monitors Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr5(&self) -> &ApbaddrPmuCpu0Pmpidr5 {
        &self.apbaddr_pmu_cpu0_pmpidr5
    }
    #[doc = "0xfd8 - Performance Monitors Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr6(&self) -> &ApbaddrPmuCpu0Pmpidr6 {
        &self.apbaddr_pmu_cpu0_pmpidr6
    }
    #[doc = "0xfdc - Performance Monitors Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr7(&self) -> &ApbaddrPmuCpu0Pmpidr7 {
        &self.apbaddr_pmu_cpu0_pmpidr7
    }
    #[doc = "0xfe0 - Performance Monitors Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr0(&self) -> &ApbaddrPmuCpu0Pmpidr0 {
        &self.apbaddr_pmu_cpu0_pmpidr0
    }
    #[doc = "0xfe4 - Performance Monitors Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr1(&self) -> &ApbaddrPmuCpu0Pmpidr1 {
        &self.apbaddr_pmu_cpu0_pmpidr1
    }
    #[doc = "0xfe8 - Performance Monitors Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr2(&self) -> &ApbaddrPmuCpu0Pmpidr2 {
        &self.apbaddr_pmu_cpu0_pmpidr2
    }
    #[doc = "0xfec - Performance Monitors Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmpidr3(&self) -> &ApbaddrPmuCpu0Pmpidr3 {
        &self.apbaddr_pmu_cpu0_pmpidr3
    }
    #[doc = "0xff0 - Performance Monitors Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcidr0(&self) -> &ApbaddrPmuCpu0Pmcidr0 {
        &self.apbaddr_pmu_cpu0_pmcidr0
    }
    #[doc = "0xff4 - Performance Monitors Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcidr1(&self) -> &ApbaddrPmuCpu0Pmcidr1 {
        &self.apbaddr_pmu_cpu0_pmcidr1
    }
    #[doc = "0xff8 - Performance Monitors Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcidr2(&self) -> &ApbaddrPmuCpu0Pmcidr2 {
        &self.apbaddr_pmu_cpu0_pmcidr2
    }
    #[doc = "0xffc - Performance Monitors Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_pmu_cpu0_pmcidr3(&self) -> &ApbaddrPmuCpu0Pmcidr3 {
        &self.apbaddr_pmu_cpu0_pmcidr3
    }
}
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR0_EL0 (rw) register accessor: Performance Monitors Event Count Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR0_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr0El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr0_el0::ApbaddrPmuCpu0Pmevcntr0El0Spec>;
#[doc = "Performance Monitors Event Count Register 0"]
pub mod apbaddr_pmu_cpu0_pmevcntr0_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR1_EL0 (rw) register accessor: Performance Monitors Event Count Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR1_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr1El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr1_el0::ApbaddrPmuCpu0Pmevcntr1El0Spec>;
#[doc = "Performance Monitors Event Count Register 1"]
pub mod apbaddr_pmu_cpu0_pmevcntr1_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR2_EL0 (rw) register accessor: Performance Monitors Event Count Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr2_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr2_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr2_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR2_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr2El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr2_el0::ApbaddrPmuCpu0Pmevcntr2El0Spec>;
#[doc = "Performance Monitors Event Count Register 2"]
pub mod apbaddr_pmu_cpu0_pmevcntr2_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR3_EL0 (rw) register accessor: Performance Monitors Event Count Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr3_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr3_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr3_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR3_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr3El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr3_el0::ApbaddrPmuCpu0Pmevcntr3El0Spec>;
#[doc = "Performance Monitors Event Count Register 3"]
pub mod apbaddr_pmu_cpu0_pmevcntr3_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR4_EL0 (rw) register accessor: Performance Monitors Event Count Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr4_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr4_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr4_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR4_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr4El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr4_el0::ApbaddrPmuCpu0Pmevcntr4El0Spec>;
#[doc = "Performance Monitors Event Count Register 4"]
pub mod apbaddr_pmu_cpu0_pmevcntr4_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVCNTR5_EL0 (rw) register accessor: Performance Monitors Event Count Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevcntr5_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevcntr5_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevcntr5_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVCNTR5_EL0")]
pub type ApbaddrPmuCpu0Pmevcntr5El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevcntr5_el0::ApbaddrPmuCpu0Pmevcntr5El0Spec>;
#[doc = "Performance Monitors Event Count Register 5"]
pub mod apbaddr_pmu_cpu0_pmevcntr5_el0;
#[doc = "APBADDR_PMU_CPU0_PMCCNTR_EL0_31_0 (rw) register accessor: Performance Monitors Cycle Counter (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmccntr_el0_31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmccntr_el0_31_0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCCNTR_EL0_31_0")]
pub type ApbaddrPmuCpu0PmccntrEl0_31_0 =
    crate::Reg<apbaddr_pmu_cpu0_pmccntr_el0_31_0::ApbaddrPmuCpu0PmccntrEl0_31_0Spec>;
#[doc = "Performance Monitors Cycle Counter (low word)"]
pub mod apbaddr_pmu_cpu0_pmccntr_el0_31_0;
#[doc = "APBADDR_PMU_CPU0_PMCCNTR_EL0_63_32 (rw) register accessor: Performance Monitors Cycle Counter (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmccntr_el0_63_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmccntr_el0_63_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmccntr_el0_63_32`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCCNTR_EL0_63_32")]
pub type ApbaddrPmuCpu0PmccntrEl0_63_32 =
    crate::Reg<apbaddr_pmu_cpu0_pmccntr_el0_63_32::ApbaddrPmuCpu0PmccntrEl0_63_32Spec>;
#[doc = "Performance Monitors Cycle Counter (high word)"]
pub mod apbaddr_pmu_cpu0_pmccntr_el0_63_32;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER0_EL0 (rw) register accessor: Performance Monitors Event Type Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER0_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper0El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper0_el0::ApbaddrPmuCpu0Pmevtyper0El0Spec>;
#[doc = "Performance Monitors Event Type Register 0"]
pub mod apbaddr_pmu_cpu0_pmevtyper0_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER1_EL0 (rw) register accessor: Performance Monitors Event Type Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER1_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper1El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper1_el0::ApbaddrPmuCpu0Pmevtyper1El0Spec>;
#[doc = "Performance Monitors Event Type Register 1"]
pub mod apbaddr_pmu_cpu0_pmevtyper1_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER2_EL0 (rw) register accessor: Performance Monitors Event Type Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper2_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper2_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper2_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER2_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper2El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper2_el0::ApbaddrPmuCpu0Pmevtyper2El0Spec>;
#[doc = "Performance Monitors Event Type Register 2"]
pub mod apbaddr_pmu_cpu0_pmevtyper2_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER3_EL0 (rw) register accessor: Performance Monitors Event Type Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper3_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper3_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper3_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER3_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper3El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper3_el0::ApbaddrPmuCpu0Pmevtyper3El0Spec>;
#[doc = "Performance Monitors Event Type Register 3"]
pub mod apbaddr_pmu_cpu0_pmevtyper3_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER4_EL0 (rw) register accessor: Performance Monitors Event Type Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper4_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper4_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper4_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER4_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper4El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper4_el0::ApbaddrPmuCpu0Pmevtyper4El0Spec>;
#[doc = "Performance Monitors Event Type Register 4"]
pub mod apbaddr_pmu_cpu0_pmevtyper4_el0;
#[doc = "APBADDR_PMU_CPU0_PMEVTYPER5_EL0 (rw) register accessor: Performance Monitors Event Type Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper5_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper5_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmevtyper5_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMEVTYPER5_EL0")]
pub type ApbaddrPmuCpu0Pmevtyper5El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmevtyper5_el0::ApbaddrPmuCpu0Pmevtyper5El0Spec>;
#[doc = "Performance Monitors Event Type Register 5"]
pub mod apbaddr_pmu_cpu0_pmevtyper5_el0;
#[doc = "APBADDR_PMU_CPU0_PMCCFILTR_EL0 (rw) register accessor: Performance Monitors Cycle Counter Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmccfiltr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmccfiltr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmccfiltr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCCFILTR_EL0")]
pub type ApbaddrPmuCpu0PmccfiltrEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmccfiltr_el0::ApbaddrPmuCpu0PmccfiltrEl0Spec>;
#[doc = "Performance Monitors Cycle Counter Filter Register"]
pub mod apbaddr_pmu_cpu0_pmccfiltr_el0;
#[doc = "APBADDR_PMU_CPU0_PMCNTENSET_EL0 (rw) register accessor: Performance Monitors Count Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcntenset_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcntenset_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcntenset_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCNTENSET_EL0")]
pub type ApbaddrPmuCpu0PmcntensetEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmcntenset_el0::ApbaddrPmuCpu0PmcntensetEl0Spec>;
#[doc = "Performance Monitors Count Enable Set Register"]
pub mod apbaddr_pmu_cpu0_pmcntenset_el0;
#[doc = "APBADDR_PMU_CPU0_PMCNTENCLR_EL0 (rw) register accessor: Performance Monitors Count Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcntenclr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcntenclr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcntenclr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCNTENCLR_EL0")]
pub type ApbaddrPmuCpu0PmcntenclrEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmcntenclr_el0::ApbaddrPmuCpu0PmcntenclrEl0Spec>;
#[doc = "Performance Monitors Count Enable Clear Register"]
pub mod apbaddr_pmu_cpu0_pmcntenclr_el0;
#[doc = "APBADDR_PMU_CPU0_PMINTENSET_EL1 (rw) register accessor: Performance Monitors Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmintenset_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmintenset_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmintenset_el1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMINTENSET_EL1")]
pub type ApbaddrPmuCpu0PmintensetEl1 =
    crate::Reg<apbaddr_pmu_cpu0_pmintenset_el1::ApbaddrPmuCpu0PmintensetEl1Spec>;
#[doc = "Performance Monitors Interrupt Enable Set Register"]
pub mod apbaddr_pmu_cpu0_pmintenset_el1;
#[doc = "APBADDR_PMU_CPU0_PMINTENCLR_EL1 (rw) register accessor: Performance Monitors Interrupt Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmintenclr_el1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmintenclr_el1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmintenclr_el1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMINTENCLR_EL1")]
pub type ApbaddrPmuCpu0PmintenclrEl1 =
    crate::Reg<apbaddr_pmu_cpu0_pmintenclr_el1::ApbaddrPmuCpu0PmintenclrEl1Spec>;
#[doc = "Performance Monitors Interrupt Enable Clear Register"]
pub mod apbaddr_pmu_cpu0_pmintenclr_el1;
#[doc = "APBADDR_PMU_CPU0_PMOVSCLR_EL0 (rw) register accessor: Performance Monitors Overflow Flag Status Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmovsclr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmovsclr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmovsclr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMOVSCLR_EL0")]
pub type ApbaddrPmuCpu0PmovsclrEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmovsclr_el0::ApbaddrPmuCpu0PmovsclrEl0Spec>;
#[doc = "Performance Monitors Overflow Flag Status Clear Register"]
pub mod apbaddr_pmu_cpu0_pmovsclr_el0;
#[doc = "APBADDR_PMU_CPU0_PMSWINC_EL0 (rw) register accessor: Performance Monitors Software Increment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmswinc_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmswinc_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmswinc_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMSWINC_EL0")]
pub type ApbaddrPmuCpu0PmswincEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmswinc_el0::ApbaddrPmuCpu0PmswincEl0Spec>;
#[doc = "Performance Monitors Software Increment Register"]
pub mod apbaddr_pmu_cpu0_pmswinc_el0;
#[doc = "APBADDR_PMU_CPU0_PMOVSSET_EL0 (rw) register accessor: Performance Monitors Overflow Flag Status Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmovsset_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmovsset_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmovsset_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMOVSSET_EL0")]
pub type ApbaddrPmuCpu0PmovssetEl0 =
    crate::Reg<apbaddr_pmu_cpu0_pmovsset_el0::ApbaddrPmuCpu0PmovssetEl0Spec>;
#[doc = "Performance Monitors Overflow Flag Status Set Register"]
pub mod apbaddr_pmu_cpu0_pmovsset_el0;
#[doc = "APBADDR_PMU_CPU0_PMCFGR (rw) register accessor: Performance Monitors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcfgr`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCFGR")]
pub type ApbaddrPmuCpu0Pmcfgr = crate::Reg<apbaddr_pmu_cpu0_pmcfgr::ApbaddrPmuCpu0PmcfgrSpec>;
#[doc = "Performance Monitors Configuration Register"]
pub mod apbaddr_pmu_cpu0_pmcfgr;
#[doc = "APBADDR_PMU_CPU0_PMCR_EL0 (rw) register accessor: Performance Monitors Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcr_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcr_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcr_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCR_EL0")]
pub type ApbaddrPmuCpu0PmcrEl0 = crate::Reg<apbaddr_pmu_cpu0_pmcr_el0::ApbaddrPmuCpu0PmcrEl0Spec>;
#[doc = "Performance Monitors Control Register"]
pub mod apbaddr_pmu_cpu0_pmcr_el0;
#[doc = "APBADDR_PMU_CPU0_PMCEID0_EL0 (rw) register accessor: Performance Monitors Common Event Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmceid0_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmceid0_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmceid0_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCEID0_EL0")]
pub type ApbaddrPmuCpu0Pmceid0El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmceid0_el0::ApbaddrPmuCpu0Pmceid0El0Spec>;
#[doc = "Performance Monitors Common Event Identification Register 0"]
pub mod apbaddr_pmu_cpu0_pmceid0_el0;
#[doc = "APBADDR_PMU_CPU0_PMCEID1_EL0 (rw) register accessor: Performance Monitors Common Event Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmceid1_el0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmceid1_el0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmceid1_el0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCEID1_EL0")]
pub type ApbaddrPmuCpu0Pmceid1El0 =
    crate::Reg<apbaddr_pmu_cpu0_pmceid1_el0::ApbaddrPmuCpu0Pmceid1El0Spec>;
#[doc = "Performance Monitors Common Event Identification Register 1"]
pub mod apbaddr_pmu_cpu0_pmceid1_el0;
#[doc = "APBADDR_PMU_CPU0_PMITCTRL (rw) register accessor: Performance Monitors Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmitctrl`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMITCTRL")]
pub type ApbaddrPmuCpu0Pmitctrl = crate::Reg<apbaddr_pmu_cpu0_pmitctrl::ApbaddrPmuCpu0PmitctrlSpec>;
#[doc = "Performance Monitors Integration mode Control Register"]
pub mod apbaddr_pmu_cpu0_pmitctrl;
#[doc = "APBADDR_PMU_CPU0_PMDEVAFF0 (rw) register accessor: Performance Monitors Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmdevaff0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMDEVAFF0")]
pub type ApbaddrPmuCpu0Pmdevaff0 =
    crate::Reg<apbaddr_pmu_cpu0_pmdevaff0::ApbaddrPmuCpu0Pmdevaff0Spec>;
#[doc = "Performance Monitors Device Affinity Register 0"]
pub mod apbaddr_pmu_cpu0_pmdevaff0;
#[doc = "APBADDR_PMU_CPU0_PMDEVAFF1 (rw) register accessor: Performance Monitors Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmdevaff1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMDEVAFF1")]
pub type ApbaddrPmuCpu0Pmdevaff1 =
    crate::Reg<apbaddr_pmu_cpu0_pmdevaff1::ApbaddrPmuCpu0Pmdevaff1Spec>;
#[doc = "Performance Monitors Device Affinity Register 1"]
pub mod apbaddr_pmu_cpu0_pmdevaff1;
#[doc = "APBADDR_PMU_CPU0_PMLAR (rw) register accessor: Performance Monitors Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmlar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmlar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmlar`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMLAR")]
pub type ApbaddrPmuCpu0Pmlar = crate::Reg<apbaddr_pmu_cpu0_pmlar::ApbaddrPmuCpu0PmlarSpec>;
#[doc = "Performance Monitors Lock Access Register"]
pub mod apbaddr_pmu_cpu0_pmlar;
#[doc = "APBADDR_PMU_CPU0_PMLSR (rw) register accessor: Performance Monitors Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmlsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmlsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmlsr`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMLSR")]
pub type ApbaddrPmuCpu0Pmlsr = crate::Reg<apbaddr_pmu_cpu0_pmlsr::ApbaddrPmuCpu0PmlsrSpec>;
#[doc = "Performance Monitors Lock Status Register"]
pub mod apbaddr_pmu_cpu0_pmlsr;
#[doc = "APBADDR_PMU_CPU0_PMAUTHSTATUS (rw) register accessor: Performance Monitors Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmauthstatus`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMAUTHSTATUS")]
pub type ApbaddrPmuCpu0Pmauthstatus =
    crate::Reg<apbaddr_pmu_cpu0_pmauthstatus::ApbaddrPmuCpu0PmauthstatusSpec>;
#[doc = "Performance Monitors Authentication Status Register"]
pub mod apbaddr_pmu_cpu0_pmauthstatus;
#[doc = "APBADDR_PMU_CPU0_PMDEVARCH (rw) register accessor: Performance Monitors Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmdevarch`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMDEVARCH")]
pub type ApbaddrPmuCpu0Pmdevarch =
    crate::Reg<apbaddr_pmu_cpu0_pmdevarch::ApbaddrPmuCpu0PmdevarchSpec>;
#[doc = "Performance Monitors Device Architecture Register"]
pub mod apbaddr_pmu_cpu0_pmdevarch;
#[doc = "APBADDR_PMU_CPU0_PMDEVTYPE (rw) register accessor: Performance Monitors Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmdevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmdevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmdevtype`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMDEVTYPE")]
pub type ApbaddrPmuCpu0Pmdevtype =
    crate::Reg<apbaddr_pmu_cpu0_pmdevtype::ApbaddrPmuCpu0PmdevtypeSpec>;
#[doc = "Performance Monitors Device Type Register"]
pub mod apbaddr_pmu_cpu0_pmdevtype;
#[doc = "APBADDR_PMU_CPU0_PMPIDR4 (rw) register accessor: Performance Monitors Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr4`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR4")]
pub type ApbaddrPmuCpu0Pmpidr4 = crate::Reg<apbaddr_pmu_cpu0_pmpidr4::ApbaddrPmuCpu0Pmpidr4Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 4"]
pub mod apbaddr_pmu_cpu0_pmpidr4;
#[doc = "APBADDR_PMU_CPU0_PMPIDR5 (rw) register accessor: Performance Monitors Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr5`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR5")]
pub type ApbaddrPmuCpu0Pmpidr5 = crate::Reg<apbaddr_pmu_cpu0_pmpidr5::ApbaddrPmuCpu0Pmpidr5Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 5"]
pub mod apbaddr_pmu_cpu0_pmpidr5;
#[doc = "APBADDR_PMU_CPU0_PMPIDR6 (rw) register accessor: Performance Monitors Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr6`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR6")]
pub type ApbaddrPmuCpu0Pmpidr6 = crate::Reg<apbaddr_pmu_cpu0_pmpidr6::ApbaddrPmuCpu0Pmpidr6Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 6"]
pub mod apbaddr_pmu_cpu0_pmpidr6;
#[doc = "APBADDR_PMU_CPU0_PMPIDR7 (rw) register accessor: Performance Monitors Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr7`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR7")]
pub type ApbaddrPmuCpu0Pmpidr7 = crate::Reg<apbaddr_pmu_cpu0_pmpidr7::ApbaddrPmuCpu0Pmpidr7Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 7"]
pub mod apbaddr_pmu_cpu0_pmpidr7;
#[doc = "APBADDR_PMU_CPU0_PMPIDR0 (rw) register accessor: Performance Monitors Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR0")]
pub type ApbaddrPmuCpu0Pmpidr0 = crate::Reg<apbaddr_pmu_cpu0_pmpidr0::ApbaddrPmuCpu0Pmpidr0Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 0"]
pub mod apbaddr_pmu_cpu0_pmpidr0;
#[doc = "APBADDR_PMU_CPU0_PMPIDR1 (rw) register accessor: Performance Monitors Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR1")]
pub type ApbaddrPmuCpu0Pmpidr1 = crate::Reg<apbaddr_pmu_cpu0_pmpidr1::ApbaddrPmuCpu0Pmpidr1Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 1"]
pub mod apbaddr_pmu_cpu0_pmpidr1;
#[doc = "APBADDR_PMU_CPU0_PMPIDR2 (rw) register accessor: Performance Monitors Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr2`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR2")]
pub type ApbaddrPmuCpu0Pmpidr2 = crate::Reg<apbaddr_pmu_cpu0_pmpidr2::ApbaddrPmuCpu0Pmpidr2Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 2"]
pub mod apbaddr_pmu_cpu0_pmpidr2;
#[doc = "APBADDR_PMU_CPU0_PMPIDR3 (rw) register accessor: Performance Monitors Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmpidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmpidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmpidr3`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMPIDR3")]
pub type ApbaddrPmuCpu0Pmpidr3 = crate::Reg<apbaddr_pmu_cpu0_pmpidr3::ApbaddrPmuCpu0Pmpidr3Spec>;
#[doc = "Performance Monitors Peripheral Identification Register 3"]
pub mod apbaddr_pmu_cpu0_pmpidr3;
#[doc = "APBADDR_PMU_CPU0_PMCIDR0 (rw) register accessor: Performance Monitors Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcidr0`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCIDR0")]
pub type ApbaddrPmuCpu0Pmcidr0 = crate::Reg<apbaddr_pmu_cpu0_pmcidr0::ApbaddrPmuCpu0Pmcidr0Spec>;
#[doc = "Performance Monitors Component Identification Register 0"]
pub mod apbaddr_pmu_cpu0_pmcidr0;
#[doc = "APBADDR_PMU_CPU0_PMCIDR1 (rw) register accessor: Performance Monitors Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcidr1`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCIDR1")]
pub type ApbaddrPmuCpu0Pmcidr1 = crate::Reg<apbaddr_pmu_cpu0_pmcidr1::ApbaddrPmuCpu0Pmcidr1Spec>;
#[doc = "Performance Monitors Component Identification Register 1"]
pub mod apbaddr_pmu_cpu0_pmcidr1;
#[doc = "APBADDR_PMU_CPU0_PMCIDR2 (rw) register accessor: Performance Monitors Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcidr2`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCIDR2")]
pub type ApbaddrPmuCpu0Pmcidr2 = crate::Reg<apbaddr_pmu_cpu0_pmcidr2::ApbaddrPmuCpu0Pmcidr2Spec>;
#[doc = "Performance Monitors Component Identification Register 2"]
pub mod apbaddr_pmu_cpu0_pmcidr2;
#[doc = "APBADDR_PMU_CPU0_PMCIDR3 (rw) register accessor: Performance Monitors Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_pmu_cpu0_pmcidr3`]
module"]
#[doc(alias = "APBADDR_PMU_CPU0_PMCIDR3")]
pub type ApbaddrPmuCpu0Pmcidr3 = crate::Reg<apbaddr_pmu_cpu0_pmcidr3::ApbaddrPmuCpu0Pmcidr3Spec>;
#[doc = "Performance Monitors Component Identification Register 3"]
pub mod apbaddr_pmu_cpu0_pmcidr3;
