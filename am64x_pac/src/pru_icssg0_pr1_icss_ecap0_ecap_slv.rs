#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_icss_ecap0__ecap_slv__regs_tscnt: Pr1IcssEcap0_EcapSlv_RegsTscnt,
    pr1_icss_ecap0__ecap_slv__regs_cntphs: Pr1IcssEcap0_EcapSlv_RegsCntphs,
    pr1_icss_ecap0__ecap_slv__regs_cap1: Pr1IcssEcap0_EcapSlv_RegsCap1,
    pr1_icss_ecap0__ecap_slv__regs_cap2: Pr1IcssEcap0_EcapSlv_RegsCap2,
    pr1_icss_ecap0__ecap_slv__regs_cap3: Pr1IcssEcap0_EcapSlv_RegsCap3,
    pr1_icss_ecap0__ecap_slv__regs_cap4: Pr1IcssEcap0_EcapSlv_RegsCap4,
    _reserved6: [u8; 0x10],
    pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1: Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1,
    pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint: Pr1IcssEcap0_EcapSlv_RegsEcflgEceint,
    _reserved8: [u8; 0x2c],
    pr1_icss_ecap0__ecap_slv__regs_pid: Pr1IcssEcap0_EcapSlv_RegsPid,
}
impl RegisterBlock {
    #[doc = "0x00 - TIME STAMP COUNTER REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_tscnt(&self) -> &Pr1IcssEcap0_EcapSlv_RegsTscnt {
        &self.pr1_icss_ecap0__ecap_slv__regs_tscnt
    }
    #[doc = "0x04 - COUNTER PHASE CONTROL REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_cntphs(&self) -> &Pr1IcssEcap0_EcapSlv_RegsCntphs {
        &self.pr1_icss_ecap0__ecap_slv__regs_cntphs
    }
    #[doc = "0x08 - CAPTURE-1 REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_cap1(&self) -> &Pr1IcssEcap0_EcapSlv_RegsCap1 {
        &self.pr1_icss_ecap0__ecap_slv__regs_cap1
    }
    #[doc = "0x0c - CAPTURE-2 REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_cap2(&self) -> &Pr1IcssEcap0_EcapSlv_RegsCap2 {
        &self.pr1_icss_ecap0__ecap_slv__regs_cap2
    }
    #[doc = "0x10 - CAPTURE-3 REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_cap3(&self) -> &Pr1IcssEcap0_EcapSlv_RegsCap3 {
        &self.pr1_icss_ecap0__ecap_slv__regs_cap3
    }
    #[doc = "0x14 - CAPTURE-4 REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_cap4(&self) -> &Pr1IcssEcap0_EcapSlv_RegsCap4 {
        &self.pr1_icss_ecap0__ecap_slv__regs_cap4
    }
    #[doc = "0x28 - ECAP CONTROL REGISTER 1"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1(
        &self,
    ) -> &Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1 {
        &self.pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1
    }
    #[doc = "0x2c - ECAP INTERRUPT ENABLE REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint(
        &self,
    ) -> &Pr1IcssEcap0_EcapSlv_RegsEcflgEceint {
        &self.pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint
    }
    #[doc = "0x5c - ECAP PERIPHERAL ID REGISTER"]
    #[inline(always)]
    pub const fn pr1_icss_ecap0__ecap_slv__regs_pid(&self) -> &Pr1IcssEcap0_EcapSlv_RegsPid {
        &self.pr1_icss_ecap0__ecap_slv__regs_pid
    }
}
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_TSCNT (rw) register accessor: TIME STAMP COUNTER REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_tscnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_tscnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_tscnt`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_TSCNT")]
pub type Pr1IcssEcap0_EcapSlv_RegsTscnt =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_tscnt::Pr1IcssEcap0_EcapSlv_RegsTscntSpec>;
#[doc = "TIME STAMP COUNTER REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_tscnt;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CNTPHS (rw) register accessor: COUNTER PHASE CONTROL REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cntphs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cntphs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_cntphs`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CNTPHS")]
pub type Pr1IcssEcap0_EcapSlv_RegsCntphs =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_cntphs::Pr1IcssEcap0_EcapSlv_RegsCntphsSpec>;
#[doc = "COUNTER PHASE CONTROL REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_cntphs;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP1 (rw) register accessor: CAPTURE-1 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_cap1`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP1")]
pub type Pr1IcssEcap0_EcapSlv_RegsCap1 =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_cap1::Pr1IcssEcap0_EcapSlv_RegsCap1Spec>;
#[doc = "CAPTURE-1 REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_cap1;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP2 (rw) register accessor: CAPTURE-2 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_cap2`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP2")]
pub type Pr1IcssEcap0_EcapSlv_RegsCap2 =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_cap2::Pr1IcssEcap0_EcapSlv_RegsCap2Spec>;
#[doc = "CAPTURE-2 REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_cap2;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP3 (rw) register accessor: CAPTURE-3 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_cap3`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP3")]
pub type Pr1IcssEcap0_EcapSlv_RegsCap3 =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_cap3::Pr1IcssEcap0_EcapSlv_RegsCap3Spec>;
#[doc = "CAPTURE-3 REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_cap3;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP4 (rw) register accessor: CAPTURE-4 REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_cap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_cap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_cap4`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_CAP4")]
pub type Pr1IcssEcap0_EcapSlv_RegsCap4 =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_cap4::Pr1IcssEcap0_EcapSlv_RegsCap4Spec>;
#[doc = "CAPTURE-4 REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_cap4;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECCTL2_ECCTL1 (rw) register accessor: ECAP CONTROL REGISTER 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECCTL2_ECCTL1")]
pub type Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1 = crate::Reg<
    pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1::Pr1IcssEcap0_EcapSlv_RegsEcctl2Ecctl1Spec,
>;
#[doc = "ECAP CONTROL REGISTER 1"]
pub mod pr1_icss_ecap0__ecap_slv__regs_ecctl2_ecctl1;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECFLG_ECEINT (rw) register accessor: ECAP INTERRUPT ENABLE REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECFLG_ECEINT")]
pub type Pr1IcssEcap0_EcapSlv_RegsEcflgEceint = crate::Reg<
    pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec,
>;
#[doc = "ECAP INTERRUPT ENABLE REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint;
#[doc = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_PID (rw) register accessor: ECAP PERIPHERAL ID REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_icss_ecap0__ecap_slv__regs_pid`]
module"]
#[doc(alias = "PR1_ICSS_ECAP0__ECAP_SLV__REGS_PID")]
pub type Pr1IcssEcap0_EcapSlv_RegsPid =
    crate::Reg<pr1_icss_ecap0__ecap_slv__regs_pid::Pr1IcssEcap0_EcapSlv_RegsPidSpec>;
#[doc = "ECAP PERIPHERAL ID REGISTER"]
pub mod pr1_icss_ecap0__ecap_slv__regs_pid;
