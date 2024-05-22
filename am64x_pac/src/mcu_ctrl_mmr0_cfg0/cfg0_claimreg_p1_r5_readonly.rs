#[doc = "Register `CFG0_CLAIMREG_P1_R5_READONLY` reader"]
pub type R = crate::R<Cfg0ClaimregP1R5ReadonlySpec>;
#[doc = "Register `CFG0_CLAIMREG_P1_R5_READONLY` writer"]
pub type W = crate::W<Cfg0ClaimregP1R5ReadonlySpec>;
#[doc = "Field `CLAIMREG_P1_R5_READONLY` reader - 31:0\\]
Claim bits for Partition 1"]
pub type ClaimregP1R5ReadonlyR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P1_R5_READONLY` writer - 31:0\\]
Claim bits for Partition 1"]
pub type ClaimregP1R5ReadonlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 1"]
    #[inline(always)]
    pub fn claimreg_p1_r5_readonly(&self) -> ClaimregP1R5ReadonlyR {
        ClaimregP1R5ReadonlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 1"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p1_r5_readonly(
        &mut self,
    ) -> ClaimregP1R5ReadonlyW<Cfg0ClaimregP1R5ReadonlySpec> {
        ClaimregP1R5ReadonlyW::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P1_R5_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p1_r5_readonly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p1_r5_readonly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP1R5ReadonlySpec;
impl crate::RegisterSpec for Cfg0ClaimregP1R5ReadonlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p1_r5_readonly::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP1R5ReadonlySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p1_r5_readonly::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP1R5ReadonlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P1_R5_READONLY to value 0"]
impl crate::Resettable for Cfg0ClaimregP1R5ReadonlySpec {
    const RESET_VALUE: u32 = 0;
}
