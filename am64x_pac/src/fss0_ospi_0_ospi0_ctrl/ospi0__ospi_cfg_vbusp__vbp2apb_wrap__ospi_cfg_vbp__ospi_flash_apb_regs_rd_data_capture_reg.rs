#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rd_data_capture_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rd_data_capture_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec>;
#[doc = "Field `BYPASS_FLD` reader - 0:0\\]
Bypass the adapted loopback clock circuit"]
pub type BypassFldR = crate::BitReader;
#[doc = "Field `BYPASS_FLD` writer - 0:0\\]
Bypass the adapted loopback clock circuit"]
pub type BypassFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_FLD` reader - 4:1\\]
Read Delay: Delay the read data capturing logic by the programmed number of ref_clk cycles"]
pub type DelayFldR = crate::FieldReader;
#[doc = "Field `DELAY_FLD` writer - 4:1\\]
Read Delay: Delay the read data capturing logic by the programmed number of ref_clk cycles"]
pub type DelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAMPLE_EDGE_SEL_FLD` reader - 5:5\\]
Sample edge selection: Choose edge on which data outputs from flash memory will be sampled"]
pub type SampleEdgeSelFldR = crate::BitReader;
#[doc = "Field `SAMPLE_EDGE_SEL_FLD` writer - 5:5\\]
Sample edge selection: Choose edge on which data outputs from flash memory will be sampled"]
pub type SampleEdgeSelFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS_ENABLE_FLD` reader - 8:8\\]
DQS enable bit: If enabled, signal from DQS input is driven into RX DLL and is used for data capturing in PHY Mode rather than internally generated gated ref_clk.."]
pub type DqsEnableFldR = crate::BitReader;
#[doc = "Field `DQS_ENABLE_FLD` writer - 8:8\\]
DQS enable bit: If enabled, signal from DQS input is driven into RX DLL and is used for data capturing in PHY Mode rather than internally generated gated ref_clk.."]
pub type DqsEnableFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR_READ_DELAY_FLD` reader - 19:16\\]
DDR read delay: Delay the transmitted data by the programmed number of ref_clk cycles.This field is only relevant when DDR Read Command is executed. Otherwise can be ignored."]
pub type DdrReadDelayFldR = crate::FieldReader;
#[doc = "Field `DDR_READ_DELAY_FLD` writer - 19:16\\]
DDR read delay: Delay the transmitted data by the programmed number of ref_clk cycles.This field is only relevant when DDR Read Command is executed. Otherwise can be ignored."]
pub type DdrReadDelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bypass the adapted loopback clock circuit"]
    #[inline(always)]
    pub fn bypass_fld(&self) -> BypassFldR {
        BypassFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Read Delay: Delay the read data capturing logic by the programmed number of ref_clk cycles"]
    #[inline(always)]
    pub fn delay_fld(&self) -> DelayFldR {
        DelayFldR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Sample edge selection: Choose edge on which data outputs from flash memory will be sampled"]
    #[inline(always)]
    pub fn sample_edge_sel_fld(&self) -> SampleEdgeSelFldR {
        SampleEdgeSelFldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
DQS enable bit: If enabled, signal from DQS input is driven into RX DLL and is used for data capturing in PHY Mode rather than internally generated gated ref_clk.."]
    #[inline(always)]
    pub fn dqs_enable_fld(&self) -> DqsEnableFldR {
        DqsEnableFldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DDR read delay: Delay the transmitted data by the programmed number of ref_clk cycles.This field is only relevant when DDR Read Command is executed. Otherwise can be ignored."]
    #[inline(always)]
    pub fn ddr_read_delay_fld(&self) -> DdrReadDelayFldR {
        DdrReadDelayFldR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bypass the adapted loopback clock circuit"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_fld(
        &mut self,
    ) -> BypassFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec>
    {
        BypassFldW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Read Delay: Delay the read data capturing logic by the programmed number of ref_clk cycles"]
    #[inline(always)]
    #[must_use]
    pub fn delay_fld(
        &mut self,
    ) -> DelayFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec>
    {
        DelayFldW::new(self, 1)
    }
    #[doc = "Bit 5 - 5:5\\]
Sample edge selection: Choose edge on which data outputs from flash memory will be sampled"]
    #[inline(always)]
    #[must_use]
    pub fn sample_edge_sel_fld(
        &mut self,
    ) -> SampleEdgeSelFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec,
    > {
        SampleEdgeSelFldW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
DQS enable bit: If enabled, signal from DQS input is driven into RX DLL and is used for data capturing in PHY Mode rather than internally generated gated ref_clk.."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_enable_fld(
        &mut self,
    ) -> DqsEnableFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec>
    {
        DqsEnableFldW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DDR read delay: Delay the transmitted data by the programmed number of ref_clk cycles.This field is only relevant when DDR Read Command is executed. Otherwise can be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn ddr_read_delay_fld(
        &mut self,
    ) -> DdrReadDelayFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec,
    > {
        DdrReadDelayFldW::new(self, 16)
    }
}
#[doc = "Read Data Capture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_rd_data_capture_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_rd_data_capture_reg to value 0x01"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsRdDataCaptureRegSpec
{
    const RESET_VALUE: u32 = 0x01;
}
