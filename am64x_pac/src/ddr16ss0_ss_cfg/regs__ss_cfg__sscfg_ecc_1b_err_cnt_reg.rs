#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgEcc1bErrCntRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgEcc1bErrCntRegSpec>;
#[doc = "Field `ECC_1B_ERR_CNT` reader - 15:0\\]
16-bit counter that displays number of 1-bit ECC errors on SDRAM data. Writing a 0x1 will clear this count. Writing any other value has no effect."]
pub type Ecc1bErrCntR = crate::FieldReader<u16>;
#[doc = "Field `ECC_1B_ERR_CNT` writer - 15:0\\]
16-bit counter that displays number of 1-bit ECC errors on SDRAM data. Writing a 0x1 will clear this count. Writing any other value has no effect."]
pub type Ecc1bErrCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
16-bit counter that displays number of 1-bit ECC errors on SDRAM data. Writing a 0x1 will clear this count. Writing any other value has no effect."]
    #[inline(always)]
    pub fn ecc_1b_err_cnt(&self) -> Ecc1bErrCntR {
        Ecc1bErrCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
16-bit counter that displays number of 1-bit ECC errors on SDRAM data. Writing a 0x1 will clear this count. Writing any other value has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_1b_err_cnt(&mut self) -> Ecc1bErrCntW<Regs_SsCfg_SscfgEcc1bErrCntRegSpec> {
        Ecc1bErrCntW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgEcc1bErrCntRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgEcc1bErrCntRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgEcc1bErrCntRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ecc_1b_err_cnt_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgEcc1bErrCntRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_ECC_1B_ERR_CNT_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgEcc1bErrCntRegSpec {
    const RESET_VALUE: u32 = 0;
}
