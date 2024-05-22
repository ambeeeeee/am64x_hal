#[doc = "Register `CFG_PIN_CNTR_PRE` reader"]
pub type R = crate::R<CfgPinCntrPreSpec>;
#[doc = "Register `CFG_PIN_CNTR_PRE` writer"]
pub type W = crate::W<CfgPinCntrPreSpec>;
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
    pub fn count(&mut self) -> CountW<CfgPinCntrPreSpec> {
        CountW::new(self, 0)
    }
}
#[doc = "This register contains the value that is loaded in to the Error Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pin_cntr_pre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pin_cntr_pre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPinCntrPreSpec;
impl crate::RegisterSpec for CfgPinCntrPreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pin_cntr_pre::R`](R) reader structure"]
impl crate::Readable for CfgPinCntrPreSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pin_cntr_pre::W`](W) writer structure"]
impl crate::Writable for CfgPinCntrPreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_PIN_CNTR_PRE to value 0"]
impl crate::Resettable for CfgPinCntrPreSpec {
    const RESET_VALUE: u32 = 0;
}
