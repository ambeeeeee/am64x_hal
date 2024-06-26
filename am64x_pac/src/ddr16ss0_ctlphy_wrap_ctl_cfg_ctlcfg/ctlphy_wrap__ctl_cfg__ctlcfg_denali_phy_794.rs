#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_794` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_794` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_3_1` reader - 19:0\\]
CA training foreground pattern 3 for address slice 1."]
pub type PhyAdrCalvlFg3_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_3_1` writer - 19:0\\]
CA training foreground pattern 3 for address slice 1."]
pub type PhyAdrCalvlFg3_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
CA training foreground pattern 3 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_3_1(&self) -> PhyAdrCalvlFg3_1R {
        PhyAdrCalvlFg3_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
CA training foreground pattern 3 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_3_1(
        &mut self,
    ) -> PhyAdrCalvlFg3_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec> {
        PhyAdrCalvlFg3_1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_794\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_794::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_794::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_794::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_794::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_794 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy794Spec {
    const RESET_VALUE: u32 = 0;
}
