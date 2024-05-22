#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec>;
#[doc = "Field `ECC_2B_ERR_ADR` reader - 28:0\\]
ECC 2-bit error address. 16-byte aligned address that had the 2-bit ECC error. Writing a 0x1 will clear this field and the ecc_2b_err_msk field. Writing any other value has no effect."]
pub type Ecc2bErrAdrR = crate::FieldReader<u32>;
#[doc = "Field `ECC_2B_ERR_ADR` writer - 28:0\\]
ECC 2-bit error address. 16-byte aligned address that had the 2-bit ECC error. Writing a 0x1 will clear this field and the ecc_2b_err_msk field. Writing any other value has no effect."]
pub type Ecc2bErrAdrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
ECC 2-bit error address. 16-byte aligned address that had the 2-bit ECC error. Writing a 0x1 will clear this field and the ecc_2b_err_msk field. Writing any other value has no effect."]
    #[inline(always)]
    pub fn ecc_2b_err_adr(&self) -> Ecc2bErrAdrR {
        Ecc2bErrAdrR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
ECC 2-bit error address. 16-byte aligned address that had the 2-bit ECC error. Writing a 0x1 will clear this field and the ecc_2b_err_msk field. Writing any other value has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_2b_err_adr(&mut self) -> Ecc2bErrAdrW<Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec> {
        Ecc2bErrAdrW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_2b_err_adr_log_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_2B_ERR_ADR_LOG_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEcc2bErrAdrLogRegSpec {
    const RESET_VALUE: u32 = 0;
}
