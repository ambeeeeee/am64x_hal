#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1046` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1046` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_1_2` reader - 19:0\\]
CA training foreground pattern 1 for address slice 2."]
pub type PhyAdrCalvlFg1_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_1_2` writer - 19:0\\]
CA training foreground pattern 1 for address slice 2."]
pub type PhyAdrCalvlFg1_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
CA training foreground pattern 1 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_1_2(&self) -> PhyAdrCalvlFg1_2R {
        PhyAdrCalvlFg1_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
CA training foreground pattern 1 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_1_2(
        &mut self,
    ) -> PhyAdrCalvlFg1_2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec> {
        PhyAdrCalvlFg1_2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1046\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1046::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1046::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1046::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1046::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1046 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1046Spec {
    const RESET_VALUE: u32 = 0;
}
