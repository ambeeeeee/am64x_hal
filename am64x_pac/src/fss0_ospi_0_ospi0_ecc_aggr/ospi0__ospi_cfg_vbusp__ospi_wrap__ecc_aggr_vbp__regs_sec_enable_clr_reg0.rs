#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec>;
#[doc = "Field `SRAM_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for sram_pend"]
pub type SramEnableClrR = crate::BitReader;
#[doc = "Field `SRAM_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for sram_pend"]
pub type SramEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for sram_pend"]
    #[inline(always)]
    pub fn sram_enable_clr(&self) -> SramEnableClrR {
        SramEnableClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for sram_pend"]
    #[inline(always)]
    #[must_use]
    pub fn sram_enable_clr(
        &mut self,
    ) -> SramEnableClrW<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec> {
        SramEnableClrW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}
