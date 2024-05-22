#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0: Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1: Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1,
    _reserved2: [u8; 0x08],
    pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0: Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1: Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1,
    _reserved4: [u8; 0x08],
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0: Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1: Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc1,
    _reserved6: [u8; 0x08],
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0: Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1: Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0: Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1: Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0: Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1: Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0: Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1: Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0: Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1: Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1,
    _reserved16: [u8; 0x08],
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0: Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1: Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel1,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0: Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel0,
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1: Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel1,
}
impl RegisterBlock {
    #[doc = "0x00 - MIIRXCFG0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0
    }
    #[doc = "0x04 - MIIRXCFG1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1
    }
    #[doc = "0x10 - MIITXCFG0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0
    }
    #[doc = "0x14 - MIITXCFG1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1
    }
    #[doc = "0x20 - MIITXCRC0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0
    }
    #[doc = "0x24 - MIITXCRC1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1
    }
    #[doc = "0x30 - MIITXIPG0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0
    }
    #[doc = "0x34 - MIITXIPG1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1
    }
    #[doc = "0x38 - MIIPortStatus0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0(&self) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0
    }
    #[doc = "0x3c - MIIPortStatus1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1(&self) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1
    }
    #[doc = "0x40 - MIIRXFRMS0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0
    }
    #[doc = "0x44 - MIIRXFRMS1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1
    }
    #[doc = "0x48 - MIIRXPCNT0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0
    }
    #[doc = "0x4c - MIIRXPCNT1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1
    }
    #[doc = "0x50 - MIIRXERR0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0
    }
    #[doc = "0x54 - MIIRXERR1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1
    }
    #[doc = "0x60 - MIIRXFIFOLEVEL0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0
    }
    #[doc = "0x64 - MIIRXFIFOLEVEL1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1
    }
    #[doc = "0x68 - MIIRXFIFOLEVEL0Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel0 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0
    }
    #[doc = "0x6c - MIIRXFIFOLEVEL1Register"]
    #[inline(always)]
    pub const fn pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1(
        &self,
    ) -> &Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel1 {
        &self.pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg0 (rw) register accessor: MIIRXCFG0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0::Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg0Spec>;
#[doc = "MIIRXCFG0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg1 (rw) register accessor: MIIRXCFG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rxcfg1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1::Pr1MiiRt_Pr1MiiRtCfg_RegsRxcfg1Spec>;
#[doc = "MIIRXCFG1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rxcfg1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg0 (rw) register accessor: MIITXCFG0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0::Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg0Spec>;
#[doc = "MIITXCFG0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg1 (rw) register accessor: MIITXCFG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_txcfg1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1::Pr1MiiRt_Pr1MiiRtCfg_RegsTxcfg1Spec>;
#[doc = "MIITXCFG1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_txcfg1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc0 (rw) register accessor: MIITXCRC0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0::Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc0Spec>;
#[doc = "MIITXCRC0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc1 (rw) register accessor: MIITXCRC1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_crc1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1::Pr1MiiRt_Pr1MiiRtCfg_RegsTxCrc1Spec>;
#[doc = "MIITXCRC1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_crc1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg0 (rw) register accessor: MIITXIPG0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0::Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg0Spec>;
#[doc = "MIITXIPG0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg1 (rw) register accessor: MIITXIPG1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_ipg1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1::Pr1MiiRt_Pr1MiiRtCfg_RegsTxIpg1Spec>;
#[doc = "MIITXIPG1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_ipg1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs0 (rw) register accessor: MIIPortStatus0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec>;
#[doc = "MIIPortStatus0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs1 (rw) register accessor: MIIPortStatus1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec>;
#[doc = "MIIPortStatus1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms0 (rw) register accessor: MIIRXFRMS0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0::Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms0Spec>;
#[doc = "MIIRXFRMS0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms1 (rw) register accessor: MIIRXFRMS1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_frms1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1::Pr1MiiRt_Pr1MiiRtCfg_RegsRxFrms1Spec>;
#[doc = "MIIRXFRMS1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_frms1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt0 (rw) register accessor: MIIRXPCNT0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0::Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt0Spec>;
#[doc = "MIIRXPCNT0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt1 (rw) register accessor: MIIRXPCNT1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_pcnt1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1::Pr1MiiRt_Pr1MiiRtCfg_RegsRxPcnt1Spec>;
#[doc = "MIIRXPCNT1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_pcnt1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err0 (rw) register accessor: MIIRXERR0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0::Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr0Spec>;
#[doc = "MIIRXERR0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err1 (rw) register accessor: MIIRXERR1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_err1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1 =
    crate::Reg<pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1::Pr1MiiRt_Pr1MiiRtCfg_RegsRxErr1Spec>;
#[doc = "MIIRXERR1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_err1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level0 (rw) register accessor: MIIRXFIFOLEVEL0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0 = crate::Reg<
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0::Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel0Spec,
>;
#[doc = "MIIRXFIFOLEVEL0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level1 (rw) register accessor: MIIRXFIFOLEVEL1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_rx_fifo_level1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel1 = crate::Reg<
    pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1::Pr1MiiRt_Pr1MiiRtCfg_RegsRxFifoLevel1Spec,
>;
#[doc = "MIIRXFIFOLEVEL1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_rx_fifo_level1;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_fifo_level0 (rw) register accessor: MIIRXFIFOLEVEL0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_fifo_level0")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel0 = crate::Reg<
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0::Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel0Spec,
>;
#[doc = "MIIRXFIFOLEVEL0Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level0;
#[doc = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_fifo_level1 (rw) register accessor: MIIRXFIFOLEVEL1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1`]
module"]
#[doc(alias = "PR1_MII_RT__PR1_MII_RT_CFG__REGS_tx_fifo_level1")]
pub type Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel1 = crate::Reg<
    pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1::Pr1MiiRt_Pr1MiiRtCfg_RegsTxFifoLevel1Spec,
>;
#[doc = "MIIRXFIFOLEVEL1Register"]
pub mod pr1_mii_rt__pr1_mii_rt_cfg__regs_tx_fifo_level1;
