#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_ctrl_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_ctrl_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec>;
#[doc = "Field `CMD_EXEC_FLD` reader - 0:0\\]
Execute the command."]
pub type CmdExecFldR = crate::BitReader;
#[doc = "Field `CMD_EXEC_FLD` writer - 0:0\\]
Execute the command."]
pub type CmdExecFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_EXEC_STATUS_FLD` reader - 1:1\\]
Command execution in progress."]
pub type CmdExecStatusFldR = crate::BitReader;
#[doc = "Field `CMD_EXEC_STATUS_FLD` writer - 1:1\\]
Command execution in progress."]
pub type CmdExecStatusFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STIG_MEM_BANK_EN_FLD` reader - 2:2\\]
STIG Memory Bank enable bit."]
pub type StigMemBankEnFldR = crate::BitReader;
#[doc = "Field `STIG_MEM_BANK_EN_FLD` writer - 2:2\\]
STIG Memory Bank enable bit."]
pub type StigMemBankEnFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_DUMMY_CYCLES_FLD` reader - 11:7\\]
Number of Dummy cycles: Set to the number of dummy cycles required. This should be setup before triggering the command via the execute field of this register."]
pub type NumDummyCyclesFldR = crate::FieldReader;
#[doc = "Field `NUM_DUMMY_CYCLES_FLD` writer - 11:7\\]
Number of Dummy cycles: Set to the number of dummy cycles required. This should be setup before triggering the command via the execute field of this register."]
pub type NumDummyCyclesFldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NUM_WR_DATA_BYTES_FLD` reader - 14:12\\]
Number of Write Data Bytes: Up to 8 Data bytes may be written using this command Set to 0 for 1 byte, 7 for 8 bytes."]
pub type NumWrDataBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_WR_DATA_BYTES_FLD` writer - 14:12\\]
Number of Write Data Bytes: Up to 8 Data bytes may be written using this command Set to 0 for 1 byte, 7 for 8 bytes."]
pub type NumWrDataBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENB_WRITE_DATA_FLD` reader - 15:15\\]
Write Data Enable: Set to 1 if the command specified in the command opcode field requires write data bytes to be sent to the device."]
pub type EnbWriteDataFldR = crate::BitReader;
#[doc = "Field `ENB_WRITE_DATA_FLD` writer - 15:15\\]
Write Data Enable: Set to 1 if the command specified in the command opcode field requires write data bytes to be sent to the device."]
pub type EnbWriteDataFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_ADDR_BYTES_FLD` reader - 17:16\\]
Number of Address Bytes: Set to the number of address bytes required \\[the address itself is programmed in the FLASH COMMAND ADDRESS REGISTERS\\]. This should be setup before triggering the command via bit 0 of this register. 2'b00 : 1 address byte 2'b01 : 2 address bytes 2'b10 : 3 address bytes 2'b11 : 4 address bytes"]
pub type NumAddrBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_ADDR_BYTES_FLD` writer - 17:16\\]
Number of Address Bytes: Set to the number of address bytes required \\[the address itself is programmed in the FLASH COMMAND ADDRESS REGISTERS\\]. This should be setup before triggering the command via bit 0 of this register. 2'b00 : 1 address byte 2'b01 : 2 address bytes 2'b10 : 3 address bytes 2'b11 : 4 address bytes"]
pub type NumAddrBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENB_MODE_BIT_FLD` reader - 18:18\\]
Mode Bit Enable: Set to 1 to ensure the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
pub type EnbModeBitFldR = crate::BitReader;
#[doc = "Field `ENB_MODE_BIT_FLD` writer - 18:18\\]
Mode Bit Enable: Set to 1 to ensure the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
pub type EnbModeBitFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_COMD_ADDR_FLD` reader - 19:19\\]
Command Address Enable: Set to 1 if the command specified in bits 31:24 requires an address. This should be setup before triggering the command via writing a 1 to the execute field."]
pub type EnbComdAddrFldR = crate::BitReader;
#[doc = "Field `ENB_COMD_ADDR_FLD` writer - 19:19\\]
Command Address Enable: Set to 1 if the command specified in bits 31:24 requires an address. This should be setup before triggering the command via writing a 1 to the execute field."]
pub type EnbComdAddrFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_RD_DATA_BYTES_FLD` reader - 22:20\\]
Number of Read Data Bytes: Up to 8 data bytes may be read using this command. Set to 0 for 1 byte and 7 for 8 bytes."]
pub type NumRdDataBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_RD_DATA_BYTES_FLD` writer - 22:20\\]
Number of Read Data Bytes: Up to 8 data bytes may be read using this command. Set to 0 for 1 byte and 7 for 8 bytes."]
pub type NumRdDataBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENB_READ_DATA_FLD` reader - 23:23\\]
Read Data Enable: Set to 1 if the command specified in the command opcode field \\[bits 31:24\\]
requires read data bytes to be received from the device."]
pub type EnbReadDataFldR = crate::BitReader;
#[doc = "Field `ENB_READ_DATA_FLD` writer - 23:23\\]
Read Data Enable: Set to 1 if the command specified in the command opcode field \\[bits 31:24\\]
requires read data bytes to be received from the device."]
pub type EnbReadDataFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_OPCODE_FLD` reader - 31:24\\]
Command Opcode: The command opcode field should be setup before triggering the command. For example, 0x20 maps to SubSector Erase. Writing to the execute field \\[bit 0\\]
of this register launches the command. NOTE : Using this approach to issue commands to the device will make use of the instruction type of the device instruction configuration register. If this field is set to 2'b00, then the command opcode, command address, command dummy bytes and command data will all be transferred in a serial fashion. If this field is set to 2'b01, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0 and DQ1 pins. If this field is set to 2'b10, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0, DQ1, DQ2 and DQ3 pins."]
pub type CmdOpcodeFldR = crate::FieldReader;
#[doc = "Field `CMD_OPCODE_FLD` writer - 31:24\\]
Command Opcode: The command opcode field should be setup before triggering the command. For example, 0x20 maps to SubSector Erase. Writing to the execute field \\[bit 0\\]
of this register launches the command. NOTE : Using this approach to issue commands to the device will make use of the instruction type of the device instruction configuration register. If this field is set to 2'b00, then the command opcode, command address, command dummy bytes and command data will all be transferred in a serial fashion. If this field is set to 2'b01, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0 and DQ1 pins. If this field is set to 2'b10, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0, DQ1, DQ2 and DQ3 pins."]
pub type CmdOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Execute the command."]
    #[inline(always)]
    pub fn cmd_exec_fld(&self) -> CmdExecFldR {
        CmdExecFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command execution in progress."]
    #[inline(always)]
    pub fn cmd_exec_status_fld(&self) -> CmdExecStatusFldR {
        CmdExecStatusFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
STIG Memory Bank enable bit."]
    #[inline(always)]
    pub fn stig_mem_bank_en_fld(&self) -> StigMemBankEnFldR {
        StigMemBankEnFldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:11 - 11:7\\]
Number of Dummy cycles: Set to the number of dummy cycles required. This should be setup before triggering the command via the execute field of this register."]
    #[inline(always)]
    pub fn num_dummy_cycles_fld(&self) -> NumDummyCyclesFldR {
        NumDummyCyclesFldR::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Number of Write Data Bytes: Up to 8 Data bytes may be written using this command Set to 0 for 1 byte, 7 for 8 bytes."]
    #[inline(always)]
    pub fn num_wr_data_bytes_fld(&self) -> NumWrDataBytesFldR {
        NumWrDataBytesFldR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Write Data Enable: Set to 1 if the command specified in the command opcode field requires write data bytes to be sent to the device."]
    #[inline(always)]
    pub fn enb_write_data_fld(&self) -> EnbWriteDataFldR {
        EnbWriteDataFldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of Address Bytes: Set to the number of address bytes required \\[the address itself is programmed in the FLASH COMMAND ADDRESS REGISTERS\\]. This should be setup before triggering the command via bit 0 of this register. 2'b00 : 1 address byte 2'b01 : 2 address bytes 2'b10 : 3 address bytes 2'b11 : 4 address bytes"]
    #[inline(always)]
    pub fn num_addr_bytes_fld(&self) -> NumAddrBytesFldR {
        NumAddrBytesFldR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Mode Bit Enable: Set to 1 to ensure the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
    #[inline(always)]
    pub fn enb_mode_bit_fld(&self) -> EnbModeBitFldR {
        EnbModeBitFldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Command Address Enable: Set to 1 if the command specified in bits 31:24 requires an address. This should be setup before triggering the command via writing a 1 to the execute field."]
    #[inline(always)]
    pub fn enb_comd_addr_fld(&self) -> EnbComdAddrFldR {
        EnbComdAddrFldR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Number of Read Data Bytes: Up to 8 data bytes may be read using this command. Set to 0 for 1 byte and 7 for 8 bytes."]
    #[inline(always)]
    pub fn num_rd_data_bytes_fld(&self) -> NumRdDataBytesFldR {
        NumRdDataBytesFldR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Read Data Enable: Set to 1 if the command specified in the command opcode field \\[bits 31:24\\]
requires read data bytes to be received from the device."]
    #[inline(always)]
    pub fn enb_read_data_fld(&self) -> EnbReadDataFldR {
        EnbReadDataFldR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Command Opcode: The command opcode field should be setup before triggering the command. For example, 0x20 maps to SubSector Erase. Writing to the execute field \\[bit 0\\]
of this register launches the command. NOTE : Using this approach to issue commands to the device will make use of the instruction type of the device instruction configuration register. If this field is set to 2'b00, then the command opcode, command address, command dummy bytes and command data will all be transferred in a serial fashion. If this field is set to 2'b01, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0 and DQ1 pins. If this field is set to 2'b10, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0, DQ1, DQ2 and DQ3 pins."]
    #[inline(always)]
    pub fn cmd_opcode_fld(&self) -> CmdOpcodeFldR {
        CmdOpcodeFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Execute the command."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_exec_fld(
        &mut self,
    ) -> CmdExecFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec>
    {
        CmdExecFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command execution in progress."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_exec_status_fld(
        &mut self,
    ) -> CmdExecStatusFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        CmdExecStatusFldW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
STIG Memory Bank enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn stig_mem_bank_en_fld(
        &mut self,
    ) -> StigMemBankEnFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        StigMemBankEnFldW::new(self, 2)
    }
    #[doc = "Bits 7:11 - 11:7\\]
Number of Dummy cycles: Set to the number of dummy cycles required. This should be setup before triggering the command via the execute field of this register."]
    #[inline(always)]
    #[must_use]
    pub fn num_dummy_cycles_fld(
        &mut self,
    ) -> NumDummyCyclesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        NumDummyCyclesFldW::new(self, 7)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Number of Write Data Bytes: Up to 8 Data bytes may be written using this command Set to 0 for 1 byte, 7 for 8 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn num_wr_data_bytes_fld(
        &mut self,
    ) -> NumWrDataBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        NumWrDataBytesFldW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
Write Data Enable: Set to 1 if the command specified in the command opcode field requires write data bytes to be sent to the device."]
    #[inline(always)]
    #[must_use]
    pub fn enb_write_data_fld(
        &mut self,
    ) -> EnbWriteDataFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        EnbWriteDataFldW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Number of Address Bytes: Set to the number of address bytes required \\[the address itself is programmed in the FLASH COMMAND ADDRESS REGISTERS\\]. This should be setup before triggering the command via bit 0 of this register. 2'b00 : 1 address byte 2'b01 : 2 address bytes 2'b10 : 3 address bytes 2'b11 : 4 address bytes"]
    #[inline(always)]
    #[must_use]
    pub fn num_addr_bytes_fld(
        &mut self,
    ) -> NumAddrBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        NumAddrBytesFldW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Mode Bit Enable: Set to 1 to ensure the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
    #[inline(always)]
    #[must_use]
    pub fn enb_mode_bit_fld(
        &mut self,
    ) -> EnbModeBitFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec>
    {
        EnbModeBitFldW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Command Address Enable: Set to 1 if the command specified in bits 31:24 requires an address. This should be setup before triggering the command via writing a 1 to the execute field."]
    #[inline(always)]
    #[must_use]
    pub fn enb_comd_addr_fld(
        &mut self,
    ) -> EnbComdAddrFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        EnbComdAddrFldW::new(self, 19)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Number of Read Data Bytes: Up to 8 data bytes may be read using this command. Set to 0 for 1 byte and 7 for 8 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn num_rd_data_bytes_fld(
        &mut self,
    ) -> NumRdDataBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        NumRdDataBytesFldW::new(self, 20)
    }
    #[doc = "Bit 23 - 23:23\\]
Read Data Enable: Set to 1 if the command specified in the command opcode field \\[bits 31:24\\]
requires read data bytes to be received from the device."]
    #[inline(always)]
    #[must_use]
    pub fn enb_read_data_fld(
        &mut self,
    ) -> EnbReadDataFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec,
    > {
        EnbReadDataFldW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Command Opcode: The command opcode field should be setup before triggering the command. For example, 0x20 maps to SubSector Erase. Writing to the execute field \\[bit 0\\]
of this register launches the command. NOTE : Using this approach to issue commands to the device will make use of the instruction type of the device instruction configuration register. If this field is set to 2'b00, then the command opcode, command address, command dummy bytes and command data will all be transferred in a serial fashion. If this field is set to 2'b01, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0 and DQ1 pins. If this field is set to 2'b10, then the command opcode, command address, command dummy bytes and command data will all be transferred in parallel using DQ0, DQ1, DQ2 and DQ3 pins."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_opcode_fld(
        &mut self,
    ) -> CmdOpcodeFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec>
    {
        CmdOpcodeFldW::new(self, 24)
    }
}
#[doc = "Flash Command Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_flash_cmd_ctrl_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_flash_cmd_ctrl_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsFlashCmdCtrlRegSpec
{
    const RESET_VALUE: u32 = 0;
}
