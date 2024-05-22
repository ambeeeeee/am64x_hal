#[doc = "Register `CFG_DCCCNTSEED0` reader"]
pub type R = crate::R<CfgDcccntseed0Spec>;
#[doc = "Register `CFG_DCCCNTSEED0` writer"]
pub type W = crate::W<CfgDcccntseed0Spec>;
#[doc = "Field `COUNTSEED0` reader - 19:0\\]
This field contains the seed value that gets loaded into counter 0 (clock source 0). User, privilege, and debug mode (read): Returns the current seed value for counter 0. Privilege and debug mode (write): Sets the current seed value for counter 0."]
pub type Countseed0R = crate::FieldReader<u32>;
#[doc = "Field `COUNTSEED0` writer - 19:0\\]
This field contains the seed value that gets loaded into counter 0 (clock source 0). User, privilege, and debug mode (read): Returns the current seed value for counter 0. Privilege and debug mode (write): Sets the current seed value for counter 0."]
pub type Countseed0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the seed value that gets loaded into counter 0 (clock source 0). User, privilege, and debug mode (read): Returns the current seed value for counter 0. Privilege and debug mode (write): Sets the current seed value for counter 0."]
    #[inline(always)]
    pub fn countseed0(&self) -> Countseed0R {
        Countseed0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the seed value that gets loaded into counter 0 (clock source 0). User, privilege, and debug mode (read): Returns the current seed value for counter 0. Privilege and debug mode (write): Sets the current seed value for counter 0."]
    #[inline(always)]
    #[must_use]
    pub fn countseed0(&mut self) -> Countseed0W<CfgDcccntseed0Spec> {
        Countseed0W::new(self, 0)
    }
}
#[doc = "Seed value for the counter attached to clock source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccntseed0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccntseed0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDcccntseed0Spec;
impl crate::RegisterSpec for CfgDcccntseed0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dcccntseed0::R`](R) reader structure"]
impl crate::Readable for CfgDcccntseed0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dcccntseed0::W`](W) writer structure"]
impl crate::Writable for CfgDcccntseed0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCNTSEED0 to value 0"]
impl crate::Resettable for CfgDcccntseed0Spec {
    const RESET_VALUE: u32 = 0;
}
