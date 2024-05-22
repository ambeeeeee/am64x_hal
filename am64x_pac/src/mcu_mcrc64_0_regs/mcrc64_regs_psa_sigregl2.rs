#[doc = "Register `MCRC64_REGS_PSA_SIGREGL2` reader"]
pub type R = crate::R<Mcrc64RegsPsaSigregl2Spec>;
#[doc = "Register `MCRC64_REGS_PSA_SIGREGL2` writer"]
pub type W = crate::W<Mcrc64RegsPsaSigregl2Spec>;
#[doc = "Field `PSASIG2` reader - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
pub type Psasig2R = crate::FieldReader<u32>;
#[doc = "Field `PSASIG2` writer - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
pub type Psasig2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
    #[inline(always)]
    pub fn psasig2(&self) -> Psasig2R {
        Psasig2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel 2 PSA Signature Low Register. This register contains the value stored at PSASIG2\\[31:0\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasig2(&mut self) -> Psasig2W<Mcrc64RegsPsaSigregl2Spec> {
        Psasig2W::new(self, 0)
    }
}
#[doc = "Channel 2 PSA signature low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsPsaSigregl2Spec;
impl crate::RegisterSpec for Mcrc64RegsPsaSigregl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_psa_sigregl2::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsPsaSigregl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_psa_sigregl2::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsPsaSigregl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_PSA_SIGREGL2 to value 0"]
impl crate::Resettable for Mcrc64RegsPsaSigregl2Spec {
    const RESET_VALUE: u32 = 0;
}
