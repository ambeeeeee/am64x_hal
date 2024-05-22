#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_polling_flash_status_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_polling_flash_status_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec>;
#[doc = "Field `DEVICE_STATUS_FLD` reader - 7:0\\]
Defines actual Status Register of Device"]
pub type DeviceStatusFldR = crate::FieldReader;
#[doc = "Field `DEVICE_STATUS_FLD` writer - 7:0\\]
Defines actual Status Register of Device"]
pub type DeviceStatusFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DEVICE_STATUS_VALID_FLD` reader - 8:8\\]
Device Status Valid: This should be set when value in bits from 7 to 0 is valid."]
pub type DeviceStatusValidFldR = crate::BitReader;
#[doc = "Field `DEVICE_STATUS_VALID_FLD` writer - 8:8\\]
Device Status Valid: This should be set when value in bits from 7 to 0 is valid."]
pub type DeviceStatusValidFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVICE_STATUS_NB_DUMMY` reader - 19:16\\]
Number of dummy cycles for auto-polling"]
pub type DeviceStatusNbDummyR = crate::FieldReader;
#[doc = "Field `DEVICE_STATUS_NB_DUMMY` writer - 19:16\\]
Number of dummy cycles for auto-polling"]
pub type DeviceStatusNbDummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines actual Status Register of Device"]
    #[inline(always)]
    pub fn device_status_fld(&self) -> DeviceStatusFldR {
        DeviceStatusFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Device Status Valid: This should be set when value in bits from 7 to 0 is valid."]
    #[inline(always)]
    pub fn device_status_valid_fld(&self) -> DeviceStatusValidFldR {
        DeviceStatusValidFldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of dummy cycles for auto-polling"]
    #[inline(always)]
    pub fn device_status_nb_dummy(&self) -> DeviceStatusNbDummyR {
        DeviceStatusNbDummyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines actual Status Register of Device"]
    #[inline(always)]
    #[must_use]
    pub fn device_status_fld(
        &mut self,
    ) -> DeviceStatusFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec,
    > {
        DeviceStatusFldW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Device Status Valid: This should be set when value in bits from 7 to 0 is valid."]
    #[inline(always)]
    #[must_use]
    pub fn device_status_valid_fld(
        &mut self,
    ) -> DeviceStatusValidFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec,
    > {
        DeviceStatusValidFldW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Number of dummy cycles for auto-polling"]
    #[inline(always)]
    #[must_use]
    pub fn device_status_nb_dummy(
        &mut self,
    ) -> DeviceStatusNbDummyW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec,
    > {
        DeviceStatusNbDummyW::new(self, 16)
    }
}
#[doc = "Polling Flash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_polling_flash_status_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_polling_flash_status_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPollingFlashStatusRegSpec
{
    const RESET_VALUE: u32 = 0;
}
