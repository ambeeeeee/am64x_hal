#[doc = "Register `CFG0_CLAIMREG_P6_R4_READONLY` reader"]
pub type R = crate::R<Cfg0ClaimregP6R4ReadonlySpec>;
#[doc = "Register `CFG0_CLAIMREG_P6_R4_READONLY` writer"]
pub type W = crate::W<Cfg0ClaimregP6R4ReadonlySpec>;
#[doc = "Field `CLAIMREG_P6_R4_READONLY` reader - 31:0\\]
Claim bits for Partition 6"]
pub type ClaimregP6R4ReadonlyR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P6_R4_READONLY` writer - 31:0\\]
Claim bits for Partition 6"]
pub type ClaimregP6R4ReadonlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 6"]
    #[inline(always)]
    pub fn claimreg_p6_r4_readonly(&self) -> ClaimregP6R4ReadonlyR {
        ClaimregP6R4ReadonlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 6"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p6_r4_readonly(
        &mut self,
    ) -> ClaimregP6R4ReadonlyW<Cfg0ClaimregP6R4ReadonlySpec> {
        ClaimregP6R4ReadonlyW::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P6_R4_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p6_r4_readonly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p6_r4_readonly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP6R4ReadonlySpec;
impl crate::RegisterSpec for Cfg0ClaimregP6R4ReadonlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p6_r4_readonly::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP6R4ReadonlySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p6_r4_readonly::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP6R4ReadonlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P6_R4_READONLY to value 0"]
impl crate::Resettable for Cfg0ClaimregP6R4ReadonlySpec {
    const RESET_VALUE: u32 = 0;
}
