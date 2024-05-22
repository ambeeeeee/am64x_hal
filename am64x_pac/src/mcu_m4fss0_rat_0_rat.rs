#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rat__cfg__mmrs_pid: Rat_Cfg_MmrsPid,
    rat__cfg__mmrs_config: Rat_Cfg_MmrsConfig,
    _reserved2: [u8; 0x18],
    rat__cfg__mmrs_ctrl: Rat_Cfg_MmrsCtrl,
    rat__cfg__mmrs_base: Rat_Cfg_MmrsBase,
    rat__cfg__mmrs_trans_l: Rat_Cfg_MmrsTransL,
    rat__cfg__mmrs_trans_u: Rat_Cfg_MmrsTransU,
    _reserved6: [u8; 0x07d4],
    rat__cfg__mmrs_destination_id: Rat_Cfg_MmrsDestinationId,
    _reserved7: [u8; 0x18],
    rat__cfg__mmrs_exception_logging_control: Rat_Cfg_MmrsExceptionLoggingControl,
    rat__cfg__mmrs_exception_logging_header0: Rat_Cfg_MmrsExceptionLoggingHeader0,
    rat__cfg__mmrs_exception_logging_header1: Rat_Cfg_MmrsExceptionLoggingHeader1,
    rat__cfg__mmrs_exception_logging_data0: Rat_Cfg_MmrsExceptionLoggingData0,
    rat__cfg__mmrs_exception_logging_data1: Rat_Cfg_MmrsExceptionLoggingData1,
    rat__cfg__mmrs_exception_logging_data2: Rat_Cfg_MmrsExceptionLoggingData2,
    rat__cfg__mmrs_exception_logging_data3: Rat_Cfg_MmrsExceptionLoggingData3,
    _reserved14: [u8; 0x04],
    rat__cfg__mmrs_exception_pend_set: Rat_Cfg_MmrsExceptionPendSet,
    rat__cfg__mmrs_exception_pend_clear: Rat_Cfg_MmrsExceptionPendClear,
    rat__cfg__mmrs_exception_enable_set: Rat_Cfg_MmrsExceptionEnableSet,
    rat__cfg__mmrs_exception_enable_clear: Rat_Cfg_MmrsExceptionEnableClear,
    rat__cfg__mmrs_eoi_reg: Rat_Cfg_MmrsEoiReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_pid(&self) -> &Rat_Cfg_MmrsPid {
        &self.rat__cfg__mmrs_pid
    }
    #[doc = "0x04 - The Config Register contains the configuration values for the module."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_config(&self) -> &Rat_Cfg_MmrsConfig {
        &self.rat__cfg__mmrs_config
    }
    #[doc = "0x20 - The Control for Region a"]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_ctrl(&self) -> &Rat_Cfg_MmrsCtrl {
        &self.rat__cfg__mmrs_ctrl
    }
    #[doc = "0x24 - The Base Address for Region a. This is the source address for matching to a region."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_base(&self) -> &Rat_Cfg_MmrsBase {
        &self.rat__cfg__mmrs_base
    }
    #[doc = "0x28 - The Translated Lower Address Bits for Region a"]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_trans_l(&self) -> &Rat_Cfg_MmrsTransL {
        &self.rat__cfg__mmrs_trans_l
    }
    #[doc = "0x2c - The Translated Upper Address Bits for Region a"]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_trans_u(&self) -> &Rat_Cfg_MmrsTransU {
        &self.rat__cfg__mmrs_trans_u
    }
    #[doc = "0x804 - The Destination ID Register defines the destination ID value for error messages."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_destination_id(&self) -> &Rat_Cfg_MmrsDestinationId {
        &self.rat__cfg__mmrs_destination_id
    }
    #[doc = "0x820 - The Exception Logging Control Register controls the exception logging."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_control(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingControl {
        &self.rat__cfg__mmrs_exception_logging_control
    }
    #[doc = "0x824 - The Exception Logging Header 0 Register contains the first word of the header."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_header0(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingHeader0 {
        &self.rat__cfg__mmrs_exception_logging_header0
    }
    #[doc = "0x828 - The Exception Logging Header 1 Register contains the second word of the header."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_header1(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingHeader1 {
        &self.rat__cfg__mmrs_exception_logging_header1
    }
    #[doc = "0x82c - The Exception Logging Data 0 Register contains the first word of the data."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_data0(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingData0 {
        &self.rat__cfg__mmrs_exception_logging_data0
    }
    #[doc = "0x830 - The Exception Logging Data 1 Register contains the second word of the data."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_data1(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingData1 {
        &self.rat__cfg__mmrs_exception_logging_data1
    }
    #[doc = "0x834 - The Exception Logging Data 2 Register contains the third word of the data."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_data2(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingData2 {
        &self.rat__cfg__mmrs_exception_logging_data2
    }
    #[doc = "0x838 - The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_logging_data3(
        &self,
    ) -> &Rat_Cfg_MmrsExceptionLoggingData3 {
        &self.rat__cfg__mmrs_exception_logging_data3
    }
    #[doc = "0x840 - The Exception Logging Interrupt Pending Set Register allows to set the pend signal."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_pend_set(&self) -> &Rat_Cfg_MmrsExceptionPendSet {
        &self.rat__cfg__mmrs_exception_pend_set
    }
    #[doc = "0x844 - The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_pend_clear(&self) -> &Rat_Cfg_MmrsExceptionPendClear {
        &self.rat__cfg__mmrs_exception_pend_clear
    }
    #[doc = "0x848 - The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_enable_set(&self) -> &Rat_Cfg_MmrsExceptionEnableSet {
        &self.rat__cfg__mmrs_exception_enable_set
    }
    #[doc = "0x84c - The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal."]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_exception_enable_clear(&self) -> &Rat_Cfg_MmrsExceptionEnableClear {
        &self.rat__cfg__mmrs_exception_enable_clear
    }
    #[doc = "0x850 - EOI Register"]
    #[inline(always)]
    pub const fn rat__cfg__mmrs_eoi_reg(&self) -> &Rat_Cfg_MmrsEoiReg {
        &self.rat__cfg__mmrs_eoi_reg
    }
}
#[doc = "RAT__CFG__MMRS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_pid`]
module"]
#[doc(alias = "RAT__CFG__MMRS_pid")]
pub type Rat_Cfg_MmrsPid = crate::Reg<rat__cfg__mmrs_pid::Rat_Cfg_MmrsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod rat__cfg__mmrs_pid;
#[doc = "RAT__CFG__MMRS_config (rw) register accessor: The Config Register contains the configuration values for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_config`]
module"]
#[doc(alias = "RAT__CFG__MMRS_config")]
pub type Rat_Cfg_MmrsConfig = crate::Reg<rat__cfg__mmrs_config::Rat_Cfg_MmrsConfigSpec>;
#[doc = "The Config Register contains the configuration values for the module."]
pub mod rat__cfg__mmrs_config;
#[doc = "RAT__CFG__MMRS_destination_id (rw) register accessor: The Destination ID Register defines the destination ID value for error messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_destination_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_destination_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_destination_id`]
module"]
#[doc(alias = "RAT__CFG__MMRS_destination_id")]
pub type Rat_Cfg_MmrsDestinationId =
    crate::Reg<rat__cfg__mmrs_destination_id::Rat_Cfg_MmrsDestinationIdSpec>;
