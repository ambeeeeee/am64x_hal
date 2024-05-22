#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mem_elm_revision: MemElmRevision,
    _reserved1: [u8; 0x0c],
    mem_elm_sysconfig: MemElmSysconfig,
    mem_elm_sysstatus: MemElmSysstatus,
    mem_elm_irqstatus: MemElmIrqstatus,
    mem_elm_irqenable: MemElmIrqenable,
    mem_elm_location_config: MemElmLocationConfig,
    _reserved6: [u8; 0x5c],
    mem_elm_page_ctrl: MemElmPageCtrl,
    _reserved7: [u8; 0x037c],
    mem_elm_syndrome_fragment_0: MemElmSyndromeFragment0,
    mem_elm_syndrome_fragment_1: MemElmSyndromeFragment1,
    mem_elm_syndrome_fragment_2: MemElmSyndromeFragment2,
    mem_elm_syndrome_fragment_3: MemElmSyndromeFragment3,
    mem_elm_syndrome_fragment_4: MemElmSyndromeFragment4,
    mem_elm_syndrome_fragment_5: MemElmSyndromeFragment5,
    mem_elm_syndrome_fragment_6: MemElmSyndromeFragment6,
    _reserved14: [u8; 0x03e4],
    mem_elm_location_status: MemElmLocationStatus,
    _reserved15: [u8; 0x7c],
    mem_elm_error_location_0: MemElmErrorLocation0,
    mem_elm_error_location_1: MemElmErrorLocation1,
    mem_elm_error_location_2: MemElmErrorLocation2,
    mem_elm_error_location_3: MemElmErrorLocation3,
    mem_elm_error_location_4: MemElmErrorLocation4,
    mem_elm_error_location_5: MemElmErrorLocation5,
    mem_elm_error_location_6: MemElmErrorLocation6,
    mem_elm_error_location_7: MemElmErrorLocation7,
    mem_elm_error_location_8: MemElmErrorLocation8,
    mem_elm_error_location_9: MemElmErrorLocation9,
    mem_elm_error_location_10: MemElmErrorLocation10,
    mem_elm_error_location_11: MemElmErrorLocation11,
    mem_elm_error_location_12: MemElmErrorLocation12,
    mem_elm_error_location_13: MemElmErrorLocation13,
    mem_elm_error_location_14: MemElmErrorLocation14,
    mem_elm_error_location_15: MemElmErrorLocation15,
}
impl RegisterBlock {
    #[doc = "0x00 - This register contains the IP revision code. (A write to this register has no effect, the same as the reset)"]
    #[inline(always)]
    pub const fn mem_elm_revision(&self) -> &MemElmRevision {
        &self.mem_elm_revision
    }
    #[doc = "0x10 - This register allows controlling various parameters of the OCP interface"]
    #[inline(always)]
    pub const fn mem_elm_sysconfig(&self) -> &MemElmSysconfig {
        &self.mem_elm_sysconfig
    }
    #[doc = "0x14 - Internal Reset monitoring (OCP domain) Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
    #[inline(always)]
    pub const fn mem_elm_sysstatus(&self) -> &MemElmSysstatus {
        &self.mem_elm_sysstatus
    }
    #[doc = "0x18 - Interrupt status. This register doubles as a status register for the error location processes."]
    #[inline(always)]
    pub const fn mem_elm_irqstatus(&self) -> &MemElmIrqstatus {
        &self.mem_elm_irqstatus
    }
    #[doc = "0x1c - Interrupt enable"]
    #[inline(always)]
    pub const fn mem_elm_irqenable(&self) -> &MemElmIrqenable {
        &self.mem_elm_irqenable
    }
    #[doc = "0x20 - ECC algorithm parameters"]
    #[inline(always)]
    pub const fn mem_elm_location_config(&self) -> &MemElmLocationConfig {
        &self.mem_elm_location_config
    }
    #[doc = "0x80 - Page definition"]
    #[inline(always)]
    pub const fn mem_elm_page_ctrl(&self) -> &MemElmPageCtrl {
        &self.mem_elm_page_ctrl
    }
    #[doc = "0x400 - Input syndrome polynomial bits 0 to 31."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_0(&self) -> &MemElmSyndromeFragment0 {
        &self.mem_elm_syndrome_fragment_0
    }
    #[doc = "0x404 - Input syndrome polynomial bits 32 to 63."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_1(&self) -> &MemElmSyndromeFragment1 {
        &self.mem_elm_syndrome_fragment_1
    }
    #[doc = "0x408 - Input syndrome polynomial bits 64 to 95."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_2(&self) -> &MemElmSyndromeFragment2 {
        &self.mem_elm_syndrome_fragment_2
    }
    #[doc = "0x40c - Input syndrome polynomial bits 96 to 127"]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_3(&self) -> &MemElmSyndromeFragment3 {
        &self.mem_elm_syndrome_fragment_3
    }
    #[doc = "0x410 - Input syndrome polynomial bits 128 to 159."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_4(&self) -> &MemElmSyndromeFragment4 {
        &self.mem_elm_syndrome_fragment_4
    }
    #[doc = "0x414 - Input syndrome polynomial bits 160 to 191."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_5(&self) -> &MemElmSyndromeFragment5 {
        &self.mem_elm_syndrome_fragment_5
    }
    #[doc = "0x418 - Input syndrome polynomial bits 192 to 207."]
    #[inline(always)]
    pub const fn mem_elm_syndrome_fragment_6(&self) -> &MemElmSyndromeFragment6 {
        &self.mem_elm_syndrome_fragment_6
    }
    #[doc = "0x800 - Exit status for the syndrome polynomial processing"]
    #[inline(always)]
    pub const fn mem_elm_location_status(&self) -> &MemElmLocationStatus {
        &self.mem_elm_location_status
    }
    #[doc = "0x880 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_0(&self) -> &MemElmErrorLocation0 {
        &self.mem_elm_error_location_0
    }
    #[doc = "0x884 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_1(&self) -> &MemElmErrorLocation1 {
        &self.mem_elm_error_location_1
    }
    #[doc = "0x888 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_2(&self) -> &MemElmErrorLocation2 {
        &self.mem_elm_error_location_2
    }
    #[doc = "0x88c - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_3(&self) -> &MemElmErrorLocation3 {
        &self.mem_elm_error_location_3
    }
    #[doc = "0x890 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_4(&self) -> &MemElmErrorLocation4 {
        &self.mem_elm_error_location_4
    }
    #[doc = "0x894 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_5(&self) -> &MemElmErrorLocation5 {
        &self.mem_elm_error_location_5
    }
    #[doc = "0x898 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_6(&self) -> &MemElmErrorLocation6 {
        &self.mem_elm_error_location_6
    }
    #[doc = "0x89c - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_7(&self) -> &MemElmErrorLocation7 {
        &self.mem_elm_error_location_7
    }
    #[doc = "0x8a0 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_8(&self) -> &MemElmErrorLocation8 {
        &self.mem_elm_error_location_8
    }
    #[doc = "0x8a4 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_9(&self) -> &MemElmErrorLocation9 {
        &self.mem_elm_error_location_9
    }
    #[doc = "0x8a8 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_10(&self) -> &MemElmErrorLocation10 {
        &self.mem_elm_error_location_10
    }
    #[doc = "0x8ac - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_11(&self) -> &MemElmErrorLocation11 {
        &self.mem_elm_error_location_11
    }
    #[doc = "0x8b0 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_12(&self) -> &MemElmErrorLocation12 {
        &self.mem_elm_error_location_12
    }
    #[doc = "0x8b4 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_13(&self) -> &MemElmErrorLocation13 {
        &self.mem_elm_error_location_13
    }
    #[doc = "0x8b8 - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_14(&self) -> &MemElmErrorLocation14 {
        &self.mem_elm_error_location_14
    }
    #[doc = "0x8bc - Error location register"]
    #[inline(always)]
    pub const fn mem_elm_error_location_15(&self) -> &MemElmErrorLocation15 {
        &self.mem_elm_error_location_15
    }
}
#[doc = "MEM_ELM_REVISION (rw) register accessor: This register contains the IP revision code. (A write to this register has no effect, the same as the reset)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_revision`]
module"]
#[doc(alias = "MEM_ELM_REVISION")]
pub type MemElmRevision = crate::Reg<mem_elm_revision::MemElmRevisionSpec>;
#[doc = "This register contains the IP revision code. (A write to this register has no effect, the same as the reset)"]
pub mod mem_elm_revision;
#[doc = "MEM_ELM_SYSCONFIG (rw) register accessor: This register allows controlling various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_sysconfig`]
module"]
#[doc(alias = "MEM_ELM_SYSCONFIG")]
pub type MemElmSysconfig = crate::Reg<mem_elm_sysconfig::MemElmSysconfigSpec>;
#[doc = "This register allows controlling various parameters of the OCP interface"]
pub mod mem_elm_sysconfig;
#[doc = "MEM_ELM_SYSSTATUS (rw) register accessor: Internal Reset monitoring (OCP domain) Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_sysstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_sysstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_sysstatus`]
module"]
#[doc(alias = "MEM_ELM_SYSSTATUS")]
pub type MemElmSysstatus = crate::Reg<mem_elm_sysstatus::MemElmSysstatusSpec>;
#[doc = "Internal Reset monitoring (OCP domain) Undefined since: on HW perspective reset state is 0 on SW user perspective when module is accessible is 1"]
pub mod mem_elm_sysstatus;
#[doc = "MEM_ELM_IRQSTATUS (rw) register accessor: Interrupt status. This register doubles as a status register for the error location processes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_irqstatus`]
module"]
#[doc(alias = "MEM_ELM_IRQSTATUS")]
pub type MemElmIrqstatus = crate::Reg<mem_elm_irqstatus::MemElmIrqstatusSpec>;
#[doc = "Interrupt status. This register doubles as a status register for the error location processes."]
pub mod mem_elm_irqstatus;
#[doc = "MEM_ELM_IRQENABLE (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_irqenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_irqenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_irqenable`]
module"]
#[doc(alias = "MEM_ELM_IRQENABLE")]
pub type MemElmIrqenable = crate::Reg<mem_elm_irqenable::MemElmIrqenableSpec>;
#[doc = "Interrupt enable"]
pub mod mem_elm_irqenable;
#[doc = "MEM_ELM_LOCATION_CONFIG (rw) register accessor: ECC algorithm parameters\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_location_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_location_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_location_config`]
module"]
#[doc(alias = "MEM_ELM_LOCATION_CONFIG")]
pub type MemElmLocationConfig = crate::Reg<mem_elm_location_config::MemElmLocationConfigSpec>;
#[doc = "ECC algorithm parameters"]
pub mod mem_elm_location_config;
#[doc = "MEM_ELM_PAGE_CTRL (rw) register accessor: Page definition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_page_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_page_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_page_ctrl`]
module"]
#[doc(alias = "MEM_ELM_PAGE_CTRL")]
pub type MemElmPageCtrl = crate::Reg<mem_elm_page_ctrl::MemElmPageCtrlSpec>;
#[doc = "Page definition"]
pub mod mem_elm_page_ctrl;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_0 (rw) register accessor: Input syndrome polynomial bits 0 to 31.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_0`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_0")]
pub type MemElmSyndromeFragment0 =
    crate::Reg<mem_elm_syndrome_fragment_0::MemElmSyndromeFragment0Spec>;
#[doc = "Input syndrome polynomial bits 0 to 31."]
pub mod mem_elm_syndrome_fragment_0;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_1 (rw) register accessor: Input syndrome polynomial bits 32 to 63.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_1`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_1")]
pub type MemElmSyndromeFragment1 =
    crate::Reg<mem_elm_syndrome_fragment_1::MemElmSyndromeFragment1Spec>;
