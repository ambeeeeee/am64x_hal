#[doc = "Register `MCRC64_REGS_PSA_SECSIGREGL4` reader"]
pub type R = crate::R<Mcrc64RegsPsaSecsigregl4Spec>;
#[doc = "Register `MCRC64_REGS_PSA_SECSIGREGL4` writer"]
pub type W = crate::W<Mcrc64RegsPsaSecsigregl4Spec>;
#[doc = "Field `PSASECSIG4` reader - 31:0\\]
Channel 4 PSA Sector Signature Low Register."]
pub type Psasecsig4R = crate::FieldReader<u32>;
#[doc = "Field `PSASECSIG4` writer - 31:0\\]
Channel 4 PSA Sector Signature Low Register."]
pub type Psasecsig4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 4 PSA Sector Signature Low Register."]
    #[inline(always)]
    pub fn psasecsig4(&self) -> Psasecsig4R {
        Psasecsig4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 4 PSA Sector Signature Low Register."]
    #[inline(always)]
    #[must_use]
    pub fn psasecsig4(&mut self) -> Psasecsig4W<Mcrc64RegsPsaSecsigregl4Spec> {
        Psasecsig4W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsPsaSecsigregl4Spec;
impl crate::RegisterSpec for Mcrc64RegsPsaSecsigregl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_psa_secsigregl4::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsPsaSecsigregl4Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_psa_secsigregl4::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsPsaSecsigregl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_PSA_SECSIGREGL4 to value 0"]
impl crate::Resettable for Mcrc64RegsPsaSecsigregl4Spec {
    const RESET_VALUE: u32 = 0;
}
