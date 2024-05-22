#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_tidr: CfgTidr,
    _reserved1: [u8; 0x0c],
    cfg_tiocp_cfg: CfgTiocpCfg,
    _reserved2: [u8; 0x0c],
    cfg_irq_eoi: CfgIrqEoi,
    cfg_irqstatus_raw: CfgIrqstatusRaw,
    cfg_irqstatus: CfgIrqstatus,
    cfg_irqstatus_set: CfgIrqstatusSet,
    cfg_irqstatus_clr: CfgIrqstatusClr,
    cfg_irqwakeen: CfgIrqwakeen,
    cfg_tclr: CfgTclr,
    cfg_tcrr: CfgTcrr,
    cfg_tldr: CfgTldr,
    cfg_ttgr: CfgTtgr,
    cfg_twps: CfgTwps,
    cfg_tmar: CfgTmar,
    cfg_tcar1: CfgTcar1,
    cfg_tsicr: CfgTsicr,
    cfg_tcar2: CfgTcar2,
    cfg_tpir: CfgTpir,
    cfg_tnir: CfgTnir,
    cfg_tcvr: CfgTcvr,
    cfg_tocr: CfgTocr,
    cfg_towr: CfgTowr,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG_TIDR"]
    #[inline(always)]
    pub const fn cfg_tidr(&self) -> &CfgTidr {
        &self.cfg_tidr
    }
    #[doc = "0x10 - This register controls the various parameters of the OCP interface"]
    #[inline(always)]
    pub const fn cfg_tiocp_cfg(&self) -> &CfgTiocpCfg {
        &self.cfg_tiocp_cfg
    }
    #[doc = "0x20 - Allows the generation of further pulses on the interrupt line, if an new interrupt event is pending, when using the pulsed output. Unused when using the level interrupt line"]
    #[inline(always)]
    pub const fn cfg_irq_eoi(&self) -> &CfgIrqEoi {
        &self.cfg_irq_eoi
    }
    #[doc = "0x24 - Component interrupt request status. Check the corresponding secondary status register. Raw status is set even if event is not enabled. Write 1 to set the (raw) status, mostly for debug"]
    #[inline(always)]
    pub const fn cfg_irqstatus_raw(&self) -> &CfgIrqstatusRaw {
        &self.cfg_irqstatus_raw
    }
    #[doc = "0x28 - Component interrupt request status. Check the corresponding secondary status register.Enabled status isn't set unless event is enabled.Write 1 to clear the status after interrupt has been serviced;raw status gets cleared, i.e. even if not enabled"]
    #[inline(always)]
    pub const fn cfg_irqstatus(&self) -> &CfgIrqstatus {
        &self.cfg_irqstatus
    }
    #[doc = "0x2c - Component interrupt request enable Write 1 to set;enable interrupt. Readout equal to corresponding _CLR register."]
    #[inline(always)]
    pub const fn cfg_irqstatus_set(&self) -> &CfgIrqstatusSet {
        &self.cfg_irqstatus_set
    }
    #[doc = "0x30 - Component interrupt request enable. Write 1 to clear; disable interrupt. Readout equal to corresponding _SET register"]
    #[inline(always)]
    pub const fn cfg_irqstatus_clr(&self) -> &CfgIrqstatusClr {
        &self.cfg_irqstatus_clr
    }
    #[doc = "0x34 - Wakeup-enabled events taking place when module is idle shall generate an asynchronous wakeup"]
    #[inline(always)]
    pub const fn cfg_irqwakeen(&self) -> &CfgIrqwakeen {
        &self.cfg_irqwakeen
    }
    #[doc = "0x38 - This register controls optional features specific to the timer functionality"]
    #[inline(always)]
    pub const fn cfg_tclr(&self) -> &CfgTclr {
        &self.cfg_tclr
    }
    #[doc = "0x3c - This register holds the value of the internal counter"]
    #[inline(always)]
    pub const fn cfg_tcrr(&self) -> &CfgTcrr {
        &self.cfg_tcrr
    }
    #[doc = "0x40 - This register holds the timer's load value"]
    #[inline(always)]
    pub const fn cfg_tldr(&self) -> &CfgTldr {
        &self.cfg_tldr
    }
    #[doc = "0x44 - This register triggers a counter reload of timer by writing any value in it"]
    #[inline(always)]
    pub const fn cfg_ttgr(&self) -> &CfgTtgr {
        &self.cfg_ttgr
    }
    #[doc = "0x48 - This register contains the write posting bits for all writ-able functional registers"]
    #[inline(always)]
    pub const fn cfg_twps(&self) -> &CfgTwps {
        &self.cfg_twps
    }
    #[doc = "0x4c - This register holds the match value to be compared with the counter's value"]
    #[inline(always)]
    pub const fn cfg_tmar(&self) -> &CfgTmar {
        &self.cfg_tmar
    }
    #[doc = "0x50 - This register holds the value of the first counter register capture"]
    #[inline(always)]
    pub const fn cfg_tcar1(&self) -> &CfgTcar1 {
        &self.cfg_tcar1
    }
    #[doc = "0x54 - Timer Synchronous Interface Control Register"]
    #[inline(always)]
    pub const fn cfg_tsicr(&self) -> &CfgTsicr {
        &self.cfg_tsicr
    }
    #[doc = "0x58 - This register holds the value of the second counter register capture"]
    #[inline(always)]
    pub const fn cfg_tcar2(&self) -> &CfgTcar2 {
        &self.cfg_tcar2
    }
    #[doc = "0x5c - This register is used for 1ms tick generation. The TPIR register holds the value of the positive increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value"]
    #[inline(always)]
    pub const fn cfg_tpir(&self) -> &CfgTpir {
        &self.cfg_tpir
    }
    #[doc = "0x60 - This register is used for 1ms tick generation. The TNIR register holds the value of the negative increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value"]
    #[inline(always)]
    pub const fn cfg_tnir(&self) -> &CfgTnir {
        &self.cfg_tnir
    }
    #[doc = "0x64 - This register is used for 1ms tick generation. The TCVR register defines whether next value loaded in TCRR will be the sub-period value or the over-period value"]
    #[inline(always)]
    pub const fn cfg_tcvr(&self) -> &CfgTcvr {
        &self.cfg_tcvr
    }
    #[doc = "0x68 - This register is used to mask the tick interrupt for a selected number of ticks"]
    #[inline(always)]
    pub const fn cfg_tocr(&self) -> &CfgTocr {
        &self.cfg_tocr
    }
    #[doc = "0x6c - This register holds the number of masked overflow interrupts"]
    #[inline(always)]
    pub const fn cfg_towr(&self) -> &CfgTowr {
        &self.cfg_towr
    }
}
#[doc = "CFG_TIDR (rw) register accessor: CFG_TIDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tidr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tidr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tidr`]
module"]
#[doc(alias = "CFG_TIDR")]
pub type CfgTidr = crate::Reg<cfg_tidr::CfgTidrSpec>;
#[doc = "CFG_TIDR"]
pub mod cfg_tidr;
#[doc = "CFG_TIOCP_CFG (rw) register accessor: This register controls the various parameters of the OCP interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tiocp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tiocp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tiocp_cfg`]
module"]
#[doc(alias = "CFG_TIOCP_CFG")]
pub type CfgTiocpCfg = crate::Reg<cfg_tiocp_cfg::CfgTiocpCfgSpec>;
#[doc = "This register controls the various parameters of the OCP interface"]
pub mod cfg_tiocp_cfg;
#[doc = "CFG_IRQ_EOI (rw) register accessor: Allows the generation of further pulses on the interrupt line, if an new interrupt event is pending, when using the pulsed output. Unused when using the level interrupt line\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irq_eoi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irq_eoi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irq_eoi`]
module"]
#[doc(alias = "CFG_IRQ_EOI")]
pub type CfgIrqEoi = crate::Reg<cfg_irq_eoi::CfgIrqEoiSpec>;
#[doc = "Allows the generation of further pulses on the interrupt line, if an new interrupt event is pending, when using the pulsed output. Unused when using the level interrupt line"]
pub mod cfg_irq_eoi;
#[doc = "CFG_IRQSTATUS_RAW (rw) register accessor: Component interrupt request status. Check the corresponding secondary status register. Raw status is set even if event is not enabled. Write 1 to set the (raw) status, mostly for debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqstatus_raw`]
module"]
#[doc(alias = "CFG_IRQSTATUS_RAW")]
pub type CfgIrqstatusRaw = crate::Reg<cfg_irqstatus_raw::CfgIrqstatusRawSpec>;
#[doc = "Component interrupt request status. Check the corresponding secondary status register. Raw status is set even if event is not enabled. Write 1 to set the (raw) status, mostly for debug"]
pub mod cfg_irqstatus_raw;
#[doc = "CFG_IRQSTATUS (rw) register accessor: Component interrupt request status. Check the corresponding secondary status register.Enabled status isn't set unless event is enabled.Write 1 to clear the status after interrupt has been serviced;raw status gets cleared, i.e. even if not enabled\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqstatus`]
module"]
#[doc(alias = "CFG_IRQSTATUS")]
pub type CfgIrqstatus = crate::Reg<cfg_irqstatus::CfgIrqstatusSpec>;
#[doc = "Component interrupt request status. Check the corresponding secondary status register.Enabled status isn't set unless event is enabled.Write 1 to clear the status after interrupt has been serviced;raw status gets cleared, i.e. even if not enabled"]
pub mod cfg_irqstatus;
#[doc = "CFG_IRQSTATUS_SET (rw) register accessor: Component interrupt request enable Write 1 to set;enable interrupt. Readout equal to corresponding _CLR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqstatus_set`]
module"]
#[doc(alias = "CFG_IRQSTATUS_SET")]
pub type CfgIrqstatusSet = crate::Reg<cfg_irqstatus_set::CfgIrqstatusSetSpec>;
#[doc = "Component interrupt request enable Write 1 to set;enable interrupt. Readout equal to corresponding _CLR register."]
pub mod cfg_irqstatus_set;
#[doc = "CFG_IRQSTATUS_CLR (rw) register accessor: Component interrupt request enable. Write 1 to clear; disable interrupt. Readout equal to corresponding _SET register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqstatus_clr`]
module"]
#[doc(alias = "CFG_IRQSTATUS_CLR")]
pub type CfgIrqstatusClr = crate::Reg<cfg_irqstatus_clr::CfgIrqstatusClrSpec>;
#[doc = "Component interrupt request enable. Write 1 to clear; disable interrupt. Readout equal to corresponding _SET register"]
pub mod cfg_irqstatus_clr;
#[doc = "CFG_IRQWAKEEN (rw) register accessor: Wakeup-enabled events taking place when module is idle shall generate an asynchronous wakeup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqwakeen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqwakeen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_irqwakeen`]
module"]
#[doc(alias = "CFG_IRQWAKEEN")]
pub type CfgIrqwakeen = crate::Reg<cfg_irqwakeen::CfgIrqwakeenSpec>;
#[doc = "Wakeup-enabled events taking place when module is idle shall generate an asynchronous wakeup"]
pub mod cfg_irqwakeen;
#[doc = "CFG_TCLR (rw) register accessor: This register controls optional features specific to the timer functionality\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tclr`]
module"]
#[doc(alias = "CFG_TCLR")]
pub type CfgTclr = crate::Reg<cfg_tclr::CfgTclrSpec>;
#[doc = "This register controls optional features specific to the timer functionality"]
pub mod cfg_tclr;
#[doc = "CFG_TCRR (rw) register accessor: This register holds the value of the internal counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcrr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcrr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tcrr`]
module"]
#[doc(alias = "CFG_TCRR")]
pub type CfgTcrr = crate::Reg<cfg_tcrr::CfgTcrrSpec>;
#[doc = "This register holds the value of the internal counter"]
pub mod cfg_tcrr;
#[doc = "CFG_TLDR (rw) register accessor: This register holds the timer's load value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tldr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tldr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tldr`]
module"]
#[doc(alias = "CFG_TLDR")]
pub type CfgTldr = crate::Reg<cfg_tldr::CfgTldrSpec>;
#[doc = "This register holds the timer's load value"]
pub mod cfg_tldr;
#[doc = "CFG_TTGR (rw) register accessor: This register triggers a counter reload of timer by writing any value in it\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ttgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ttgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ttgr`]
module"]
#[doc(alias = "CFG_TTGR")]
pub type CfgTtgr = crate::Reg<cfg_ttgr::CfgTtgrSpec>;
#[doc = "This register triggers a counter reload of timer by writing any value in it"]
pub mod cfg_ttgr;
#[doc = "CFG_TWPS (rw) register accessor: This register contains the write posting bits for all writ-able functional registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_twps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_twps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_twps`]
module"]
#[doc(alias = "CFG_TWPS")]
pub type CfgTwps = crate::Reg<cfg_twps::CfgTwpsSpec>;
#[doc = "This register contains the write posting bits for all writ-able functional registers"]
pub mod cfg_twps;
#[doc = "CFG_TMAR (rw) register accessor: This register holds the match value to be compared with the counter's value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tmar`]
module"]
#[doc(alias = "CFG_TMAR")]
pub type CfgTmar = crate::Reg<cfg_tmar::CfgTmarSpec>;
#[doc = "This register holds the match value to be compared with the counter's value"]
pub mod cfg_tmar;
#[doc = "CFG_TCAR1 (rw) register accessor: This register holds the value of the first counter register capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tcar1`]
module"]
#[doc(alias = "CFG_TCAR1")]
pub type CfgTcar1 = crate::Reg<cfg_tcar1::CfgTcar1Spec>;
#[doc = "This register holds the value of the first counter register capture"]
pub mod cfg_tcar1;
#[doc = "CFG_TSICR (rw) register accessor: Timer Synchronous Interface Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tsicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tsicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tsicr`]
module"]
#[doc(alias = "CFG_TSICR")]
pub type CfgTsicr = crate::Reg<cfg_tsicr::CfgTsicrSpec>;
#[doc = "Timer Synchronous Interface Control Register"]
pub mod cfg_tsicr;
#[doc = "CFG_TCAR2 (rw) register accessor: This register holds the value of the second counter register capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tcar2`]
module"]
#[doc(alias = "CFG_TCAR2")]
pub type CfgTcar2 = crate::Reg<cfg_tcar2::CfgTcar2Spec>;
#[doc = "This register holds the value of the second counter register capture"]
pub mod cfg_tcar2;
#[doc = "CFG_TPIR (rw) register accessor: This register is used for 1ms tick generation. The TPIR register holds the value of the positive increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tpir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tpir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tpir`]
module"]
#[doc(alias = "CFG_TPIR")]
pub type CfgTpir = crate::Reg<cfg_tpir::CfgTpirSpec>;
#[doc = "This register is used for 1ms tick generation. The TPIR register holds the value of the positive increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value"]
pub mod cfg_tpir;
#[doc = "CFG_TNIR (rw) register accessor: This register is used for 1ms tick generation. The TNIR register holds the value of the negative increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tnir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tnir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tnir`]
module"]
#[doc(alias = "CFG_TNIR")]
pub type CfgTnir = crate::Reg<cfg_tnir::CfgTnirSpec>;
#[doc = "This register is used for 1ms tick generation. The TNIR register holds the value of the negative increment. The value of this register is added with the value of the TCVR to define whether next value loaded in TCRR will be the sub-period value or the over-period value"]
pub mod cfg_tnir;
#[doc = "CFG_TCVR (rw) register accessor: This register is used for 1ms tick generation. The TCVR register defines whether next value loaded in TCRR will be the sub-period value or the over-period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tcvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tcvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tcvr`]
module"]
#[doc(alias = "CFG_TCVR")]
pub type CfgTcvr = crate::Reg<cfg_tcvr::CfgTcvrSpec>;
#[doc = "This register is used for 1ms tick generation. The TCVR register defines whether next value loaded in TCRR will be the sub-period value or the over-period value"]
pub mod cfg_tcvr;
#[doc = "CFG_TOCR (rw) register accessor: This register is used to mask the tick interrupt for a selected number of ticks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tocr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tocr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tocr`]
module"]
#[doc(alias = "CFG_TOCR")]
pub type CfgTocr = crate::Reg<cfg_tocr::CfgTocrSpec>;
#[doc = "This register is used to mask the tick interrupt for a selected number of ticks"]
pub mod cfg_tocr;
#[doc = "CFG_TOWR (rw) register accessor: This register holds the number of masked overflow interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_towr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_towr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_towr`]
module"]
#[doc(alias = "CFG_TOWR")]
pub type CfgTowr = crate::Reg<cfg_towr::CfgTowrSpec>;
#[doc = "This register holds the number of masked overflow interrupts"]
pub mod cfg_towr;