#[doc = "Input syndrome polynomial bits 32 to 63."]
pub mod mem_elm_syndrome_fragment_1;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_2 (rw) register accessor: Input syndrome polynomial bits 64 to 95.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_2`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_2")]
pub type MemElmSyndromeFragment2 =
    crate::Reg<mem_elm_syndrome_fragment_2::MemElmSyndromeFragment2Spec>;
#[doc = "Input syndrome polynomial bits 64 to 95."]
pub mod mem_elm_syndrome_fragment_2;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_3 (rw) register accessor: Input syndrome polynomial bits 96 to 127\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_3`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_3")]
pub type MemElmSyndromeFragment3 =
    crate::Reg<mem_elm_syndrome_fragment_3::MemElmSyndromeFragment3Spec>;
#[doc = "Input syndrome polynomial bits 96 to 127"]
pub mod mem_elm_syndrome_fragment_3;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_4 (rw) register accessor: Input syndrome polynomial bits 128 to 159.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_4`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_4")]
pub type MemElmSyndromeFragment4 =
    crate::Reg<mem_elm_syndrome_fragment_4::MemElmSyndromeFragment4Spec>;
#[doc = "Input syndrome polynomial bits 128 to 159."]
pub mod mem_elm_syndrome_fragment_4;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_5 (rw) register accessor: Input syndrome polynomial bits 160 to 191.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_5`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_5")]
pub type MemElmSyndromeFragment5 =
    crate::Reg<mem_elm_syndrome_fragment_5::MemElmSyndromeFragment5Spec>;
