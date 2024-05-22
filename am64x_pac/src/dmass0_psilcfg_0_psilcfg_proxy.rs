#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psilcfg_proxy_revision: PsilcfgProxyRevision,
    _reserved1: [u8; 0x0c],
    psilcfg_proxy_psil_to: PsilcfgProxyPsilTo,
    _reserved2: [u8; 0xec],
    psilcfg_proxy_psil_cmda: PsilcfgProxyPsilCmda,
    psilcfg_proxy_psil_cmdb: PsilcfgProxyPsilCmdb,
    psilcfg_proxy_psil_wdata: PsilcfgProxyPsilWdata,
    _reserved5: [u8; 0x34],
    psilcfg_proxy_psil_rdata: PsilcfgProxyPsilRdata,
}
impl RegisterBlock {
    #[doc = "0x00 - The Revision Register contains the major and minor revisions for the module."]
    #[inline(always)]
    pub const fn psilcfg_proxy_revision(&self) -> &PsilcfgProxyRevision {
        &self.psilcfg_proxy_revision
    }
    #[doc = "0x10 - The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
    #[inline(always)]
    pub const fn psilcfg_proxy_psil_to(&self) -> &PsilcfgProxyPsilTo {
        &self.psilcfg_proxy_psil_to
    }
    #[doc = "0x100 - The Command Register A contains the busy indicator, direction, and thread number for the configuration transaction."]
    #[inline(always)]
    pub const fn psilcfg_proxy_psil_cmda(&self) -> &PsilcfgProxyPsilCmda {
        &self.psilcfg_proxy_psil_cmda
    }
    #[doc = "0x104 - The Command Register B contains the byte enables and word address for the configuration transaction."]
    #[inline(always)]
    pub const fn psilcfg_proxy_psil_cmdb(&self) -> &PsilcfgProxyPsilCmdb {
        &self.psilcfg_proxy_psil_cmdb
    }
    #[doc = "0x108 - The Write Data Register contains the data which is to be written during the configuration transaction."]
    #[inline(always)]
    pub const fn psilcfg_proxy_psil_wdata(&self) -> &PsilcfgProxyPsilWdata {
        &self.psilcfg_proxy_psil_wdata
    }
    #[doc = "0x140 - The Read Data Register contains the data which which was read back during the configuration transaction."]
    #[inline(always)]
    pub const fn psilcfg_proxy_psil_rdata(&self) -> &PsilcfgProxyPsilRdata {
        &self.psilcfg_proxy_psil_rdata
    }
}
#[doc = "PSILCFG_PROXY_REVISION (rw) register accessor: The Revision Register contains the major and minor revisions for the module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_revision`]
module"]
#[doc(alias = "PSILCFG_PROXY_REVISION")]
pub type PsilcfgProxyRevision = crate::Reg<psilcfg_proxy_revision::PsilcfgProxyRevisionSpec>;
#[doc = "The Revision Register contains the major and minor revisions for the module."]
pub mod psilcfg_proxy_revision;
#[doc = "PSILCFG_PROXY_PSIL_TO (rw) register accessor: The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_to::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_to::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_psil_to`]
module"]
#[doc(alias = "PSILCFG_PROXY_PSIL_TO")]
pub type PsilcfgProxyPsilTo = crate::Reg<psilcfg_proxy_psil_to::PsilcfgProxyPsilToSpec>;
#[doc = "The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy."]
pub mod psilcfg_proxy_psil_to;
#[doc = "PSILCFG_PROXY_PSIL_CMDA (rw) register accessor: The Command Register A contains the busy indicator, direction, and thread number for the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_cmda::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_cmda::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_psil_cmda`]
module"]
#[doc(alias = "PSILCFG_PROXY_PSIL_CMDA")]
pub type PsilcfgProxyPsilCmda = crate::Reg<psilcfg_proxy_psil_cmda::PsilcfgProxyPsilCmdaSpec>;
#[doc = "The Command Register A contains the busy indicator, direction, and thread number for the configuration transaction."]
pub mod psilcfg_proxy_psil_cmda;
#[doc = "PSILCFG_PROXY_PSIL_CMDB (rw) register accessor: The Command Register B contains the byte enables and word address for the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_cmdb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_cmdb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_psil_cmdb`]
module"]
#[doc(alias = "PSILCFG_PROXY_PSIL_CMDB")]
pub type PsilcfgProxyPsilCmdb = crate::Reg<psilcfg_proxy_psil_cmdb::PsilcfgProxyPsilCmdbSpec>;
#[doc = "The Command Register B contains the byte enables and word address for the configuration transaction."]
pub mod psilcfg_proxy_psil_cmdb;
#[doc = "PSILCFG_PROXY_PSIL_WDATA (rw) register accessor: The Write Data Register contains the data which is to be written during the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_psil_wdata`]
module"]
#[doc(alias = "PSILCFG_PROXY_PSIL_WDATA")]
pub type PsilcfgProxyPsilWdata = crate::Reg<psilcfg_proxy_psil_wdata::PsilcfgProxyPsilWdataSpec>;
#[doc = "The Write Data Register contains the data which is to be written during the configuration transaction."]
pub mod psilcfg_proxy_psil_wdata;
#[doc = "PSILCFG_PROXY_PSIL_RDATA (rw) register accessor: The Read Data Register contains the data which which was read back during the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_rdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_rdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psilcfg_proxy_psil_rdata`]
module"]
#[doc(alias = "PSILCFG_PROXY_PSIL_RDATA")]
pub type PsilcfgProxyPsilRdata = crate::Reg<psilcfg_proxy_psil_rdata::PsilcfgProxyPsilRdataSpec>;
#[doc = "The Read Data Register contains the data which which was read back during the configuration transaction."]
pub mod psilcfg_proxy_psil_rdata;
