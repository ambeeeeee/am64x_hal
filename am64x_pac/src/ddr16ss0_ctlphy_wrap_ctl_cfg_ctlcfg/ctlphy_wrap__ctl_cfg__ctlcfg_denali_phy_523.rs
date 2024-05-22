#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_523` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_523` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec>;
#[doc = "Field `PHY_ADR_CALVL_START_0` reader - 10:0\\]
CA training DDL start value for address slice 0."]
pub type PhyAdrCalvlStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_START_0` writer - 10:0\\]
CA training DDL start value for address slice 0."]
pub type PhyAdrCalvlStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_0` reader - 26:16\\]
Coarse CA training DDL increment value for address slice 0."]
pub type PhyAdrCalvlCoarseDly0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_0` writer - 26:16\\]
Coarse CA training DDL increment value for address slice 0."]
pub type PhyAdrCalvlCoarseDly0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
CA training DDL start value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_start_0(&self) -> PhyAdrCalvlStart0R {
        PhyAdrCalvlStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Coarse CA training DDL increment value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_coarse_dly_0(&self) -> PhyAdrCalvlCoarseDly0R {
        PhyAdrCalvlCoarseDly0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
CA training DDL start value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_start_0(
        &mut self,
    ) -> PhyAdrCalvlStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec> {
        PhyAdrCalvlStart0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Coarse CA training DDL increment value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_coarse_dly_0(
        &mut self,
    ) -> PhyAdrCalvlCoarseDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec> {
        PhyAdrCalvlCoarseDly0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_523\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_523::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_523::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_523::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_523::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_523 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy523Spec {
    const RESET_VALUE: u32 = 0;
}
