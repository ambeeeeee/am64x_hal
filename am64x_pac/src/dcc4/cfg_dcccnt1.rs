#[doc = "Register `CFG_DCCCNT1` reader"]
pub type R = crate::R<CfgDcccnt1Spec>;
#[doc = "Register `CFG_DCCCNT1` writer"]
pub type W = crate::W<CfgDcccnt1Spec>;
#[doc = "Field `COUNT1` reader - 19:0\\]
This field contains the current value of counter 1. User, privilege, and debug mode (read): Returns the current value for counter 1. Privilege and debug mode (write): writes have no effect."]
pub type Count1R = crate::FieldReader<u32>;
#[doc = "Field `COUNT1` writer - 19:0\\]
This field contains the current value of counter 1. User, privilege, and debug mode (read): Returns the current value for counter 1. Privilege and debug mode (write): writes have no effect."]
pub type Count1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 1. User, privilege, and debug mode (read): Returns the current value for counter 1. Privilege and debug mode (write): writes have no effect."]
    #[inline(always)]
    pub fn count1(&self) -> Count1R {
        Count1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 1. User, privilege, and debug mode (read): Returns the current value for counter 1. Privilege and debug mode (write): writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count1(&mut self) -> Count1W<CfgDcccnt1Spec> {
        Count1W::new(self, 0)
    }
}
#[doc = "Value of the counter attached to clock source 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccnt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccnt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDcccnt1Spec;
impl crate::RegisterSpec for CfgDcccnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dcccnt1::R`](R) reader structure"]
impl crate::Readable for CfgDcccnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dcccnt1::W`](W) writer structure"]
impl crate::Writable for CfgDcccnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCNT1 to value 0"]
impl crate::Resettable for CfgDcccnt1Spec {
    const RESET_VALUE: u32 = 0;
}
