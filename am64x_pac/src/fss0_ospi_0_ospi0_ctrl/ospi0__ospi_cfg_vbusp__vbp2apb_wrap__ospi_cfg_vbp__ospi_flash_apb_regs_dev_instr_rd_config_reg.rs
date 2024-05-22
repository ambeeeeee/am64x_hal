#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_rd_config_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_rd_config_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec>;
#[doc = "Field `RD_OPCODE_NON_XIP_FLD` reader - 7:0\\]
Read Opcode in non-XIP mode: Read Opcode to use when not in XIP mode"]
pub type RdOpcodeNonXipFldR = crate::FieldReader;
#[doc = "Field `RD_OPCODE_NON_XIP_FLD` writer - 7:0\\]
Read Opcode in non-XIP mode: Read Opcode to use when not in XIP mode"]
pub type RdOpcodeNonXipFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSTR_TYPE_FLD` reader - 9:8\\]
Instruction Type: 0 : Use Standard SPI mode \\[instruction always shifted into the device on DQ0 only\\]
1 : Use DIO-SPI mode \\[Instructions, Address and Data always sent on DQ0 and DQ1\\]
2 : Use QIO-SPI mode \\[Instructions, Address and Data always sent on DQ0, DQ1, DQ2 and DQ3\\]
3 : Use Octal-IO-SPI mode \\[Instructions, Address and Data always sent on DQ\\[7:0\\]\\]"]
pub type InstrTypeFldR = crate::FieldReader;
#[doc = "Field `INSTR_TYPE_FLD` writer - 9:8\\]
Instruction Type: 0 : Use Standard SPI mode \\[instruction always shifted into the device on DQ0 only\\]
1 : Use DIO-SPI mode \\[Instructions, Address and Data always sent on DQ0 and DQ1\\]
2 : Use QIO-SPI mode \\[Instructions, Address and Data always sent on DQ0, DQ1, DQ2 and DQ3\\]
3 : Use Octal-IO-SPI mode \\[Instructions, Address and Data always sent on DQ\\[7:0\\]\\]"]
pub type InstrTypeFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR_EN_FLD` reader - 10:10\\]
DDR Enable: This is to inform that opcode from rd_opcode_non_xip_fld is compliant with one of the DDR READ Commands"]
pub type DdrEnFldR = crate::BitReader;
#[doc = "Field `DDR_EN_FLD` writer - 10:10\\]
DDR Enable: This is to inform that opcode from rd_opcode_non_xip_fld is compliant with one of the DDR READ Commands"]
pub type DdrEnFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_XFER_TYPE_STD_MODE_FLD` reader - 13:12\\]
Address Transfer Type for Standard SPI modes: 0 : Addresses can be shifted to the device on DQ0 only 1 : Addresses can be shifted to the device on DQ0 and DQ1 only 2 : Addresses can be shifted to the device on DQ0, DQ1, DQ2 and DQ3 3 : Addresses can be shifted to the device on DQ\\[7:0\\]"]
pub type AddrXferTypeStdModeFldR = crate::FieldReader;
#[doc = "Field `ADDR_XFER_TYPE_STD_MODE_FLD` writer - 13:12\\]
Address Transfer Type for Standard SPI modes: 0 : Addresses can be shifted to the device on DQ0 only 1 : Addresses can be shifted to the device on DQ0 and DQ1 only 2 : Addresses can be shifted to the device on DQ0, DQ1, DQ2 and DQ3 3 : Addresses can be shifted to the device on DQ\\[7:0\\]"]
pub type AddrXferTypeStdModeFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATA_XFER_TYPE_EXT_MODE_FLD` reader - 17:16\\]
Data Transfer Type for Standard SPI modes: 0 : SIO mode data is shifted to the device on DQ0 only and from the device on DQ1 only 1 : Used for Dual Input/Output instructions. For data transfers, DQ0 and DQ1 are used as both inputs and outputs. 2 : Used for Quad Input/Output instructions. For data transfers, DQ0,DQ1,DQ2 and DQ3 are used as both inputs and outputs. 3 : Used for Quad Input/Output instructions. For data transfers, DQ\\[7:0\\]
are used as both inputs and outputs."]
pub type DataXferTypeExtModeFldR = crate::FieldReader;
#[doc = "Field `DATA_XFER_TYPE_EXT_MODE_FLD` writer - 17:16\\]
Data Transfer Type for Standard SPI modes: 0 : SIO mode data is shifted to the device on DQ0 only and from the device on DQ1 only 1 : Used for Dual Input/Output instructions. For data transfers, DQ0 and DQ1 are used as both inputs and outputs. 2 : Used for Quad Input/Output instructions. For data transfers, DQ0,DQ1,DQ2 and DQ3 are used as both inputs and outputs. 3 : Used for Quad Input/Output instructions. For data transfers, DQ\\[7:0\\]
are used as both inputs and outputs."]
pub type DataXferTypeExtModeFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE_BIT_ENABLE_FLD` reader - 20:20\\]
Mode Bit Enable: Set this field to 1 to ensure that the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
pub type ModeBitEnableFldR = crate::BitReader;
#[doc = "Field `MODE_BIT_ENABLE_FLD` writer - 20:20\\]
Mode Bit Enable: Set this field to 1 to ensure that the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
pub type ModeBitEnableFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMY_RD_CLK_CYCLES_FLD` reader - 28:24\\]
Dummy Read Clock Cycles: Number of dummy clock cycles required by device for read instruction."]
pub type DummyRdClkCyclesFldR = crate::FieldReader;
#[doc = "Field `DUMMY_RD_CLK_CYCLES_FLD` writer - 28:24\\]
Dummy Read Clock Cycles: Number of dummy clock cycles required by device for read instruction."]
pub type DummyRdClkCyclesFldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read Opcode in non-XIP mode: Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    pub fn rd_opcode_non_xip_fld(&self) -> RdOpcodeNonXipFldR {
        RdOpcodeNonXipFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Instruction Type: 0 : Use Standard SPI mode \\[instruction always shifted into the device on DQ0 only\\]
1 : Use DIO-SPI mode \\[Instructions, Address and Data always sent on DQ0 and DQ1\\]
2 : Use QIO-SPI mode \\[Instructions, Address and Data always sent on DQ0, DQ1, DQ2 and DQ3\\]
3 : Use Octal-IO-SPI mode \\[Instructions, Address and Data always sent on DQ\\[7:0\\]\\]"]
    #[inline(always)]
    pub fn instr_type_fld(&self) -> InstrTypeFldR {
        InstrTypeFldR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
DDR Enable: This is to inform that opcode from rd_opcode_non_xip_fld is compliant with one of the DDR READ Commands"]
    #[inline(always)]
    pub fn ddr_en_fld(&self) -> DdrEnFldR {
        DdrEnFldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Address Transfer Type for Standard SPI modes: 0 : Addresses can be shifted to the device on DQ0 only 1 : Addresses can be shifted to the device on DQ0 and DQ1 only 2 : Addresses can be shifted to the device on DQ0, DQ1, DQ2 and DQ3 3 : Addresses can be shifted to the device on DQ\\[7:0\\]"]
    #[inline(always)]
    pub fn addr_xfer_type_std_mode_fld(&self) -> AddrXferTypeStdModeFldR {
        AddrXferTypeStdModeFldR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Data Transfer Type for Standard SPI modes: 0 : SIO mode data is shifted to the device on DQ0 only and from the device on DQ1 only 1 : Used for Dual Input/Output instructions. For data transfers, DQ0 and DQ1 are used as both inputs and outputs. 2 : Used for Quad Input/Output instructions. For data transfers, DQ0,DQ1,DQ2 and DQ3 are used as both inputs and outputs. 3 : Used for Quad Input/Output instructions. For data transfers, DQ\\[7:0\\]
are used as both inputs and outputs."]
    #[inline(always)]
    pub fn data_xfer_type_ext_mode_fld(&self) -> DataXferTypeExtModeFldR {
        DataXferTypeExtModeFldR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Mode Bit Enable: Set this field to 1 to ensure that the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
    #[inline(always)]
    pub fn mode_bit_enable_fld(&self) -> ModeBitEnableFldR {
        ModeBitEnableFldR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Dummy Read Clock Cycles: Number of dummy clock cycles required by device for read instruction."]
    #[inline(always)]
    pub fn dummy_rd_clk_cycles_fld(&self) -> DummyRdClkCyclesFldR {
        DummyRdClkCyclesFldR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read Opcode in non-XIP mode: Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    #[must_use]
    pub fn rd_opcode_non_xip_fld(
        &mut self,
    ) -> RdOpcodeNonXipFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        RdOpcodeNonXipFldW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Instruction Type: 0 : Use Standard SPI mode \\[instruction always shifted into the device on DQ0 only\\]
1 : Use DIO-SPI mode \\[Instructions, Address and Data always sent on DQ0 and DQ1\\]
2 : Use QIO-SPI mode \\[Instructions, Address and Data always sent on DQ0, DQ1, DQ2 and DQ3\\]
3 : Use Octal-IO-SPI mode \\[Instructions, Address and Data always sent on DQ\\[7:0\\]\\]"]
    #[inline(always)]
    #[must_use]
    pub fn instr_type_fld(
        &mut self,
    ) -> InstrTypeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        InstrTypeFldW::new(self, 8)
    }
    #[doc = "Bit 10 - 10:10\\]
DDR Enable: This is to inform that opcode from rd_opcode_non_xip_fld is compliant with one of the DDR READ Commands"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_en_fld(
        &mut self,
    ) -> DdrEnFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec>
    {
        DdrEnFldW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Address Transfer Type for Standard SPI modes: 0 : Addresses can be shifted to the device on DQ0 only 1 : Addresses can be shifted to the device on DQ0 and DQ1 only 2 : Addresses can be shifted to the device on DQ0, DQ1, DQ2 and DQ3 3 : Addresses can be shifted to the device on DQ\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addr_xfer_type_std_mode_fld(
        &mut self,
    ) -> AddrXferTypeStdModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        AddrXferTypeStdModeFldW::new(self, 12)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Data Transfer Type for Standard SPI modes: 0 : SIO mode data is shifted to the device on DQ0 only and from the device on DQ1 only 1 : Used for Dual Input/Output instructions. For data transfers, DQ0 and DQ1 are used as both inputs and outputs. 2 : Used for Quad Input/Output instructions. For data transfers, DQ0,DQ1,DQ2 and DQ3 are used as both inputs and outputs. 3 : Used for Quad Input/Output instructions. For data transfers, DQ\\[7:0\\]
are used as both inputs and outputs."]
    #[inline(always)]
    #[must_use]
    pub fn data_xfer_type_ext_mode_fld(
        &mut self,
    ) -> DataXferTypeExtModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        DataXferTypeExtModeFldW::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Mode Bit Enable: Set this field to 1 to ensure that the mode bits as defined in the Mode Bit Configuration register are sent following the address bytes."]
    #[inline(always)]
    #[must_use]
    pub fn mode_bit_enable_fld(
        &mut self,
    ) -> ModeBitEnableFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        ModeBitEnableFldW::new(self, 20)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Dummy Read Clock Cycles: Number of dummy clock cycles required by device for read instruction."]
    #[inline(always)]
    #[must_use]
    pub fn dummy_rd_clk_cycles_fld(
        &mut self,
    ) -> DummyRdClkCyclesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec,
    > {
        DummyRdClkCyclesFldW::new(self, 24)
    }
}
#[doc = "Device Read Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_rd_config_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_rd_config_reg to value 0x03"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrRdConfigRegSpec
{
    const RESET_VALUE: u32 = 0x03;
}
