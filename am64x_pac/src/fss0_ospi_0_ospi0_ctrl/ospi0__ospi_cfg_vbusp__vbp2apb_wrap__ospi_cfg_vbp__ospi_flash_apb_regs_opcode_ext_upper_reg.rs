#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_upper_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_upper_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec>;
#[doc = "Field `EXT_WEL_OPCODE_FLD` reader - 23:16\\]
Supplement byte of any WEL Opcode"]
pub type ExtWelOpcodeFldR = crate::FieldReader;
#[doc = "Field `EXT_WEL_OPCODE_FLD` writer - 23:16\\]
Supplement byte of any WEL Opcode"]
pub type ExtWelOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WEL_OPCODE_FLD` reader - 31:24\\]
First byte of any WEL Opcode"]
pub type WelOpcodeFldR = crate::FieldReader;
#[doc = "Field `WEL_OPCODE_FLD` writer - 31:24\\]
First byte of any WEL Opcode"]
pub type WelOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Supplement byte of any WEL Opcode"]
    #[inline(always)]
    pub fn ext_wel_opcode_fld(&self) -> ExtWelOpcodeFldR {
        ExtWelOpcodeFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
First byte of any WEL Opcode"]
    #[inline(always)]
    pub fn wel_opcode_fld(&self) -> WelOpcodeFldR {
        WelOpcodeFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Supplement byte of any WEL Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wel_opcode_fld(
        &mut self,
    ) -> ExtWelOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec,
    > {
        ExtWelOpcodeFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
First byte of any WEL Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn wel_opcode_fld(
        &mut self,
    ) -> WelOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec,
    > {
        WelOpcodeFldW::new(self, 24)
    }
}
#[doc = "Opcode Extension Register (Upper)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_opcode_ext_upper_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_opcode_ext_upper_reg to value 0x0649_0000"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsOpcodeExtUpperRegSpec
{
    const RESET_VALUE: u32 = 0x0649_0000;
}
