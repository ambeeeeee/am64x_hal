#[doc = "Register `CFG0_CLAIMREG_P4_R11_READONLY` reader"]
pub type R = crate::R<Cfg0ClaimregP4R11ReadonlySpec>;
#[doc = "Register `CFG0_CLAIMREG_P4_R11_READONLY` writer"]
pub type W = crate::W<Cfg0ClaimregP4R11ReadonlySpec>;
#[doc = "Field `CLAIMREG_P4_R11_READONLY` reader - 31:0\\]
Claim bits for Partition 4"]
pub type ClaimregP4R11ReadonlyR = crate::FieldReader<u32>;
#[doc = "Field `CLAIMREG_P4_R11_READONLY` writer - 31:0\\]
Claim bits for Partition 4"]
pub type ClaimregP4R11ReadonlyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 4"]
    #[inline(always)]
    pub fn claimreg_p4_r11_readonly(&self) -> ClaimregP4R11ReadonlyR {
        ClaimregP4R11ReadonlyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Claim bits for Partition 4"]
    #[inline(always)]
    #[must_use]
    pub fn claimreg_p4_r11_readonly(
        &mut self,
    ) -> ClaimregP4R11ReadonlyW<Cfg0ClaimregP4R11ReadonlySpec> {
        ClaimregP4R11ReadonlyW::new(self, 0)
    }
}
#[doc = "CFG0_CLAIMREG_P4_R11_READONLY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_claimreg_p4_r11_readonly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_claimreg_p4_r11_readonly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ClaimregP4R11ReadonlySpec;
impl crate::RegisterSpec for Cfg0ClaimregP4R11ReadonlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_claimreg_p4_r11_readonly::R`](R) reader structure"]
impl crate::Readable for Cfg0ClaimregP4R11ReadonlySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_claimreg_p4_r11_readonly::W`](W) writer structure"]
impl crate::Writable for Cfg0ClaimregP4R11ReadonlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CLAIMREG_P4_R11_READONLY to value 0"]
impl crate::Resettable for Cfg0ClaimregP4R11ReadonlySpec {
    const RESET_VALUE: u32 = 0;
}
