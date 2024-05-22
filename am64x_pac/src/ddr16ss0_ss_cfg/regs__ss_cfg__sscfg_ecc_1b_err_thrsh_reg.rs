#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEcc1bErrThrshRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEcc1bErrThrshRegSpec>;
#[doc = "Field `ECC_1B_ERR_THRSH` reader - 15:0\\]
ECC 1-bit error threshold. The bridge will generate an interrupt when the ECC 1-bit error count is equal to or greater than this threshold. A value of 0 will disable the generation of interrupt."]
pub type Ecc1bErrThrshR = crate::FieldReader<u16>;
#[doc = "Field `ECC_1B_ERR_THRSH` writer - 15:0\\]
ECC 1-bit error threshold. The bridge will generate an interrupt when the ECC 1-bit error count is equal to or greater than this threshold. A value of 0 will disable the generation of interrupt."]
pub type Ecc1bErrThrshW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
ECC 1-bit error threshold. The bridge will generate an interrupt when the ECC 1-bit error count is equal to or greater than this threshold. A value of 0 will disable the generation of interrupt."]
    #[inline(always)]
    pub fn ecc_1b_err_thrsh(&self) -> Ecc1bErrThrshR {
        Ecc1bErrThrshR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
ECC 1-bit error threshold. The bridge will generate an interrupt when the ECC 1-bit error count is equal to or greater than this threshold. A value of 0 will disable the generation of interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_1b_err_thrsh(&mut self) -> Ecc1bErrThrshW<Regs_SsCfg_SscfgEcc1bErrThrshRegSpec> {
        Ecc1bErrThrshW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEcc1bErrThrshRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEcc1bErrThrshRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEcc1bErrThrshRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_1b_err_thrsh_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEcc1bErrThrshRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_1B_ERR_THRSH_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEcc1bErrThrshRegSpec {
    const RESET_VALUE: u32 = 0;
}
