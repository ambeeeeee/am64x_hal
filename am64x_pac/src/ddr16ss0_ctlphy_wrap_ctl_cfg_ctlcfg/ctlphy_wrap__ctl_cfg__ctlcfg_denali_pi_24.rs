#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_24` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_24` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec>;
#[doc = "Field `PI_WRLVL_CS_SW` reader - 1:0\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type PiWrlvlCsSwR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_CS_SW` writer - 1:0\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type PiWrlvlCsSwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_WRLVL_CS` reader - 8:8\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type PiWrlvlCsR = crate::BitReader;
#[doc = "Field `PI_WRLVL_CS` writer - 8:8\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
pub type PiWrlvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WLDQSEN` reader - 21:16\\]
Delay from issuing MRS to first DQS strobe for write leveling."]
pub type PiWldqsenR = crate::FieldReader;
#[doc = "Field `PI_WLDQSEN` writer - 21:16\\]
Delay from issuing MRS to first DQS strobe for write leveling."]
pub type PiWldqsenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WLMRD` reader - 29:24\\]
Delay from issuing MRS to first write leveling strobe."]
pub type PiWlmrdR = crate::FieldReader;
#[doc = "Field `PI_WLMRD` writer - 29:24\\]
Delay from issuing MRS to first write leveling strobe."]
pub type PiWlmrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    pub fn pi_wrlvl_cs_sw(&self) -> PiWrlvlCsSwR {
        PiWrlvlCsSwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    pub fn pi_wrlvl_cs(&self) -> PiWrlvlCsR {
        PiWrlvlCsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Delay from issuing MRS to first DQS strobe for write leveling."]
    #[inline(always)]
    pub fn pi_wldqsen(&self) -> PiWldqsenR {
        PiWldqsenR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Delay from issuing MRS to first write leveling strobe."]
    #[inline(always)]
    pub fn pi_wlmrd(&self) -> PiWlmrdR {
        PiWlmrdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_cs_sw(&mut self) -> PiWrlvlCsSwW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec> {
        PiWrlvlCsSwW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies the target chip select for the write leveling operation initiated through the WRLVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_cs(&mut self) -> PiWrlvlCsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec> {
        PiWrlvlCsW::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Delay from issuing MRS to first DQS strobe for write leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wldqsen(&mut self) -> PiWldqsenW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec> {
        PiWldqsenW::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Delay from issuing MRS to first write leveling strobe."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wlmrd(&mut self) -> PiWlmrdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec> {
        PiWlmrdW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_24::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_24::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_24 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi24Spec {
    const RESET_VALUE: u32 = 0;
}
