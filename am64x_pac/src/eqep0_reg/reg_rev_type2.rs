#[doc = "Register `REG_REV_TYPE2` reader"]
pub type R = crate::R<RegRevType2Spec>;
#[doc = "Register `REG_REV_TYPE2` writer"]
pub type W = crate::W<RegRevType2Spec>;
#[doc = "Field `MAJOR` reader - 2:0\\]
This field specifies the Major Revision number for the eQEP IP."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 2:0\\]
This field specifies the Major Revision number for the eQEP IP."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINOR` reader - 5:3\\]
This field specifies the Minor Revision number for the eQEP IP."]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - 5:3\\]
This field specifies the Minor Revision number for the eQEP IP."]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
This field specifies the Major Revision number for the eQEP IP."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
This field specifies the Minor Revision number for the eQEP IP."]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
This field specifies the Major Revision number for the eQEP IP."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<RegRevType2Spec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
This field specifies the Minor Revision number for the eQEP IP."]
    #[inline(always)]
    #[must_use]
    pub fn minor(&mut self) -> MinorW<RegRevType2Spec> {
        MinorW::new(self, 3)
    }
}
#[doc = "QEP Revision Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_rev_type2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_rev_type2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegRevType2Spec;
impl crate::RegisterSpec for RegRevType2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_rev_type2::R`](R) reader structure"]
impl crate::Readable for RegRevType2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_rev_type2::W`](W) writer structure"]
impl crate::Writable for RegRevType2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_REV_TYPE2 to value 0x09"]
impl crate::Resettable for RegRevType2Spec {
    const RESET_VALUE: u32 = 0x09;
}
