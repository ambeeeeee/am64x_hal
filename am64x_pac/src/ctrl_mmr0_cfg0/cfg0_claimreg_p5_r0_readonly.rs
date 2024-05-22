#[doc = "Register `CFG0_CLAIMREG_P5_R0_READONLY` reader"]
pub type R = crate::R<Cfg0ClaimregP5R0ReadonlySpec>;
#[doc = "Register `CFG0_CLAIMREG_P5_R0_READONLY` writer"]
pub type W = crate::W<Cfg0ClaimregP5R0ReadonlySpec>;
#[doc = "Field `CLAIMREG_P5_R0_READONLY` reader - 31:0\\]
Claim bits for Partition 5"]
pub type ClaimregP5R0ReadonlyR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P5_R0_READONLY` writer - 31:0\\]
Claim bits for Partition 5"]
pub type ClaimregP5R0ReadonlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 5"]
    #[inline(always)]
    pub fn claimreg_p5_r0_readonly(&self) -> ClaimregP5R0ReadonlyR {
        ClaimregP5R0ReadonlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 5"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p5_r0_readonly(
        &mut self,
    ) -> ClaimregP5R0ReadonlyW<Cfg0ClaimregP5R0ReadonlySpec> {
        ClaimregP5R0ReadonlyW::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P5_R0_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p5_r0_readonly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p5_r0_readonly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP5R0ReadonlySpec;
impl crate::RegisterSpec for Cfg0ClaimregP5R0ReadonlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p5_r0_readonly::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP5R0ReadonlySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p5_r0_readonly::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP5R0ReadonlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P5_R0_READONLY to value 0"]
impl crate::Resettable for Cfg0ClaimregP5R0ReadonlySpec {
    const RESET_VALUE: u32 = 0;
}
