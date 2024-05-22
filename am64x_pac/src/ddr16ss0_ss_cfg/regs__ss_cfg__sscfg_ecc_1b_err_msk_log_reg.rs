#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec>;
#[doc = "Field `ECC_1B_ERR_MSK` reader - 3:0\\]
ECC 1-bit error mask. Mask for the 32-byte data block that had the 1-bit ECC errors. Each bit represents an ECC quanta (8 bytes) in the 32-byte data block starting at address specified by ecc_1b_err_adr. Value of 1 on the bit represents an error in that particular 8 bytes. This field displays the first mask logged in the 2 deep logging FIFO along with the ecc_1b_err_adr. The same mechanism used to pop ecc_1b_err_adr is used to pop this field."]
pub type Ecc1bErrMskR = crate::FieldReader;
#[doc = "Field `ECC_1B_ERR_MSK` writer - 3:0\\]
ECC 1-bit error mask. Mask for the 32-byte data block that had the 1-bit ECC errors. Each bit represents an ECC quanta (8 bytes) in the 32-byte data block starting at address specified by ecc_1b_err_adr. Value of 1 on the bit represents an error in that particular 8 bytes. This field displays the first mask logged in the 2 deep logging FIFO along with the ecc_1b_err_adr. The same mechanism used to pop ecc_1b_err_adr is used to pop this field."]
pub type Ecc1bErrMskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
ECC 1-bit error mask. Mask for the 32-byte data block that had the 1-bit ECC errors. Each bit represents an ECC quanta (8 bytes) in the 32-byte data block starting at address specified by ecc_1b_err_adr. Value of 1 on the bit represents an error in that particular 8 bytes. This field displays the first mask logged in the 2 deep logging FIFO along with the ecc_1b_err_adr. The same mechanism used to pop ecc_1b_err_adr is used to pop this field."]
    #[inline(always)]
    pub fn ecc_1b_err_msk(&self) -> Ecc1bErrMskR {
        Ecc1bErrMskR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
ECC 1-bit error mask. Mask for the 32-byte data block that had the 1-bit ECC errors. Each bit represents an ECC quanta (8 bytes) in the 32-byte data block starting at address specified by ecc_1b_err_adr. Value of 1 on the bit represents an error in that particular 8 bytes. This field displays the first mask logged in the 2 deep logging FIFO along with the ecc_1b_err_adr. The same mechanism used to pop ecc_1b_err_adr is used to pop this field."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_1b_err_msk(&mut self) -> Ecc1bErrMskW<Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec> {
        Ecc1bErrMskW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_1b_err_msk_log_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_1B_ERR_MSK_LOG_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEcc1bErrMskLogRegSpec {
    const RESET_VALUE: u32 = 0;
}
