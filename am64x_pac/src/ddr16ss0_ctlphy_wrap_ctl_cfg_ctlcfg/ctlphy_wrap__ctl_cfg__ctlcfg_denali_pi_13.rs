#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_13` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_13` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec>;
#[doc = "Field `PI_SW_RST_N` reader - 0:0\\]
User request to reset the whole PI except the parameter modules. Set 0 to reset, set to 1 to release."]
pub type PiSwRstNR = crate::BitReader;
#[doc = "Field `PI_SW_RST_N` writer - 0:0\\]
User request to reset the whole PI except the parameter modules. Set 0 to reset, set to 1 to release."]
pub type PiSwRstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CS_MAP` reader - 17:16\\]
Defines which chip selects are active."]
pub type PiCsMapR = crate::FieldReader;
#[doc = "Field `PI_CS_MAP` writer - 17:16\\]
Defines which chip selects are active."]
pub type PiCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_SWLVL_CS_SEL` reader - 24:24\\]
Defines which chip selects are active in swlvl, 0 for binary, 1 for one-hot."]
pub type PiSwlvlCsSelR = crate::BitReader;
#[doc = "Field `PI_SWLVL_CS_SEL` writer - 24:24\\]
Defines which chip selects are active in swlvl, 0 for binary, 1 for one-hot."]
pub type PiSwlvlCsSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User request to reset the whole PI except the parameter modules. Set 0 to reset, set to 1 to release."]
    #[inline(always)]
    pub fn pi_sw_rst_n(&self) -> PiSwRstNR {
        PiSwRstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip selects are active."]
    #[inline(always)]
    pub fn pi_cs_map(&self) -> PiCsMapR {
        PiCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines which chip selects are active in swlvl, 0 for binary, 1 for one-hot."]
    #[inline(always)]
    pub fn pi_swlvl_cs_sel(&self) -> PiSwlvlCsSelR {
        PiSwlvlCsSelR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User request to reset the whole PI except the parameter modules. Set 0 to reset, set to 1 to release."]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_rst_n(&mut self) -> PiSwRstNW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec> {
        PiSwRstNW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines which chip selects are active."]
    #[inline(always)]
    #[must_use]
    pub fn pi_cs_map(&mut self) -> PiCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec> {
        PiCsMapW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Defines which chip selects are active in swlvl, 0 for binary, 1 for one-hot."]
    #[inline(always)]
    #[must_use]
    pub fn pi_swlvl_cs_sel(&mut self) -> PiSwlvlCsSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec> {
        PiSwlvlCsSelW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_13::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_13::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_13 to value 0x0003_0001"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi13Spec {
    const RESET_VALUE: u32 = 0x0003_0001;
}
