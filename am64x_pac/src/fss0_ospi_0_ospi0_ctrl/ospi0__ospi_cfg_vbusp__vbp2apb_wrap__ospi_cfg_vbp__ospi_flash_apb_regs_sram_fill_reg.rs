#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_fill_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_fill_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec>;
#[doc = "Field `SRAM_FILL_INDAC_READ_FLD` reader - 15:0\\]
SRAM Fill Level \\[Indirect Read Partition\\]: Identifies the current fill level of the SRAM Indirect Read partition"]
pub type SramFillIndacReadFldR = crate::FieldReader<u16>;
#[doc = "Field `SRAM_FILL_INDAC_READ_FLD` writer - 15:0\\]
SRAM Fill Level \\[Indirect Read Partition\\]: Identifies the current fill level of the SRAM Indirect Read partition"]
pub type SramFillIndacReadFldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRAM_FILL_INDAC_WRITE_FLD` reader - 31:16\\]
SRAM Fill Level \\[Indirect Write Partition\\]: Identifies the current fill level of the SRAM Indirect Write partition"]
pub type SramFillIndacWriteFldR = crate::FieldReader<u16>;
#[doc = "Field `SRAM_FILL_INDAC_WRITE_FLD` writer - 31:16\\]
SRAM Fill Level \\[Indirect Write Partition\\]: Identifies the current fill level of the SRAM Indirect Write partition"]
pub type SramFillIndacWriteFldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SRAM Fill Level \\[Indirect Read Partition\\]: Identifies the current fill level of the SRAM Indirect Read partition"]
    #[inline(always)]
    pub fn sram_fill_indac_read_fld(&self) -> SramFillIndacReadFldR {
        SramFillIndacReadFldR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
SRAM Fill Level \\[Indirect Write Partition\\]: Identifies the current fill level of the SRAM Indirect Write partition"]
    #[inline(always)]
    pub fn sram_fill_indac_write_fld(&self) -> SramFillIndacWriteFldR {
        SramFillIndacWriteFldR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SRAM Fill Level \\[Indirect Read Partition\\]: Identifies the current fill level of the SRAM Indirect Read partition"]
    #[inline(always)]
    #[must_use]
    pub fn sram_fill_indac_read_fld(
        &mut self,
    ) -> SramFillIndacReadFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec,
    > {
        SramFillIndacReadFldW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
SRAM Fill Level \\[Indirect Write Partition\\]: Identifies the current fill level of the SRAM Indirect Write partition"]
    #[inline(always)]
    #[must_use]
    pub fn sram_fill_indac_write_fld(
        &mut self,
    ) -> SramFillIndacWriteFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec,
    > {
        SramFillIndacWriteFldW::new(self, 16)
    }
}
#[doc = "SRAM Fill Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_sram_fill_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_sram_fill_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsSramFillRegSpec
{
    const RESET_VALUE: u32 = 0;
}
