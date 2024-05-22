#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_rat_slice1__cfg__mmrs_pid: Pr1RatSlice1_Cfg_MmrsPid,
    pr1_rat_slice1__cfg__mmrs_config: Pr1RatSlice1_Cfg_MmrsConfig,
    _reserved2: [u8; 0x07fc],
    pr1_rat_slice1__cfg__mmrs_destination_id: Pr1RatSlice1_Cfg_MmrsDestinationId,
    _reserved3: [u8; 0x18],
    pr1_rat_slice1__cfg__mmrs_exception_logging_control:
        Pr1RatSlice1_Cfg_MmrsExceptionLoggingControl,
    pr1_rat_slice1__cfg__mmrs_exception_logging_header0:
        Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader0,
    pr1_rat_slice1__cfg__mmrs_exception_logging_header1:
        Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader1,
    pr1_rat_slice1__cfg__mmrs_exception_logging_data0: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData0,
    pr1_rat_slice1__cfg__mmrs_exception_logging_data1: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData1,
    pr1_rat_slice1__cfg__mmrs_exception_logging_data2: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData2,
    pr1_rat_slice1__cfg__mmrs_exception_logging_data3: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData3,
    _reserved10: [u8; 0x04],
    pr1_rat_slice1__cfg__mmrs_exception_pend_set: Pr1RatSlice1_Cfg_MmrsExceptionPendSet,
    pr1_rat_slice1__cfg__mmrs_exception_pend_clear: Pr1RatSlice1_Cfg_MmrsExceptionPendClear,
    pr1_rat_slice1__cfg__mmrs_exception_enable_set: Pr1RatSlice1_Cfg_MmrsExceptionEnableSet,
    pr1_rat_slice1__cfg__mmrs_exception_enable_clear: Pr1RatSlice1_Cfg_MmrsExceptionEnableClear,
    pr1_rat_slice1__cfg__mmrs_eoi_reg: Pr1RatSlice1_Cfg_MmrsEoiReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_pid(&self) -> &Pr1RatSlice1_Cfg_MmrsPid {
        &self.pr1_rat_slice1__cfg__mmrs_pid
    }
    #[doc = "0x04 - The Config Register contains the configuration values for the module."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_config(&self) -> &Pr1RatSlice1_Cfg_MmrsConfig {
        &self.pr1_rat_slice1__cfg__mmrs_config
    }
    #[doc = "0x804 - The Destination ID Register defines the destination ID value for error messages."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_destination_id(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsDestinationId {
        &self.pr1_rat_slice1__cfg__mmrs_destination_id
    }
    #[doc = "0x820 - The Exception Logging Control Register controls the exception logging."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_control(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingControl {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_control
    }
    #[doc = "0x824 - The Exception Logging Header 0 Register contains the first word of the header."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_header0(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader0 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_header0
    }
    #[doc = "0x828 - The Exception Logging Header 1 Register contains the second word of the header."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_header1(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader1 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_header1
    }
    #[doc = "0x82c - The Exception Logging Data 0 Register contains the first word of the data."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_data0(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingData0 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_data0
    }
    #[doc = "0x830 - The Exception Logging Data 1 Register contains the second word of the data."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_data1(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingData1 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_data1
    }
    #[doc = "0x834 - The Exception Logging Data 2 Register contains the third word of the data."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_data2(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingData2 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_data2
    }
    #[doc = "0x838 - The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_logging_data3(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionLoggingData3 {
        &self.pr1_rat_slice1__cfg__mmrs_exception_logging_data3
    }
    #[doc = "0x840 - The Exception Logging Interrupt Pending Set Register allows to set the pend signal."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_pend_set(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionPendSet {
        &self.pr1_rat_slice1__cfg__mmrs_exception_pend_set
    }
    #[doc = "0x844 - The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_pend_clear(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionPendClear {
        &self.pr1_rat_slice1__cfg__mmrs_exception_pend_clear
    }
    #[doc = "0x848 - The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_enable_set(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionEnableSet {
        &self.pr1_rat_slice1__cfg__mmrs_exception_enable_set
    }
    #[doc = "0x84c - The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal."]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_exception_enable_clear(
        &self,
    ) -> &Pr1RatSlice1_Cfg_MmrsExceptionEnableClear {
        &self.pr1_rat_slice1__cfg__mmrs_exception_enable_clear
    }
    #[doc = "0x850 - EOI Register"]
    #[inline(always)]
    pub const fn pr1_rat_slice1__cfg__mmrs_eoi_reg(&self) -> &Pr1RatSlice1_Cfg_MmrsEoiReg {
        &self.pr1_rat_slice1__cfg__mmrs_eoi_reg
    }
}
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_pid`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_pid")]
pub type Pr1RatSlice1_Cfg_MmrsPid =
    crate::Reg<pr1_rat_slice1__cfg__mmrs_pid::Pr1RatSlice1_Cfg_MmrsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod pr1_rat_slice1__cfg__mmrs_pid;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_config (rw) register accessor: The Config Register contains the configuration values for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_config`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_config")]
pub type Pr1RatSlice1_Cfg_MmrsConfig =
    crate::Reg<pr1_rat_slice1__cfg__mmrs_config::Pr1RatSlice1_Cfg_MmrsConfigSpec>;
