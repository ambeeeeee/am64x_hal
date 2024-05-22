#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_l2g_map: IntaggrL2gMap,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - This register determines how the ordinal local event is translated to a global event on the outgoing event transport lane. Both pulse and rising edge local event types are supported. With pulsed events, the event count is determined by the number of cycles for which the event signal remains high. For rising edge events, the count represents the total number of rising edge transitions. The index field of the register determines the outgoing global event index, and the mode bit specifies either pulsed or rising edge local event detection."]
    #[inline(always)]
    pub const fn intaggr_l2g_map(&self) -> &IntaggrL2gMap {
        &self.intaggr_l2g_map
    }
}
#[doc = "INTAGGR_L2G_map (rw) register accessor: This register determines how the ordinal local event is translated to a global event on the outgoing event transport lane. Both pulse and rising edge local event types are supported. With pulsed events, the event count is determined by the number of cycles for which the event signal remains high. For rising edge events, the count represents the total number of rising edge transitions. The index field of the register determines the outgoing global event index, and the mode bit specifies either pulsed or rising edge local event detection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_l2g_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_l2g_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_l2g_map`]
module"]
#[doc(alias = "INTAGGR_L2G_map")]
pub type IntaggrL2gMap = crate::Reg<intaggr_l2g_map::IntaggrL2gMapSpec>;
#[doc = "This register determines how the ordinal local event is translated to a global event on the outgoing event transport lane. Both pulse and rising edge local event types are supported. With pulsed events, the event count is determined by the number of cycles for which the event signal remains high. For rising edge events, the count represents the total number of rising edge transitions. The index field of the register determines the outgoing global event index, and the mode bit specifies either pulsed or rising edge local event detection."]
pub mod intaggr_l2g_map;
