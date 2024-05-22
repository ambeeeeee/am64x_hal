#[doc = "Register `ADCREGS_STEPDELAY_1` reader"]
pub type R = crate::R<AdcregsStepdelay1Spec>;
#[doc = "Register `ADCREGS_STEPDELAY_1` writer"]
pub type W = crate::W<AdcregsStepdelay1Spec>;
#[doc = "Field `OPENDELAY` reader - 17:0\\]
Program the number of ADC clock cycles to wait after applying the step configuration registers and before sending the start of ADC conversion"]
pub type OpendelayR = crate::FieldReader<u32>;
#[doc = "Field `OPENDELAY` writer - 17:0\\]
Program the number of ADC clock cycles to wait after applying the step configuration registers and before sending the start of ADC conversion"]
pub type OpendelayW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SAMPLEDELAY` reader - 31:24\\]
number of ADC clock cycles to sample. Any value programmed here will be added to the minimum requirement of 1 clock cycle."]
pub type SampledelayR = crate::FieldReader;
#[doc = "Field `SAMPLEDELAY` writer - 31:24\\]
number of ADC clock cycles to sample. Any value programmed here will be added to the minimum requirement of 1 clock cycle."]
pub type SampledelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
Program the number of ADC clock cycles to wait after applying the step configuration registers and before sending the start of ADC conversion"]
    #[inline(always)]
    pub fn opendelay(&self) -> OpendelayR {
        OpendelayR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
number of ADC clock cycles to sample. Any value programmed here will be added to the minimum requirement of 1 clock cycle."]
    #[inline(always)]
    pub fn sampledelay(&self) -> SampledelayR {
        SampledelayR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
Program the number of ADC clock cycles to wait after applying the step configuration registers and before sending the start of ADC conversion"]
    #[inline(always)]
    #[must_use]
    pub fn opendelay(&mut self) -> OpendelayW<AdcregsStepdelay1Spec> {
        OpendelayW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
number of ADC clock cycles to sample. Any value programmed here will be added to the minimum requirement of 1 clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn sampledelay(&mut self) -> SampledelayW<AdcregsStepdelay1Spec> {
        SampledelayW::new(self, 24)
    }
}
#[doc = "Controls number of clock periods to sample and delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_stepdelay_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_stepdelay_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsStepdelay1Spec;
impl crate::RegisterSpec for AdcregsStepdelay1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_stepdelay_1::R`](R) reader structure"]
impl crate::Readable for AdcregsStepdelay1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_stepdelay_1::W`](W) writer structure"]
impl crate::Writable for AdcregsStepdelay1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_STEPDELAY_1 to value 0"]
impl crate::Resettable for AdcregsStepdelay1Spec {
    const RESET_VALUE: u32 = 0;
}
