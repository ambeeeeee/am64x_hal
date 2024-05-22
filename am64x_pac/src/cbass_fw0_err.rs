#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    err_regs_pid: ErrRegsPid,
    err_regs_destination_id: ErrRegsDestinationId,
    _reserved2: [u8; 0x1c],
    err_regs_exception_logging_header0: ErrRegsExceptionLoggingHeader0,
    err_regs_exception_logging_header1: ErrRegsExceptionLoggingHeader1,
    err_regs_exception_logging_data0: ErrRegsExceptionLoggingData0,
    err_regs_exception_logging_data1: ErrRegsExceptionLoggingData1,
    err_regs_exception_logging_data2: ErrRegsExceptionLoggingData2,
    err_regs_exception_logging_data3: ErrRegsExceptionLoggingData3,
    _reserved8: [u8; 0x14],
    err_regs_err_intr_raw_stat: ErrRegsErrIntrRawStat,
    err_regs_err_intr_enabled_stat: ErrRegsErrIntrEnabledStat,
    err_regs_err_intr_enable_set: ErrRegsErrIntrEnableSet,
    err_regs_err_intr_enable_clr: ErrRegsErrIntrEnableClr,
    err_regs_err_eoi: ErrRegsErrEoi,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn err_regs_pid(&self) -> &ErrRegsPid {
        &self.err_regs_pid
    }
    #[doc = "0x04 - The Destination ID Register defines the destination ID value for error messages."]
    #[inline(always)]
    pub const fn err_regs_destination_id(&self) -> &ErrRegsDestinationId {
        &self.err_regs_destination_id
    }
    #[doc = "0x24 - The Exception Logging Header 0 Register contains the first word of the header."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_header0(&self) -> &ErrRegsExceptionLoggingHeader0 {
        &self.err_regs_exception_logging_header0
    }
    #[doc = "0x28 - The Exception Logging Header 1 Register contains the second word of the header."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_header1(&self) -> &ErrRegsExceptionLoggingHeader1 {
        &self.err_regs_exception_logging_header1
    }
    #[doc = "0x2c - The Exception Logging Data 0 Register contains the first word of the data."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_data0(&self) -> &ErrRegsExceptionLoggingData0 {
        &self.err_regs_exception_logging_data0
    }
    #[doc = "0x30 - The Exception Logging Data 1 Register contains the second word of the data."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_data1(&self) -> &ErrRegsExceptionLoggingData1 {
        &self.err_regs_exception_logging_data1
    }
    #[doc = "0x34 - The Exception Logging Data 2 Register contains the third word of the data."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_data2(&self) -> &ErrRegsExceptionLoggingData2 {
        &self.err_regs_exception_logging_data2
    }
    #[doc = "0x38 - The Exception Logging Data 3 Register contains the fourth word of the data."]
    #[inline(always)]
    pub const fn err_regs_exception_logging_data3(&self) -> &ErrRegsExceptionLoggingData3 {
        &self.err_regs_exception_logging_data3
    }
    #[doc = "0x50 - Global Interrupt Raw Status Register"]
    #[inline(always)]
    pub const fn err_regs_err_intr_raw_stat(&self) -> &ErrRegsErrIntrRawStat {
        &self.err_regs_err_intr_raw_stat
    }
    #[doc = "0x54 - Global Interrupt Enabled Status Register"]
    #[inline(always)]
    pub const fn err_regs_err_intr_enabled_stat(&self) -> &ErrRegsErrIntrEnabledStat {
        &self.err_regs_err_intr_enabled_stat
    }
    #[doc = "0x58 - Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn err_regs_err_intr_enable_set(&self) -> &ErrRegsErrIntrEnableSet {
        &self.err_regs_err_intr_enable_set
    }
    #[doc = "0x5c - Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn err_regs_err_intr_enable_clr(&self) -> &ErrRegsErrIntrEnableClr {
        &self.err_regs_err_intr_enable_clr
    }
    #[doc = "0x60 - EOI Register"]
    #[inline(always)]
    pub const fn err_regs_err_eoi(&self) -> &ErrRegsErrEoi {
        &self.err_regs_err_eoi
    }
}
#[doc = "ERR_REGS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_pid`]
module"]
#[doc(alias = "ERR_REGS_pid")]
pub type ErrRegsPid = crate::Reg<err_regs_pid::ErrRegsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod err_regs_pid;
#[doc = "ERR_REGS_destination_id (rw) register accessor: The Destination ID Register defines the destination ID value for error messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_destination_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_destination_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_destination_id`]
module"]
#[doc(alias = "ERR_REGS_destination_id")]
pub type ErrRegsDestinationId = crate::Reg<err_regs_destination_id::ErrRegsDestinationIdSpec>;
#[doc = "The Destination ID Register defines the destination ID value for error messages."]
pub mod err_regs_destination_id;
#[doc = "ERR_REGS_exception_logging_header0 (rw) register accessor: The Exception Logging Header 0 Register contains the first word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_header0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_header0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_header0`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_header0")]
pub type ErrRegsExceptionLoggingHeader0 =
    crate::Reg<err_regs_exception_logging_header0::ErrRegsExceptionLoggingHeader0Spec>;
