#[doc = "Register `EPWM_REGS_AQCSFRC` reader"]
pub type R = crate::R<EpwmRegsAqcsfrcSpec>;
#[doc = "Register `EPWM_REGS_AQCSFRC` writer"]
pub type W = crate::W<EpwmRegsAqcsfrcSpec>;
#[doc = "Field `CSFA` reader - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register"]
pub type CsfaR = crate::FieldReader;
#[doc = "Field `CSFA` writer - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register"]
pub type CsfaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSFB` reader - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register To configure shadow mode, use AQSFRC\\[RLDCSF\\]"]
pub type CsfbR = crate::FieldReader;
#[doc = "Field `CSFB` writer - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register To configure shadow mode, use AQSFRC\\[RLDCSF\\]"]
pub type CsfbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register"]
    #[inline(always)]
    pub fn csfa(&self) -> CsfaR {
        CsfaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register To configure shadow mode, use AQSFRC\\[RLDCSF\\]"]
    #[inline(always)]
    pub fn csfb(&self) -> CsfbR {
        CsfbR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Continuous Software Force on Output A In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register"]
    #[inline(always)]
    #[must_use]
    pub fn csfa(&mut self) -> CsfaW<EpwmRegsAqcsfrcSpec> {
        CsfaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Continuous Software Force on Output B In immediate mode, a continuous force takes effect on the next TBCLK edge In shadow mode, a continuous force takes effect on the next TBCLK edge after a shadow load into the active register To configure shadow mode, use AQSFRC\\[RLDCSF\\]"]
    #[inline(always)]
    #[must_use]
    pub fn csfb(&mut self) -> CsfbW<EpwmRegsAqcsfrcSpec> {
        CsfbW::new(self, 2)
    }
}
#[doc = "Action Qualifier Continuous S/W Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqcsfrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqcsfrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsAqcsfrcSpec;
impl crate::RegisterSpec for EpwmRegsAqcsfrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_aqcsfrc::R`](R) reader structure"]
impl crate::Readable for EpwmRegsAqcsfrcSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_aqcsfrc::W`](W) writer structure"]
impl crate::Writable for EpwmRegsAqcsfrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_AQCSFRC to value 0"]
impl crate::Resettable for EpwmRegsAqcsfrcSpec {
    const RESET_VALUE: u16 = 0;
}
