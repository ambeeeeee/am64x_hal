#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_hl_rev: CfgHlRev,
    cfg_hl_hwinfo: CfgHlHwinfo,
    _reserved2: [u8; 0x08],
    cfg_hl_sysconfig: CfgHlSysconfig,
    _reserved3: [u8; 0xec],
    cfg_revision: CfgRevision,
    _reserved4: [u8; 0x0c],
    cfg_sysconfig: CfgSysconfig,
    cfg_sysstatus: CfgSysstatus,
    cfg_irqstatus: CfgIrqstatus,
    cfg_irqenable: CfgIrqenable,
    cfg_wakeupenable: CfgWakeupenable,
    cfg_syst: CfgSyst,
    cfg_modulctrl: CfgModulctrl,
    cfg_ch0conf: CfgCh0conf,
    cfg_ch0stat: CfgCh0stat,
    cfg_ch0ctrl: CfgCh0ctrl,
    cfg_tx0: CfgTx0,
    cfg_rx0: CfgRx0,
    cfg_ch1conf: CfgCh1conf,
    cfg_ch1stat: CfgCh1stat,
    cfg_ch1ctrl: CfgCh1ctrl,
    cfg_tx1: CfgTx1,
    cfg_rx1: CfgRx1,
    cfg_ch2conf: CfgCh2conf,
    cfg_ch2stat: CfgCh2stat,
    cfg_ch2ctrl: CfgCh2ctrl,
    cfg_tx2: CfgTx2,
    cfg_rx2: CfgRx2,
    cfg_ch3conf: CfgCh3conf,
    cfg_ch3stat: CfgCh3stat,
    cfg_ch3ctrl: CfgCh3ctrl,
    cfg_tx3: CfgTx3,
    cfg_rx3: CfgRx3,
    cfg_xferlevel: CfgXferlevel,
    cfg_daftx: CfgDaftx,
    _reserved33: [u8; 0x1c],
    cfg_dafrx: CfgDafrx,
}
impl RegisterBlock {
    #[doc = "0x00 - IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
    #[inline(always)]
    pub const fn cfg_hl_rev(&self) -> &CfgHlRev {
        &self.cfg_hl_rev
    }
    #[doc = "0x04 - Information about the IP module's hardware configuration, i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
    #[inline(always)]
    pub const fn cfg_hl_hwinfo(&self) -> &CfgHlHwinfo {
        &self.cfg_hl_hwinfo
    }
    #[doc = "0x10 - Clock management configuration"]
    #[inline(always)]
    pub const fn cfg_hl_sysconfig(&self) -> &CfgHlSysconfig {
        &self.cfg_hl_sysconfig
    }
    #[doc = "0x100 - This register contains the hard coded RTL revision number."]
    #[inline(always)]
    pub const fn cfg_revision(&self) -> &CfgRevision {
        &self.cfg_revision
    }
    #[doc = "0x110 - This register allows controlling various parameters of the OCP interface."]
    #[inline(always)]
    pub const fn cfg_sysconfig(&self) -> &CfgSysconfig {
        &self.cfg_sysconfig
    }
    #[doc = "0x114 - This register provides status information about the module excluding the interrupt status information"]
    #[inline(always)]
    pub const fn cfg_sysstatus(&self) -> &CfgSysstatus {
        &self.cfg_sysstatus
    }
    #[doc = "0x118 - The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
    #[inline(always)]
    pub const fn cfg_irqstatus(&self) -> &CfgIrqstatus {
        &self.cfg_irqstatus
    }
    #[doc = "0x11c - This register allows to enable/disable the module internal sources of interrupt, on an event-by-event basis."]
    #[inline(always)]
    pub const fn cfg_irqenable(&self) -> &CfgIrqenable {
        &self.cfg_irqenable
    }
    #[doc = "0x120 - The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
    #[inline(always)]
    pub const fn cfg_wakeupenable(&self) -> &CfgWakeupenable {
        &self.cfg_wakeupenable
    }
    #[doc = "0x124 - This register is used to check the correctness of the system interconnect either internally to peripheral bus, or externally to device IO pads, when the module is configured in system test (SYSTEST) mode."]
    #[inline(always)]
    pub const fn cfg_syst(&self) -> &CfgSyst {
        &self.cfg_syst
    }
    #[doc = "0x128 - This register is dedicated to the configuration of the serial port interface."]
    #[inline(always)]
    pub const fn cfg_modulctrl(&self) -> &CfgModulctrl {
        &self.cfg_modulctrl
    }
    #[doc = "0x12c - This register is dedicated to the configuration of the channel 0"]
    #[inline(always)]
    pub const fn cfg_ch0conf(&self) -> &CfgCh0conf {
        &self.cfg_ch0conf
    }
    #[doc = "0x130 - This register provides status information about transmitter and receiver registers of channel 0"]
    #[inline(always)]
    pub const fn cfg_ch0stat(&self) -> &CfgCh0stat {
        &self.cfg_ch0stat
    }
    #[doc = "0x134 - This register is dedicated to enable the channel 0"]
    #[inline(always)]
    pub const fn cfg_ch0ctrl(&self) -> &CfgCh0ctrl {
        &self.cfg_ch0ctrl
    }
    #[doc = "0x138 - This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_tx0(&self) -> &CfgTx0 {
        &self.cfg_tx0
    }
    #[doc = "0x13c - This register contains a single SPI word received through the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_rx0(&self) -> &CfgRx0 {
        &self.cfg_rx0
    }
    #[doc = "0x140 - This register is dedicated to the configuration of the channel."]
    #[inline(always)]
    pub const fn cfg_ch1conf(&self) -> &CfgCh1conf {
        &self.cfg_ch1conf
    }
    #[doc = "0x144 - This register provides status information about transmitter and receiver registers of channel 1"]
    #[inline(always)]
    pub const fn cfg_ch1stat(&self) -> &CfgCh1stat {
        &self.cfg_ch1stat
    }
    #[doc = "0x148 - This register is dedicated to enable the channel 1"]
    #[inline(always)]
    pub const fn cfg_ch1ctrl(&self) -> &CfgCh1ctrl {
        &self.cfg_ch1ctrl
    }
    #[doc = "0x14c - This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_tx1(&self) -> &CfgTx1 {
        &self.cfg_tx1
    }
    #[doc = "0x150 - This register contains a single SPI word received through the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_rx1(&self) -> &CfgRx1 {
        &self.cfg_rx1
    }
    #[doc = "0x154 - This register is dedicated to the configuration of the channel 2"]
    #[inline(always)]
    pub const fn cfg_ch2conf(&self) -> &CfgCh2conf {
        &self.cfg_ch2conf
    }
    #[doc = "0x158 - This register provides status information about transmitter and receiver registers of channel 2"]
    #[inline(always)]
    pub const fn cfg_ch2stat(&self) -> &CfgCh2stat {
        &self.cfg_ch2stat
    }
    #[doc = "0x15c - This register is dedicated to enable the channel 2"]
    #[inline(always)]
    pub const fn cfg_ch2ctrl(&self) -> &CfgCh2ctrl {
        &self.cfg_ch2ctrl
    }
    #[doc = "0x160 - This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_tx2(&self) -> &CfgTx2 {
        &self.cfg_tx2
    }
    #[doc = "0x164 - This register contains a single SPI word received through the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_rx2(&self) -> &CfgRx2 {
        &self.cfg_rx2
    }
    #[doc = "0x168 - This register is dedicated to the configuration of the channel 3"]
    #[inline(always)]
    pub const fn cfg_ch3conf(&self) -> &CfgCh3conf {
        &self.cfg_ch3conf
    }
    #[doc = "0x16c - This register provides status information about transmitter and receiver registers of channel 3"]
    #[inline(always)]
    pub const fn cfg_ch3stat(&self) -> &CfgCh3stat {
        &self.cfg_ch3stat
    }
    #[doc = "0x170 - This register is dedicated to enable the channel 3"]
    #[inline(always)]
    pub const fn cfg_ch3ctrl(&self) -> &CfgCh3ctrl {
        &self.cfg_ch3ctrl
    }
    #[doc = "0x174 - This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_tx3(&self) -> &CfgTx3 {
        &self.cfg_tx3
    }
    #[doc = "0x178 - This register contains a single SPI word received through the serial link, what ever SPI word length is."]
    #[inline(always)]
    pub const fn cfg_rx3(&self) -> &CfgRx3 {
        &self.cfg_rx3
    }
    #[doc = "0x17c - This register provides transfer levels needed while using FIFO buffer during transfer."]
    #[inline(always)]
    pub const fn cfg_xferlevel(&self) -> &CfgXferlevel {
        &self.cfg_xferlevel
    }
    #[doc = "0x180 - This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
    #[inline(always)]
    pub const fn cfg_daftx(&self) -> &CfgDaftx {
        &self.cfg_daftx
    }
    #[doc = "0x1a0 - This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled."]
    #[inline(always)]
    pub const fn cfg_dafrx(&self) -> &CfgDafrx {
        &self.cfg_dafrx
    }
}
#[doc = "CFG_HL_REV (rw) register accessor: IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hl_rev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hl_rev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hl_rev`]
module"]
#[doc(alias = "CFG_HL_REV")]
pub type CfgHlRev = crate::Reg<cfg_hl_rev::CfgHlRevSpec>;
#[doc = "IP Revision Identifier (X.Y.R) Used by software to track features, bugs, and compatibility"]
pub mod cfg_hl_rev;
#[doc = "CFG_HL_HWINFO (rw) register accessor: Information about the IP module's hardware configuration, i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hl_hwinfo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hl_hwinfo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hl_hwinfo`]
module"]
#[doc(alias = "CFG_HL_HWINFO")]
pub type CfgHlHwinfo = crate::Reg<cfg_hl_hwinfo::CfgHlHwinfoSpec>;
#[doc = "Information about the IP module's hardware configuration, i.e. typically the module's HDL generics (if any). Actual field format and encoding is up to the module's designer to decide."]
pub mod cfg_hl_hwinfo;
#[doc = "CFG_HL_SYSCONFIG (rw) register accessor: Clock management configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_hl_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_hl_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_hl_sysconfig`]
module"]
#[doc(alias = "CFG_HL_SYSCONFIG")]
pub type CfgHlSysconfig = crate::Reg<cfg_hl_sysconfig::CfgHlSysconfigSpec>;
#[doc = "Clock management configuration"]
pub mod cfg_hl_sysconfig;
#[doc = "CFG_REVISION (rw) register accessor: This register contains the hard coded RTL revision number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_revision::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_revision::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_revision`]
module"]
#[doc(alias = "CFG_REVISION")]
pub type CfgRevision = crate::Reg<cfg_revision::CfgRevisionSpec>;
#[doc = "This register contains the hard coded RTL revision number."]
pub mod cfg_revision;
#[doc = "CFG_SYSCONFIG (rw) register accessor: This register allows controlling various parameters of the OCP interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sysconfig`]
module"]
#[doc(alias = "CFG_SYSCONFIG")]
pub type CfgSysconfig = crate::Reg<cfg_sysconfig::CfgSysconfigSpec>;
#[doc = "This register allows controlling various parameters of the OCP interface."]
pub mod cfg_sysconfig;
#[doc = "CFG_SYSSTATUS (rw) register accessor: This register provides status information about the module excluding the interrupt status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_sysstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_sysstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sysstatus`]
module"]
#[doc(alias = "CFG_SYSSTATUS")]
pub type CfgSysstatus = crate::Reg<cfg_sysstatus::CfgSysstatusSpec>;
#[doc = "This register provides status information about the module excluding the interrupt status information"]
pub mod cfg_sysstatus;
#[doc = "CFG_IRQSTATUS (rw) register accessor: The interrupt status regroups all the status of the module internal events that can generate an interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqstatus`]
module"]
#[doc(alias = "CFG_IRQSTATUS")]
pub type CfgIrqstatus = crate::Reg<cfg_irqstatus::CfgIrqstatusSpec>;
#[doc = "The interrupt status regroups all the status of the module internal events that can generate an interrupt"]
pub mod cfg_irqstatus;
#[doc = "CFG_IRQENABLE (rw) register accessor: This register allows to enable/disable the module internal sources of interrupt, on an event-by-event basis.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqenable`]
module"]
#[doc(alias = "CFG_IRQENABLE")]
pub type CfgIrqenable = crate::Reg<cfg_irqenable::CfgIrqenableSpec>;
#[doc = "This register allows to enable/disable the module internal sources of interrupt, on an event-by-event basis."]
pub mod cfg_irqenable;
#[doc = "CFG_WAKEUPENABLE (rw) register accessor: The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wakeupenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wakeupenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_wakeupenable`]
module"]
#[doc(alias = "CFG_WAKEUPENABLE")]
pub type CfgWakeupenable = crate::Reg<cfg_wakeupenable::CfgWakeupenableSpec>;
#[doc = "The wakeup enable register allows to enable/disable the module internal sources of wakeup on event-by-event basis."]
pub mod cfg_wakeupenable;
#[doc = "CFG_SYST (rw) register accessor: This register is used to check the correctness of the system interconnect either internally to peripheral bus, or externally to device IO pads, when the module is configured in system test (SYSTEST) mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_syst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_syst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_syst`]
module"]
#[doc(alias = "CFG_SYST")]
pub type CfgSyst = crate::Reg<cfg_syst::CfgSystSpec>;
#[doc = "This register is used to check the correctness of the system interconnect either internally to peripheral bus, or externally to device IO pads, when the module is configured in system test (SYSTEST) mode."]
pub mod cfg_syst;
#[doc = "CFG_MODULCTRL (rw) register accessor: This register is dedicated to the configuration of the serial port interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_modulctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_modulctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_modulctrl`]
module"]
#[doc(alias = "CFG_MODULCTRL")]
pub type CfgModulctrl = crate::Reg<cfg_modulctrl::CfgModulctrlSpec>;
#[doc = "This register is dedicated to the configuration of the serial port interface."]
pub mod cfg_modulctrl;
#[doc = "CFG_CH0CONF (rw) register accessor: This register is dedicated to the configuration of the channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch0conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch0conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch0conf`]
module"]
#[doc(alias = "CFG_CH0CONF")]
pub type CfgCh0conf = crate::Reg<cfg_ch0conf::CfgCh0confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 0"]
pub mod cfg_ch0conf;
#[doc = "CFG_CH0STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch0stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch0stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch0stat`]
module"]
#[doc(alias = "CFG_CH0STAT")]
pub type CfgCh0stat = crate::Reg<cfg_ch0stat::CfgCh0statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 0"]
pub mod cfg_ch0stat;
#[doc = "CFG_CH0CTRL (rw) register accessor: This register is dedicated to enable the channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch0ctrl`]
module"]
#[doc(alias = "CFG_CH0CTRL")]
pub type CfgCh0ctrl = crate::Reg<cfg_ch0ctrl::CfgCh0ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 0"]
pub mod cfg_ch0ctrl;
#[doc = "CFG_TX0 (rw) register accessor: This register contains a single SPI word to transmit on the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx0`]
module"]
#[doc(alias = "CFG_TX0")]
pub type CfgTx0 = crate::Reg<cfg_tx0::CfgTx0Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
pub mod cfg_tx0;
#[doc = "CFG_RX0 (rw) register accessor: This register contains a single SPI word received through the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx0`]
module"]
#[doc(alias = "CFG_RX0")]
pub type CfgRx0 = crate::Reg<cfg_rx0::CfgRx0Spec>;
#[doc = "This register contains a single SPI word received through the serial link, what ever SPI word length is."]
pub mod cfg_rx0;
#[doc = "CFG_CH1CONF (rw) register accessor: This register is dedicated to the configuration of the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch1conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch1conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch1conf`]
module"]
#[doc(alias = "CFG_CH1CONF")]
pub type CfgCh1conf = crate::Reg<cfg_ch1conf::CfgCh1confSpec>;
#[doc = "This register is dedicated to the configuration of the channel."]
pub mod cfg_ch1conf;
#[doc = "CFG_CH1STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch1stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch1stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch1stat`]
module"]
#[doc(alias = "CFG_CH1STAT")]
pub type CfgCh1stat = crate::Reg<cfg_ch1stat::CfgCh1statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 1"]
pub mod cfg_ch1stat;
#[doc = "CFG_CH1CTRL (rw) register accessor: This register is dedicated to enable the channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch1ctrl`]
module"]
#[doc(alias = "CFG_CH1CTRL")]
pub type CfgCh1ctrl = crate::Reg<cfg_ch1ctrl::CfgCh1ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 1"]
pub mod cfg_ch1ctrl;
#[doc = "CFG_TX1 (rw) register accessor: This register contains a single SPI word to transmit on the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx1`]
module"]
#[doc(alias = "CFG_TX1")]
pub type CfgTx1 = crate::Reg<cfg_tx1::CfgTx1Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
pub mod cfg_tx1;
#[doc = "CFG_RX1 (rw) register accessor: This register contains a single SPI word received through the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx1`]
module"]
#[doc(alias = "CFG_RX1")]
pub type CfgRx1 = crate::Reg<cfg_rx1::CfgRx1Spec>;
#[doc = "This register contains a single SPI word received through the serial link, what ever SPI word length is."]
pub mod cfg_rx1;
#[doc = "CFG_CH2CONF (rw) register accessor: This register is dedicated to the configuration of the channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch2conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch2conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch2conf`]
module"]
#[doc(alias = "CFG_CH2CONF")]
pub type CfgCh2conf = crate::Reg<cfg_ch2conf::CfgCh2confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 2"]
pub mod cfg_ch2conf;
#[doc = "CFG_CH2STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch2stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch2stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch2stat`]
module"]
#[doc(alias = "CFG_CH2STAT")]
pub type CfgCh2stat = crate::Reg<cfg_ch2stat::CfgCh2statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 2"]
pub mod cfg_ch2stat;
#[doc = "CFG_CH2CTRL (rw) register accessor: This register is dedicated to enable the channel 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch2ctrl`]
module"]
#[doc(alias = "CFG_CH2CTRL")]
pub type CfgCh2ctrl = crate::Reg<cfg_ch2ctrl::CfgCh2ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 2"]
pub mod cfg_ch2ctrl;
#[doc = "CFG_TX2 (rw) register accessor: This register contains a single SPI word to transmit on the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx2`]
module"]
#[doc(alias = "CFG_TX2")]
pub type CfgTx2 = crate::Reg<cfg_tx2::CfgTx2Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
pub mod cfg_tx2;
#[doc = "CFG_RX2 (rw) register accessor: This register contains a single SPI word received through the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx2`]
module"]
#[doc(alias = "CFG_RX2")]
pub type CfgRx2 = crate::Reg<cfg_rx2::CfgRx2Spec>;
#[doc = "This register contains a single SPI word received through the serial link, what ever SPI word length is."]
pub mod cfg_rx2;
#[doc = "CFG_CH3CONF (rw) register accessor: This register is dedicated to the configuration of the channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch3conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch3conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch3conf`]
module"]
#[doc(alias = "CFG_CH3CONF")]
pub type CfgCh3conf = crate::Reg<cfg_ch3conf::CfgCh3confSpec>;
#[doc = "This register is dedicated to the configuration of the channel 3"]
pub mod cfg_ch3conf;
#[doc = "CFG_CH3STAT (rw) register accessor: This register provides status information about transmitter and receiver registers of channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch3stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch3stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch3stat`]
module"]
#[doc(alias = "CFG_CH3STAT")]
pub type CfgCh3stat = crate::Reg<cfg_ch3stat::CfgCh3statSpec>;
#[doc = "This register provides status information about transmitter and receiver registers of channel 3"]
pub mod cfg_ch3stat;
#[doc = "CFG_CH3CTRL (rw) register accessor: This register is dedicated to enable the channel 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ch3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ch3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ch3ctrl`]
module"]
#[doc(alias = "CFG_CH3CTRL")]
pub type CfgCh3ctrl = crate::Reg<cfg_ch3ctrl::CfgCh3ctrlSpec>;
#[doc = "This register is dedicated to enable the channel 3"]
pub mod cfg_ch3ctrl;
#[doc = "CFG_TX3 (rw) register accessor: This register contains a single SPI word to transmit on the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tx3`]
module"]
#[doc(alias = "CFG_TX3")]
pub type CfgTx3 = crate::Reg<cfg_tx3::CfgTx3Spec>;
#[doc = "This register contains a single SPI word to transmit on the serial link, what ever SPI word length is."]
pub mod cfg_tx3;
#[doc = "CFG_RX3 (rw) register accessor: This register contains a single SPI word received through the serial link, what ever SPI word length is.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_rx3`]
module"]
#[doc(alias = "CFG_RX3")]
pub type CfgRx3 = crate::Reg<cfg_rx3::CfgRx3Spec>;
#[doc = "This register contains a single SPI word received through the serial link, what ever SPI word length is."]
pub mod cfg_rx3;
#[doc = "CFG_XFERLEVEL (rw) register accessor: This register provides transfer levels needed while using FIFO buffer during transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_xferlevel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_xferlevel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_xferlevel`]
module"]
#[doc(alias = "CFG_XFERLEVEL")]
pub type CfgXferlevel = crate::Reg<cfg_xferlevel::CfgXferlevelSpec>;
#[doc = "This register provides transfer levels needed while using FIFO buffer during transfer."]
pub mod cfg_xferlevel;
#[doc = "CFG_DAFTX (rw) register accessor: This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_daftx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_daftx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_daftx`]
module"]
#[doc(alias = "CFG_DAFTX")]
pub type CfgDaftx = crate::Reg<cfg_daftx::CfgDaftxSpec>;
#[doc = "This register contains the SPI words to transmit on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_TX(i) register corresponding to the channel which have its FIFO enabled."]
pub mod cfg_daftx;
#[doc = "CFG_DAFRX (rw) register accessor: This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dafrx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dafrx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dafrx`]
module"]
#[doc(alias = "CFG_DAFRX")]
pub type CfgDafrx = crate::Reg<cfg_dafrx::CfgDafrxSpec>;
#[doc = "This register contains the SPI words to received on the serial link when FIFO used and DMA address is aligned on 256 bit.This register is an image of one of MCSPI_RX(i) register corresponding to the channel which have its FIFO enabled."]
pub mod cfg_dafrx;
