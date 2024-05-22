#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_513` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_513` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_0` reader - 31:0\\]
Observation register containing loopback status/results for address slice 0. READ-ONLY"]
pub type PhyAdrLpbkResultObs0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_0` writer - 31:0\\]
Observation register containing loopback status/results for address slice 0. READ-ONLY"]
pub type PhyAdrLpbkResultObs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register containing loopback status/results for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_lpbk_result_obs_0(&self) -> PhyAdrLpbkResultObs0R {
        PhyAdrLpbkResultObs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Observation register containing loopback status/results for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_lpbk_result_obs_0(
        &mut self,
    ) -> PhyAdrLpbkResultObs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec> {
        PhyAdrLpbkResultObs0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_513\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_513::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_513::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_513::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_513::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_513 to value 0x4096"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy513Spec {
    const RESET_VALUE: u32 = 0x4096;
}