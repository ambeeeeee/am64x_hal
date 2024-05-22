#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ringacc_isc_isc_control: RingaccIscIscControl,
    ringacc_isc_isc_control2: RingaccIscIscControl2,
}
impl RegisterBlock {
    #[doc = "0x00 - The ISC a Region b Control Register defines the control fields for the ISC."]
    #[inline(always)]
    pub const fn ringacc_isc_isc_control(&self) -> &RingaccIscIscControl {
        &self.ringacc_isc_isc_control
    }
    #[doc = "0x04 - The ISC a Region b Control Register 2 defines the control fields for the ISC."]
    #[inline(always)]
    pub const fn ringacc_isc_isc_control2(&self) -> &RingaccIscIscControl2 {
        &self.ringacc_isc_isc_control2
    }
}
#[doc = "RINGACC_ISC_ISC_control (rw) register accessor: The ISC a Region b Control Register defines the control fields for the ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_isc_isc_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_isc_isc_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_isc_isc_control`]
module"]
#[doc(alias = "RINGACC_ISC_ISC_control")]
pub type RingaccIscIscControl = crate::Reg<ringacc_isc_isc_control::RingaccIscIscControlSpec>;
#[doc = "The ISC a Region b Control Register defines the control fields for the ISC."]
pub mod ringacc_isc_isc_control;
#[doc = "RINGACC_ISC_ISC_control2 (rw) register accessor: The ISC a Region b Control Register 2 defines the control fields for the ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_isc_isc_control2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_isc_isc_control2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ringacc_isc_isc_control2`]
module"]
#[doc(alias = "RINGACC_ISC_ISC_control2")]
pub type RingaccIscIscControl2 = crate::Reg<ringacc_isc_isc_control2::RingaccIscIscControl2Spec>;
#[doc = "The ISC a Region b Control Register 2 defines the control fields for the ISC."]
pub mod ringacc_isc_isc_control2;
