#[doc = "Register `CFG_PWML_PIN_CNTR_PRE` reader"]
pub type R = crate::R<CfgPwmlPinCntrPreSpec>;
#[doc = "Register `CFG_PWML_PIN_CNTR_PRE` writer"]
pub type W = crate::W<CfgPwmlPinCntrPreSpec>;
#[doc = "Field `COUNT` reader - 23:0\\]
Counter Pre-Load Value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 23:0\\]
Counter Pre-Load Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Counter Pre-Load Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Counter Pre-Load Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<CfgPwmlPinCntrPreSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "This register contains the value that is loaded in to the Error PWM Low Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwml_pin_cntr_pre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwml_pin_cntr_pre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPwmlPinCntrPreSpec;
impl crate::RegisterSpec for CfgPwmlPinCntrPreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pwml_pin_cntr_pre::R`](R) reader structure"]
impl crate::Readable for CfgPwmlPinCntrPreSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pwml_pin_cntr_pre::W`](W) writer structure"]
impl crate::Writable for CfgPwmlPinCntrPreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_PWML_PIN_CNTR_PRE to value 0"]
impl crate::Resettable for CfgPwmlPinCntrPreSpec {
    const RESET_VALUE: u32 = 0;
}