#[doc = "The Destination ID Register defines the destination ID value for error messages."]
pub mod rat__cfg__mmrs_destination_id;
#[doc = "RAT__CFG__MMRS_exception_logging_control (rw) register accessor: The Exception Logging Control Register controls the exception logging.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_control`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_control")]
pub type Rat_Cfg_MmrsExceptionLoggingControl =
    crate::Reg<rat__cfg__mmrs_exception_logging_control::Rat_Cfg_MmrsExceptionLoggingControlSpec>;
#[doc = "The Exception Logging Control Register controls the exception logging."]
pub mod rat__cfg__mmrs_exception_logging_control;
#[doc = "RAT__CFG__MMRS_exception_logging_header0 (rw) register accessor: The Exception Logging Header 0 Register contains the first word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_header0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_header0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_header0`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_header0")]
pub type Rat_Cfg_MmrsExceptionLoggingHeader0 =
    crate::Reg<rat__cfg__mmrs_exception_logging_header0::Rat_Cfg_MmrsExceptionLoggingHeader0Spec>;
#[doc = "The Exception Logging Header 0 Register contains the first word of the header."]
pub mod rat__cfg__mmrs_exception_logging_header0;
#[doc = "RAT__CFG__MMRS_exception_logging_header1 (rw) register accessor: The Exception Logging Header 1 Register contains the second word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_header1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_header1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_header1`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_header1")]
pub type Rat_Cfg_MmrsExceptionLoggingHeader1 =
    crate::Reg<rat__cfg__mmrs_exception_logging_header1::Rat_Cfg_MmrsExceptionLoggingHeader1Spec>;
