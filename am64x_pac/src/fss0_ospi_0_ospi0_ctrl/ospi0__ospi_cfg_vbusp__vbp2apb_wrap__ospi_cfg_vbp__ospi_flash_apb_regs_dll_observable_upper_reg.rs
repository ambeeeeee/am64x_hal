#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_upper_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_upper_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec>;
#[doc = "Field `DLL_OBSERVABLE__UPPER_RX_DECODER_OUTPUT_FLD` reader - 6:0\\]
Holds the encoded value for the RX delay line for this slice."]
pub type DllObservable_UpperRxDecoderOutputFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE__UPPER_RX_DECODER_OUTPUT_FLD` writer - 6:0\\]
Holds the encoded value for the RX delay line for this slice."]
pub type DllObservable_UpperRxDecoderOutputFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DLL_OBSERVABLE_UPPER_TX_DECODER_OUTPUT_FLD` reader - 22:16\\]
Holds the encoded value for the TX delay line for this slice."]
pub type DllObservableUpperTxDecoderOutputFldR = crate::FieldReader;
#[doc = "Field `DLL_OBSERVABLE_UPPER_TX_DECODER_OUTPUT_FLD` writer - 22:16\\]
Holds the encoded value for the TX delay line for this slice."]
pub type DllObservableUpperTxDecoderOutputFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the encoded value for the RX delay line for this slice."]
    #[inline(always)]
    pub fn dll_observable__upper_rx_decoder_output_fld(
        &self,
    ) -> DllObservable_UpperRxDecoderOutputFldR {
        DllObservable_UpperRxDecoderOutputFldR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the encoded value for the TX delay line for this slice."]
    #[inline(always)]
    pub fn dll_observable_upper_tx_decoder_output_fld(
        &self,
    ) -> DllObservableUpperTxDecoderOutputFldR {
        DllObservableUpperTxDecoderOutputFldR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Holds the encoded value for the RX delay line for this slice."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable__upper_rx_decoder_output_fld(
        &mut self,
    ) -> DllObservable_UpperRxDecoderOutputFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec,
    > {
        DllObservable_UpperRxDecoderOutputFldW::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Holds the encoded value for the TX delay line for this slice."]
    #[inline(always)]
    #[must_use]
    pub fn dll_observable_upper_tx_decoder_output_fld(
        &mut self,
    ) -> DllObservableUpperTxDecoderOutputFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec,
    > {
        DllObservableUpperTxDecoderOutputFldW::new(self, 16)
    }
}
#[doc = "DLL Observable Register Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dll_observable_upper_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dll_observable_upper_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDllObservableUpperRegSpec
{
    const RESET_VALUE: u32 = 0;
}
