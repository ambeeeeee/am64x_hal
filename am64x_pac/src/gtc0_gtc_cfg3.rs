#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    gtc_cfg3_cnttidr: GtcCfg3Cnttidr,
}
impl RegisterBlock {
    #[doc = "0x08 - GTC_CFG3_CNTTIDR"]
    #[inline(always)]
    pub const fn gtc_cfg3_cnttidr(&self) -> &GtcCfg3Cnttidr {
        &self.gtc_cfg3_cnttidr
    }
}
#[doc = "GTC_CFG3_CNTTIDR (rw) register accessor: GTC_CFG3_CNTTIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gtc_cfg3_cnttidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gtc_cfg3_cnttidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gtc_cfg3_cnttidr`]
module"]
#[doc(alias = "GTC_CFG3_CNTTIDR")]
pub type GtcCfg3Cnttidr = crate::Reg<gtc_cfg3_cnttidr::GtcCfg3CnttidrSpec>;
#[doc = "GTC_CFG3_CNTTIDR"]
pub mod gtc_cfg3_cnttidr;
