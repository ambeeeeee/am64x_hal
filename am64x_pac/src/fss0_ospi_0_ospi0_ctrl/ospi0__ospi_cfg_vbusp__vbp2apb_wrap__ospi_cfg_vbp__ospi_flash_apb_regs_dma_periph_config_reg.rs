#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dma_periph_config_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dma_periph_config_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec>;
#[doc = "Field `NUM_SINGLE_REQ_BYTES_FLD` reader - 3:0\\]
Number of Single Bytes: Number of bytes in a single type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
pub type NumSingleReqBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_SINGLE_REQ_BYTES_FLD` writer - 3:0\\]
Number of Single Bytes: Number of bytes in a single type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
pub type NumSingleReqBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUM_BURST_REQ_BYTES_FLD` reader - 11:8\\]
Number of Burst Bytes: Number of bytes in a burst type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
pub type NumBurstReqBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_BURST_REQ_BYTES_FLD` writer - 11:8\\]
Number of Burst Bytes: Number of bytes in a burst type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
pub type NumBurstReqBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of Single Bytes: Number of bytes in a single type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
    #[inline(always)]
    pub fn num_single_req_bytes_fld(&self) -> NumSingleReqBytesFldR {
        NumSingleReqBytesFldR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of Burst Bytes: Number of bytes in a burst type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
    #[inline(always)]
    pub fn num_burst_req_bytes_fld(&self) -> NumBurstReqBytesFldR {
        NumBurstReqBytesFldR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of Single Bytes: Number of bytes in a single type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
    #[inline(always)]
    #[must_use]
    pub fn num_single_req_bytes_fld(
        &mut self,
    ) -> NumSingleReqBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec,
    > {
        NumSingleReqBytesFldW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of Burst Bytes: Number of bytes in a burst type request on the DMA peripheral request. A programmed value of 0 represents a single byte. This should be setup before starting the indirect read or write operation. The actual number of bytes used is 2**\\[value in this register\\]
which will simplify implementation."]
    #[inline(always)]
    #[must_use]
    pub fn num_burst_req_bytes_fld(
        &mut self,
    ) -> NumBurstReqBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec,
    > {
        NumBurstReqBytesFldW::new(self, 8)
    }
}
#[doc = "DMA Peripheral Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dma_periph_config_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dma_periph_config_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDmaPeriphConfigRegSpec
{
    const RESET_VALUE: u32 = 0;
}
