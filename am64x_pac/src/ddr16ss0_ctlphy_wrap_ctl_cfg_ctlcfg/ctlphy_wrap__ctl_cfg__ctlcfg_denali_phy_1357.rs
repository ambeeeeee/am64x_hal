#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1357` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1357` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec>;
#[doc = "Field `PHY_AC_CLK_LPBK_RESULT_OBS` reader - 15:0\\]
Observation register for loopback mem clk blocks. READ-ONLY"]
pub type PhyAcClkLpbkResultObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_AC_CLK_LPBK_RESULT_OBS` writer - 15:0\\]
Observation register for loopback mem clk blocks. READ-ONLY"]
pub type PhyAcClkLpbkResultObsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_AC_PWR_RDC_DISABLE` reader - 16:16\\]
ac slice power reduction disable."]
pub type PhyAcPwrRdcDisableR = crate::BitReader;
#[doc = "Field `PHY_AC_PWR_RDC_DISABLE` writer - 16:16\\]
ac slice power reduction disable."]
pub type PhyAcPwrRdcDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TOP_PWR_RDC_DISABLE` reader - 24:24\\]
top param power reduction disable."]
pub type PhyTopPwrRdcDisableR = crate::BitReader;
#[doc = "Field `PHY_TOP_PWR_RDC_DISABLE` writer - 24:24\\]
top param power reduction disable."]
pub type PhyTopPwrRdcDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register for loopback mem clk blocks. READ-ONLY"]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_result_obs(&self) -> PhyAcClkLpbkResultObsR {
        PhyAcClkLpbkResultObsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
ac slice power reduction disable."]
    #[inline(always)]
    pub fn phy_ac_pwr_rdc_disable(&self) -> PhyAcPwrRdcDisableR {
        PhyAcPwrRdcDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
top param power reduction disable."]
    #[inline(always)]
    pub fn phy_top_pwr_rdc_disable(&self) -> PhyTopPwrRdcDisableR {
        PhyTopPwrRdcDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Observation register for loopback mem clk blocks. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_result_obs(
        &mut self,
    ) -> PhyAcClkLpbkResultObsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec> {
        PhyAcClkLpbkResultObsW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
ac slice power reduction disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_pwr_rdc_disable(
        &mut self,
    ) -> PhyAcPwrRdcDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec> {
        PhyAcPwrRdcDisableW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
top param power reduction disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_top_pwr_rdc_disable(
        &mut self,
    ) -> PhyTopPwrRdcDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec> {
        PhyTopPwrRdcDisableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1357\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1357::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1357::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1357::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1357::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1357 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1357Spec {
    const RESET_VALUE: u32 = 0;
}
