#[doc = "Register `CFG_DCCVALID0` reader"]
pub type R = crate::R<CfgDccvalid0Spec>;
#[doc = "Register `CFG_DCCVALID0` writer"]
pub type W = crate::W<CfgDccvalid0Spec>;
#[doc = "Field `VALID0` reader - 15:0\\]
This field contains the current value of valid counter 0. User, privilege, and debug mode (read): Returns the current value for valid counter 0. Privilege and debug mode (write): writes have no effect."]
pub type Valid0R = crate::FieldReader<u16>;
#[doc = "Field `VALID0` writer - 15:0\\]
This field contains the current value of valid counter 0. User, privilege, and debug mode (read): Returns the current value for valid counter 0. Privilege and debug mode (write): writes have no effect."]
pub type Valid0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the current value of valid counter 0. User, privilege, and debug mode (read): Returns the current value for valid counter 0. Privilege and debug mode (write): writes have no effect."]
    #[inline(always)]
    pub fn valid0(&self) -> Valid0R {
        Valid0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
This field contains the current value of valid counter 0. User, privilege, and debug mode (read): Returns the current value for valid counter 0. Privilege and debug mode (write): writes have no effect."]
    #[inline(always)]
    #[must_use]
    pub fn valid0(&mut self) -> Valid0W<CfgDccvalid0Spec> {
        Valid0W::new(self, 0)
    }
}
#[doc = "Value of the valid counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccvalid0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccvalid0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccvalid0Spec;
impl crate::RegisterSpec for CfgDccvalid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccvalid0::R`](R) reader structure"]
impl crate::Readable for CfgDccvalid0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccvalid0::W`](W) writer structure"]
impl crate::Writable for CfgDccvalid0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCVALID0 to value 0"]
impl crate::Resettable for CfgDccvalid0Spec {
    const RESET_VALUE: u32 = 0;
}
