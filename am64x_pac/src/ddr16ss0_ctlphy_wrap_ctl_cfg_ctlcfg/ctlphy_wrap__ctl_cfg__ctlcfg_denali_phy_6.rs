#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_6` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_6` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_0` reader - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyLp4BootRddataEnTselDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_0` writer - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyLp4BootRddataEnTselDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_0` reader - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 0."]
pub type PhyLp4BootRptrUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_0` writer - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 0."]
pub type PhyLp4BootRptrUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_0` reader - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyLp4BootRddqsLatencyAdjust0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_0` writer - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyLp4BootRddqsLatencyAdjust0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_WRPATH_GATE_DISABLE_0` reader - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 0. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyLp4BootWrpathGateDisable0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_WRPATH_GATE_DISABLE_0` writer - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 0. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyLp4BootWrpathGateDisable0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_0(&self) -> PhyLp4BootRddataEnTselDly0R {
        PhyLp4BootRddataEnTselDly0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_0(&self) -> PhyLp4BootRptrUpdate0R {
        PhyLp4BootRptrUpdate0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_0(&self) -> PhyLp4BootRddqsLatencyAdjust0R {
        PhyLp4BootRddqsLatencyAdjust0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 0. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    pub fn phy_lp4_boot_wrpath_gate_disable_0(&self) -> PhyLp4BootWrpathGateDisable0R {
        PhyLp4BootWrpathGateDisable0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec> {
        PhyLp4BootRddataEnTselDly0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_0(
        &mut self,
    ) -> PhyLp4BootRptrUpdate0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec> {
        PhyLp4BootRptrUpdate0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_0(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec> {
        PhyLp4BootRddqsLatencyAdjust0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 0. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_wrpath_gate_disable_0(
        &mut self,
    ) -> PhyLp4BootWrpathGateDisable0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec> {
        PhyLp4BootWrpathGateDisable0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_6::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_6::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_6 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy6Spec {
    const RESET_VALUE: u32 = 0;
}
