#[doc = "Register `CFG0_CLAIMREG_P3_R7_READONLY` reader"]
pub type R = crate::R<Cfg0ClaimregP3R7ReadonlySpec>;
#[doc = "Register `CFG0_CLAIMREG_P3_R7_READONLY` writer"]
pub type W = crate::W<Cfg0ClaimregP3R7ReadonlySpec>;
#[doc = "Field `CLAIMREG_P3_R7_READONLY` reader - 31:0\\]
Claim bits for Partition 3"]
pub type ClaimregP3R7ReadonlyR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P3_R7_READONLY` writer - 31:0\\]
Claim bits for Partition 3"]
pub type ClaimregP3R7ReadonlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 3"]
    #[inline(always)]
    pub fn claimreg_p3_r7_readonly(&self) -> ClaimregP3R7ReadonlyR {
        ClaimregP3R7ReadonlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 3"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p3_r7_readonly(
        &mut self,
    ) -> ClaimregP3R7ReadonlyW<Cfg0ClaimregP3R7ReadonlySpec> {
        ClaimregP3R7ReadonlyW::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P3_R7_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p3_r7_readonly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p3_r7_readonly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP3R7ReadonlySpec;
impl crate::RegisterSpec for Cfg0ClaimregP3R7ReadonlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p3_r7_readonly::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP3R7ReadonlySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p3_r7_readonly::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP3R7ReadonlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P3_R7_READONLY to value 0"]
impl crate::Resettable for Cfg0ClaimregP3R7ReadonlySpec {
    const RESET_VALUE: u32 = 0;
}
