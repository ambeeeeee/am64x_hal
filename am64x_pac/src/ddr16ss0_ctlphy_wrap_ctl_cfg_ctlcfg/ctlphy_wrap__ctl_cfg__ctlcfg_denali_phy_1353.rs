#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1353` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1353` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec>;
#[doc = "Field `PHY_AC_LPBK_OBS_SELECT` reader - 1:0\\]
Select value to map an individual loopback address/control slice observation register to the global observation register."]
pub type PhyAcLpbkObsSelectR = crate::FieldReader;
#[doc = "Field `PHY_AC_LPBK_OBS_SELECT` writer - 1:0\\]
Select value to map an individual loopback address/control slice observation register to the global observation register."]
pub type PhyAcLpbkObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_AC_LPBK_ENABLE` reader - 11:8\\]
Loopback enable for the address/control slices."]
pub type PhyAcLpbkEnableR = crate::FieldReader;
#[doc = "Field `PHY_AC_LPBK_ENABLE` writer - 11:8\\]
Loopback enable for the address/control slices."]
pub type PhyAcLpbkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_AC_LPBK_CONTROL` reader - 24:16\\]
Address/control slice loopback control setting."]
pub type PhyAcLpbkControlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_AC_LPBK_CONTROL` writer - 24:16\\]
Address/control slice loopback control setting."]
pub type PhyAcLpbkControlW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select value to map an individual loopback address/control slice observation register to the global observation register."]
    #[inline(always)]
    pub fn phy_ac_lpbk_obs_select(&self) -> PhyAcLpbkObsSelectR {
        PhyAcLpbkObsSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Loopback enable for the address/control slices."]
    #[inline(always)]
    pub fn phy_ac_lpbk_enable(&self) -> PhyAcLpbkEnableR {
        PhyAcLpbkEnableR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Address/control slice loopback control setting."]
    #[inline(always)]
    pub fn phy_ac_lpbk_control(&self) -> PhyAcLpbkControlR {
        PhyAcLpbkControlR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select value to map an individual loopback address/control slice observation register to the global observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_obs_select(
        &mut self,
    ) -> PhyAcLpbkObsSelectW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec> {
        PhyAcLpbkObsSelectW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Loopback enable for the address/control slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_enable(
        &mut self,
    ) -> PhyAcLpbkEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec> {
        PhyAcLpbkEnableW::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Address/control slice loopback control setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_control(
        &mut self,
    ) -> PhyAcLpbkControlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec> {
        PhyAcLpbkControlW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1353\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1353::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1353::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1353::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1353::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1353 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1353Spec {
    const RESET_VALUE: u32 = 0;
}
