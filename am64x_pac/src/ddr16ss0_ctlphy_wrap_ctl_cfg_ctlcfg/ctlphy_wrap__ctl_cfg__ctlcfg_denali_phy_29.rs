#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_29` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_29` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_0` reader - 0:0\\]
Enables leveling debug mode for slice 0. Set to 1 to enable."]
pub type PhyLvlDebugMode0R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_0` writer - 0:0\\]
Enables leveling debug mode for slice 0. Set to 1 to enable."]
pub type PhyLvlDebugMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_0` reader - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont0R = crate::BitReader;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_0` writer - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_ALGO_0` reader - 17:16\\]
Write leveling algorithm selection for slice 0."]
pub type PhyWrlvlAlgo0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_ALGO_0` writer - 17:16\\]
Write leveling algorithm selection for slice 0."]
pub type PhyWrlvlAlgo0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_0` reader - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_0` writer - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables leveling debug mode for slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_0(&self) -> PhyLvlDebugMode0R {
        PhyLvlDebugMode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_lvl_debug_cont_0(&self) -> ScPhyLvlDebugCont0R {
        ScPhyLvlDebugCont0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write leveling algorithm selection for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_algo_0(&self) -> PhyWrlvlAlgo0R {
        PhyWrlvlAlgo0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_0(&self) -> PhyWrlvlCaptureCnt0R {
        PhyWrlvlCaptureCnt0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables leveling debug mode for slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_0(
        &mut self,
    ) -> PhyLvlDebugMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec> {
        PhyLvlDebugMode0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows the leveling state machine to advance \\[when in debug mode\\]
for slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_0(
        &mut self,
    ) -> ScPhyLvlDebugCont0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec> {
        ScPhyLvlDebugCont0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Write leveling algorithm selection for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_algo_0(&mut self) -> PhyWrlvlAlgo0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec> {
        PhyWrlvlAlgo0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_0(
        &mut self,
    ) -> PhyWrlvlCaptureCnt0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec> {
        PhyWrlvlCaptureCnt0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_29::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_29::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_29 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy29Spec {
    const RESET_VALUE: u32 = 0;
}
