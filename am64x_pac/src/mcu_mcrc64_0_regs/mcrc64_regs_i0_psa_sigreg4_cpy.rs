#[doc = "Register `MCRC64_REGS_I0_PSA_SIGREG4_CPY` reader"]
pub type R = crate::R<Mcrc64RegsI0PsaSigreg4CpySpec>;
#[doc = "Register `MCRC64_REGS_I0_PSA_SIGREG4_CPY` writer"]
pub type W = crate::W<Mcrc64RegsI0PsaSigreg4CpySpec>;
#[doc = "Field `I0_PSASIG4_CPY0` reader - 31:0\\]
This register is a 128 byte block copy of the PSASIG4 register for DMA destination, it is write only, the result can be found in the PSASIG4 register."]
pub type I0Psasig4Cpy0R = crate::FieldReader<u32>;
#[doc = "Field `I0_PSASIG4_CPY0` writer - 31:0\\]
This register is a 128 byte block copy of the PSASIG4 register for DMA destination, it is write only, the result can be found in the PSASIG4 register."]
pub type I0Psasig4Cpy0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is a 128 byte block copy of the PSASIG4 register for DMA destination, it is write only, the result can be found in the PSASIG4 register."]
    #[inline(always)]
    pub fn i0_psasig4_cpy0(&self) -> I0Psasig4Cpy0R {
        I0Psasig4Cpy0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is a 128 byte block copy of the PSASIG4 register for DMA destination, it is write only, the result can be found in the PSASIG4 register."]
    #[inline(always)]
    #[must_use]
    pub fn i0_psasig4_cpy0(&mut self) -> I0Psasig4Cpy0W<Mcrc64RegsI0PsaSigreg4CpySpec> {
        I0Psasig4Cpy0W::new(self, 0)
    }
}
#[doc = "Region for Channel 4 PSA signature block used by DMA based systems.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_i0_psa_sigreg4_cpy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_i0_psa_sigreg4_cpy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsI0PsaSigreg4CpySpec;
impl crate::RegisterSpec for Mcrc64RegsI0PsaSigreg4CpySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_i0_psa_sigreg4_cpy::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsI0PsaSigreg4CpySpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_i0_psa_sigreg4_cpy::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsI0PsaSigreg4CpySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_I0_PSA_SIGREG4_CPY to value 0"]
impl crate::Resettable for Mcrc64RegsI0PsaSigreg4CpySpec {
    const RESET_VALUE: u32 = 0;
}