#[doc = "The Exception Logging Header 0 Register contains the first word of the header."]
pub mod err_regs_exception_logging_header0;
#[doc = "ERR_REGS_exception_logging_header1 (rw) register accessor: The Exception Logging Header 1 Register contains the second word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_header1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_header1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_header1`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_header1")]
pub type ErrRegsExceptionLoggingHeader1 =
    crate::Reg<err_regs_exception_logging_header1::ErrRegsExceptionLoggingHeader1Spec>;
#[doc = "The Exception Logging Header 1 Register contains the second word of the header."]
pub mod err_regs_exception_logging_header1;
#[doc = "ERR_REGS_exception_logging_data0 (rw) register accessor: The Exception Logging Data 0 Register contains the first word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_data0`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_data0")]
pub type ErrRegsExceptionLoggingData0 =
    crate::Reg<err_regs_exception_logging_data0::ErrRegsExceptionLoggingData0Spec>;
#[doc = "The Exception Logging Data 0 Register contains the first word of the data."]
pub mod err_regs_exception_logging_data0;
#[doc = "ERR_REGS_exception_logging_data1 (rw) register accessor: The Exception Logging Data 1 Register contains the second word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_data1`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_data1")]
pub type ErrRegsExceptionLoggingData1 =
    crate::Reg<err_regs_exception_logging_data1::ErrRegsExceptionLoggingData1Spec>;
#[doc = "The Exception Logging Data 1 Register contains the second word of the data."]
pub mod err_regs_exception_logging_data1;
#[doc = "ERR_REGS_exception_logging_data2 (rw) register accessor: The Exception Logging Data 2 Register contains the third word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_data2`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_data2")]
pub type ErrRegsExceptionLoggingData2 =
    crate::Reg<err_regs_exception_logging_data2::ErrRegsExceptionLoggingData2Spec>;
#[doc = "The Exception Logging Data 2 Register contains the third word of the data."]
pub mod err_regs_exception_logging_data2;
#[doc = "ERR_REGS_exception_logging_data3 (rw) register accessor: The Exception Logging Data 3 Register contains the fourth word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_exception_logging_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_exception_logging_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_exception_logging_data3`]
module"]
#[doc(alias = "ERR_REGS_exception_logging_data3")]
pub type ErrRegsExceptionLoggingData3 =
    crate::Reg<err_regs_exception_logging_data3::ErrRegsExceptionLoggingData3Spec>;
#[doc = "The Exception Logging Data 3 Register contains the fourth word of the data."]
pub mod err_regs_exception_logging_data3;
#[doc = "ERR_REGS_err_intr_raw_stat (rw) register accessor: Global Interrupt Raw Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_raw_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_raw_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_err_intr_raw_stat`]
module"]
#[doc(alias = "ERR_REGS_err_intr_raw_stat")]
pub type ErrRegsErrIntrRawStat = crate::Reg<err_regs_err_intr_raw_stat::ErrRegsErrIntrRawStatSpec>;
#[doc = "Global Interrupt Raw Status Register"]
pub mod err_regs_err_intr_raw_stat;
#[doc = "ERR_REGS_err_intr_enabled_stat (rw) register accessor: Global Interrupt Enabled Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enabled_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enabled_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_err_intr_enabled_stat`]
module"]
#[doc(alias = "ERR_REGS_err_intr_enabled_stat")]
pub type ErrRegsErrIntrEnabledStat =
    crate::Reg<err_regs_err_intr_enabled_stat::ErrRegsErrIntrEnabledStatSpec>;
#[doc = "Global Interrupt Enabled Status Register"]
pub mod err_regs_err_intr_enabled_stat;
#[doc = "ERR_REGS_err_intr_enable_set (rw) register accessor: Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enable_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enable_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_err_intr_enable_set`]
module"]
#[doc(alias = "ERR_REGS_err_intr_enable_set")]
pub type ErrRegsErrIntrEnableSet =
    crate::Reg<err_regs_err_intr_enable_set::ErrRegsErrIntrEnableSetSpec>;
#[doc = "Interrupt Enable Set Register"]
pub mod err_regs_err_intr_enable_set;
#[doc = "ERR_REGS_err_intr_enable_clr (rw) register accessor: Interrupt Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enable_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enable_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_err_intr_enable_clr`]
module"]
#[doc(alias = "ERR_REGS_err_intr_enable_clr")]
pub type ErrRegsErrIntrEnableClr =
    crate::Reg<err_regs_err_intr_enable_clr::ErrRegsErrIntrEnableClrSpec>;
#[doc = "Interrupt Enable Clear Register"]
pub mod err_regs_err_intr_enable_clr;
#[doc = "ERR_REGS_err_eoi (rw) register accessor: EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_regs_err_eoi`]
module"]
#[doc(alias = "ERR_REGS_err_eoi")]
pub type ErrRegsErrEoi = crate::Reg<err_regs_err_eoi::ErrRegsErrEoiSpec>;
#[doc = "EOI Register"]
pub mod err_regs_err_eoi;
