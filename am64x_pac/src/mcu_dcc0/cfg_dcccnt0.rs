#[doc = "Register `CFG_DCCCNT0` reader"]
pub type R = crate::R<CfgDcccnt0Spec>;
#[doc = "Register `CFG_DCCCNT0` writer"]
pub type W = crate::W<CfgDcccnt0Spec>;
#[doc = "Field `COUNT0` reader - 19:0\\]
This field contains the current value of counter 0. User, privilege, and debug mode (read): Returns the current value for counter 0. Privilege and debug mode (write): Writes have no effect."]
pub type Count0R = crate::FieldReader<u32>;
#[doc = "Field `COUNT0` writer - 19:0\\]
This field contains the current value of counter 0. User, privilege, and debug mode (read): Returns the current value for counter 0. Privilege and debug mode (write): Writes have no effect."]
pub type Count0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 0. User, privilege, and debug mode (read): Returns the current value for counter 0. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    pub fn count0(&self) -> Count0R {
        Count0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the current value of counter 0. User, privilege, and debug mode (read): Returns the current value for counter 0. Privilege and debug mode (write): Writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn count0(&mut self) -> Count0W<CfgDcccnt0Spec> {
        Count0W::new(self, 0)
    }
}
#[doc = "Value of the counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccnt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccnt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDcccnt0Spec;
impl crate::RegisterSpec for CfgDcccnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dcccnt0::R`](R) reader structure"]
impl crate::Readable for CfgDcccnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dcccnt0::W`](W) writer structure"]
impl crate::Writable for CfgDcccnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCNT0 to value 0"]
impl crate::Resettable for CfgDcccnt0Spec {
    const RESET_VALUE: u32 = 0;
}
