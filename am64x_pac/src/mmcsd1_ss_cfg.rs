#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs__ss_cfg__sscfg_ss_id_rev_reg: Regs_SsCfg_SscfgSsIdRevReg,
    _reserved1: [u8; 0x0c],
    regs__ss_cfg__sscfg_ctl_cfg_1_reg: Regs_SsCfg_SscfgCtlCfg1Reg,
    regs__ss_cfg__sscfg_ctl_cfg_2_reg: Regs_SsCfg_SscfgCtlCfg2Reg,
    regs__ss_cfg__sscfg_ctl_cfg_3_reg: Regs_SsCfg_SscfgCtlCfg3Reg,
    regs__ss_cfg__sscfg_ctl_cfg_4_reg: Regs_SsCfg_SscfgCtlCfg4Reg,
    regs__ss_cfg__sscfg_ctl_cfg_5_reg: Regs_SsCfg_SscfgCtlCfg5Reg,
    regs__ss_cfg__sscfg_ctl_cfg_6_reg: Regs_SsCfg_SscfgCtlCfg6Reg,
    regs__ss_cfg__sscfg_ctl_cfg_7_reg: Regs_SsCfg_SscfgCtlCfg7Reg,
    regs__ss_cfg__sscfg_ctl_cfg_8_reg: Regs_SsCfg_SscfgCtlCfg8Reg,
    regs__ss_cfg__sscfg_ctl_cfg_9_reg: Regs_SsCfg_SscfgCtlCfg9Reg,
    regs__ss_cfg__sscfg_ctl_cfg_10_reg: Regs_SsCfg_SscfgCtlCfg10Reg,
    regs__ss_cfg__sscfg_ctl_cfg_11_reg: Regs_SsCfg_SscfgCtlCfg11Reg,
    regs__ss_cfg__sscfg_ctl_cfg_12_reg: Regs_SsCfg_SscfgCtlCfg12Reg,
    regs__ss_cfg__sscfg_ctl_cfg_13_reg: Regs_SsCfg_SscfgCtlCfg13Reg,
    _reserved14: [u8; 0x1c],
    regs__ss_cfg__sscfg_ctl_stat_1_reg: Regs_SsCfg_SscfgCtlStat1Reg,
    regs__ss_cfg__sscfg_ctl_stat_2_reg: Regs_SsCfg_SscfgCtlStat2Reg,
    regs__ss_cfg__sscfg_ctl_stat_3_reg: Regs_SsCfg_SscfgCtlStat3Reg,
    regs__ss_cfg__sscfg_ctl_stat_4_reg: Regs_SsCfg_SscfgCtlStat4Reg,
    regs__ss_cfg__sscfg_ctl_stat_5_reg: Regs_SsCfg_SscfgCtlStat5Reg,
    regs__ss_cfg__sscfg_ctl_stat_6_reg: Regs_SsCfg_SscfgCtlStat6Reg,
    _reserved20: [u8; 0x88],
    regs__ss_cfg__sscfg_phy_ctrl_1_reg: Regs_SsCfg_SscfgPhyCtrl1Reg,
    regs__ss_cfg__sscfg_phy_ctrl_2_reg: Regs_SsCfg_SscfgPhyCtrl2Reg,
    regs__ss_cfg__sscfg_phy_ctrl_3_reg: Regs_SsCfg_SscfgPhyCtrl3Reg,
    regs__ss_cfg__sscfg_phy_ctrl_4_reg: Regs_SsCfg_SscfgPhyCtrl4Reg,
    regs__ss_cfg__sscfg_phy_ctrl_5_reg: Regs_SsCfg_SscfgPhyCtrl5Reg,
    regs__ss_cfg__sscfg_phy_ctrl_6_reg: Regs_SsCfg_SscfgPhyCtrl6Reg,
    _reserved26: [u8; 0x18],
    regs__ss_cfg__sscfg_phy_stat_1_reg: Regs_SsCfg_SscfgPhyStat1Reg,
    regs__ss_cfg__sscfg_phy_stat_2_reg: Regs_SsCfg_SscfgPhyStat2Reg,
}
impl RegisterBlock {
    #[doc = "0x00 - The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem"]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ss_id_rev_reg(&self) -> &Regs_SsCfg_SscfgSsIdRevReg {
        &self.regs__ss_cfg__sscfg_ss_id_rev_reg
    }
    #[doc = "0x10 - The Controller Config 1 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_1_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg1Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_1_reg
    }
    #[doc = "0x14 - The Controller Config 2 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Capabilities Register inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_2_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg2Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_2_reg
    }
    #[doc = "0x18 - The Controller Config 3 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Capabilities Register inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_3_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg3Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_3_reg
    }
    #[doc = "0x1c - The Controller Config 4 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_4_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg4Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_4_reg
    }
    #[doc = "0x20 - The Controller Config 5 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_5_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg5Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_5_reg
    }
    #[doc = "0x24 - The Controller Config 6 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Initialization inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_6_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg6Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_6_reg
    }
    #[doc = "0x28 - The Controller Config 7 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Default Speed inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_7_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg7Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_7_reg
    }
    #[doc = "0x2c - The Controller Config 8 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for High Speed inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_8_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg8Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_8_reg
    }
    #[doc = "0x30 - The Controller Config 9 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR12 inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_9_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg9Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_9_reg
    }
    #[doc = "0x34 - The Controller Config 10 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR25 inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_10_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg10Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_10_reg
    }
    #[doc = "0x38 - The Controller Config 11 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR50 inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_11_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg11Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_11_reg
    }
    #[doc = "0x3c - The Controller Config 12 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR104 inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_12_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg12Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_12_reg
    }
    #[doc = "0x40 - The Controller Config 13 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for DDR50 inside the Arasan eMMC/SD Controller."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_cfg_13_reg(&self) -> &Regs_SsCfg_SscfgCtlCfg13Reg {
        &self.regs__ss_cfg__sscfg_ctl_cfg_13_reg
    }
    #[doc = "0x60 - The Controller Status 1 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_1_reg(&self) -> &Regs_SsCfg_SscfgCtlStat1Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_1_reg
    }
    #[doc = "0x64 - The Controller Status 2 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_2_reg(&self) -> &Regs_SsCfg_SscfgCtlStat2Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_2_reg
    }
    #[doc = "0x68 - The Controller Status 3 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_3_reg(&self) -> &Regs_SsCfg_SscfgCtlStat3Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_3_reg
    }
    #[doc = "0x6c - The Controller Status 4 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_4_reg(&self) -> &Regs_SsCfg_SscfgCtlStat4Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_4_reg
    }
    #[doc = "0x70 - The Controller Status 5 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_5_reg(&self) -> &Regs_SsCfg_SscfgCtlStat5Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_5_reg
    }
    #[doc = "0x74 - The Controller Status 6 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_ctl_stat_6_reg(&self) -> &Regs_SsCfg_SscfgCtlStat6Reg {
        &self.regs__ss_cfg__sscfg_ctl_stat_6_reg
    }
    #[doc = "0x100 - The PHY Control 1 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_1_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl1Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_1_reg
    }
    #[doc = "0x104 - The PHY Control 2 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_2_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl2Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_2_reg
    }
    #[doc = "0x108 - The PHY Control 3 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_3_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl3Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_3_reg
    }
    #[doc = "0x10c - The PHY Control 4 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_4_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl4Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_4_reg
    }
    #[doc = "0x110 - The PHY Control 5 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_5_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl5Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_5_reg
    }
    #[doc = "0x114 - The PHY Control 6 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_ctrl_6_reg(&self) -> &Regs_SsCfg_SscfgPhyCtrl6Reg {
        &self.regs__ss_cfg__sscfg_phy_ctrl_6_reg
    }
    #[doc = "0x130 - The PHY Status 1 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_stat_1_reg(&self) -> &Regs_SsCfg_SscfgPhyStat1Reg {
        &self.regs__ss_cfg__sscfg_phy_stat_1_reg
    }
    #[doc = "0x134 - The PHY Status 2 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2."]
    #[inline(always)]
    pub const fn regs__ss_cfg__sscfg_phy_stat_2_reg(&self) -> &Regs_SsCfg_SscfgPhyStat2Reg {
        &self.regs__ss_cfg__sscfg_phy_stat_2_reg
    }
}
#[doc = "REGS__SS_CFG__SSCFG_SS_ID_REV_REG (rw) register accessor: The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ss_id_rev_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ss_id_rev_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ss_id_rev_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_SS_ID_REV_REG")]
pub type Regs_SsCfg_SscfgSsIdRevReg =
    crate::Reg<regs__ss_cfg__sscfg_ss_id_rev_reg::Regs_SsCfg_SscfgSsIdRevRegSpec>;
