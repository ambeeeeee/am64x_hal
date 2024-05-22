#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_misc_status` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_misc_status` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec>;
#[doc = "Field `NUM_RAMS` reader - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsR = crate::FieldReader<u16>;
#[doc = "Field `NUM_RAMS` writer - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
pub type NumRamsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    pub fn num_rams(&self) -> NumRamsR {
        NumRamsR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Indicates the number of RAMS serviced by the ECC aggregator"]
    #[inline(always)]
    #[must_use]
    pub fn num_rams(
        &mut self,
    ) -> NumRamsW<Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec> {
        NumRamsW::new(self, 0)
    }
}
#[doc = "Misc Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec;
impl crate::RegisterSpec for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__ospi_wrap__ecc_aggr_vbp__regs_misc_status::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__OSPI_WRAP__ECC_AGGR_VBP__REGS_misc_status to value 0x01"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_OspiWrap_EccAggrVbp_RegsMiscStatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
