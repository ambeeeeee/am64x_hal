#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtc_cfg1_cntcr: GtcCfg1Cntcr,
    gtc_cfg1_cntsr: GtcCfg1Cntsr,
    gtc_cfg1_cntcv_lo: GtcCfg1CntcvLo,
    gtc_cfg1_cntcv_hi: GtcCfg1CntcvHi,
    _reserved4: [u8; 0x10],
    gtc_cfg1_cntfid0: GtcCfg1Cntfid0,
    gtc_cfg1_cntfid1: GtcCfg1Cntfid1,
}
impl RegisterBlock {
    #[doc = "0x00 - GTC_CFG1_CNTCR"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntcr(&self) -> &GtcCfg1Cntcr {
        &self.gtc_cfg1_cntcr
    }
    #[doc = "0x04 - GTC_CFG1_CNTSR"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntsr(&self) -> &GtcCfg1Cntsr {
        &self.gtc_cfg1_cntsr
    }
    #[doc = "0x08 - GTC_CFG1_CNTCV_LO"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntcv_lo(&self) -> &GtcCfg1CntcvLo {
        &self.gtc_cfg1_cntcv_lo
    }
    #[doc = "0x0c - GTC_CFG1_CNTCV_HI"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntcv_hi(&self) -> &GtcCfg1CntcvHi {
        &self.gtc_cfg1_cntcv_hi
    }
    #[doc = "0x20 - GTC_CFG1_CNTFID0"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntfid0(&self) -> &GtcCfg1Cntfid0 {
        &self.gtc_cfg1_cntfid0
    }
    #[doc = "0x24 - GTC_CFG1_CNTFID1"]
    #[inline(always)]
    pub const fn gtc_cfg1_cntfid1(&self) -> &GtcCfg1Cntfid1 {
        &self.gtc_cfg1_cntfid1
    }
}
#[doc = "GTC_CFG1_CNTCR (rw) register accessor: GTC_CFG1_CNTCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntcr`]
module"]
#[doc(alias = "GTC_CFG1_CNTCR")]
pub type GtcCfg1Cntcr = crate::Reg<gtc_cfg1_cntcr::GtcCfg1CntcrSpec>;
#[doc = "GTC_CFG1_CNTCR"]
pub mod gtc_cfg1_cntcr;
#[doc = "GTC_CFG1_CNTSR (rw) register accessor: GTC_CFG1_CNTSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntsr`]
module"]
#[doc(alias = "GTC_CFG1_CNTSR")]
pub type GtcCfg1Cntsr = crate::Reg<gtc_cfg1_cntsr::GtcCfg1CntsrSpec>;
#[doc = "GTC_CFG1_CNTSR"]
pub mod gtc_cfg1_cntsr;
#[doc = "GTC_CFG1_CNTCV_LO (rw) register accessor: GTC_CFG1_CNTCV_LO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcv_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcv_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntcv_lo`]
module"]
#[doc(alias = "GTC_CFG1_CNTCV_LO")]
pub type GtcCfg1CntcvLo = crate::Reg<gtc_cfg1_cntcv_lo::GtcCfg1CntcvLoSpec>;
#[doc = "GTC_CFG1_CNTCV_LO"]
pub mod gtc_cfg1_cntcv_lo;
#[doc = "GTC_CFG1_CNTCV_HI (rw) register accessor: GTC_CFG1_CNTCV_HI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntcv_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntcv_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntcv_hi`]
module"]
#[doc(alias = "GTC_CFG1_CNTCV_HI")]
pub type GtcCfg1CntcvHi = crate::Reg<gtc_cfg1_cntcv_hi::GtcCfg1CntcvHiSpec>;
#[doc = "GTC_CFG1_CNTCV_HI"]
pub mod gtc_cfg1_cntcv_hi;
#[doc = "GTC_CFG1_CNTFID0 (rw) register accessor: GTC_CFG1_CNTFID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntfid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntfid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntfid0`]
module"]
#[doc(alias = "GTC_CFG1_CNTFID0")]
pub type GtcCfg1Cntfid0 = crate::Reg<gtc_cfg1_cntfid0::GtcCfg1Cntfid0Spec>;
#[doc = "GTC_CFG1_CNTFID0"]
pub mod gtc_cfg1_cntfid0;
#[doc = "GTC_CFG1_CNTFID1 (rw) register accessor: GTC_CFG1_CNTFID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg1_cntfid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg1_cntfid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg1_cntfid1`]
module"]
#[doc(alias = "GTC_CFG1_CNTFID1")]
pub type GtcCfg1Cntfid1 = crate::Reg<gtc_cfg1_cntfid1::GtcCfg1Cntfid1Spec>;
#[doc = "GTC_CFG1_CNTFID1"]
pub mod gtc_cfg1_cntfid1;
