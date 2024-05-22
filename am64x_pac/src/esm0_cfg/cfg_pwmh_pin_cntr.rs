#[doc = "Register `CFG_PWMH_PIN_CNTR` reader"]
pub type R = crate::R<CfgPwmhPinCntrSpec>;
#[doc = "Register `CFG_PWMH_PIN_CNTR` writer"]
pub type W = crate::W<CfgPwmhPinCntrSpec>;
#[doc = "Field `COUNT` reader - 23:0\\]
Current Counter Value"]
pub type CountR = crate::FieldReader<u32>;
#[doc = "Field `COUNT` writer - 23:0\\]
Current Counter Value"]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Current Counter Value"]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Current Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> CountW<CfgPwmhPinCntrSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "This register shows the current value of the error pin PWM high counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pwmh_pin_cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pwmh_pin_cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPwmhPinCntrSpec;
impl crate::RegisterSpec for CfgPwmhPinCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pwmh_pin_cntr::R`](R) reader structure"]
impl crate::Readable for CfgPwmhPinCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pwmh_pin_cntr::W`](W) writer structure"]
impl crate::Writable for CfgPwmhPinCntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_PWMH_PIN_CNTR to value 0"]
impl crate::Resettable for CfgPwmhPinCntrSpec {
    const RESET_VALUE: u32 = 0;
}
