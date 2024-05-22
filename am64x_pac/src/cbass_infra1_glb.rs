#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    glb_regs_pid: GlbRegsPid,
    glb_regs_destination_id: GlbRegsDestinationId,
    _reserved2: [u8; 0x18],
    glb_regs_exception_logging_control: GlbRegsExceptionLoggingControl,
    glb_regs_exception_logging_header0: GlbRegsExceptionLoggingHeader0,
    glb_regs_exception_logging_header1: GlbRegsExceptionLoggingHeader1,
    glb_regs_exception_logging_data0: GlbRegsExceptionLoggingData0,
    glb_regs_exception_logging_data1: GlbRegsExceptionLoggingData1,
    glb_regs_exception_logging_data2: GlbRegsExceptionLoggingData2,
    glb_regs_exception_logging_data3: GlbRegsExceptionLoggingData3,
    _reserved9: [u8; 0x04],
    glb_regs_exception_pend_set: GlbRegsExceptionPendSet,
    glb_regs_exception_pend_clear: GlbRegsExceptionPendClear,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn glb_regs_pid(&self) -> &GlbRegsPid {
        &self.glb_regs_pid
    }
    #[doc = "0x04 - The Destination ID Register defines the destination ID value for error messages."]
    #[inline(always)]
    pub const fn glb_regs_destination_id(&self) -> &GlbRegsDestinationId {
        &self.glb_regs_destination_id
    }
    #[doc = "0x20 - The Exception Logging Control Register controls the exception logging."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_control(&self) -> &GlbRegsExceptionLoggingControl {
        &self.glb_regs_exception_logging_control
    }
    #[doc = "0x24 - The Exception Logging Header 0 Register contains the first word of the header."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_header0(&self) -> &GlbRegsExceptionLoggingHeader0 {
        &self.glb_regs_exception_logging_header0
    }
    #[doc = "0x28 - The Exception Logging Header 1 Register contains the second word of the header."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_header1(&self) -> &GlbRegsExceptionLoggingHeader1 {
        &self.glb_regs_exception_logging_header1
    }
    #[doc = "0x2c - The Exception Logging Data 0 Register contains the first word of the data."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_data0(&self) -> &GlbRegsExceptionLoggingData0 {
        &self.glb_regs_exception_logging_data0
    }
    #[doc = "0x30 - The Exception Logging Data 1 Register contains the second word of the data."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_data1(&self) -> &GlbRegsExceptionLoggingData1 {
        &self.glb_regs_exception_logging_data1
    }
    #[doc = "0x34 - The Exception Logging Data 2 Register contains the third word of the data."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_data2(&self) -> &GlbRegsExceptionLoggingData2 {
        &self.glb_regs_exception_logging_data2
    }
    #[doc = "0x38 - The Exception Logging Data 3 Register contains the fourth word of the data."]
    #[inline(always)]
    pub const fn glb_regs_exception_logging_data3(&self) -> &GlbRegsExceptionLoggingData3 {
        &self.glb_regs_exception_logging_data3
    }
    #[doc = "0x40 - The Exception Logging Pending Set Register allows to set the pend signal."]
    #[inline(always)]
    pub const fn glb_regs_exception_pend_set(&self) -> &GlbRegsExceptionPendSet {
        &self.glb_regs_exception_pend_set
    }
    #[doc = "0x44 - The Exception Logging Pending Clear Register allows to clear the pend signal."]
    #[inline(always)]
    pub const fn glb_regs_exception_pend_clear(&self) -> &GlbRegsExceptionPendClear {
        &self.glb_regs_exception_pend_clear
    }
}
#[doc = "GLB_REGS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_pid`]
module"]
#[doc(alias = "GLB_REGS_pid")]
pub type GlbRegsPid = crate::Reg<glb_regs_pid::GlbRegsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod glb_regs_pid;
#[doc = "GLB_REGS_destination_id (rw) register accessor: The Destination ID Register defines the destination ID value for error messages.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_destination_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_destination_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_destination_id`]
module"]
#[doc(alias = "GLB_REGS_destination_id")]
pub type GlbRegsDestinationId = crate::Reg<glb_regs_destination_id::GlbRegsDestinationIdSpec>;
#[doc = "The Destination ID Register defines the destination ID value for error messages."]
pub mod glb_regs_destination_id;
#[doc = "GLB_REGS_exception_logging_control (rw) register accessor: The Exception Logging Control Register controls the exception logging.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_control`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_control")]
pub type GlbRegsExceptionLoggingControl =
    crate::Reg<glb_regs_exception_logging_control::GlbRegsExceptionLoggingControlSpec>;
