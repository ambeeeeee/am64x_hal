#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_trigger_addr_range_reg` reader"]
pub type R = crate::R<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec,
>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_trigger_addr_range_reg` writer"]
pub type W = crate::W<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec,
>;
#[doc = "Field `IND_RANGE_WIDTH_FLD` reader - 3:0\\]
This is the address offset of Indirect Trigger Address Register."]
pub type IndRangeWidthFldR = crate::FieldReader;
#[doc = "Field `IND_RANGE_WIDTH_FLD` writer - 3:0\\]
This is the address offset of Indirect Trigger Address Register."]
pub type IndRangeWidthFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This is the address offset of Indirect Trigger Address Register."]
    #[inline(always)]
    pub fn ind_range_width_fld(&self) -> IndRangeWidthFldR {
        IndRangeWidthFldR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This is the address offset of Indirect Trigger Address Register."]
    #[inline(always)]
    #[must_use]
    pub fn ind_range_width_fld(
        &mut self,
    ) -> IndRangeWidthFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec,
    > {
        IndRangeWidthFldW::new(self, 0)
    }
}
#[doc = "Indirect Trigger Address Range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_trigger_addr_range_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_trigger_addr_range_reg to value 0x04"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectTriggerAddrRangeRegSpec
{
    const RESET_VALUE: u32 = 0x04;
}
