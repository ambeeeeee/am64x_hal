#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec>;
#[doc = "Field `SRAM_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
pub type SramEnableSetR = crate::BitReader;
#[doc = "Field `SRAM_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
pub type SramEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
    #[inline(always)]
    pub fn sram_enable_set(&self) -> SramEnableSetR {
        SramEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for sram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sram_enable_set(
        &mut self,
    ) -> SramEnableSetW<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec> {
        SramEnableSetW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_enable_set_reg0 to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}
