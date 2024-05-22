#[doc = "Register `CFG_DCCCNTSEED1` reader"]
pub type R = crate::R<CfgDcccntseed1Spec>;
#[doc = "Register `CFG_DCCCNTSEED1` writer"]
pub type W = crate::W<CfgDcccntseed1Spec>;
#[doc = "Field `COUNTSEED1` reader - 19:0\\]
This field contains the seed value that gets loaded into counter 1 (clock source 1). User, privilege, and debug mode (read): Returns the current seed value for counter 1. Privilege and debug mode (write): Sets the current seed value for counter 1."]
pub type Countseed1R = crate::FieldReader<u32>;
#[doc = "Field `COUNTSEED1` writer - 19:0\\]
This field contains the seed value that gets loaded into counter 1 (clock source 1). User, privilege, and debug mode (read): Returns the current seed value for counter 1. Privilege and debug mode (write): Sets the current seed value for counter 1."]
pub type Countseed1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the seed value that gets loaded into counter 1 (clock source 1). User, privilege, and debug mode (read): Returns the current seed value for counter 1. Privilege and debug mode (write): Sets the current seed value for counter 1."]
    #[inline(always)]
    pub fn countseed1(&self) -> Countseed1R {
        Countseed1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This field contains the seed value that gets loaded into counter 1 (clock source 1). User, privilege, and debug mode (read): Returns the current seed value for counter 1. Privilege and debug mode (write): Sets the current seed value for counter 1."]
    #[inline(always)]
    #[must_use]
    pub fn countseed1(&mut self) -> Countseed1W<CfgDcccntseed1Spec> {
        Countseed1W::new(self, 0)
    }
}
#[doc = "Seed value for the counter attached to clock source 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccntseed1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccntseed1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDcccntseed1Spec;
impl crate::RegisterSpec for CfgDcccntseed1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dcccntseed1::R`](R) reader structure"]
impl crate::Readable for CfgDcccntseed1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dcccntseed1::W`](W) writer structure"]
impl crate::Writable for CfgDcccntseed1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCCNTSEED1 to value 0"]
impl crate::Resettable for CfgDcccntseed1Spec {
    const RESET_VALUE: u32 = 0;
}
