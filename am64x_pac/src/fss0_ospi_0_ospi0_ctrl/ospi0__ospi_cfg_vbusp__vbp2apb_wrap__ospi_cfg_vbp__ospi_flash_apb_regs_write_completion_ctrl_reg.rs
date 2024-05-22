#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_write_completion_ctrl_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_write_completion_ctrl_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec>;
#[doc = "Field `OPCODE_FLD` reader - 7:0\\]
Defines the opcode that should be issued by the controller when it is automatically polling for device program completion. This command is issued followed all device write operations. By default, this will poll the standard device STATUS register using opcode 0x05"]
pub type OpcodeFldR = crate::FieldReader;
#[doc = "Field `OPCODE_FLD` writer - 7:0\\]
Defines the opcode that should be issued by the controller when it is automatically polling for device program completion. This command is issued followed all device write operations. By default, this will poll the standard device STATUS register using opcode 0x05"]
pub type OpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLLING_BIT_INDEX_FLD` reader - 10:8\\]
Defines the bit index that should be polled. A value of 010 means that bit 2 of the returned data will be polled for.A value of 111 means that bit 7 of the returned data will be polled for."]
pub type PollingBitIndexFldR = crate::FieldReader;
#[doc = "Field `POLLING_BIT_INDEX_FLD` writer - 10:8\\]
Defines the bit index that should be polled. A value of 010 means that bit 2 of the returned data will be polled for.A value of 111 means that bit 7 of the returned data will be polled for."]
pub type PollingBitIndexFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POLLING_POLARITY_FLD` reader - 13:13\\]
Defines the polling polarity. If '1', then the write transfer to the device will be complete if the polled bit is equal to '1'. If '0', then the write transfer to the device will be complete if the polled bit is equal to '0'."]
pub type PollingPolarityFldR = crate::BitReader;
#[doc = "Field `POLLING_POLARITY_FLD` writer - 13:13\\]
Defines the polling polarity. If '1', then the write transfer to the device will be complete if the polled bit is equal to '1'. If '0', then the write transfer to the device will be complete if the polled bit is equal to '0'."]
pub type PollingPolarityFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_POLLING_FLD` reader - 14:14\\]
This switches off the automatic polling function"]
pub type DisablePollingFldR = crate::BitReader;
#[doc = "Field `DISABLE_POLLING_FLD` writer - 14:14\\]
This switches off the automatic polling function"]
pub type DisablePollingFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_POLLING_EXP_FLD` reader - 15:15\\]
Set to '1' for enabling auto-polling expiration."]
pub type EnablePollingExpFldR = crate::BitReader;
#[doc = "Field `ENABLE_POLLING_EXP_FLD` writer - 15:15\\]
Set to '1' for enabling auto-polling expiration."]
pub type EnablePollingExpFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLL_COUNT_FLD` reader - 23:16\\]
Defines the number of times the controller should expect to see a true result from the polling in successive reads of the device register."]
pub type PollCountFldR = crate::FieldReader;
#[doc = "Field `POLL_COUNT_FLD` writer - 23:16\\]
Defines the number of times the controller should expect to see a true result from the polling in successive reads of the device register."]
pub type PollCountFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLL_REP_DELAY_FLD` reader - 31:24\\]
Defines additional delay for maintain Chip Select de-asserted during auto-polling phase"]
pub type PollRepDelayFldR = crate::FieldReader;
#[doc = "Field `POLL_REP_DELAY_FLD` writer - 31:24\\]
Defines additional delay for maintain Chip Select de-asserted during auto-polling phase"]
pub type PollRepDelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the opcode that should be issued by the controller when it is automatically polling for device program completion. This command is issued followed all device write operations. By default, this will poll the standard device STATUS register using opcode 0x05"]
    #[inline(always)]
    pub fn opcode_fld(&self) -> OpcodeFldR {
        OpcodeFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the bit index that should be polled. A value of 010 means that bit 2 of the returned data will be polled for.A value of 111 means that bit 7 of the returned data will be polled for."]
    #[inline(always)]
    pub fn polling_bit_index_fld(&self) -> PollingBitIndexFldR {
        PollingBitIndexFldR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Defines the polling polarity. If '1', then the write transfer to the device will be complete if the polled bit is equal to '1'. If '0', then the write transfer to the device will be complete if the polled bit is equal to '0'."]
    #[inline(always)]
    pub fn polling_polarity_fld(&self) -> PollingPolarityFldR {
        PollingPolarityFldR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This switches off the automatic polling function"]
    #[inline(always)]
    pub fn disable_polling_fld(&self) -> DisablePollingFldR {
        DisablePollingFldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Set to '1' for enabling auto-polling expiration."]
    #[inline(always)]
    pub fn enable_polling_exp_fld(&self) -> EnablePollingExpFldR {
        EnablePollingExpFldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the number of times the controller should expect to see a true result from the polling in successive reads of the device register."]
    #[inline(always)]
    pub fn poll_count_fld(&self) -> PollCountFldR {
        PollCountFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines additional delay for maintain Chip Select de-asserted during auto-polling phase"]
    #[inline(always)]
    pub fn poll_rep_delay_fld(&self) -> PollRepDelayFldR {
        PollRepDelayFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Defines the opcode that should be issued by the controller when it is automatically polling for device program completion. This command is issued followed all device write operations. By default, this will poll the standard device STATUS register using opcode 0x05"]
    #[inline(always)]
    #[must_use]
    pub fn opcode_fld(
        &mut self,
    ) -> OpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        OpcodeFldW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the bit index that should be polled. A value of 010 means that bit 2 of the returned data will be polled for.A value of 111 means that bit 7 of the returned data will be polled for."]
    #[inline(always)]
    #[must_use]
    pub fn polling_bit_index_fld(
        &mut self,
    ) -> PollingBitIndexFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        PollingBitIndexFldW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Defines the polling polarity. If '1', then the write transfer to the device will be complete if the polled bit is equal to '1'. If '0', then the write transfer to the device will be complete if the polled bit is equal to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn polling_polarity_fld(
        &mut self,
    ) -> PollingPolarityFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        PollingPolarityFldW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
This switches off the automatic polling function"]
    #[inline(always)]
    #[must_use]
    pub fn disable_polling_fld(
        &mut self,
    ) -> DisablePollingFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        DisablePollingFldW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Set to '1' for enabling auto-polling expiration."]
    #[inline(always)]
    #[must_use]
    pub fn enable_polling_exp_fld(
        &mut self,
    ) -> EnablePollingExpFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        EnablePollingExpFldW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the number of times the controller should expect to see a true result from the polling in successive reads of the device register."]
    #[inline(always)]
    #[must_use]
    pub fn poll_count_fld(
        &mut self,
    ) -> PollCountFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        PollCountFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines additional delay for maintain Chip Select de-asserted during auto-polling phase"]
    #[inline(always)]
    #[must_use]
    pub fn poll_rep_delay_fld(
        &mut self,
    ) -> PollRepDelayFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec,
    > {
        PollRepDelayFldW::new(self, 24)
    }
}
#[doc = "Write Completion Control Register: This register defines how the controller will poll the device following a write transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_write_completion_ctrl_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_write_completion_ctrl_reg to value 0x0001_0005"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsWriteCompletionCtrlRegSpec
{
    const RESET_VALUE: u32 = 0x0001_0005;
}