#[doc = "Input syndrome polynomial bits 160 to 191."]
pub mod mem_elm_syndrome_fragment_5;
#[doc = "MEM_ELM_SYNDROME_FRAGMENT_6 (rw) register accessor: Input syndrome polynomial bits 192 to 207.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_syndrome_fragment_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_syndrome_fragment_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_syndrome_fragment_6`]
module"]
#[doc(alias = "MEM_ELM_SYNDROME_FRAGMENT_6")]
pub type MemElmSyndromeFragment6 =
    crate::Reg<mem_elm_syndrome_fragment_6::MemElmSyndromeFragment6Spec>;
#[doc = "Input syndrome polynomial bits 192 to 207."]
pub mod mem_elm_syndrome_fragment_6;
#[doc = "MEM_ELM_LOCATION_STATUS (rw) register accessor: Exit status for the syndrome polynomial processing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_location_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_location_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_location_status`]
module"]
#[doc(alias = "MEM_ELM_LOCATION_STATUS")]
pub type MemElmLocationStatus = crate::Reg<mem_elm_location_status::MemElmLocationStatusSpec>;
#[doc = "Exit status for the syndrome polynomial processing"]
pub mod mem_elm_location_status;
#[doc = "MEM_ELM_ERROR_LOCATION_0 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_0`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_0")]
pub type MemElmErrorLocation0 = crate::Reg<mem_elm_error_location_0::MemElmErrorLocation0Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_0;
#[doc = "MEM_ELM_ERROR_LOCATION_1 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_1`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_1")]
pub type MemElmErrorLocation1 = crate::Reg<mem_elm_error_location_1::MemElmErrorLocation1Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_1;
#[doc = "MEM_ELM_ERROR_LOCATION_2 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_2`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_2")]
pub type MemElmErrorLocation2 = crate::Reg<mem_elm_error_location_2::MemElmErrorLocation2Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_2;
#[doc = "MEM_ELM_ERROR_LOCATION_3 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_3`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_3")]
pub type MemElmErrorLocation3 = crate::Reg<mem_elm_error_location_3::MemElmErrorLocation3Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_3;
#[doc = "MEM_ELM_ERROR_LOCATION_4 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_4`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_4")]
pub type MemElmErrorLocation4 = crate::Reg<mem_elm_error_location_4::MemElmErrorLocation4Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_4;
#[doc = "MEM_ELM_ERROR_LOCATION_5 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_5`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_5")]
pub type MemElmErrorLocation5 = crate::Reg<mem_elm_error_location_5::MemElmErrorLocation5Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_5;
#[doc = "MEM_ELM_ERROR_LOCATION_6 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_6`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_6")]
pub type MemElmErrorLocation6 = crate::Reg<mem_elm_error_location_6::MemElmErrorLocation6Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_6;
#[doc = "MEM_ELM_ERROR_LOCATION_7 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_7`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_7")]
pub type MemElmErrorLocation7 = crate::Reg<mem_elm_error_location_7::MemElmErrorLocation7Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_7;
#[doc = "MEM_ELM_ERROR_LOCATION_8 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_8`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_8")]
pub type MemElmErrorLocation8 = crate::Reg<mem_elm_error_location_8::MemElmErrorLocation8Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_8;
#[doc = "MEM_ELM_ERROR_LOCATION_9 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_9`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_9")]
pub type MemElmErrorLocation9 = crate::Reg<mem_elm_error_location_9::MemElmErrorLocation9Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_9;
#[doc = "MEM_ELM_ERROR_LOCATION_10 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_10`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_10")]
pub type MemElmErrorLocation10 = crate::Reg<mem_elm_error_location_10::MemElmErrorLocation10Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_10;
#[doc = "MEM_ELM_ERROR_LOCATION_11 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_11`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_11")]
pub type MemElmErrorLocation11 = crate::Reg<mem_elm_error_location_11::MemElmErrorLocation11Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_11;
#[doc = "MEM_ELM_ERROR_LOCATION_12 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_12`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_12")]
pub type MemElmErrorLocation12 = crate::Reg<mem_elm_error_location_12::MemElmErrorLocation12Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_12;
#[doc = "MEM_ELM_ERROR_LOCATION_13 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_13`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_13")]
pub type MemElmErrorLocation13 = crate::Reg<mem_elm_error_location_13::MemElmErrorLocation13Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_13;
#[doc = "MEM_ELM_ERROR_LOCATION_14 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_14`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_14")]
pub type MemElmErrorLocation14 = crate::Reg<mem_elm_error_location_14::MemElmErrorLocation14Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_14;
#[doc = "MEM_ELM_ERROR_LOCATION_15 (rw) register accessor: Error location register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_error_location_15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_error_location_15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_elm_error_location_15`]
module"]
#[doc(alias = "MEM_ELM_ERROR_LOCATION_15")]
pub type MemElmErrorLocation15 = crate::Reg<mem_elm_error_location_15::MemElmErrorLocation15Spec>;
#[doc = "Error location register"]
pub mod mem_elm_error_location_15;
