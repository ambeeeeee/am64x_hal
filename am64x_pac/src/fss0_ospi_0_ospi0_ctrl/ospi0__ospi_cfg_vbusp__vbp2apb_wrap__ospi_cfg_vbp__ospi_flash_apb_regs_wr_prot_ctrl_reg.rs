#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_wr_prot_ctrl_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_wr_prot_ctrl_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec>;
#[doc = "Field `INV_FLD` reader - 0:0\\]
Write Protection Inversion Bit: When set to 1, the protection region defined in the lower and upper write protection registers is inverted meaning it is the region that the system is permitted to write to. When set to 0, the protection region defined in the lower and upper write protection registers is the region that the system is not permitted to write to."]
pub type InvFldR = crate::BitReader;
#[doc = "Field `INV_FLD` writer - 0:0\\]
Write Protection Inversion Bit: When set to 1, the protection region defined in the lower and upper write protection registers is inverted meaning it is the region that the system is permitted to write to. When set to 0, the protection region defined in the lower and upper write protection registers is the region that the system is not permitted to write to."]
pub type InvFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_FLD` reader - 1:1\\]
Write Protection Enable Bit: When set to 1, any AHB write access with an address within the protection region defined in the lower and upper write protection registers is rejected. An AHB error response is generated and an interrupt source triggered. When set to 0, the protection region is disabled."]
pub type EnbFldR = crate::BitReader;
#[doc = "Field `ENB_FLD` writer - 1:1\\]
Write Protection Enable Bit: When set to 1, any AHB write access with an address within the protection region defined in the lower and upper write protection registers is rejected. An AHB error response is generated and an interrupt source triggered. When set to 0, the protection region is disabled."]
pub type EnbFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write Protection Inversion Bit: When set to 1, the protection region defined in the lower and upper write protection registers is inverted meaning it is the region that the system is permitted to write to. When set to 0, the protection region defined in the lower and upper write protection registers is the region that the system is not permitted to write to."]
    #[inline(always)]
    pub fn inv_fld(&self) -> InvFldR {
        InvFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write Protection Enable Bit: When set to 1, any AHB write access with an address within the protection region defined in the lower and upper write protection registers is rejected. An AHB error response is generated and an interrupt source triggered. When set to 0, the protection region is disabled."]
    #[inline(always)]
    pub fn enb_fld(&self) -> EnbFldR {
        EnbFldR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write Protection Inversion Bit: When set to 1, the protection region defined in the lower and upper write protection registers is inverted meaning it is the region that the system is permitted to write to. When set to 0, the protection region defined in the lower and upper write protection registers is the region that the system is not permitted to write to."]
    #[inline(always)]
    #[must_use]
    pub fn inv_fld(
        &mut self,
    ) -> InvFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec> {
        InvFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write Protection Enable Bit: When set to 1, any AHB write access with an address within the protection region defined in the lower and upper write protection registers is rejected. An AHB error response is generated and an interrupt source triggered. When set to 0, the protection region is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn enb_fld(
        &mut self,
    ) -> EnbFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec> {
        EnbFldW::new(self, 1)
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_wr_prot_ctrl_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_wr_prot_ctrl_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWrProtCtrlRegSpec
{
    const RESET_VALUE: u32 = 0;
}