#[doc = "The Subsystem ID and Revision Register contains the module ID, major, and minor revisions for the subsystem"]
pub mod regs__ss_cfg__sscfg_ss_id_rev_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_1_REG (rw) register accessor: The Controller Config 1 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_1_REG")]
pub type Regs_SsCfg_SscfgCtlCfg1Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_1_reg::Regs_SsCfg_SscfgCtlCfg1RegSpec>;
#[doc = "The Controller Config 1 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_1_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_2_REG (rw) register accessor: The Controller Config 2 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_2_REG")]
pub type Regs_SsCfg_SscfgCtlCfg2Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_2_reg::Regs_SsCfg_SscfgCtlCfg2RegSpec>;
#[doc = "The Controller Config 2 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Capabilities Register inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_2_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_3_REG (rw) register accessor: The Controller Config 3 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_3_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_3_REG")]
pub type Regs_SsCfg_SscfgCtlCfg3Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_3_reg::Regs_SsCfg_SscfgCtlCfg3RegSpec>;
#[doc = "The Controller Config 3 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Capabilities Register inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_3_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_4_REG (rw) register accessor: The Controller Config 4 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_4_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_4_REG")]
pub type Regs_SsCfg_SscfgCtlCfg4Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_4_reg::Regs_SsCfg_SscfgCtlCfg4RegSpec>;
#[doc = "The Controller Config 4 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_4_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_5_REG (rw) register accessor: The Controller Config 5 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_5_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_5_REG")]
pub type Regs_SsCfg_SscfgCtlCfg5Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_5_reg::Regs_SsCfg_SscfgCtlCfg5RegSpec>;
#[doc = "The Controller Config 5 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_5_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_6_REG (rw) register accessor: The Controller Config 6 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Initialization inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_6_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_6_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_6_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_6_REG")]
pub type Regs_SsCfg_SscfgCtlCfg6Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_6_reg::Regs_SsCfg_SscfgCtlCfg6RegSpec>;
#[doc = "The Controller Config 6 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Initialization inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_6_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_7_REG (rw) register accessor: The Controller Config 7 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Default Speed inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_7_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_7_REG")]
pub type Regs_SsCfg_SscfgCtlCfg7Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_7_reg::Regs_SsCfg_SscfgCtlCfg7RegSpec>;
#[doc = "The Controller Config 7 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Default Speed inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_7_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_8_REG (rw) register accessor: The Controller Config 8 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for High Speed inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_8_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_8_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_8_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_8_REG")]
pub type Regs_SsCfg_SscfgCtlCfg8Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_8_reg::Regs_SsCfg_SscfgCtlCfg8RegSpec>;
#[doc = "The Controller Config 8 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for High Speed inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_8_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_9_REG (rw) register accessor: The Controller Config 9 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR12 inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_9_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_9_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_9_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_9_REG")]
pub type Regs_SsCfg_SscfgCtlCfg9Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_9_reg::Regs_SsCfg_SscfgCtlCfg9RegSpec>;
#[doc = "The Controller Config 9 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR12 inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_9_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_10_REG (rw) register accessor: The Controller Config 10 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR25 inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_10_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_10_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_10_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_10_REG")]
pub type Regs_SsCfg_SscfgCtlCfg10Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_10_reg::Regs_SsCfg_SscfgCtlCfg10RegSpec>;
#[doc = "The Controller Config 10 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR25 inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_10_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_11_REG (rw) register accessor: The Controller Config 11 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR50 inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_11_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_11_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_11_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_11_REG")]
pub type Regs_SsCfg_SscfgCtlCfg11Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_11_reg::Regs_SsCfg_SscfgCtlCfg11RegSpec>;
#[doc = "The Controller Config 11 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR50 inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_11_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_12_REG (rw) register accessor: The Controller Config 12 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR104 inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_12_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_12_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_12_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_12_REG")]
pub type Regs_SsCfg_SscfgCtlCfg12Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_12_reg::Regs_SsCfg_SscfgCtlCfg12RegSpec>;
#[doc = "The Controller Config 12 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for SDR104 inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_12_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_CFG_13_REG (rw) register accessor: The Controller Config 13 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for DDR50 inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_13_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_13_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_cfg_13_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_CFG_13_REG")]
pub type Regs_SsCfg_SscfgCtlCfg13Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_cfg_13_reg::Regs_SsCfg_SscfgCtlCfg13RegSpec>;
#[doc = "The Controller Config 13 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for DDR50 inside the Arasan eMMC/SD Controller."]
pub mod regs__ss_cfg__sscfg_ctl_cfg_13_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_1_REG (rw) register accessor: The Controller Status 1 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_1_REG")]
pub type Regs_SsCfg_SscfgCtlStat1Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_1_reg::Regs_SsCfg_SscfgCtlStat1RegSpec>;
#[doc = "The Controller Status 1 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_1_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_2_REG (rw) register accessor: The Controller Status 2 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_2_REG")]
pub type Regs_SsCfg_SscfgCtlStat2Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_2_reg::Regs_SsCfg_SscfgCtlStat2RegSpec>;
#[doc = "The Controller Status 2 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_2_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_3_REG (rw) register accessor: The Controller Status 3 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_3_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_3_REG")]
pub type Regs_SsCfg_SscfgCtlStat3Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_3_reg::Regs_SsCfg_SscfgCtlStat3RegSpec>;
#[doc = "The Controller Status 3 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_3_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_4_REG (rw) register accessor: The Controller Status 4 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_4_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_4_REG")]
pub type Regs_SsCfg_SscfgCtlStat4Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_4_reg::Regs_SsCfg_SscfgCtlStat4RegSpec>;
#[doc = "The Controller Status 4 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_4_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_5_REG (rw) register accessor: The Controller Status 5 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_5_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_5_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_5_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_5_REG")]
pub type Regs_SsCfg_SscfgCtlStat5Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_5_reg::Regs_SsCfg_SscfgCtlStat5RegSpec>;
#[doc = "The Controller Status 5 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_5_reg;
#[doc = "REGS__SS_CFG__SSCFG_CTL_STAT_6_REG (rw) register accessor: The Controller Status 6 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_6_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_6_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_ctl_stat_6_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_CTL_STAT_6_REG")]
pub type Regs_SsCfg_SscfgCtlStat6Reg =
    crate::Reg<regs__ss_cfg__sscfg_ctl_stat_6_reg::Regs_SsCfg_SscfgCtlStat6RegSpec>;
