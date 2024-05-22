#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_gcntcfg_map: IntaggrGcntcfgMap,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The Global Event Mapping register controls the egress global event index for this event count. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
    #[inline(always)]
    pub const fn intaggr_gcntcfg_map(&self) -> &IntaggrGcntcfgMap {
        &self.intaggr_gcntcfg_map
    }
}
#[doc = "INTAGGR_GCNTCFG_map (rw) register accessor: The Global Event Mapping register controls the egress global event index for this event count. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_gcntcfg_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_gcntcfg_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_gcntcfg_map`]
module"]
#[doc(alias = "INTAGGR_GCNTCFG_map")]
pub type IntaggrGcntcfgMap = crate::Reg<intaggr_gcntcfg_map::IntaggrGcntcfgMapSpec>;
#[doc = "The Global Event Mapping register controls the egress global event index for this event count. This register may also be optionally used to directly set an interrupt status bit by using the irqmode flag."]
pub mod intaggr_gcntcfg_map;
