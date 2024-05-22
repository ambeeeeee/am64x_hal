#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_gcntrti_count: IntaggrGcntrtiCount,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The ETL Count register is read by software to determine how many times the event message has been received. This register can be written to decrement the count by a specified amount to acknowledge that a count has been processed by the host."]
    #[inline(always)]
    pub const fn intaggr_gcntrti_count(&self) -> &IntaggrGcntrtiCount {
        &self.intaggr_gcntrti_count
    }
}
#[doc = "INTAGGR_GCNTRTI_count (rw) register accessor: The ETL Count register is read by software to determine how many times the event message has been received. This register can be written to decrement the count by a specified amount to acknowledge that a count has been processed by the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_gcntrti_count::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_gcntrti_count::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_gcntrti_count`]
module"]
#[doc(alias = "INTAGGR_GCNTRTI_count")]
pub type IntaggrGcntrtiCount = crate::Reg<intaggr_gcntrti_count::IntaggrGcntrtiCountSpec>;
#[doc = "The ETL Count register is read by software to determine how many times the event message has been received. This register can be written to decrement the count by a specified amount to acknowledge that a count has been processed by the host."]
pub mod intaggr_gcntrti_count;
