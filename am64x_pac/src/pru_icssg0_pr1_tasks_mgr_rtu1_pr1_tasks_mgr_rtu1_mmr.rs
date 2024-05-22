#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalCfg,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalStatus,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS0,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS1,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS2,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS3,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS4,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS1,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS2,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS3,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS4,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsRxCfg,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTxCfg,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg1,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg2,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg1,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg2,
    pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg:
        Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsCapEnCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Configuration"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalCfg {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg
    }
    #[doc = "0x04 - Global Status"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalStatus {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status
    }
    #[doc = "0x08 - TS1 Sub0 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS0 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0
    }
    #[doc = "0x0c - TS1 Sub1 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS1 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1
    }
    #[doc = "0x10 - TS1 Sub2 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS2 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2
    }
    #[doc = "0x14 - TS1 Sub3 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS3 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3
    }
    #[doc = "0x18 - TS1 Sub4 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS4 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4
    }
    #[doc = "0x1c - TS2 Sub0 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0
    }
    #[doc = "0x20 - TS2 Sub1 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS1 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1
    }
    #[doc = "0x24 - TS2 Sub2 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS2 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2
    }
    #[doc = "0x28 - TS2 Sub3 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS3 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3
    }
    #[doc = "0x2c - TS2 Sub4 PC"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS4 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4
    }
    #[doc = "0x30 - RX Configuration"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsRxCfg {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg
    }
    #[doc = "0x34 - TX Configuration"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTxCfg {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg
    }
    #[doc = "0x38 - Generic TS1 Configuration1"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg1 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1
    }
    #[doc = "0x3c - Generic TS1 Configuration2"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg2 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2
    }
    #[doc = "0x40 - Generic TS2 Configuration1"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg1 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1
    }
    #[doc = "0x44 - Generic TS2 Configuration2"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg2 {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2
    }
    #[doc = "0x48 - Enable capture new event cfg"]
    #[inline(always)]
    pub const fn pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg(
        &self,
    ) -> &Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsCapEnCfg {
        &self.pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg
    }
}
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_global_cfg (rw) register accessor: Global Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_global_cfg")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalCfg = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalCfgSpec > ;
#[doc = "Global Configuration"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_cfg;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_global_status (rw) register accessor: Global Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_global_status")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalStatus = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsGlobalStatusSpec > ;
#[doc = "Global Status"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_global_status;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s0 (rw) register accessor: TS1 Sub0 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s0")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS0 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS0Spec > ;
#[doc = "TS1 Sub0 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s0;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s1 (rw) register accessor: TS1 Sub1 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s1")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS1 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS1Spec > ;
#[doc = "TS1 Sub1 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s1;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s2 (rw) register accessor: TS1 Sub2 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s2")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS2 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS2Spec > ;
#[doc = "TS1 Sub2 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s2;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s3 (rw) register accessor: TS1 Sub3 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s3")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS3 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS3Spec > ;
#[doc = "TS1 Sub3 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s3;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s4 (rw) register accessor: TS1 Sub4 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_pc_s4")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS4 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1PcS4Spec > ;
#[doc = "TS1 Sub4 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_pc_s4;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s0 (rw) register accessor: TS2 Sub0 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s0")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS0Spec > ;
#[doc = "TS2 Sub0 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s0;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s1 (rw) register accessor: TS2 Sub1 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s1")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS1 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS1Spec > ;
#[doc = "TS2 Sub1 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s1;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s2 (rw) register accessor: TS2 Sub2 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s2")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS2 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS2Spec > ;
#[doc = "TS2 Sub2 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s2;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s3 (rw) register accessor: TS2 Sub3 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s3")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS3 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS3Spec > ;
#[doc = "TS2 Sub3 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s3;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s4 (rw) register accessor: TS2 Sub4 PC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_pc_s4")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS4 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2PcS4Spec > ;
#[doc = "TS2 Sub4 PC"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_pc_s4;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_rx_cfg (rw) register accessor: RX Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_rx_cfg")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsRxCfg = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsRxCfgSpec > ;
#[doc = "RX Configuration"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_rx_cfg;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_tx_cfg (rw) register accessor: TX Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_tx_cfg")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTxCfg = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTxCfgSpec > ;
#[doc = "TX Configuration"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_tx_cfg;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_gen_cfg1 (rw) register accessor: Generic TS1 Configuration1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_gen_cfg1")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg1 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg1Spec > ;
#[doc = "Generic TS1 Configuration1"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg1;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_gen_cfg2 (rw) register accessor: Generic TS1 Configuration2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts1_gen_cfg2")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg2 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs1GenCfg2Spec > ;
#[doc = "Generic TS1 Configuration2"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts1_gen_cfg2;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_gen_cfg1 (rw) register accessor: Generic TS2 Configuration1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_gen_cfg1")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg1 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg1Spec > ;
#[doc = "Generic TS2 Configuration1"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg1;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_gen_cfg2 (rw) register accessor: Generic TS2 Configuration2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_ts2_gen_cfg2")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg2 = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2 :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsTs2GenCfg2Spec > ;
#[doc = "Generic TS2 Configuration2"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_ts2_gen_cfg2;
#[doc = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_cap_en_cfg (rw) register accessor: Enable capture new event cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg`]
module"]
#[doc(alias = "PR1_TASKS_MGR_RTU1__PR1_TASKS_MGR_RTU1_MMR__REGS_cap_en_cfg")]
pub type Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsCapEnCfg = crate :: Reg < pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg :: Pr1TasksMgrRtu1_Pr1TasksMgrRtu1Mmr_RegsCapEnCfgSpec > ;
#[doc = "Enable capture new event cfg"]
pub mod pr1_tasks_mgr_rtu1__pr1_tasks_mgr_rtu1_mmr__regs_cap_en_cfg;
