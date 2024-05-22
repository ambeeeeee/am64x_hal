#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_EOI_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aEoiRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_EOI_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aEoiRegSpec>;
#[doc = "Field `EOI` reader - 1:0\\]
Software End Of Interrupt (EOI) control. Write 0 for aerr/toerr interrupt. Write 1 for ecc1b interrupt. Write 2 for ecc2b interrupt. This field always reads 0 (no EOI memory)."]
pub type EoiR = crate::FieldReader;
#[doc = "Field `EOI` writer - 1:0\\]
Software End Of Interrupt (EOI) control. Write 0 for aerr/toerr interrupt. Write 1 for ecc1b interrupt. Write 2 for ecc2b interrupt. This field always reads 0 (no EOI memory)."]
pub type EoiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software End Of Interrupt (EOI) control. Write 0 for aerr/toerr interrupt. Write 1 for ecc1b interrupt. Write 2 for ecc2b interrupt. This field always reads 0 (no EOI memory)."]
    #[inline(always)]
    pub fn eoi(&self) -> EoiR {
        EoiR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software End Of Interrupt (EOI) control. Write 0 for aerr/toerr interrupt. Write 1 for ecc1b interrupt. Write 2 for ecc2b interrupt. This field always reads 0 (no EOI memory)."]
    #[inline(always)]
    #[must_use]
    pub fn eoi(&mut self) -> EoiW<Regs_SsCfg_SscfgV2aEoiRegSpec> {
        EoiW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_V2A_EOI_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aEoiRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_eoi_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_eoi_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_EOI_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
