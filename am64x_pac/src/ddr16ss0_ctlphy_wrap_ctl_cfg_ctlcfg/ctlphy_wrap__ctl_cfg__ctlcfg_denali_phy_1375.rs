#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1375` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1375` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec>;
#[doc = "Field `PHY_PAD_VREF_CTRL_AC` reader - 11:0\\]
Pad VREF control settings for the address/control."]
pub type PhyPadVrefCtrlAcR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_VREF_CTRL_AC` writer - 11:0\\]
Pad VREF control settings for the address/control."]
pub type PhyPadVrefCtrlAcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_CSLVL_CAPTURE_CNT` reader - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training."]
pub type PhyCslvlCaptureCntR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_CAPTURE_CNT` writer - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training."]
pub type PhyCslvlCaptureCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_CSLVL_DLY_STEP` reader - 27:24\\]
Sets the delay step size plus 1 during CS training."]
pub type PhyCslvlDlyStepR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_DLY_STEP` writer - 27:24\\]
Sets the delay step size plus 1 during CS training."]
pub type PhyCslvlDlyStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Pad VREF control settings for the address/control."]
    #[inline(always)]
    pub fn phy_pad_vref_ctrl_ac(&self) -> PhyPadVrefCtrlAcR {
        PhyPadVrefCtrlAcR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training."]
    #[inline(always)]
    pub fn phy_cslvl_capture_cnt(&self) -> PhyCslvlCaptureCntR {
        PhyCslvlCaptureCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Sets the delay step size plus 1 during CS training."]
    #[inline(always)]
    pub fn phy_cslvl_dly_step(&self) -> PhyCslvlDlyStepR {
        PhyCslvlDlyStepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Pad VREF control settings for the address/control."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_vref_ctrl_ac(
        &mut self,
    ) -> PhyPadVrefCtrlAcW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec> {
        PhyPadVrefCtrlAcW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the number of samples to take at each GRP slave delay setting during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_capture_cnt(
        &mut self,
    ) -> PhyCslvlCaptureCntW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec> {
        PhyCslvlCaptureCntW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Sets the delay step size plus 1 during CS training."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_dly_step(
        &mut self,
    ) -> PhyCslvlDlyStepW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec> {
        PhyCslvlDlyStepW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1375\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1375::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1375::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1375::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1375::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1375 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1375Spec {
    const RESET_VALUE: u32 = 0;
}
