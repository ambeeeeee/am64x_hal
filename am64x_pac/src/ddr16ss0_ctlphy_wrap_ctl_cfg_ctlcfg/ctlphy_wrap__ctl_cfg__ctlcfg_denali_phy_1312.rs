#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1312` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1312` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec>;
#[doc = "Field `PHY_USE_PLL_DSKEWCALLOCK` reader - 0:0\\]
Use DSKEWCALLOCK or not."]
pub type PhyUsePllDskewcallockR = crate::BitReader;
#[doc = "Field `PHY_USE_PLL_DSKEWCALLOCK` writer - 0:0\\]
Use DSKEWCALLOCK or not."]
pub type PhyUsePllDskewcallockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_PLL_SPO_CAL_CTRL` reader - 15:8\\]
PLL SPO Cal controls."]
pub type PhyPllSpoCalCtrlR = crate::FieldReader;
#[doc = "Field `PHY_PLL_SPO_CAL_CTRL` writer - 15:8\\]
PLL SPO Cal controls."]
pub type PhyPllSpoCalCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SC_PHY_PLL_SPO_CAL_SNAP_OBS` reader - 17:16\\]
Register command to take a snapshot of PLL output. WRITE-ONLY"]
pub type ScPhyPllSpoCalSnapObsR = crate::FieldReader;
#[doc = "Field `SC_PHY_PLL_SPO_CAL_SNAP_OBS` writer - 17:16\\]
Register command to take a snapshot of PLL output. WRITE-ONLY"]
pub type ScPhyPllSpoCalSnapObsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Use DSKEWCALLOCK or not."]
    #[inline(always)]
    pub fn phy_use_pll_dskewcallock(&self) -> PhyUsePllDskewcallockR {
        PhyUsePllDskewcallockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PLL SPO Cal controls."]
    #[inline(always)]
    pub fn phy_pll_spo_cal_ctrl(&self) -> PhyPllSpoCalCtrlR {
        PhyPllSpoCalCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Register command to take a snapshot of PLL output. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_pll_spo_cal_snap_obs(&self) -> ScPhyPllSpoCalSnapObsR {
        ScPhyPllSpoCalSnapObsR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Use DSKEWCALLOCK or not."]
    #[inline(always)]
    #[must_use]
    pub fn phy_use_pll_dskewcallock(
        &mut self,
    ) -> PhyUsePllDskewcallockW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec> {
        PhyUsePllDskewcallockW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
PLL SPO Cal controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pll_spo_cal_ctrl(
        &mut self,
    ) -> PhyPllSpoCalCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec> {
        PhyPllSpoCalCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Register command to take a snapshot of PLL output. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_pll_spo_cal_snap_obs(
        &mut self,
    ) -> ScPhyPllSpoCalSnapObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec> {
        ScPhyPllSpoCalSnapObsW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1312\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1312::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1312::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1312::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1312 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1312Spec {
    const RESET_VALUE: u32 = 0;
}
