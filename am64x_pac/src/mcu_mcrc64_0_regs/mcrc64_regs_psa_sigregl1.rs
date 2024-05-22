#[doc = "Register `MCRC64_REGS_PSA_SIGREGL1` reader"]
pub type R = crate::R<Mcrc64RegsPsaSigregl1Spec>;
#[doc = "Register `MCRC64_REGS_PSA_SIGREGL1` writer"]
pub type W = crate::W<Mcrc64RegsPsaSigregl1Spec>;
#[doc = "Field `PSASIG1` reader - 31:0\\]
Channel 1 PSA Signature Low Register. This register contains the value stored at PSASIG1\\[31:0\\]
register."]
pub type Psasig1R = crate::FieldReader<u32>;
#[doc = "Field `PSASIG1` writer - 31:0\\]
Channel 1 PSA Signature Low Register. This register contains the value stored at PSASIG1\\[31:0\\]
register."]
pub type Psasig1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Signature Low Register. This register contains the value stored at PSASIG1\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasig1(&self) -> Psasig1R {
        Psasig1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 1 PSA Signature Low Register. This register contains the value stored at PSASIG1\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasig1(&mut self) -> Psasig1W<Mcrc64RegsPsaSigregl1Spec> {
        Psasig1W::new(self, 0)
    }
}
#[doc = "Channel 1 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsPsaSigregl1Spec;
impl crate::RegisterSpec for Mcrc64RegsPsaSigregl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_psa_sigregl1::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsPsaSigregl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_psa_sigregl1::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsPsaSigregl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_PSA_SIGREGL1 to value 0"]
impl crate::Resettable for Mcrc64RegsPsaSigregl1Spec {
    const RESET_VALUE: u32 = 0;
}