#[doc = "The Exception Logging Control Register controls the exception logging."]
pub mod glb_regs_exception_logging_control;
#[doc = "GLB_REGS_exception_logging_header0 (rw) register accessor: The Exception Logging Header 0 Register contains the first word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_header0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_header0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_header0`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_header0")]
pub type GlbRegsExceptionLoggingHeader0 =
    crate::Reg<glb_regs_exception_logging_header0::GlbRegsExceptionLoggingHeader0Spec>;
#[doc = "The Exception Logging Header 0 Register contains the first word of the header."]
pub mod glb_regs_exception_logging_header0;
#[doc = "GLB_REGS_exception_logging_header1 (rw) register accessor: The Exception Logging Header 1 Register contains the second word of the header.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_header1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_header1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_header1`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_header1")]
pub type GlbRegsExceptionLoggingHeader1 =
    crate::Reg<glb_regs_exception_logging_header1::GlbRegsExceptionLoggingHeader1Spec>;
#[doc = "The Exception Logging Header 1 Register contains the second word of the header."]
pub mod glb_regs_exception_logging_header1;
#[doc = "GLB_REGS_exception_logging_data0 (rw) register accessor: The Exception Logging Data 0 Register contains the first word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_data0`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_data0")]
pub type GlbRegsExceptionLoggingData0 =
    crate::Reg<glb_regs_exception_logging_data0::GlbRegsExceptionLoggingData0Spec>;
#[doc = "The Exception Logging Data 0 Register contains the first word of the data."]
pub mod glb_regs_exception_logging_data0;
#[doc = "GLB_REGS_exception_logging_data1 (rw) register accessor: The Exception Logging Data 1 Register contains the second word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_data1`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_data1")]
pub type GlbRegsExceptionLoggingData1 =
    crate::Reg<glb_regs_exception_logging_data1::GlbRegsExceptionLoggingData1Spec>;
#[doc = "The Exception Logging Data 1 Register contains the second word of the data."]
pub mod glb_regs_exception_logging_data1;
#[doc = "GLB_REGS_exception_logging_data2 (rw) register accessor: The Exception Logging Data 2 Register contains the third word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_data2`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_data2")]
pub type GlbRegsExceptionLoggingData2 =
    crate::Reg<glb_regs_exception_logging_data2::GlbRegsExceptionLoggingData2Spec>;
#[doc = "The Exception Logging Data 2 Register contains the third word of the data."]
pub mod glb_regs_exception_logging_data2;
#[doc = "GLB_REGS_exception_logging_data3 (rw) register accessor: The Exception Logging Data 3 Register contains the fourth word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_logging_data3`]
module"]
#[doc(alias = "GLB_REGS_exception_logging_data3")]
pub type GlbRegsExceptionLoggingData3 =
    crate::Reg<glb_regs_exception_logging_data3::GlbRegsExceptionLoggingData3Spec>;
#[doc = "The Exception Logging Data 3 Register contains the fourth word of the data."]
pub mod glb_regs_exception_logging_data3;
#[doc = "GLB_REGS_exception_pend_set (rw) register accessor: The Exception Logging Pending Set Register allows to set the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_pend_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_pend_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_pend_set`]
module"]
#[doc(alias = "GLB_REGS_exception_pend_set")]
pub type GlbRegsExceptionPendSet =
    crate::Reg<glb_regs_exception_pend_set::GlbRegsExceptionPendSetSpec>;
#[doc = "The Exception Logging Pending Set Register allows to set the pend signal."]
pub mod glb_regs_exception_pend_set;
#[doc = "GLB_REGS_exception_pend_clear (rw) register accessor: The Exception Logging Pending Clear Register allows to clear the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_pend_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_pend_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_regs_exception_pend_clear`]
module"]
#[doc(alias = "GLB_REGS_exception_pend_clear")]
pub type GlbRegsExceptionPendClear =
    crate::Reg<glb_regs_exception_pend_clear::GlbRegsExceptionPendClearSpec>;
#[doc = "The Exception Logging Pending Clear Register allows to clear the pend signal."]
pub mod glb_regs_exception_pend_clear;
