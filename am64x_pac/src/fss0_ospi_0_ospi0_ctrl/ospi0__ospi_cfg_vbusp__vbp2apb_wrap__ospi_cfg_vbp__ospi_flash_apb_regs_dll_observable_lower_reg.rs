#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_lower_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_lower_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_FLD` reader - 0:0\\]
Indicates status of DLL."]
pub type DllObservableLowerDllLockFldR = crate::BitReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_FLD` writer - 0:0\\]
Indicates status of DLL."]
pub type DllObservableLowerDllLockFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOCK_MODE_FLD` reader - 2:1\\]
Defines the mode in which the DLL has achieved the lock."]
pub type DllObservableLowerLockModeFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOCK_MODE_FLD` writer - 2:1\\]
Defines the mode in which the DLL has achieved the lock."]
pub type DllObservableLowerLockModeFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_UNLOCK_COUNTER_FLD` reader - 7:3\\]
Reports the number of increments or decrements required for the master DLL to complete the locking process."]
pub type DllObservableLowerUnlockCounterFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_UNLOCK_COUNTER_FLD` writer - 7:3\\]
Reports the number of increments or decrements required for the master DLL to complete the locking process."]
pub type DllObservableLowerUnlockCounterFldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOCK_VALUE_FLD` reader - 14:8\\]
Reports the DLL encoder value from the master DLL to the slave DLLs."]
pub type DllObservableLowerLockValueFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOCK_VALUE_FLD` writer - 14:8\\]
Reports the DLL encoder value from the master DLL to the slave DLLs."]
pub type DllObservableLowerLockValueFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOOPBACK_LOCK_FLD` reader - 15:15\\]
This bit indicates that lock of loopback is done."]
pub type DllObservableLowerLoopbackLockFldR = crate::BitReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_LOOPBACK_LOCK_FLD` writer - 15:15\\]
This bit indicates that lock of loopback is done."]
pub type DllObservableLowerLoopbackLockFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_DEC_FLD` reader - 23:16\\]
Holds the state of the cumulative dll_lock_dec register."]
pub type DllObservableLowerDllLockDecFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_DEC_FLD` writer - 23:16\\]
Holds the state of the cumulative dll_lock_dec register."]
pub type DllObservableLowerDllLockDecFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_INC_FLD` reader - 31:24\\]
Holds the state of the cumulative dll_lock_inc register."]
pub type DllObservableLowerDllLockIncFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_LOWER_DLL_LOCK_INC_FLD` writer - 31:24\\]
Holds the state of the cumulative dll_lock_inc register."]
pub type DllObservableLowerDllLockIncFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates status of DLL."]
    #[inline(always)]
    pub fn dll_observable_lower_dll_lock_fld(&self) -> DllObservableLowerDllLockFldR {
        DllObservableLowerDllLockFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the mode in which the DLL has achieved the lock."]
    #[inline(always)]
    pub fn dll_observable_lower_lock_mode_fld(&self) -> DllObservableLowerLockModeFldR {
        DllObservableLowerLockModeFldR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reports the number of increments or decrements required for the master DLL to complete the locking process."]
    #[inline(always)]
    pub fn dll_observable_lower_unlock_counter_fld(&self) -> DllObservableLowerUnlockCounterFldR {
        DllObservableLowerUnlockCounterFldR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Reports the DLL encoder value from the master DLL to the slave DLLs."]
    #[inline(always)]
    pub fn dll_observable_lower_lock_value_fld(&self) -> DllObservableLowerLockValueFldR {
        DllObservableLowerLockValueFldR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit indicates that lock of loopback is done."]
    #[inline(always)]
    pub fn dll_observable_lower_loopback_lock_fld(&self) -> DllObservableLowerLoopbackLockFldR {
        DllObservableLowerLoopbackLockFldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Holds the state of the cumulative dll_lock_dec register."]
    #[inline(always)]
    pub fn dll_observable_lower_dll_lock_dec_fld(&self) -> DllObservableLowerDllLockDecFldR {
        DllObservableLowerDllLockDecFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Holds the state of the cumulative dll_lock_inc register."]
    #[inline(always)]
    pub fn dll_observable_lower_dll_lock_inc_fld(&self) -> DllObservableLowerDllLockIncFldR {
        DllObservableLowerDllLockIncFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates status of DLL."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_dll_lock_fld(
        &mut self,
    ) -> DllObservableLowerDllLockFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerDllLockFldW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the mode in which the DLL has achieved the lock."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_lock_mode_fld(
        &mut self,
    ) -> DllObservableLowerLockModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerLockModeFldW::new(self, 1)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Reports the number of increments or decrements required for the master DLL to complete the locking process."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_unlock_counter_fld(
        &mut self,
    ) -> DllObservableLowerUnlockCounterFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerUnlockCounterFldW::new(self, 3)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Reports the DLL encoder value from the master DLL to the slave DLLs."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_lock_value_fld(
        &mut self,
    ) -> DllObservableLowerLockValueFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerLockValueFldW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
This bit indicates that lock of loopback is done."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_loopback_lock_fld(
        &mut self,
    ) -> DllObservableLowerLoopbackLockFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerLoopbackLockFldW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Holds the state of the cumulative dll_lock_dec register."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_dll_lock_dec_fld(
        &mut self,
    ) -> DllObservableLowerDllLockDecFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerDllLockDecFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Holds the state of the cumulative dll_lock_inc register."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_lower_dll_lock_inc_fld(
        &mut self,
    ) -> DllObservableLowerDllLockIncFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec,
    > {
        DllObservableLowerDllLockIncFldW::new(self, 24)
    }
}
#[doc = "DLL Observable Register Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_lower_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_lower_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableLowerRegSpec
{
    const RESET_VALUE: u32 = 0;
}
