#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_203` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_203` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec>;
#[doc = "Field `PI_TVREF_SHORT_F0` reader - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 0."]
pub type PiTvrefShortF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F0` writer - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 0."]
pub type PiTvrefShortF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TVREF_LONG_F0` reader - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 0."]
pub type PiTvrefLongF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F0` writer - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 0."]
pub type PiTvrefLongF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 0."]
    #[inline(always)]
    pub fn pi_tvref_short_f0(&self) -> PiTvrefShortF0R {
        PiTvrefShortF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 0."]
    #[inline(always)]
    pub fn pi_tvref_long_f0(&self) -> PiTvrefLongF0R {
        PiTvrefLongF0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f0(
        &mut self,
    ) -> PiTvrefShortF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec> {
        PiTvrefShortF0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f0(&mut self) -> PiTvrefLongF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec> {
        PiTvrefLongF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_203\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_203::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_203::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_203::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_203::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_203 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi203Spec {
    const RESET_VALUE: u32 = 0;
}
