#[doc = "Register `CFG_CPUC1` reader"]
pub type R = crate::R<CfgCpuc1Spec>;
#[doc = "Register `CFG_CPUC1` writer"]
pub type W = crate::W<CfgCpuc1Spec>;
#[doc = "Field `CPUC1` reader - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: fFRC1 = R-----T---I---C----L----K-- 232 If CPUC1 0: fFRC1 = ----R----T----I--C-----L---K------- CPUC1 + 1 User and privilege mode (read): current compare value Privilege mode (write): the compare value is updated"]
pub type Cpuc1R = crate::FieldReader<u32>;
#[doc = "Field `CPUC1` writer - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: fFRC1 = R-----T---I---C----L----K-- 232 If CPUC1 0: fFRC1 = ----R----T----I--C-----L---K------- CPUC1 + 1 User and privilege mode (read): current compare value Privilege mode (write): the compare value is updated"]
pub type Cpuc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: fFRC1 = R-----T---I---C----L----K-- 232 If CPUC1 0: fFRC1 = ----R----T----I--C-----L---K------- CPUC1 + 1 User and privilege mode (read): current compare value Privilege mode (write): the compare value is updated"]
    #[inline(always)]
    pub fn cpuc1(&self) -> Cpuc1R {
        Cpuc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This registers holds the compare value, which is compared with the Up Counter 1. When the compare matches, Free Running Counter 1 is incremented. The Up Counter is set to zero when the counter value matches the CPUC1 value. The value set in this prescales the RTI clock. If CPUC1 = 0: fFRC1 = R-----T---I---C----L----K-- 232 If CPUC1 0: fFRC1 = ----R----T----I--C-----L---K------- CPUC1 + 1 User and privilege mode (read): current compare value Privilege mode (write): the compare value is updated"]
    #[inline(always)]
    #[must_use]
    pub fn cpuc1(&mut self) -> Cpuc1W<CfgCpuc1Spec> {
        Cpuc1W::new(self, 0)
    }
}
#[doc = "CFG_CPUC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cpuc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cpuc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCpuc1Spec;
impl crate::RegisterSpec for CfgCpuc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_cpuc1::R`](R) reader structure"]
impl crate::Readable for CfgCpuc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_cpuc1::W`](W) writer structure"]
impl crate::Writable for CfgCpuc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CPUC1 to value 0"]
impl crate::Resettable for CfgCpuc1Spec {
    const RESET_VALUE: u32 = 0;
}
