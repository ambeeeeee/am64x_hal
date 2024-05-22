#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    etlsw_mmrs_pid: EtlswMmrsPid,
    etlsw_mmrs_config: EtlswMmrsConfig,
    _reserved2: [u8; 0x08],
    etlsw_mmrs_event: EtlswMmrsEvent,
    _reserved3: [u8; 0x0c],
    etlsw_mmrs_link: EtlswMmrsLink,
    _reserved4: [u8; 0x1c],
    etlsw_mmrs_down: EtlswMmrsDown,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn etlsw_mmrs_pid(&self) -> &EtlswMmrsPid {
        &self.etlsw_mmrs_pid
    }
    #[doc = "0x04 - The Config Register shows configured params."]
    #[inline(always)]
    pub const fn etlsw_mmrs_config(&self) -> &EtlswMmrsConfig {
        &self.etlsw_mmrs_config
    }
    #[doc = "0x10 - The Event Register defines the event to produce for a link down event."]
    #[inline(always)]
    pub const fn etlsw_mmrs_event(&self) -> &EtlswMmrsEvent {
        &self.etlsw_mmrs_event
    }
    #[doc = "0x20 - The Link Register shows the current status of the endpoint links."]
    #[inline(always)]
    pub const fn etlsw_mmrs_link(&self) -> &EtlswMmrsLink {
        &self.etlsw_mmrs_link
    }
    #[doc = "0x40 - The Link Down Register shows which links are down for the endpoints."]
    #[inline(always)]
    pub const fn etlsw_mmrs_down(&self) -> &EtlswMmrsDown {
        &self.etlsw_mmrs_down
    }
}
#[doc = "ETLSW_MMRS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etlsw_mmrs_pid`]
module"]
#[doc(alias = "ETLSW_MMRS_pid")]
pub type EtlswMmrsPid = crate::Reg<etlsw_mmrs_pid::EtlswMmrsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod etlsw_mmrs_pid;
#[doc = "ETLSW_MMRS_config (rw) register accessor: The Config Register shows configured params.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etlsw_mmrs_config`]
module"]
#[doc(alias = "ETLSW_MMRS_config")]
pub type EtlswMmrsConfig = crate::Reg<etlsw_mmrs_config::EtlswMmrsConfigSpec>;
#[doc = "The Config Register shows configured params."]
pub mod etlsw_mmrs_config;
#[doc = "ETLSW_MMRS_event (rw) register accessor: The Event Register defines the event to produce for a link down event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etlsw_mmrs_event`]
module"]
#[doc(alias = "ETLSW_MMRS_event")]
pub type EtlswMmrsEvent = crate::Reg<etlsw_mmrs_event::EtlswMmrsEventSpec>;
#[doc = "The Event Register defines the event to produce for a link down event."]
pub mod etlsw_mmrs_event;
#[doc = "ETLSW_MMRS_link (rw) register accessor: The Link Register shows the current status of the endpoint links.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etlsw_mmrs_link`]
module"]
#[doc(alias = "ETLSW_MMRS_link")]
pub type EtlswMmrsLink = crate::Reg<etlsw_mmrs_link::EtlswMmrsLinkSpec>;
#[doc = "The Link Register shows the current status of the endpoint links."]
pub mod etlsw_mmrs_link;
#[doc = "ETLSW_MMRS_down (rw) register accessor: The Link Down Register shows which links are down for the endpoints.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etlsw_mmrs_down::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etlsw_mmrs_down::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etlsw_mmrs_down`]
module"]
#[doc(alias = "ETLSW_MMRS_down")]
pub type EtlswMmrsDown = crate::Reg<etlsw_mmrs_down::EtlswMmrsDownSpec>;
#[doc = "The Link Down Register shows which links are down for the endpoints."]
pub mod etlsw_mmrs_down;
