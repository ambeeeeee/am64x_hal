#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pktdma_rflow_rfa: PktdmaRflowRfa,
}
impl RegisterBlock {
    #[doc = "0x00 - The Rx Flow N Configuration Register A contains static configuration information for the Rx DMA flow. The fields in this register can only be changed when all of the DMA channels that use this flow have been disabled. The fields in this register are as follows:"]
    #[inline(always)]
    pub const fn pktdma_rflow_rfa(&self) -> &PktdmaRflowRfa {
        &self.pktdma_rflow_rfa
    }
}
#[doc = "PKTDMA_RFLOW_RFA (rw) register accessor: The Rx Flow N Configuration Register A contains static configuration information for the Rx DMA flow. The fields in this register can only be changed when all of the DMA channels that use this flow have been disabled. The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rflow_rfa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rflow_rfa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pktdma_rflow_rfa`]
module"]
#[doc(alias = "PKTDMA_RFLOW_RFA")]
pub type PktdmaRflowRfa = crate::Reg<pktdma_rflow_rfa::PktdmaRflowRfaSpec>;
#[doc = "The Rx Flow N Configuration Register A contains static configuration information for the Rx DMA flow. The fields in this register can only be changed when all of the DMA channels that use this flow have been disabled. The fields in this register are as follows:"]
pub mod pktdma_rflow_rfa;
