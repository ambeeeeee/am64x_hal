#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_num_bytes_reg` reader"]
pub type R = crate::R<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec,
>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_num_bytes_reg` writer"]
pub type W = crate::W<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec,
>;
#[doc = "Field `VALUE_FLD` reader - 31:0\\]
This is the number of bytes that the indirect access will consume. This can be bigger than the configured size of SRAM."]
pub type ValueFldR = crate::FieldReader<u32>;
#[doc = "Field `VALUE_FLD` writer - 31:0\\]
This is the number of bytes that the indirect access will consume. This can be bigger than the configured size of SRAM."]
pub type ValueFldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the number of bytes that the indirect access will consume. This can be bigger than the configured size of SRAM."]
    #[inline(always)]
    pub fn value_fld(&self) -> ValueFldR {
        ValueFldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the number of bytes that the indirect access will consume. This can be bigger than the configured size of SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn value_fld(
        &mut self,
    ) -> ValueFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec,
    > {
        ValueFldW::new(self, 0)
    }
}
#[doc = "Indirect Read Transfer Number Bytes Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_read_xfer_num_bytes_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_read_xfer_num_bytes_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectReadXferNumBytesRegSpec
{
    const RESET_VALUE: u32 = 0;
}
