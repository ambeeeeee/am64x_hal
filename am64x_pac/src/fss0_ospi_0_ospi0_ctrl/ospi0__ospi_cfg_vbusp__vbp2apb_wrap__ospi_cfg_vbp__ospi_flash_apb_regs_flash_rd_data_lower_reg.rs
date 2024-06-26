#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_lower_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_lower_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec>;
#[doc = "Field `DATA_FLD` reader - 31:0\\]
This is the data that is returned by the flash device for any status or configuration read operation carried out by triggering the event in the control register. The register will be valid when the polling bit in the control register is low."]
pub type DataFldR = crate::FieldReader<u32>;
#[doc = "Field `DATA_FLD` writer - 31:0\\]
This is the data that is returned by the flash device for any status or configuration read operation carried out by triggering the event in the control register. The register will be valid when the polling bit in the control register is low."]
pub type DataFldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This is the data that is returned by the flash device for any status or configuration read operation carried out by triggering the event in the control register. The register will be valid when the polling bit in the control register is low."]
    #[inline(always)]
    pub fn data_fld(&self) -> DataFldR {
        DataFldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This is the data that is returned by the flash device for any status or configuration read operation carried out by triggering the event in the control register. The register will be valid when the polling bit in the control register is low."]
    #[inline(always)]
    #[must_use]
    pub fn data_fld(
        &mut self,
    ) -> DataFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec>
    {
        DataFldW::new(self, 0)
    }
}
#[doc = "Flash Command Read Data Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_rd_data_lower_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_rd_data_lower_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashRdDataLowerRegSpec
{
    const RESET_VALUE: u32 = 0;
}
