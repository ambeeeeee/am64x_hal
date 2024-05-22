#[doc = "Register `CFG0_CLAIMREG_P3_R2` reader"]
pub type R = crate::R<Cfg0ClaimregP3R2Spec>;
#[doc = "Register `CFG0_CLAIMREG_P3_R2` writer"]
pub type W = crate::W<Cfg0ClaimregP3R2Spec>;
#[doc = "Field `CLAIMREG_P3_R2` reader - 31:0\\]
Claim bits for Partition 3"]
pub type ClaimregP3R2R = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P3_R2` writer - 31:0\\]
Claim bits for Partition 3"]
pub type ClaimregP3R2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 3"]
    #[inline(always)]
    pub fn claimreg_p3_r2(&self) -> ClaimregP3R2R {
        ClaimregP3R2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 3"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p3_r2(&mut self) -> ClaimregP3R2W<Cfg0ClaimregP3R2Spec> {
        ClaimregP3R2W::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P3_R2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP3R2Spec;
impl crate::RegisterSpec for Cfg0ClaimregP3R2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p3_r2::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP3R2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p3_r2::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP3R2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P3_R2 to value 0"]
impl crate::Resettable for Cfg0ClaimregP3R2Spec {
    const RESET_VALUE: u32 = 0;
}