#[doc = "The Config Register contains the configuration values for the module."]
pub mod pr1_rat_slice1__cfg__mmrs_config;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_destination_id (rw) register accessor: The Destination ID Register defines the destination ID value for error messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_destination_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_destination_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_destination_id`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_destination_id")]
pub type Pr1RatSlice1_Cfg_MmrsDestinationId =
    crate::Reg<pr1_rat_slice1__cfg__mmrs_destination_id::Pr1RatSlice1_Cfg_MmrsDestinationIdSpec>;
#[doc = "The Destination ID Register defines the destination ID value for error messages."]
pub mod pr1_rat_slice1__cfg__mmrs_destination_id;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_control (rw) register accessor: The Exception Logging Control Register controls the exception logging.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_control`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_control")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingControl = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_control :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingControlSpec > ;
#[doc = "The Exception Logging Control Register controls the exception logging."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_control;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_header0 (rw) register accessor: The Exception Logging Header 0 Register contains the first word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_header0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_header0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_header0`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_header0")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader0 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_header0 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader0Spec > ;
#[doc = "The Exception Logging Header 0 Register contains the first word of the header."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_header0;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_header1 (rw) register accessor: The Exception Logging Header 1 Register contains the second word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_header1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_header1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_header1`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_header1")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader1 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_header1 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingHeader1Spec > ;
#[doc = "The Exception Logging Header 1 Register contains the second word of the header."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_header1;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data0 (rw) register accessor: The Exception Logging Data 0 Register contains the first word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_data0`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data0")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingData0 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_data0 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData0Spec > ;
#[doc = "The Exception Logging Data 0 Register contains the first word of the data."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_data0;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data1 (rw) register accessor: The Exception Logging Data 1 Register contains the second word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_data1`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data1")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingData1 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_data1 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData1Spec > ;
#[doc = "The Exception Logging Data 1 Register contains the second word of the data."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_data1;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data2 (rw) register accessor: The Exception Logging Data 2 Register contains the third word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_data2`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data2")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingData2 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_data2 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData2Spec > ;
#[doc = "The Exception Logging Data 2 Register contains the third word of the data."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_data2;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data3 (rw) register accessor: The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_logging_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_logging_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_logging_data3`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_logging_data3")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionLoggingData3 = crate :: Reg < pr1_rat_slice1__cfg__mmrs_exception_logging_data3 :: Pr1RatSlice1_Cfg_MmrsExceptionLoggingData3Spec > ;
#[doc = "The Exception Logging Data 3 Register contains the fourth word of the data. Reading this register will clear the error pending bit."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_logging_data3;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_pend_set (rw) register accessor: The Exception Logging Interrupt Pending Set Register allows to set the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_pend_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_pend_set`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_pend_set")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionPendSet = crate::Reg<
    pr1_rat_slice1__cfg__mmrs_exception_pend_set::Pr1RatSlice1_Cfg_MmrsExceptionPendSetSpec,
>;
#[doc = "The Exception Logging Interrupt Pending Set Register allows to set the pend signal."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_pend_set;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_pend_clear (rw) register accessor: The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_pend_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_pend_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_pend_clear`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_pend_clear")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionPendClear = crate::Reg<
    pr1_rat_slice1__cfg__mmrs_exception_pend_clear::Pr1RatSlice1_Cfg_MmrsExceptionPendClearSpec,
>;
#[doc = "The Exception Logging Interrupt Pending Clear Register allows to clear the pend signal."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_pend_clear;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_enable_set (rw) register accessor: The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_enable_set`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_enable_set")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionEnableSet = crate::Reg<
    pr1_rat_slice1__cfg__mmrs_exception_enable_set::Pr1RatSlice1_Cfg_MmrsExceptionEnableSetSpec,
>;
#[doc = "The Exception Logging Interrupt Enable Set Register allows to set the interrupt enable signal."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_enable_set;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_exception_enable_clear (rw) register accessor: The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_exception_enable_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_exception_enable_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_exception_enable_clear`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_exception_enable_clear")]
pub type Pr1RatSlice1_Cfg_MmrsExceptionEnableClear = crate::Reg<
    pr1_rat_slice1__cfg__mmrs_exception_enable_clear::Pr1RatSlice1_Cfg_MmrsExceptionEnableClearSpec,
>;
#[doc = "The Exception Logging Interrupt Enable Clear Register allows to clear the interrupt enable signal."]
pub mod pr1_rat_slice1__cfg__mmrs_exception_enable_clear;
#[doc = "PR1_RAT_SLICE1__CFG__MMRS_eoi_reg (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_rat_slice1__cfg__mmrs_eoi_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_rat_slice1__cfg__mmrs_eoi_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_rat_slice1__cfg__mmrs_eoi_reg`]
module"]
#[doc(alias = "PR1_RAT_SLICE1__CFG__MMRS_eoi_reg")]
pub type Pr1RatSlice1_Cfg_MmrsEoiReg =
    crate::Reg<pr1_rat_slice1__cfg__mmrs_eoi_reg::Pr1RatSlice1_Cfg_MmrsEoiRegSpec>;
#[doc = "EOI Register"]
pub mod pr1_rat_slice1__cfg__mmrs_eoi_reg;
