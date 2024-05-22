#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_285` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_285` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_1` reader - 0:0\\]
Enables leveling debug mode for slice 1. Set to 1 to enable."]
pub type PhyLvlDebugMode1R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_1` writer - 0:0\\]
Enables leveling debug mode for slice 1. Set to 1 to enable."]
pub type PhyLvlDebugMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_1` reader - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont1R = crate::BitReader;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_1` writer - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 1. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_ALGO_1` reader - 17:16\\]
Write leveling algorithm selection for slice 1."]
pub type PhyWrlvlAlgo1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_ALGO_1` writer - 17:16\\]
Write leveling algorithm selection for slice 1."]
pub type PhyWrlvlAlgo1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_1` reader - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_1` writer - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables leveling debug mode for slice 1. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_1(&self) -> PhyLvlDebugMode1R {
        PhyLvlDebugMode1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_lvl_debug_cont_1(&self) -> ScPhyLvlDebugCont1R {
        ScPhyLvlDebugCont1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write leveling algorithm selection for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_algo_1(&self) -> PhyWrlvlAlgo1R {
        PhyWrlvlAlgo1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_1(&self) -> PhyWrlvlCaptureCnt1R {
        PhyWrlvlCaptureCnt1R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables leveling debug mode for slice 1. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_1(
        &mut self,
    ) -> PhyLvlDebugMode1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec> {
        PhyLvlDebugMode1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 1. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_1(
        &mut self,
    ) -> ScPhyLvlDebugCont1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec> {
        ScPhyLvlDebugCont1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write leveling algorithm selection for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_algo_1(&mut self) -> PhyWrlvlAlgo1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec> {
        PhyWrlvlAlgo1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_1(
        &mut self,
    ) -> PhyWrlvlCaptureCnt1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec> {
        PhyWrlvlCaptureCnt1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_285\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_285::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_285::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_285::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_285::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_285 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy285Spec {
    const RESET_VALUE: u32 = 0;
}