#[doc = "The Controller Status 6 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_ctl_stat_6_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG (rw) register accessor: The PHY Control 1 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl1Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_1_reg::Regs_SsCfg_SscfgPhyCtrl1RegSpec>;
#[doc = "The PHY Control 1 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_1_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_2_REG (rw) register accessor: The PHY Control 2 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_2_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl2Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_2_reg::Regs_SsCfg_SscfgPhyCtrl2RegSpec>;
#[doc = "The PHY Control 2 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_2_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_3_REG (rw) register accessor: The PHY Control 3 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_3_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_3_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl3Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_3_reg::Regs_SsCfg_SscfgPhyCtrl3RegSpec>;
#[doc = "The PHY Control 3 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_3_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_4_REG (rw) register accessor: The PHY Control 4 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_4_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_4_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl4Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_4_reg::Regs_SsCfg_SscfgPhyCtrl4RegSpec>;
#[doc = "The PHY Control 4 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_4_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_5_REG (rw) register accessor: The PHY Control 5 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_5_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_5_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl5Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_5_reg::Regs_SsCfg_SscfgPhyCtrl5RegSpec>;
#[doc = "The PHY Control 5 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_5_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_CTRL_6_REG (rw) register accessor: The PHY Control 6 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_ctrl_6_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_CTRL_6_REG")]
pub type Regs_SsCfg_SscfgPhyCtrl6Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_ctrl_6_reg::Regs_SsCfg_SscfgPhyCtrl6RegSpec>;
#[doc = "The PHY Control 6 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_ctrl_6_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_STAT_1_REG (rw) register accessor: The PHY Status 1 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_stat_1_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_stat_1_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_stat_1_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_STAT_1_REG")]
pub type Regs_SsCfg_SscfgPhyStat1Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_stat_1_reg::Regs_SsCfg_SscfgPhyStat1RegSpec>;
#[doc = "The PHY Status 1 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_stat_1_reg;
#[doc = "REGS__SS_CFG__SSCFG_PHY_STAT_2_REG (rw) register accessor: The PHY Status 2 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_stat_2_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_stat_2_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs__ss_cfg__sscfg_phy_stat_2_reg`]
module"]
#[doc(alias = "REGS__SS_CFG__SSCFG_PHY_STAT_2_REG")]
pub type Regs_SsCfg_SscfgPhyStat2Reg =
    crate::Reg<regs__ss_cfg__sscfg_phy_stat_2_reg::Regs_SsCfg_SscfgPhyStat2RegSpec>;
#[doc = "The PHY Status 2 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2."]
pub mod regs__ss_cfg__sscfg_phy_stat_2_reg;
