#[doc = "Register `MCRC64_REGS_PSA_SECSIGREGL3` reader"]
pub type R = crate::R<Mcrc64RegsPsaSecsigregl3Spec>;
#[doc = "Register `MCRC64_REGS_PSA_SECSIGREGL3` writer"]
pub type W = crate::W<Mcrc64RegsPsaSecsigregl3Spec>;
#[doc = "Field `PSASECSIG3` reader - 31:0\\]
Channel 3 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG3\\[31:0\\]
register."]
pub type Psasecsig3R = crate::FieldReader<u32>;
#[doc = "Field `PSASECSIG3` writer - 31:0\\]
Channel 3 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG3\\[31:0\\]
register."]
pub type Psasecsig3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG3\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasecsig3(&self) -> Psasecsig3R {
        Psasecsig3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 3 PSA Sector Signature Low Register. This register contains the value stored at PSASECSIG3\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasecsig3(&mut self) -> Psasecsig3W<Mcrc64RegsPsaSecsigregl3Spec> {
        Psasecsig3W::new(self, 0)
    }
}
#[doc = "Channel 3 PSA sector signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_secsigregl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_secsigregl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsPsaSecsigregl3Spec;
impl crate::RegisterSpec for Mcrc64RegsPsaSecsigregl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_psa_secsigregl3::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsPsaSecsigregl3Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_psa_secsigregl3::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsPsaSecsigregl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_PSA_SECSIGREGL3 to value 0"]
impl crate::Resettable for Mcrc64RegsPsaSecsigregl3Spec {
    const RESET_VALUE: u32 = 0;
}
