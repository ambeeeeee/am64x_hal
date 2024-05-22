#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec>;
#[doc = "Field `EOI_WR` reader - 0:0\\]
EOI Register"]
pub type EoiWrR = crate::BitReader;
#[doc = "Field `EOI_WR` writer - 0:0\\]
EOI Register"]
pub type EoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    pub fn eoi_wr(&self) -> EoiWrR {
        EoiWrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOI Register"]
    #[inline(always)]
    #[must_use]
    pub fn eoi_wr(&mut self) -> EoiWrW<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec> {
        EoiWrW::new(self, 0)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_ded_eoi_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_ded_eoi_reg to value 0"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsDedEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
