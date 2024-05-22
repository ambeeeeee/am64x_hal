#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gtc_cfg2_cntcvs_lo: GtcCfg2CntcvsLo,
    gtc_cfg2_cntcvs_hi: GtcCfg2CntcvsHi,
}
impl RegisterBlock {
    #[doc = "0x00 - GTC_CFG2_CNTCVS_LO"]
    #[inline(always)]
    pub const fn gtc_cfg2_cntcvs_lo(&self) -> &GtcCfg2CntcvsLo {
        &self.gtc_cfg2_cntcvs_lo
    }
    #[doc = "0x04 - GTC_CFG2_CNTCVS_HI"]
    #[inline(always)]
    pub const fn gtc_cfg2_cntcvs_hi(&self) -> &GtcCfg2CntcvsHi {
        &self.gtc_cfg2_cntcvs_hi
    }
}
#[doc = "GTC_CFG2_CNTCVS_LO (rw) register accessor: GTC_CFG2_CNTCVS_LO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg2_cntcvs_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg2_cntcvs_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg2_cntcvs_lo`]
module"]
#[doc(alias = "GTC_CFG2_CNTCVS_LO")]
pub type GtcCfg2CntcvsLo = crate::Reg<gtc_cfg2_cntcvs_lo::GtcCfg2CntcvsLoSpec>;
#[doc = "GTC_CFG2_CNTCVS_LO"]
pub mod gtc_cfg2_cntcvs_lo;
#[doc = "GTC_CFG2_CNTCVS_HI (rw) register accessor: GTC_CFG2_CNTCVS_HI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg2_cntcvs_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg2_cntcvs_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg2_cntcvs_hi`]
module"]
#[doc(alias = "GTC_CFG2_CNTCVS_HI")]
pub type GtcCfg2CntcvsHi = crate::Reg<gtc_cfg2_cntcvs_hi::GtcCfg2CntcvsHiSpec>;
#[doc = "GTC_CFG2_CNTCVS_HI"]
pub mod gtc_cfg2_cntcvs_hi;
