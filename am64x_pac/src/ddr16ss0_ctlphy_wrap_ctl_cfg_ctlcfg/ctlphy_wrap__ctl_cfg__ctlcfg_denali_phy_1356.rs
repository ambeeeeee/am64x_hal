#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1356` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1356` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec>;
#[doc = "Field `PHY_AC_CLK_LPBK_OBS_SELECT` reader - 0:0\\]
Select value to map an individual loopback mem clk block observation register to the global observation register."]
pub type PhyAcClkLpbkObsSelectR = crate::BitReader;
#[doc = "Field `PHY_AC_CLK_LPBK_OBS_SELECT` writer - 0:0\\]
Select value to map an individual loopback mem clk block observation register to the global observation register."]
pub type PhyAcClkLpbkObsSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_AC_CLK_LPBK_ENABLE` reader - 8:8\\]
Loopback enable for mem clk blocks."]
pub type PhyAcClkLpbkEnableR = crate::BitReader;
#[doc = "Field `PHY_AC_CLK_LPBK_ENABLE` writer - 8:8\\]
Loopback enable for mem clk blocks."]
pub type PhyAcClkLpbkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_AC_CLK_LPBK_CONTROL` reader - 21:16\\]
Mem clk block loopback control setting."]
pub type PhyAcClkLpbkControlR = crate::FieldReader;
#[doc = "Field `PHY_AC_CLK_LPBK_CONTROL` writer - 21:16\\]
Mem clk block loopback control setting."]
pub type PhyAcClkLpbkControlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select value to map an individual loopback mem clk block observation register to the global observation register."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_obs_select(&self) -> PhyAcClkLpbkObsSelectR {
        PhyAcClkLpbkObsSelectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Loopback enable for mem clk blocks."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_enable(&self) -> PhyAcClkLpbkEnableR {
        PhyAcClkLpbkEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mem clk block loopback control setting."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_control(&self) -> PhyAcClkLpbkControlR {
        PhyAcClkLpbkControlR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select value to map an individual loopback mem clk block observation register to the global observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_obs_select(
        &mut self,
    ) -> PhyAcClkLpbkObsSelectW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec> {
        PhyAcClkLpbkObsSelectW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Loopback enable for mem clk blocks."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_enable(
        &mut self,
    ) -> PhyAcClkLpbkEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec> {
        PhyAcClkLpbkEnableW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Mem clk block loopback control setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_control(
        &mut self,
    ) -> PhyAcClkLpbkControlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec> {
        PhyAcClkLpbkControlW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1356\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1356::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1356::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1356::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1356::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1356 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1356Spec {
    const RESET_VALUE: u32 = 0;
}
