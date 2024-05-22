#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intaggr_imap_intmap: IntaggrImapIntmap,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - The Interrupt Mapping Register controls which of N virtual interrupt source outputs this channels physical interrupt sources will map onto."]
    #[inline(always)]
    pub const fn intaggr_imap_intmap(&self) -> &IntaggrImapIntmap {
        &self.intaggr_imap_intmap
    }
}
#[doc = "INTAGGR_IMAP_INTMAP (rw) register accessor: The Interrupt Mapping Register controls which of N virtual interrupt source outputs this channels physical interrupt sources will map onto.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_imap_intmap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_imap_intmap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intaggr_imap_intmap`]
module"]
#[doc(alias = "INTAGGR_IMAP_INTMAP")]
pub type IntaggrImapIntmap = crate::Reg<intaggr_imap_intmap::IntaggrImapIntmapSpec>;
#[doc = "The Interrupt Mapping Register controls which of N virtual interrupt source outputs this channels physical interrupt sources will map onto."]
pub mod intaggr_imap_intmap;
