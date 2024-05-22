#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_master_control_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_master_control_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec>;
#[doc = "Field `PHY_MASTER_INITIAL_DELAY_FLD` reader - 6:0\\]
This value is the initial delay value for the DLL."]
pub type PhyMasterInitialDelayFldR = crate::FieldReader;
#[doc = "Field `PHY_MASTER_INITIAL_DELAY_FLD` writer - 6:0\\]
This value is the initial delay value for the DLL."]
pub type PhyMasterInitialDelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_MASTER_NB_INDICATIONS_FLD` reader - 18:16\\]
Holds the number of consecutive increment or decrement indications."]
pub type PhyMasterNbIndicationsFldR = crate::FieldReader;
#[doc = "Field `PHY_MASTER_NB_INDICATIONS_FLD` writer - 18:16\\]
Holds the number of consecutive increment or decrement indications."]
pub type PhyMasterNbIndicationsFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_MASTER_PHASE_DETECT_SELECTOR_FLD` reader - 22:20\\]
Selects the number of delay elements to be inserted between the phase detect flip-flops."]
pub type PhyMasterPhaseDetectSelectorFldR = crate::FieldReader;
#[doc = "Field `PHY_MASTER_PHASE_DETECT_SELECTOR_FLD` writer - 22:20\\]
Selects the number of delay elements to be inserted between the phase detect flip-flops."]
pub type PhyMasterPhaseDetectSelectorFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_MASTER_BYPASS_MODE_FLD` reader - 23:23\\]
Controls the bypass mode of the master and slave DLLs."]
pub type PhyMasterBypassModeFldR = crate::BitReader;
#[doc = "Field `PHY_MASTER_BYPASS_MODE_FLD` writer - 23:23\\]
Controls the bypass mode of the master and slave DLLs."]
pub type PhyMasterBypassModeFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MASTER_LOCK_MODE_FLD` reader - 24:24\\]
Determines if the master delay line locks on a full cycle or half cycle of delay."]
pub type PhyMasterLockModeFldR = crate::BitReader;
#[doc = "Field `PHY_MASTER_LOCK_MODE_FLD` writer - 24:24\\]
Determines if the master delay line locks on a full cycle or half cycle of delay."]
pub type PhyMasterLockModeFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
This value is the initial delay value for the DLL."]
    #[inline(always)]
    pub fn phy_master_initial_delay_fld(&self) -> PhyMasterInitialDelayFldR {
        PhyMasterInitialDelayFldR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Holds the number of consecutive increment or decrement indications."]
    #[inline(always)]
    pub fn phy_master_nb_indications_fld(&self) -> PhyMasterNbIndicationsFldR {
        PhyMasterNbIndicationsFldR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Selects the number of delay elements to be inserted between the phase detect flip-flops."]
    #[inline(always)]
    pub fn phy_master_phase_detect_selector_fld(&self) -> PhyMasterPhaseDetectSelectorFldR {
        PhyMasterPhaseDetectSelectorFldR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Controls the bypass mode of the master and slave DLLs."]
    #[inline(always)]
    pub fn phy_master_bypass_mode_fld(&self) -> PhyMasterBypassModeFldR {
        PhyMasterBypassModeFldR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Determines if the master delay line locks on a full cycle or half cycle of delay."]
    #[inline(always)]
    pub fn phy_master_lock_mode_fld(&self) -> PhyMasterLockModeFldR {
        PhyMasterLockModeFldR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
This value is the initial delay value for the DLL."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_initial_delay_fld(
        &mut self,
    ) -> PhyMasterInitialDelayFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec,
    > {
        PhyMasterInitialDelayFldW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Holds the number of consecutive increment or decrement indications."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_nb_indications_fld(
        &mut self,
    ) -> PhyMasterNbIndicationsFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec,
    > {
        PhyMasterNbIndicationsFldW::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Selects the number of delay elements to be inserted between the phase detect flip-flops."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_phase_detect_selector_fld(
        &mut self,
    ) -> PhyMasterPhaseDetectSelectorFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec,
    > {
        PhyMasterPhaseDetectSelectorFldW::new(self, 20)
    }
    #[doc = "Bit 23 - 23:23\\]
Controls the bypass mode of the master and slave DLLs."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_bypass_mode_fld(
        &mut self,
    ) -> PhyMasterBypassModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec,
    > {
        PhyMasterBypassModeFldW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Determines if the master delay line locks on a full cycle or half cycle of delay."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_lock_mode_fld(
        &mut self,
    ) -> PhyMasterLockModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec,
    > {
        PhyMasterLockModeFldW::new(self, 24)
    }
}
#[doc = "PHY DLL Master Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_master_control_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_master_control_reg to value 0x0080_0000"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyMasterControlRegSpec
{
    const RESET_VALUE: u32 = 0x0080_0000;
}
