#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec>;
#[doc = "Field `ECC_1B_ERR_ADR` reader - 28:0\\]
ECC 1-bit error address. 16-byte aligned address that had the 1-bit ECC error. This field displays the first address logged in the 2 deep logging FIFO. Writing a 0x1 will pop the top element of the FIFO. Writing any other value has no effect."]
pub type Ecc1bErrAdrR = crate::FieldReader<u32>;
#[doc = "Field `ECC_1B_ERR_ADR` writer - 28:0\\]
ECC 1-bit error address. 16-byte aligned address that had the 1-bit ECC error. This field displays the first address logged in the 2 deep logging FIFO. Writing a 0x1 will pop the top element of the FIFO. Writing any other value has no effect."]
pub type Ecc1bErrAdrW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - 28:0\\]
ECC 1-bit error address. 16-byte aligned address that had the 1-bit ECC error. This field displays the first address logged in the 2 deep logging FIFO. Writing a 0x1 will pop the top element of the FIFO. Writing any other value has no effect."]
    #[inline(always)]
    pub fn ecc_1b_err_adr(&self) -> Ecc1bErrAdrR {
        Ecc1bErrAdrR::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - 28:0\\]
ECC 1-bit error address. 16-byte aligned address that had the 1-bit ECC error. This field displays the first address logged in the 2 deep logging FIFO. Writing a 0x1 will pop the top element of the FIFO. Writing any other value has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_1b_err_adr(&mut self) -> Ecc1bErrAdrW<Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec> {
        Ecc1bErrAdrW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_1b_err_adr_log_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_1B_ERR_ADR_LOG_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEcc1bErrAdrLogRegSpec {
    const RESET_VALUE: u32 = 0;
}
