#[doc = "Register `MCRC64_REGS_PSA_SIGREGH4` reader"]
pub type R = crate::R<Mcrc64RegsPsaSigregh4Spec>;
#[doc = "Register `MCRC64_REGS_PSA_SIGREGH4` writer"]
pub type W = crate::W<Mcrc64RegsPsaSigregh4Spec>;
#[doc = "Field `PSASIG4_63_32` reader - 31:0\\]
This register contains the value stored at PSASIG4\\[63:32\\]
register."]
pub type Psasig4_63_32R = crate::FieldReader<u32>;
#[doc = "Field `PSASIG4_63_32` writer - 31:0\\]
This register contains the value stored at PSASIG4\\[63:32\\]
register."]
pub type Psasig4_63_32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the value stored at PSASIG4\\[63:32\\]
register."]
    #[inline(always)]
    pub fn psasig4_63_32(&self) -> Psasig4_63_32R {
        Psasig4_63_32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the value stored at PSASIG4\\[63:32\\]
register."]
    #[inline(always)]
    #[must_use]
    pub fn psasig4_63_32(&mut self) -> Psasig4_63_32W<Mcrc64RegsPsaSigregh4Spec> {
        Psasig4_63_32W::new(self, 0)
    }
}
#[doc = "Channel 4 PSA signature high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_psa_sigregh4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_psa_sigregh4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsPsaSigregh4Spec;
impl crate::RegisterSpec for Mcrc64RegsPsaSigregh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_psa_sigregh4::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsPsaSigregh4Spec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_psa_sigregh4::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsPsaSigregh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_PSA_SIGREGH4 to value 0"]
impl crate::Resettable for Mcrc64RegsPsaSigregh4Spec {
    const RESET_VALUE: u32 = 0;
}
