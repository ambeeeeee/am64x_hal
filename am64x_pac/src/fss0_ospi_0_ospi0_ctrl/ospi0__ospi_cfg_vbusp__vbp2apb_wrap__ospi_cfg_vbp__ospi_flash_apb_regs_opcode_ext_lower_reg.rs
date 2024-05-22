#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_lower_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_lower_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec>;
#[doc = "Field `EXT_STIG_OPCODE_FLD` reader - 7:0\\]
Supplement byte of any STIG Opcode"]
pub type ExtStigOpcodeFldR = crate::FieldReader;
#[doc = "Field `EXT_STIG_OPCODE_FLD` writer - 7:0\\]
Supplement byte of any STIG Opcode"]
pub type ExtStigOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXT_POLL_OPCODE_FLD` reader - 15:8\\]
Supplement byte of any Polling Opcode"]
pub type ExtPollOpcodeFldR = crate::FieldReader;
#[doc = "Field `EXT_POLL_OPCODE_FLD` writer - 15:8\\]
Supplement byte of any Polling Opcode"]
pub type ExtPollOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXT_WRITE_OPCODE_FLD` reader - 23:16\\]
Supplement byte of any Write Opcode"]
pub type ExtWriteOpcodeFldR = crate::FieldReader;
#[doc = "Field `EXT_WRITE_OPCODE_FLD` writer - 23:16\\]
Supplement byte of any Write Opcode"]
pub type ExtWriteOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXT_READ_OPCODE_FLD` reader - 31:24\\]
Supplement byte of any Read Opcode"]
pub type ExtReadOpcodeFldR = crate::FieldReader;
#[doc = "Field `EXT_READ_OPCODE_FLD` writer - 31:24\\]
Supplement byte of any Read Opcode"]
pub type ExtReadOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Supplement byte of any STIG Opcode"]
    #[inline(always)]
    pub fn ext_stig_opcode_fld(&self) -> ExtStigOpcodeFldR {
        ExtStigOpcodeFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Supplement byte of any Polling Opcode"]
    #[inline(always)]
    pub fn ext_poll_opcode_fld(&self) -> ExtPollOpcodeFldR {
        ExtPollOpcodeFldR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Supplement byte of any Write Opcode"]
    #[inline(always)]
    pub fn ext_write_opcode_fld(&self) -> ExtWriteOpcodeFldR {
        ExtWriteOpcodeFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Supplement byte of any Read Opcode"]
    #[inline(always)]
    pub fn ext_read_opcode_fld(&self) -> ExtReadOpcodeFldR {
        ExtReadOpcodeFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Supplement byte of any STIG Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn ext_stig_opcode_fld(
        &mut self,
    ) -> ExtStigOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec,
    > {
        ExtStigOpcodeFldW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Supplement byte of any Polling Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn ext_poll_opcode_fld(
        &mut self,
    ) -> ExtPollOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec,
    > {
        ExtPollOpcodeFldW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Supplement byte of any Write Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn ext_write_opcode_fld(
        &mut self,
    ) -> ExtWriteOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec,
    > {
        ExtWriteOpcodeFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Supplement byte of any Read Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn ext_read_opcode_fld(
        &mut self,
    ) -> ExtReadOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec,
    > {
        ExtReadOpcodeFldW::new(self, 24)
    }
}
#[doc = "Opcode Extension Register (Lower)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_lower_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_lower_reg to value 0x1b37_5000"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtLowerRegSpec
{
    const RESET_VALUE: u32 = 0x1b37_5000;
}