#[doc = "The Exception Logging Header 1 Register contains the second word of the header."]
pub mod rat__cfg__mmrs_exception_logging_header1;
#[doc = "RAT__CFG__MMRS_exception_logging_data0 (rw) register accessor: The Exception Logging Data 0 Register contains the first word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_data0`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_data0")]
pub type Rat_Cfg_MmrsExceptionLoggingData0 =
    crate::Reg<rat__cfg__mmrs_exception_logging_data0::Rat_Cfg_MmrsExceptionLoggingData0Spec>;
#[doc = "The Exception Logging Data 0 Register contains the first word of the data."]
pub mod rat__cfg__mmrs_exception_logging_data0;
#[doc = "RAT__CFG__MMRS_exception_logging_data1 (rw) register accessor: The Exception Logging Data 1 Register contains the second word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_data1`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_data1")]
pub type Rat_Cfg_MmrsExceptionLoggingData1 =
    crate::Reg<rat__cfg__mmrs_exception_logging_data1::Rat_Cfg_MmrsExceptionLoggingData1Spec>;
#[doc = "The Exception Logging Data 1 Register contains the second word of the data."]
pub mod rat__cfg__mmrs_exception_logging_data1;
#[doc = "RAT__CFG__MMRS_exception_logging_data2 (rw) register accessor: The Exception Logging Data 2 Register contains the third word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_data2`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_data2")]
pub type Rat_Cfg_MmrsExceptionLoggingData2 =
    crate::Reg<rat__cfg__mmrs_exception_logging_data2::Rat_Cfg_MmrsExceptionLoggingData2Spec>;
#[doc = "The Exception Logging Data 2 Register contains the third word of the data."]
pub mod rat__cfg__mmrs_exception_logging_data2;
#[doc = "RAT__CFG__MMRS_exception_logging_data3 (rw) register accessor: The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_logging_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_logging_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_logging_data3`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_logging_data3")]
pub type Rat_Cfg_MmrsExceptionLoggingData3 =
    crate::Reg<rat__cfg__mmrs_exception_logging_data3::Rat_Cfg_MmrsExceptionLoggingData3Spec>;
#[doc = "The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit."]
pub mod rat__cfg__mmrs_exception_logging_data3;
#[doc = "RAT__CFG__MMRS_exception_pend_set (rw) register accessor: The Exception Logging Interrupt Pending Set Register allows to set the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_pend_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_pend_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_pend_set`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_pend_set")]
pub type Rat_Cfg_MmrsExceptionPendSet =
    crate::Reg<rat__cfg__mmrs_exception_pend_set::Rat_Cfg_MmrsExceptionPendSetSpec>;
