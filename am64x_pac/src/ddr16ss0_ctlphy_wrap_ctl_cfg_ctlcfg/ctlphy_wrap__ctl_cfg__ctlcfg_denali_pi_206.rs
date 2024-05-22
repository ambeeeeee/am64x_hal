#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_206` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_206` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec>;
#[doc = "Field `PI_TVREF_SHORT_F2` reader - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 2."]
pub type PiTvrefShortF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_SHORT_F2` writer - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 2."]
pub type PiTvrefShortF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_TVREF_LONG_F2` reader - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 2."]
pub type PiTvrefLongF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TVREF_LONG_F2` writer - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 2."]
pub type PiTvrefLongF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 2."]
    #[inline(always)]
    pub fn pi_tvref_short_f2(&self) -> PiTvrefShortF2R {
        PiTvrefShortF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 2."]
    #[inline(always)]
    pub fn pi_tvref_long_f2(&self) -> PiTvrefLongF2R {
        PiTvrefLongF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter = 1 for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_short_f2(
        &mut self,
    ) -> PiTvrefShortF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec> {
        PiTvrefShortF2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Defines the delay in PI clock cycles between the dfi_calvl_strobe to the next command if the pi_calvl_vref_stepsize parameter gt 1 for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tvref_long_f2(&mut self) -> PiTvrefLongF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec> {
        PiTvrefLongF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_206\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_206::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_206::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_206::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_206::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_206 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi206Spec {
    const RESET_VALUE: u32 = 0;
}
