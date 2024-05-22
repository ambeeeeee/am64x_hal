#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_addr_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_addr_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec>;
#[doc = "Field `ADDR_FLD` reader - 31:0\\]
Command Address: This should be setup before triggering the command with execute field \\[bit 0\\]
of the Flash Command Control register. It is the address used by the command specified in the opcode field \\[bits 31:24\\]
of the Flash Command Control register."]
pub type AddrFldR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_FLD` writer - 31:0\\]
Command Address: This should be setup before triggering the command with execute field \\[bit 0\\]
of the Flash Command Control register. It is the address used by the command specified in the opcode field \\[bits 31:24\\]
of the Flash Command Control register."]
pub type AddrFldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Command Address: This should be setup before triggering the command with execute field \\[bit 0\\]
of the Flash Command Control register. It is the address used by the command specified in the opcode field \\[bits 31:24\\]
of the Flash Command Control register."]
    #[inline(always)]
    pub fn addr_fld(&self) -> AddrFldR {
        AddrFldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Command Address: This should be setup before triggering the command with execute field \\[bit 0\\]
of the Flash Command Control register. It is the address used by the command specified in the opcode field \\[bits 31:24\\]
of the Flash Command Control register."]
    #[inline(always)]
    #[must_use]
    pub fn addr_fld(
        &mut self,
    ) -> AddrFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec>
    {
        AddrFldW::new(self, 0)
    }
}
#[doc = "Flash Command Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_addr_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_addr_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdAddrRegSpec
{
    const RESET_VALUE: u32 = 0;
}
