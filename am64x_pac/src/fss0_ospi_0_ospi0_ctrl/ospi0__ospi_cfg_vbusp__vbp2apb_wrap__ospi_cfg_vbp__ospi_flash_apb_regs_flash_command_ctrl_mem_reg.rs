#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_command_ctrl_mem_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_command_ctrl_mem_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec>;
#[doc = "Field `TRIGGER_MEM_BANK_REQ_FLD` reader - 0:0\\]
Trigger the Memory Bank data request."]
pub type TriggerMemBankReqFldR = crate::BitReader;
#[doc = "Field `TRIGGER_MEM_BANK_REQ_FLD` writer - 0:0\\]
Trigger the Memory Bank data request."]
pub type TriggerMemBankReqFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BANK_REQ_IN_PROGRESS_FLD` reader - 1:1\\]
Memory Bank data request in progress."]
pub type MemBankReqInProgressFldR = crate::BitReader;
#[doc = "Field `MEM_BANK_REQ_IN_PROGRESS_FLD` writer - 1:1\\]
Memory Bank data request in progress."]
pub type MemBankReqInProgressFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BANK_READ_DATA_FLD` reader - 15:8\\]
Last requested data from the STIG Memory Bank."]
pub type MemBankReadDataFldR = crate::FieldReader;
#[doc = "Field `MEM_BANK_READ_DATA_FLD` writer - 15:8\\]
Last requested data from the STIG Memory Bank."]
pub type MemBankReadDataFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NB_OF_STIG_READ_BYTES_FLD` reader - 18:16\\]
It defines the number of read bytes for the extended STIG."]
pub type NbOfStigReadBytesFldR = crate::FieldReader;
#[doc = "Field `NB_OF_STIG_READ_BYTES_FLD` writer - 18:16\\]
It defines the number of read bytes for the extended STIG."]
pub type NbOfStigReadBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MEM_BANK_ADDR_FLD` reader - 28:20\\]
The address of the Memory Bank which data will be read from."]
pub type MemBankAddrFldR = crate::FieldReader<u16>;
#[doc = "Field `MEM_BANK_ADDR_FLD` writer - 28:20\\]
The address of the Memory Bank which data will be read from."]
pub type MemBankAddrFldW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trigger the Memory Bank data request."]
    #[inline(always)]
    pub fn trigger_mem_bank_req_fld(&self) -> TriggerMemBankReqFldR {
        TriggerMemBankReqFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Bank data request in progress."]
    #[inline(always)]
    pub fn mem_bank_req_in_progress_fld(&self) -> MemBankReqInProgressFldR {
        MemBankReqInProgressFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Last requested data from the STIG Memory Bank."]
    #[inline(always)]
    pub fn mem_bank_read_data_fld(&self) -> MemBankReadDataFldR {
        MemBankReadDataFldR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
It defines the number of read bytes for the extended STIG."]
    #[inline(always)]
    pub fn nb_of_stig_read_bytes_fld(&self) -> NbOfStigReadBytesFldR {
        NbOfStigReadBytesFldR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:28 - 28:20\\]
The address of the Memory Bank which data will be read from."]
    #[inline(always)]
    pub fn mem_bank_addr_fld(&self) -> MemBankAddrFldR {
        MemBankAddrFldR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trigger the Memory Bank data request."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_mem_bank_req_fld(
        &mut self,
    ) -> TriggerMemBankReqFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec,
    > {
        TriggerMemBankReqFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Memory Bank data request in progress."]
    #[inline(always)]
    #[must_use]
    pub fn mem_bank_req_in_progress_fld(
        &mut self,
    ) -> MemBankReqInProgressFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec,
    > {
        MemBankReqInProgressFldW::new(self, 1)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Last requested data from the STIG Memory Bank."]
    #[inline(always)]
    #[must_use]
    pub fn mem_bank_read_data_fld(
        &mut self,
    ) -> MemBankReadDataFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec,
    > {
        MemBankReadDataFldW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
It defines the number of read bytes for the extended STIG."]
    #[inline(always)]
    #[must_use]
    pub fn nb_of_stig_read_bytes_fld(
        &mut self,
    ) -> NbOfStigReadBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec,
    > {
        NbOfStigReadBytesFldW::new(self, 16)
    }
    #[doc = "Bits 20:28 - 28:20\\]
The address of the Memory Bank which data will be read from."]
    #[inline(always)]
    #[must_use]
    pub fn mem_bank_addr_fld(
        &mut self,
    ) -> MemBankAddrFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec,
    > {
        MemBankAddrFldW::new(self, 20)
    }
}
#[doc = "Flash Command Control Memory Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_command_ctrl_mem_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_command_ctrl_mem_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCommandCtrlMemRegSpec
{
    const RESET_VALUE: u32 = 0;
}
