#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_wr_config_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_wr_config_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec>;
#[doc = "Field `WR_OPCODE_FLD` reader - 7:0\\]
Write Opcode"]
pub type WrOpcodeFldR = crate::FieldReader;
#[doc = "Field `WR_OPCODE_FLD` writer - 7:0\\]
Write Opcode"]
pub type WrOpcodeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WEL_DIS_FLD` reader - 8:8\\]
WEL Disable: This is to turn off automatic issuing of WEL Command before write operation for DAC or INDAC"]
pub type WelDisFldR = crate::BitReader;
#[doc = "Field `WEL_DIS_FLD` writer - 8:8\\]
WEL Disable: This is to turn off automatic issuing of WEL Command before write operation for DAC or INDAC"]
pub type WelDisFldW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `DUMMY_WR_CLK_CYCLES_FLD` reader - 28:24\\]
Dummy Write Clock Cycles: Number of dummy clock cycles required by device for write instruction."]
pub type DummyWrClkCyclesFldR = crate::FieldReader;
#[doc = "Field `DUMMY_WR_CLK_CYCLES_FLD` writer - 28:24\\]
Dummy Write Clock Cycles: Number of dummy clock cycles required by device for write instruction."]
pub type DummyWrClkCyclesFldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Write Opcode"]
    #[inline(always)]
    pub fn wr_opcode_fld(&self) -> WrOpcodeFldR {
        WrOpcodeFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
WEL Disable: This is to turn off automatic issuing of WEL Command before write operation for DAC or INDAC"]
    #[inline(always)]
    pub fn wel_dis_fld(&self) -> WelDisFldR {
        WelDisFldR::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bits 24:28 - 28:24\\]
Dummy Write Clock Cycles: Number of dummy clock cycles required by device for write instruction."]
    #[inline(always)]
    pub fn dummy_wr_clk_cycles_fld(&self) -> DummyWrClkCyclesFldR {
        DummyWrClkCyclesFldR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Write Opcode"]
    #[inline(always)]
    #[must_use]
    pub fn wr_opcode_fld(
        &mut self,
    ) -> WrOpcodeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec,
    > {
        WrOpcodeFldW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
WEL Disable: This is to turn off automatic issuing of WEL Command before write operation for DAC or INDAC"]
    #[inline(always)]
    #[must_use]
    pub fn wel_dis_fld(
        &mut self,
    ) -> WelDisFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec>
    {
        WelDisFldW::new(self, 8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Address Transfer Type for Standard SPI modes: 0 : Addresses can be shifted to the device on DQ0 only 1 : Addresses can be shifted to the device on DQ0 and DQ1 only 2 : Addresses can be shifted to the device on DQ0, DQ1, DQ2 and DQ3 3 : Addresses can be shifted to the device on DQ\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn addr_xfer_type_std_mode_fld(
        &mut self,
    ) -> AddrXferTypeStdModeFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec,
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
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec,
    > {
        DataXferTypeExtModeFldW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Dummy Write Clock Cycles: Number of dummy clock cycles required by device for write instruction."]
    #[inline(always)]
    #[must_use]
    pub fn dummy_wr_clk_cycles_fld(
        &mut self,
    ) -> DummyWrClkCyclesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec,
    > {
        DummyWrClkCyclesFldW::new(self, 24)
    }
}
#[doc = "Device Write Instruction Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_instr_wr_config_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_instr_wr_config_reg to value 0x02"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevInstrWrConfigRegSpec
{
    const RESET_VALUE: u32 = 0x02;
}
