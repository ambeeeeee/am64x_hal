#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_64` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_64` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_0` reader - 31:0\\]
DDL test observation delays for slice 0 master DDL. READ-ONLY"]
pub type PhyDdlTestMstrDlyObs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_0` writer - 31:0\\]
DDL test observation delays for slice 0 master DDL. READ-ONLY"]
pub type PhyDdlTestMstrDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
DDL test observation delays for slice 0 master DDL. READ-ONLY"]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_0(&self) -> PhyDdlTestMstrDlyObs0R {
        PhyDdlTestMstrDlyObs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
DDL test observation delays for slice 0 master DDL. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_test_mstr_dly_obs_0(
        &mut self,
    ) -> PhyDdlTestMstrDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec> {
        PhyDdlTestMstrDlyObs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_64::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_64::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_64 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy64Spec {
    const RESET_VALUE: u32 = 0;
}
