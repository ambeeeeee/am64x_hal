#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1376` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1376` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec>;
#[doc = "Field `PHY_SW_CSLVL_DVW_MIN` reader - 9:0\\]
Sets the software override data valid window size during CS training."]
pub type PhySwCslvlDvwMinR = crate::FieldReader<u16>;
#[doc = "Field `PHY_SW_CSLVL_DVW_MIN` writer - 9:0\\]
Sets the software override data valid window size during CS training."]
pub type PhySwCslvlDvwMinW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_SW_CSLVL_DVW_MIN_EN` reader - 16:16\\]
Enables the software override data valid window size during CS training."]
pub type PhySwCslvlDvwMinEnR = crate::BitReader;
#[doc = "Field `PHY_SW_CSLVL_DVW_MIN_EN` writer - 16:16\\]
Enables the software override data valid window size during CS training."]
pub type PhySwCslvlDvwMinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LVL_MEAS_DLY_STEP_ENABLE` reader - 24:24\\]
Enables the phy_adr_meas_dly_step_value to be used instead of the phy_cslvl_dly_step parameter."]
pub type PhyLvlMeasDlyStepEnableR = crate::BitReader;
#[doc = "Field `PHY_LVL_MEAS_DLY_STEP_ENABLE` writer - 24:24\\]
Enables the phy_adr_meas_dly_step_value to be used instead of the phy_cslvl_dly_step parameter."]
pub type PhyLvlMeasDlyStepEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Sets the software override data valid window size during CS training."]
    #[inline(always)]
    pub fn phy_sw_cslvl_dvw_min(&self) -> PhySwCslvlDvwMinR {
        PhySwCslvlDvwMinR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the software override data valid window size during CS training."]
    #[inline(always)]
    pub fn phy_sw_cslvl_dvw_min_en(&self) -> PhySwCslvlDvwMinEnR {
        PhySwCslvlDvwMinEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the phy_adr_meas_dly_step_value to be used instead of the phy_cslvl_dly_step parameter."]
    #[inline(always)]
    pub fn phy_lvl_meas_dly_step_enable(&self) -> PhyLvlMeasDlyStepEnableR {
        PhyLvlMeasDlyStepEnableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Sets the software override data valid window size during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_cslvl_dvw_min(
        &mut self,
    ) -> PhySwCslvlDvwMinW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec> {
        PhySwCslvlDvwMinW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables the software override data valid window size during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_cslvl_dvw_min_en(
        &mut self,
    ) -> PhySwCslvlDvwMinEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec> {
        PhySwCslvlDvwMinEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the phy_adr_meas_dly_step_value to be used instead of the phy_cslvl_dly_step parameter."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_meas_dly_step_enable(
        &mut self,
    ) -> PhyLvlMeasDlyStepEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec> {
        PhyLvlMeasDlyStepEnableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1376\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1376::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1376::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1376::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1376::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1376 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1376Spec {
    const RESET_VALUE: u32 = 0;
}
