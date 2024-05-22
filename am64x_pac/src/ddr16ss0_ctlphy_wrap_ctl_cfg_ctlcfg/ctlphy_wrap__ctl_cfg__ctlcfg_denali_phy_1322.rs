#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1322` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1322` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec>;
#[doc = "Field `PHY_PAD_FDBK_TERM` reader - 17:0\\]
Controls term settings for gate feedback pads."]
pub type PhyPadFdbkTermR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_FDBK_TERM` writer - 17:0\\]
Controls term settings for gate feedback pads."]
pub type PhyPadFdbkTermW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
Controls term settings for gate feedback pads."]
    #[inline(always)]
    pub fn phy_pad_fdbk_term(&self) -> PhyPadFdbkTermR {
        PhyPadFdbkTermR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Controls term settings for gate feedback pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_fdbk_term(
        &mut self,
    ) -> PhyPadFdbkTermW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec> {
        PhyPadFdbkTermW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1322\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1322::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1322::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1322::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1322::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1322 to value 0x0001_7424"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1322Spec {
    const RESET_VALUE: u32 = 0x0001_7424;
}
