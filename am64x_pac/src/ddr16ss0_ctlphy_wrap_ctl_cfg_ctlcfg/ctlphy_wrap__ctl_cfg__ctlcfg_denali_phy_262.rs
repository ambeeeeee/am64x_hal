#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_262` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_262` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_1` reader - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyLp4BootRddataEnTselDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_1` writer - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyLp4BootRddataEnTselDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_1` reader - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 1."]
pub type PhyLp4BootRptrUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_1` writer - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 1."]
pub type PhyLp4BootRptrUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_1` reader - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyLp4BootRddqsLatencyAdjust1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_1` writer - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyLp4BootRddqsLatencyAdjust1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_WRPATH_GATE_DISABLE_1` reader - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 1. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyLp4BootWrpathGateDisable1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_WRPATH_GATE_DISABLE_1` writer - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 1. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
pub type PhyLp4BootWrpathGateDisable1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_1(&self) -> PhyLp4BootRddataEnTselDly1R {
        PhyLp4BootRddataEnTselDly1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_1(&self) -> PhyLp4BootRptrUpdate1R {
        PhyLp4BootRptrUpdate1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_1(&self) -> PhyLp4BootRddqsLatencyAdjust1R {
        PhyLp4BootRddqsLatencyAdjust1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 1. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    pub fn phy_lp4_boot_wrpath_gate_disable_1(&self) -> PhyLp4BootWrpathGateDisable1R {
        PhyLp4BootWrpathGateDisable1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec> {
        PhyLp4BootRddataEnTselDly1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to releasing data from the entry FIFO for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_1(
        &mut self,
    ) -> PhyLp4BootRptrUpdate1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec> {
        PhyLp4BootRptrUpdate1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_1(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec> {
        PhyLp4BootRddqsLatencyAdjust1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
For LPDDR4 boot frequency, write path clock gating disable for slice 1. Bit \\[0\\]: disable pull in wrdata_en; Bit \\[1\\]: disable write path clock gating, clock always on"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_wrpath_gate_disable_1(
        &mut self,
    ) -> PhyLp4BootWrpathGateDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec> {
        PhyLp4BootWrpathGateDisable1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_262\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_262::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_262::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_262::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_262::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_262 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy262Spec {
    const RESET_VALUE: u32 = 0;
}
