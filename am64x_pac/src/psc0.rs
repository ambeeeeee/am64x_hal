#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    vbus_pid: VbusPid,
    _reserved1: [u8; 0x0c],
    vbus_gblctl: VbusGblctl,
    vbus_gblstat: VbusGblstat,
    vbus_inteval: VbusInteval,
    _reserved4: [u8; 0x24],
    vbus_merrpr: VbusMerrpr,
    _reserved5: [u8; 0x0c],
    vbus_merrcr: VbusMerrcr,
    _reserved6: [u8; 0x0c],
    vbus_perrpr: VbusPerrpr,
    _reserved7: [u8; 0x04],
    vbus_perrcr: VbusPerrcr,
    _reserved8: [u8; 0x04],
    vbus_epcpr: VbusEpcpr,
    _reserved9: [u8; 0x04],
    vbus_epccr: VbusEpccr,
    _reserved10: [u8; 0x84],
    vbus_railstat: VbusRailstat,
    vbus_railctl: VbusRailctl,
    vbus_railsel: VbusRailsel,
    _reserved13: [u8; 0x14],
    vbus_ptcmd: VbusPtcmd,
    _reserved14: [u8; 0x04],
    vbus_ptstat: VbusPtstat,
    _reserved15: [u8; 0xd4],
    vbus_pdstat: VbusPdstat,
    _reserved16: [u8; 0xfc],
    vbus_pdctl: VbusPdctl,
    _reserved17: [u8; 0xfc],
    vbus_pdcfg: VbusPdcfg,
    _reserved18: [u8; 0x01fc],
    vbus_mdcfg: VbusMdcfg,
    _reserved19: [u8; 0x01fc],
    vbus_mdstat: VbusMdstat,
    _reserved20: [u8; 0x01fc],
    vbus_mdctl: VbusMdctl,
}
impl RegisterBlock {
    #[doc = "0x00 - The peripheral identification register is a constant register that contains the ID and ID revision number for that module. The PID stores version information used to identify the module. All bits within this register are read-only (writes have no effect) meaning that the values within this register should be hard-coded with the appropriate values and must not change from their hard-coded values."]
    #[inline(always)]
    pub const fn vbus_pid(&self) -> &VbusPid {
        &self.vbus_pid
    }
    #[doc = "0x10 - This register contains global control to PSC."]
    #[inline(always)]
    pub const fn vbus_gblctl(&self) -> &VbusGblctl {
        &self.vbus_gblctl
    }
    #[doc = "0x14 - This register shows the PSC global status."]
    #[inline(always)]
    pub const fn vbus_gblstat(&self) -> &VbusGblstat {
        &self.vbus_gblstat
    }
    #[doc = "0x18 - This register has no storage. Read from this register returns 0."]
    #[inline(always)]
    pub const fn vbus_inteval(&self) -> &VbusInteval {
        &self.vbus_inteval
    }
    #[doc = "0x40 - This register records pending error conditions for all modules. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_merrpr(&self) -> &VbusMerrpr {
        &self.vbus_merrpr
    }
    #[doc = "0x50 - This register has no storage. Read from this register returns 0. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_merrcr(&self) -> &VbusMerrcr {
        &self.vbus_merrcr
    }
    #[doc = "0x60 - This register records pending error conditions for each power domain. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_perrpr(&self) -> &VbusPerrpr {
        &self.vbus_perrpr
    }
    #[doc = "0x68 - This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_perrcr(&self) -> &VbusPerrcr {
        &self.vbus_perrcr
    }
    #[doc = "0x70 - This register records pending external power control conditions. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_epcpr(&self) -> &VbusEpcpr {
        &self.vbus_epcpr
    }
    #[doc = "0x78 - This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_epccr(&self) -> &VbusEpccr {
        &self.vbus_epccr
    }
    #[doc = "0x100 - This register is a read-only and shows the current rail requestor whose request is being granted and the current value of the counter associated with this requestor."]
    #[inline(always)]
    pub const fn vbus_railstat(&self) -> &VbusRailstat {
        &self.vbus_railstat
    }
    #[doc = "0x104 - This register is user programmable. It holds the counter values for rail counter. User can select one of the two counter values to be used for each power domain (see RAILSEL register)."]
    #[inline(always)]
    pub const fn vbus_railctl(&self) -> &VbusRailctl {
        &self.vbus_railctl
    }
    #[doc = "0x108 - User can use this register to select the counter value (RAILCTL) for each power domain."]
    #[inline(always)]
    pub const fn vbus_railsel(&self) -> &VbusRailsel {
        &self.vbus_railsel
    }
    #[doc = "0x120 - This is a pseudo-command register with no actual storage. Reads return 0. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_ptcmd(&self) -> &VbusPtcmd {
        &self.vbus_ptcmd
    }
    #[doc = "0x128 - This is a status register. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
    #[inline(always)]
    pub const fn vbus_ptstat(&self) -> &VbusPtstat {
        &self.vbus_ptstat
    }
    #[doc = "0x200 - This is a status register. One register per power domain. Each register contains the status for the given power domain."]
    #[inline(always)]
    pub const fn vbus_pdstat(&self) -> &VbusPdstat {
        &self.vbus_pdstat
    }
    #[doc = "0x300 - This is a control register. One register per power domain."]
    #[inline(always)]
    pub const fn vbus_pdctl(&self) -> &VbusPdctl {
        &self.vbus_pdctl
    }
    #[doc = "0x400 - This is a status register. It shows PSC settings for easy debug."]
    #[inline(always)]
    pub const fn vbus_pdcfg(&self) -> &VbusPdcfg {
        &self.vbus_pdcfg
    }
    #[doc = "0x600 - This is a constant register showing some PSC settings for easy debug. This register is read only."]
    #[inline(always)]
    pub const fn vbus_mdcfg(&self) -> &VbusMdcfg {
        &self.vbus_mdcfg
    }
    #[doc = "0x800 - This register shows the status of each module. Requires one register per module on the device."]
    #[inline(always)]
    pub const fn vbus_mdstat(&self) -> &VbusMdstat {
        &self.vbus_mdstat
    }
    #[doc = "0xa00 - This register provides specific control for the individual module. One register per module on the device."]
    #[inline(always)]
    pub const fn vbus_mdctl(&self) -> &VbusMdctl {
        &self.vbus_mdctl
    }
}
#[doc = "VBUS_PID (rw) register accessor: The peripheral identification register is a constant register that contains the ID and ID revision number for that module. The PID stores version information used to identify the module. All bits within this register are read-only (writes have no effect) meaning that the values within this register should be hard-coded with the appropriate values and must not change from their hard-coded values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_pid`]
module"]
#[doc(alias = "VBUS_PID")]
pub type VbusPid = crate::Reg<vbus_pid::VbusPidSpec>;
#[doc = "The peripheral identification register is a constant register that contains the ID and ID revision number for that module. The PID stores version information used to identify the module. All bits within this register are read-only (writes have no effect) meaning that the values within this register should be hard-coded with the appropriate values and must not change from their hard-coded values."]
pub mod vbus_pid;
#[doc = "VBUS_GBLCTL (rw) register accessor: This register contains global control to PSC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_gblctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_gblctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_gblctl`]
module"]
#[doc(alias = "VBUS_GBLCTL")]
pub type VbusGblctl = crate::Reg<vbus_gblctl::VbusGblctlSpec>;
#[doc = "This register contains global control to PSC."]
pub mod vbus_gblctl;
#[doc = "VBUS_GBLSTAT (rw) register accessor: This register shows the PSC global status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_gblstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_gblstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_gblstat`]
module"]
#[doc(alias = "VBUS_GBLSTAT")]
pub type VbusGblstat = crate::Reg<vbus_gblstat::VbusGblstatSpec>;
#[doc = "This register shows the PSC global status."]
pub mod vbus_gblstat;
#[doc = "VBUS_INTEVAL (rw) register accessor: This register has no storage. Read from this register returns 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_inteval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_inteval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_inteval`]
module"]
#[doc(alias = "VBUS_INTEVAL")]
pub type VbusInteval = crate::Reg<vbus_inteval::VbusIntevalSpec>;
#[doc = "This register has no storage. Read from this register returns 0."]
pub mod vbus_inteval;
#[doc = "VBUS_MERRPR (rw) register accessor: This register records pending error conditions for all modules. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_merrpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_merrpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_merrpr`]
module"]
#[doc(alias = "VBUS_MERRPR")]
pub type VbusMerrpr = crate::Reg<vbus_merrpr::VbusMerrprSpec>;
#[doc = "This register records pending error conditions for all modules. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.)."]
pub mod vbus_merrpr;
#[doc = "VBUS_MERRCR (rw) register accessor: This register has no storage. Read from this register returns 0. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_merrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_merrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_merrcr`]
module"]
#[doc(alias = "VBUS_MERRCR")]
pub type VbusMerrcr = crate::Reg<vbus_merrcr::VbusMerrcrSpec>;
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one module (index 0 for modules 0-31, index 1 for modules 32-63, etc.)."]
pub mod vbus_merrcr;
#[doc = "VBUS_PERRPR (rw) register accessor: This register records pending error conditions for each power domain. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_perrpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_perrpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_perrpr`]
module"]
#[doc(alias = "VBUS_PERRPR")]
pub type VbusPerrpr = crate::Reg<vbus_perrpr::VbusPerrprSpec>;
#[doc = "This register records pending error conditions for each power domain. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_perrpr;
#[doc = "VBUS_PERRCR (rw) register accessor: This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_perrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_perrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_perrcr`]
module"]
#[doc(alias = "VBUS_PERRCR")]
pub type VbusPerrcr = crate::Reg<vbus_perrcr::VbusPerrcrSpec>;
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_perrcr;
#[doc = "VBUS_EPCPR (rw) register accessor: This register records pending external power control conditions. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_epcpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_epcpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_epcpr`]
module"]
#[doc(alias = "VBUS_EPCPR")]
pub type VbusEpcpr = crate::Reg<vbus_epcpr::VbusEpcprSpec>;
#[doc = "This register records pending external power control conditions. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_epcpr;
#[doc = "VBUS_EPCCR (rw) register accessor: This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_epccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_epccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_epccr`]
module"]
#[doc(alias = "VBUS_EPCCR")]
pub type VbusEpccr = crate::Reg<vbus_epccr::VbusEpccrSpec>;
#[doc = "This register has no storage. Read from this register returns 0. Each bit represents one domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_epccr;
#[doc = "VBUS_RAILSTAT (rw) register accessor: This register is a read-only and shows the current rail requestor whose request is being granted and the current value of the counter associated with this requestor.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_railstat`]
module"]
#[doc(alias = "VBUS_RAILSTAT")]
pub type VbusRailstat = crate::Reg<vbus_railstat::VbusRailstatSpec>;
#[doc = "This register is a read-only and shows the current rail requestor whose request is being granted and the current value of the counter associated with this requestor."]
pub mod vbus_railstat;
#[doc = "VBUS_RAILCTL (rw) register accessor: This register is user programmable. It holds the counter values for rail counter. User can select one of the two counter values to be used for each power domain (see RAILSEL register).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_railctl`]
module"]
#[doc(alias = "VBUS_RAILCTL")]
pub type VbusRailctl = crate::Reg<vbus_railctl::VbusRailctlSpec>;
#[doc = "This register is user programmable. It holds the counter values for rail counter. User can select one of the two counter values to be used for each power domain (see RAILSEL register)."]
pub mod vbus_railctl;
#[doc = "VBUS_RAILSEL (rw) register accessor: User can use this register to select the counter value (RAILCTL) for each power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_railsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_railsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_railsel`]
module"]
#[doc(alias = "VBUS_RAILSEL")]
pub type VbusRailsel = crate::Reg<vbus_railsel::VbusRailselSpec>;
#[doc = "User can use this register to select the counter value (RAILCTL) for each power domain."]
pub mod vbus_railsel;
#[doc = "VBUS_PTCMD (rw) register accessor: This is a pseudo-command register with no actual storage. Reads return 0. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_ptcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_ptcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_ptcmd`]
module"]
#[doc(alias = "VBUS_PTCMD")]
pub type VbusPtcmd = crate::Reg<vbus_ptcmd::VbusPtcmdSpec>;
#[doc = "This is a pseudo-command register with no actual storage. Reads return 0. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_ptcmd;
#[doc = "VBUS_PTSTAT (rw) register accessor: This is a status register. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_ptstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_ptstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_ptstat`]
module"]
#[doc(alias = "VBUS_PTSTAT")]
pub type VbusPtstat = crate::Reg<vbus_ptstat::VbusPtstatSpec>;
#[doc = "This is a status register. One bit for each power domain (index 0 for domains 0-31, index 1 for domains 32-63, etc.)."]
pub mod vbus_ptstat;
#[doc = "VBUS_PDSTAT (rw) register accessor: This is a status register. One register per power domain. Each register contains the status for the given power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_pdstat`]
module"]
#[doc(alias = "VBUS_PDSTAT")]
pub type VbusPdstat = crate::Reg<vbus_pdstat::VbusPdstatSpec>;
#[doc = "This is a status register. One register per power domain. Each register contains the status for the given power domain."]
pub mod vbus_pdstat;
#[doc = "VBUS_PDCTL (rw) register accessor: This is a control register. One register per power domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_pdctl`]
module"]
#[doc(alias = "VBUS_PDCTL")]
pub type VbusPdctl = crate::Reg<vbus_pdctl::VbusPdctlSpec>;
#[doc = "This is a control register. One register per power domain."]
pub mod vbus_pdctl;
#[doc = "VBUS_PDCFG (rw) register accessor: This is a status register. It shows PSC settings for easy debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_pdcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_pdcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_pdcfg`]
module"]
#[doc(alias = "VBUS_PDCFG")]
pub type VbusPdcfg = crate::Reg<vbus_pdcfg::VbusPdcfgSpec>;
#[doc = "This is a status register. It shows PSC settings for easy debug."]
pub mod vbus_pdcfg;
#[doc = "VBUS_MDCFG (rw) register accessor: This is a constant register showing some PSC settings for easy debug. This register is read only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_mdcfg`]
module"]
#[doc(alias = "VBUS_MDCFG")]
pub type VbusMdcfg = crate::Reg<vbus_mdcfg::VbusMdcfgSpec>;
#[doc = "This is a constant register showing some PSC settings for easy debug. This register is read only."]
pub mod vbus_mdcfg;
#[doc = "VBUS_MDSTAT (rw) register accessor: This register shows the status of each module. Requires one register per module on the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_mdstat`]
module"]
#[doc(alias = "VBUS_MDSTAT")]
pub type VbusMdstat = crate::Reg<vbus_mdstat::VbusMdstatSpec>;
#[doc = "This register shows the status of each module. Requires one register per module on the device."]
pub mod vbus_mdstat;
#[doc = "VBUS_MDCTL (rw) register accessor: This register provides specific control for the individual module. One register per module on the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_mdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_mdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbus_mdctl`]
module"]
#[doc(alias = "VBUS_MDCTL")]
pub type VbusMdctl = crate::Reg<vbus_mdctl::VbusMdctlSpec>;
#[doc = "This register provides specific control for the individual module. One register per module on the device."]
pub mod vbus_mdctl;
