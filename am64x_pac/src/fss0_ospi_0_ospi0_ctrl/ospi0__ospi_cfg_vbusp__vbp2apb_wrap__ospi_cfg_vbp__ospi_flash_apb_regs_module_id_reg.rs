#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_module_id_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_module_id_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec>;
#[doc = "Field `CONF_FLD` reader - 1:0\\]
Configuration ID number: 0 : OCTAL + PHY Configuration 1 : OCTAL Configuration 2 : QUAD + PHY Configuration 3 : QUAD Configuration"]
pub type ConfFldR = crate::FieldReader;
#[doc = "Field `CONF_FLD` writer - 1:0\\]
Configuration ID number: 0 : OCTAL + PHY Configuration 1 : OCTAL Configuration 2 : QUAD + PHY Configuration 3 : QUAD Configuration"]
pub type ConfFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODULE_ID_FLD` reader - 23:8\\]
Module/Revision ID number"]
pub type ModuleIdFldR = crate::FieldReader<u16>;
#[doc = "Field `MODULE_ID_FLD` writer - 23:8\\]
Module/Revision ID number"]
pub type ModuleIdFldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FIX_PATCH_FLD` reader - 31:24\\]
Fix/path number related to revision described by 3 LSBs of this register"]
pub type FixPatchFldR = crate::FieldReader;
#[doc = "Field `FIX_PATCH_FLD` writer - 31:24\\]
Fix/path number related to revision described by 3 LSBs of this register"]
pub type FixPatchFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Configuration ID number: 0 : OCTAL + PHY Configuration 1 : OCTAL Configuration 2 : QUAD + PHY Configuration 3 : QUAD Configuration"]
    #[inline(always)]
    pub fn conf_fld(&self) -> ConfFldR {
        ConfFldR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Module/Revision ID number"]
    #[inline(always)]
    pub fn module_id_fld(&self) -> ModuleIdFldR {
        ModuleIdFldR::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fix/path number related to revision described by 3 LSBs of this register"]
    #[inline(always)]
    pub fn fix_patch_fld(&self) -> FixPatchFldR {
        FixPatchFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Configuration ID number: 0 : OCTAL + PHY Configuration 1 : OCTAL Configuration 2 : QUAD + PHY Configuration 3 : QUAD Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn conf_fld(
        &mut self,
    ) -> ConfFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec> {
        ConfFldW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Module/Revision ID number"]
    #[inline(always)]
    #[must_use]
    pub fn module_id_fld(
        &mut self,
    ) -> ModuleIdFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec>
    {
        ModuleIdFldW::new(self, 8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Fix/path number related to revision described by 3 LSBs of this register"]
    #[inline(always)]
    #[must_use]
    pub fn fix_patch_fld(
        &mut self,
    ) -> FixPatchFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec>
    {
        FixPatchFldW::new(self, 24)
    }
}
#[doc = "Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_module_id_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_module_id_reg to value 0x0300_0300"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModuleIdRegSpec
{
    const RESET_VALUE: u32 = 0x0300_0300;
}
