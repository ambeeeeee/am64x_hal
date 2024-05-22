#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_mcast_mcmap: IntaggrMcastMcmap,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - This register determines how ingress global events from the ingress global event ETL are written out to the two egress global event ETL intefaces. The index of each of the two egress events is stored in this register, which is selected based in the ingress event index value."]
    #[inline(always)]
    pub const fn intaggr_mcast_mcmap(&self) -> &IntaggrMcastMcmap {
        &self.intaggr_mcast_mcmap
    }
}
#[doc = "INTAGGR_MCAST_mcmap (rw) register accessor: This register determines how ingress global events from the ingress global event ETL are written out to the two egress global event ETL intefaces. The index of each of the two egress events is stored in this register, which is selected based in the ingress event index value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_mcast_mcmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_mcast_mcmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_mcast_mcmap`]
module"]
#[doc(alias = "INTAGGR_MCAST_mcmap")]
pub type IntaggrMcastMcmap = crate::Reg<intaggr_mcast_mcmap::IntaggrMcastMcmapSpec>;
#[doc = "This register determines how ingress global events from the ingress global event ETL are written out to the two egress global event ETL intefaces. The index of each of the two egress events is stored in this register, which is selected based in the ingress event index value."]
pub mod intaggr_mcast_mcmap;