#[doc = "The Exception Logging Interrupt Pending Set Register allows to set the pend signal."]
pub mod rat__cfg__mmrs_exception_pend_set;
#[doc = "RAT__CFG__MMRS_exception_pend_clear (rw) register accessor: The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_pend_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_pend_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_pend_clear`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_pend_clear")]
pub type Rat_Cfg_MmrsExceptionPendClear =
    crate::Reg<rat__cfg__mmrs_exception_pend_clear::Rat_Cfg_MmrsExceptionPendClearSpec>;
#[doc = "The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal."]
pub mod rat__cfg__mmrs_exception_pend_clear;
#[doc = "RAT__CFG__MMRS_exception_enable_set (rw) register accessor: The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_enable_set`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_enable_set")]
pub type Rat_Cfg_MmrsExceptionEnableSet =
    crate::Reg<rat__cfg__mmrs_exception_enable_set::Rat_Cfg_MmrsExceptionEnableSetSpec>;
#[doc = "The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal."]
pub mod rat__cfg__mmrs_exception_enable_set;
#[doc = "RAT__CFG__MMRS_exception_enable_clear (rw) register accessor: The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_exception_enable_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_exception_enable_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_exception_enable_clear`]
module"]
#[doc(alias = "RAT__CFG__MMRS_exception_enable_clear")]
pub type Rat_Cfg_MmrsExceptionEnableClear =
    crate::Reg<rat__cfg__mmrs_exception_enable_clear::Rat_Cfg_MmrsExceptionEnableClearSpec>;
#[doc = "The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal."]
pub mod rat__cfg__mmrs_exception_enable_clear;
#[doc = "RAT__CFG__MMRS_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_eoi_reg`]
module"]
#[doc(alias = "RAT__CFG__MMRS_eoi_reg")]
pub type Rat_Cfg_MmrsEoiReg = crate::Reg<rat__cfg__mmrs_eoi_reg::Rat_Cfg_MmrsEoiRegSpec>;
#[doc = "EOI Register"]
pub mod rat__cfg__mmrs_eoi_reg;
#[doc = "RAT__CFG__MMRS_ctrl (rw) register accessor: The Control for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_ctrl`]
module"]
#[doc(alias = "RAT__CFG__MMRS_ctrl")]
pub type Rat_Cfg_MmrsCtrl = crate::Reg<rat__cfg__mmrs_ctrl::Rat_Cfg_MmrsCtrlSpec>;
#[doc = "The Control for Region a"]
pub mod rat__cfg__mmrs_ctrl;
#[doc = "RAT__CFG__MMRS_base (rw) register accessor: The Base Address for Region a. This is the source address for matching to a region.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_base`]
module"]
#[doc(alias = "RAT__CFG__MMRS_base")]
pub type Rat_Cfg_MmrsBase = crate::Reg<rat__cfg__mmrs_base::Rat_Cfg_MmrsBaseSpec>;
#[doc = "The Base Address for Region a. This is the source address for matching to a region."]
pub mod rat__cfg__mmrs_base;
#[doc = "RAT__CFG__MMRS_trans_l (rw) register accessor: The Translated Lower Address Bits for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_trans_l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_trans_l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_trans_l`]
module"]
#[doc(alias = "RAT__CFG__MMRS_trans_l")]
pub type Rat_Cfg_MmrsTransL = crate::Reg<rat__cfg__mmrs_trans_l::Rat_Cfg_MmrsTransLSpec>;
#[doc = "The Translated Lower Address Bits for Region a"]
pub mod rat__cfg__mmrs_trans_l;
#[doc = "RAT__CFG__MMRS_trans_u (rw) register accessor: The Translated Upper Address Bits for Region a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rat__cfg__mmrs_trans_u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rat__cfg__mmrs_trans_u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rat__cfg__mmrs_trans_u`]
module"]
#[doc(alias = "RAT__CFG__MMRS_trans_u")]
pub type Rat_Cfg_MmrsTransU = crate::Reg<rat__cfg__mmrs_trans_u::Rat_Cfg_MmrsTransUSpec>;
#[doc = "The Translated Upper Address Bits for Region a"]
pub mod rat__cfg__mmrs_trans_u;
