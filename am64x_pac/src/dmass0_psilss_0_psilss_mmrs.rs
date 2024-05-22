#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psilss_mmrs_pid: PsilssMmrsPid,
    psilss_mmrs_config: PsilssMmrsConfig,
    _reserved2: [u8; 0x08],
    psilss_mmrs_event: PsilssMmrsEvent,
    _reserved3: [u8; 0x0c],
    psilss_mmrs_link: PsilssMmrsLink,
    _reserved4: [u8; 0x1c],
    psilss_mmrs_down: PsilssMmrsDown,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn psilss_mmrs_pid(&self) -> &PsilssMmrsPid {
        &self.psilss_mmrs_pid
    }
    #[doc = "0x04 - The Config Register shows configured params."]
    #[inline(always)]
    pub const fn psilss_mmrs_config(&self) -> &PsilssMmrsConfig {
        &self.psilss_mmrs_config
    }
    #[doc = "0x10 - The Event Register defines the event to produce for a link down event."]
    #[inline(always)]
    pub const fn psilss_mmrs_event(&self) -> &PsilssMmrsEvent {
        &self.psilss_mmrs_event
    }
    #[doc = "0x20 - The Link Register shows the current status of the endpoint links."]
    #[inline(always)]
    pub const fn psilss_mmrs_link(&self) -> &PsilssMmrsLink {
        &self.psilss_mmrs_link
    }
    #[doc = "0x40 - The Link Down Register shows which links are down for the endpoints."]
    #[inline(always)]
    pub const fn psilss_mmrs_down(&self) -> &PsilssMmrsDown {
        &self.psilss_mmrs_down
    }
}
#[doc = "PSILSS_MMRS_pid (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilss_mmrs_pid`]
module"]
#[doc(alias = "PSILSS_MMRS_pid")]
pub type PsilssMmrsPid = crate::Reg<psilss_mmrs_pid::PsilssMmrsPidSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod psilss_mmrs_pid;
#[doc = "PSILSS_MMRS_config (rw) register accessor: The Config Register shows configured params.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilss_mmrs_config`]
module"]
#[doc(alias = "PSILSS_MMRS_config")]
pub type PsilssMmrsConfig = crate::Reg<psilss_mmrs_config::PsilssMmrsConfigSpec>;
#[doc = "The Config Register shows configured params."]
pub mod psilss_mmrs_config;
#[doc = "PSILSS_MMRS_event (rw) register accessor: The Event Register defines the event to produce for a link down event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilss_mmrs_event`]
module"]
#[doc(alias = "PSILSS_MMRS_event")]
pub type PsilssMmrsEvent = crate::Reg<psilss_mmrs_event::PsilssMmrsEventSpec>;
#[doc = "The Event Register defines the event to produce for a link down event."]
pub mod psilss_mmrs_event;
#[doc = "PSILSS_MMRS_link (rw) register accessor: The Link Register shows the current status of the endpoint links.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_link::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_link::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilss_mmrs_link`]
module"]
#[doc(alias = "PSILSS_MMRS_link")]
pub type PsilssMmrsLink = crate::Reg<psilss_mmrs_link::PsilssMmrsLinkSpec>;
#[doc = "The Link Register shows the current status of the endpoint links."]
pub mod psilss_mmrs_link;
#[doc = "PSILSS_MMRS_down (rw) register accessor: The Link Down Register shows which links are down for the endpoints.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilss_mmrs_down::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilss_mmrs_down::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilss_mmrs_down`]
module"]
#[doc(alias = "PSILSS_MMRS_down")]
pub type PsilssMmrsDown = crate::Reg<psilss_mmrs_down::PsilssMmrsDownSpec>;
#[doc = "The Link Down Register shows which links are down for the endpoints."]
pub mod psilss_mmrs_down;
