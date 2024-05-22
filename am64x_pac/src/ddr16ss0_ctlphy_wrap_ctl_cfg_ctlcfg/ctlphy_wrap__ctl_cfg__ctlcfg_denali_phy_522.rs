#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_522` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_522` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_0` reader - 31:0\\]
Observation register containing master DDL bits for address slice 0. READ-ONLY"]
pub type PhyAdrDdlTestMstrDlyObs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_0` writer - 31:0\\]
Observation register containing master DDL bits for address slice 0. READ-ONLY"]
pub type PhyAdrDdlTestMstrDlyObs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register containing master DDL bits for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_ddl_test_mstr_dly_obs_0(&self) -> PhyAdrDdlTestMstrDlyObs0R {
        PhyAdrDdlTestMstrDlyObs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register containing master DDL bits for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ddl_test_mstr_dly_obs_0(
        &mut self,
    ) -> PhyAdrDdlTestMstrDlyObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec> {
        PhyAdrDdlTestMstrDlyObs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_522\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_522::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_522::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_522::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_522::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_522 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy522Spec {
    const RESET_VALUE: u32 = 0;
}
