#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl_sts_tscnt: CtlStsTscnt,
    ctl_sts_cntphs: CtlStsCntphs,
    ctl_sts_cap1: CtlStsCap1,
    ctl_sts_cap2: CtlStsCap2,
    ctl_sts_cap3: CtlStsCap3,
    ctl_sts_cap4: CtlStsCap4,
    _reserved6: [u8; 0x10],
    ctl_sts_ecctl: CtlStsEcctl,
    ctl_sts_ecint_en_flg: CtlStsEcintEnFlg,
    ctl_sts_ecint_clr_frc: CtlStsEcintClrFrc,
    _reserved9: [u8; 0x28],
    ctl_sts_pid: CtlStsPid,
}
impl RegisterBlock {
    #[doc = "0x00 - CTL_STS_TSCNT"]
    #[inline(always)]
    pub const fn ctl_sts_tscnt(&self) -> &CtlStsTscnt {
        &self.ctl_sts_tscnt
    }
    #[doc = "0x04 - CTL_STS_CNTPHS"]
    #[inline(always)]
    pub const fn ctl_sts_cntphs(&self) -> &CtlStsCntphs {
        &self.ctl_sts_cntphs
    }
    #[doc = "0x08 - CTL_STS_CAP1"]
    #[inline(always)]
    pub const fn ctl_sts_cap1(&self) -> &CtlStsCap1 {
        &self.ctl_sts_cap1
    }
    #[doc = "0x0c - CTL_STS_CAP2"]
    #[inline(always)]
    pub const fn ctl_sts_cap2(&self) -> &CtlStsCap2 {
        &self.ctl_sts_cap2
    }
    #[doc = "0x10 - CTL_STS_CAP3"]
    #[inline(always)]
    pub const fn ctl_sts_cap3(&self) -> &CtlStsCap3 {
        &self.ctl_sts_cap3
    }
    #[doc = "0x14 - CTL_STS_CAP4"]
    #[inline(always)]
    pub const fn ctl_sts_cap4(&self) -> &CtlStsCap4 {
        &self.ctl_sts_cap4
    }
    #[doc = "0x28 - CTL_STS_ECCTL"]
    #[inline(always)]
    pub const fn ctl_sts_ecctl(&self) -> &CtlStsEcctl {
        &self.ctl_sts_ecctl
    }
    #[doc = "0x2c - CTL_STS_ECINT_EN_FLG"]
    #[inline(always)]
    pub const fn ctl_sts_ecint_en_flg(&self) -> &CtlStsEcintEnFlg {
        &self.ctl_sts_ecint_en_flg
    }
    #[doc = "0x30 - CTL_STS_ECINT_CLR_FRC"]
    #[inline(always)]
    pub const fn ctl_sts_ecint_clr_frc(&self) -> &CtlStsEcintClrFrc {
        &self.ctl_sts_ecint_clr_frc
    }
    #[doc = "0x5c - CTL_STS_PID"]
    #[inline(always)]
    pub const fn ctl_sts_pid(&self) -> &CtlStsPid {
        &self.ctl_sts_pid
    }
}
#[doc = "CTL_STS_TSCNT (rw) register accessor: CTL_STS_TSCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_tscnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_tscnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_tscnt`]
module"]
#[doc(alias = "CTL_STS_TSCNT")]
pub type CtlStsTscnt = crate::Reg<ctl_sts_tscnt::CtlStsTscntSpec>;
#[doc = "CTL_STS_TSCNT"]
pub mod ctl_sts_tscnt;
#[doc = "CTL_STS_CNTPHS (rw) register accessor: CTL_STS_CNTPHS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cntphs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cntphs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_cntphs`]
module"]
#[doc(alias = "CTL_STS_CNTPHS")]
pub type CtlStsCntphs = crate::Reg<ctl_sts_cntphs::CtlStsCntphsSpec>;
#[doc = "CTL_STS_CNTPHS"]
pub mod ctl_sts_cntphs;
#[doc = "CTL_STS_CAP1 (rw) register accessor: CTL_STS_CAP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_cap1`]
module"]
#[doc(alias = "CTL_STS_CAP1")]
pub type CtlStsCap1 = crate::Reg<ctl_sts_cap1::CtlStsCap1Spec>;
#[doc = "CTL_STS_CAP1"]
pub mod ctl_sts_cap1;
#[doc = "CTL_STS_CAP2 (rw) register accessor: CTL_STS_CAP2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_cap2`]
module"]
#[doc(alias = "CTL_STS_CAP2")]
pub type CtlStsCap2 = crate::Reg<ctl_sts_cap2::CtlStsCap2Spec>;
#[doc = "CTL_STS_CAP2"]
pub mod ctl_sts_cap2;
#[doc = "CTL_STS_CAP3 (rw) register accessor: CTL_STS_CAP3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_cap3`]
module"]
#[doc(alias = "CTL_STS_CAP3")]
pub type CtlStsCap3 = crate::Reg<ctl_sts_cap3::CtlStsCap3Spec>;
#[doc = "CTL_STS_CAP3"]
pub mod ctl_sts_cap3;
#[doc = "CTL_STS_CAP4 (rw) register accessor: CTL_STS_CAP4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_cap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_cap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_cap4`]
module"]
#[doc(alias = "CTL_STS_CAP4")]
pub type CtlStsCap4 = crate::Reg<ctl_sts_cap4::CtlStsCap4Spec>;
#[doc = "CTL_STS_CAP4"]
pub mod ctl_sts_cap4;
#[doc = "CTL_STS_ECCTL (rw) register accessor: CTL_STS_ECCTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_ecctl`]
module"]
#[doc(alias = "CTL_STS_ECCTL")]
pub type CtlStsEcctl = crate::Reg<ctl_sts_ecctl::CtlStsEcctlSpec>;
#[doc = "CTL_STS_ECCTL"]
pub mod ctl_sts_ecctl;
#[doc = "CTL_STS_ECINT_EN_FLG (rw) register accessor: CTL_STS_ECINT_EN_FLG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecint_en_flg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecint_en_flg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_ecint_en_flg`]
module"]
#[doc(alias = "CTL_STS_ECINT_EN_FLG")]
pub type CtlStsEcintEnFlg = crate::Reg<ctl_sts_ecint_en_flg::CtlStsEcintEnFlgSpec>;
#[doc = "CTL_STS_ECINT_EN_FLG"]
pub mod ctl_sts_ecint_en_flg;
#[doc = "CTL_STS_ECINT_CLR_FRC (rw) register accessor: CTL_STS_ECINT_CLR_FRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecint_clr_frc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecint_clr_frc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_ecint_clr_frc`]
module"]
#[doc(alias = "CTL_STS_ECINT_CLR_FRC")]
pub type CtlStsEcintClrFrc = crate::Reg<ctl_sts_ecint_clr_frc::CtlStsEcintClrFrcSpec>;
#[doc = "CTL_STS_ECINT_CLR_FRC"]
pub mod ctl_sts_ecint_clr_frc;
#[doc = "CTL_STS_PID (rw) register accessor: CTL_STS_PID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl_sts_pid`]
module"]
#[doc(alias = "CTL_STS_PID")]
pub type CtlStsPid = crate::Reg<ctl_sts_pid::CtlStsPidSpec>;
#[doc = "CTL_STS_PID"]
pub mod ctl_sts_pid;
