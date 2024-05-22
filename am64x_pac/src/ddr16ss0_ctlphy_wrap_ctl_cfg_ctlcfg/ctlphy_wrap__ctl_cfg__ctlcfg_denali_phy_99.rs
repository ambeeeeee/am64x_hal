#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_99` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_99` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec>;
#[doc = "Field `PHY_TOGGLE_PRE_SUPPORT_0` reader - 0:0\\]
Support the toggle read preamble for LPDDR4 for slice 0."]
pub type PhyTogglePreSupport0R = crate::BitReader;
#[doc = "Field `PHY_TOGGLE_PRE_SUPPORT_0` writer - 0:0\\]
Support the toggle read preamble for LPDDR4 for slice 0."]
pub type PhyTogglePreSupport0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RDLVL_DLY_STEP_0` reader - 11:8\\]
DQS slave delay step size during read leveling for slice 0."]
pub type PhyRdlvlDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_RDLVL_DLY_STEP_0` writer - 11:8\\]
DQS slave delay step size during read leveling for slice 0."]
pub type PhyRdlvlDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Support the toggle read preamble for LPDDR4 for slice 0."]
    #[inline(always)]
    pub fn phy_toggle_pre_support_0(&self) -> PhyTogglePreSupport0R {
        PhyTogglePreSupport0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during read leveling for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_dly_step_0(&self) -> PhyRdlvlDlyStep0R {
        PhyRdlvlDlyStep0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Support the toggle read preamble for LPDDR4 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_toggle_pre_support_0(
        &mut self,
    ) -> PhyTogglePreSupport0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec> {
        PhyTogglePreSupport0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
DQS slave delay step size during read leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_dly_step_0(
        &mut self,
    ) -> PhyRdlvlDlyStep0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec> {
        PhyRdlvlDlyStep0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_99::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_99::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_99 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy99Spec {
    const RESET_VALUE: u32 = 0;
}
