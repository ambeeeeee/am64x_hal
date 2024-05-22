#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_control1` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_host_control1` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec>;
#[doc = "Field `LED_CONTROL` reader - 0:0\\]
This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
pub type LedControlR = crate::BitReader;
#[doc = "Field `LED_CONTROL` writer - 0:0\\]
This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
pub type LedControlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_WIDTH` reader - 1:1\\]
This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card. This bit is not effective in UHS-II mode."]
pub type DataWidthR = crate::BitReader;
#[doc = "Field `DATA_WIDTH` writer - 1:1\\]
This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card. This bit is not effective in UHS-II mode."]
pub type DataWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGH_SPEED_ENA` reader - 2:2\\]
This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 \\[default\\], the HC outputs CMD line and DAT lines at the falling edge of the SD clock \\[up to 25 MHz/20MHz for MMC\\]. If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock \\[up to 50 MHz for SD/52MHz for MMC\\]/ 208Mhz \\[for SD3.0\\]
If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again. This bit is not effective in UHS-II mode."]
pub type HighSpeedEnaR = crate::BitReader;
#[doc = "Field `HIGH_SPEED_ENA` writer - 2:2\\]
This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 \\[default\\], the HC outputs CMD line and DAT lines at the falling edge of the SD clock \\[up to 25 MHz/20MHz for MMC\\]. If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock \\[up to 50 MHz for SD/52MHz for MMC\\]/ 208Mhz \\[for SD3.0\\]
If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again. This bit is not effective in UHS-II mode."]
pub type HighSpeedEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SELECT` reader - 4:3\\]
This field is used to select DMA type. The Host Driver shall check support of DMA modes by referring the Capabilities register. Selected DMA is enabled by DMA Enable of the Transfer Mode register in SD mode and DMA Enable of UHS-II Transfer Mode register in UHS-II mode. \\[1\\]
Up to Version 3.00 When Host Version 4 Enable is set to 0, setting of this field is compatible to Host Controller Version 3.00.SDMA is initiated by writing to the Command register when this field is set to 00b and the SDMA System Address regis-ter \\[32-bit\\]
is used. SDMA does not support 64-bit address-ing.ADMA2 is initiated by writing to the Command register when this field is set to 10b or 11b. Lower 32-bit of the ADMA Sys-tem Address register is used when this field is set to 10b and 64-bit of the ADMA System Address register is used when this field is set to 11b. Support of 64-bit System Addressing is indicated by 64-bit System Address Support for V3 in the Capabilities register. 64-bit AMDA2 uses 96-bit Descriptor. 00 - SDMA is selected 01 - 32-bit Address ADMA1 is selected 10 -32-bit Address ADMA2 is selected 11 - 64-bit Address ADMA2 is selected \\[Optional\\]
\\[2\\]
Version 4.00 or later When Host Version 4 Enable is set to 1, setting of this field is changed as follows. SDMA is initiated by Host Driver writes to the Command reg-ister when this field is set to 00b.ADMA2 is initiated by Host Driver writes to the Command register when this field is set to 10b or 11b and by ADMA3 sets to the ADMA System Address register when this field is set to 11b. ADMA3 is initiated by Host Driver writes to the ADMA3 IDAddress register when this field is set to 11b. 00 - SDMA is selected 01 - Not Used \\[New assignment is not allowed\\]
10 - ADMA2 is selected \\[AMDA3 is not supported or dis-abled\\]
11 - ADMA2 or ADMA3 is selected Support of 64-bit DMA and 128-bit Descriptor is indicated by 64-bit System Address Support for V4 in the Capabilities register. If the support bit is set to 1, all supported DMAs \\[depends on Support, ADMA2 Support and ADMA3 Sup-port\\]
shall support 64-bit addressing. 64-bit Addressing in the Host Controller 2 register selects either 32-bit or 64-bit system addressing of DMAs."]
pub type DmaSelectR = crate::FieldReader;
#[doc = "Field `DMA_SELECT` writer - 4:3\\]
This field is used to select DMA type. The Host Driver shall check support of DMA modes by referring the Capabilities register. Selected DMA is enabled by DMA Enable of the Transfer Mode register in SD mode and DMA Enable of UHS-II Transfer Mode register in UHS-II mode. \\[1\\]
Up to Version 3.00 When Host Version 4 Enable is set to 0, setting of this field is compatible to Host Controller Version 3.00.SDMA is initiated by writing to the Command register when this field is set to 00b and the SDMA System Address regis-ter \\[32-bit\\]
is used. SDMA does not support 64-bit address-ing.ADMA2 is initiated by writing to the Command register when this field is set to 10b or 11b. Lower 32-bit of the ADMA Sys-tem Address register is used when this field is set to 10b and 64-bit of the ADMA System Address register is used when this field is set to 11b. Support of 64-bit System Addressing is indicated by 64-bit System Address Support for V3 in the Capabilities register. 64-bit AMDA2 uses 96-bit Descriptor. 00 - SDMA is selected 01 - 32-bit Address ADMA1 is selected 10 -32-bit Address ADMA2 is selected 11 - 64-bit Address ADMA2 is selected \\[Optional\\]
\\[2\\]
Version 4.00 or later When Host Version 4 Enable is set to 1, setting of this field is changed as follows. SDMA is initiated by Host Driver writes to the Command reg-ister when this field is set to 00b.ADMA2 is initiated by Host Driver writes to the Command register when this field is set to 10b or 11b and by ADMA3 sets to the ADMA System Address register when this field is set to 11b. ADMA3 is initiated by Host Driver writes to the ADMA3 IDAddress register when this field is set to 11b. 00 - SDMA is selected 01 - Not Used \\[New assignment is not allowed\\]
10 - ADMA2 is selected \\[AMDA3 is not supported or dis-abled\\]
11 - ADMA2 or ADMA3 is selected Support of 64-bit DMA and 128-bit Descriptor is indicated by 64-bit System Address Support for V4 in the Capabilities register. If the support bit is set to 1, all supported DMAs \\[depends on Support, ADMA2 Support and ADMA3 Sup-port\\]
shall support 64-bit addressing. 64-bit Addressing in the Host Controller 2 register selects either 32-bit or 64-bit system addressing of DMAs."]
pub type DmaSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXT_DATA_WIDTH` reader - 5:5\\]
This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot \\[Slot Type is set to 10b in the Capabilities register\\]. In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
pub type ExtDataWidthR = crate::BitReader;
#[doc = "Field `EXT_DATA_WIDTH` writer - 5:5\\]
This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot \\[Slot Type is set to 10b in the Capabilities register\\]. In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
pub type ExtDataWidthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_TEST_LEVEL` reader - 6:6\\]
This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates \\[card ins or card removal\\]
interrupt when the normal int sts enable bit is set. '0' No Card '1' Card Inserted"]
pub type CdTestLevelR = crate::BitReader;
#[doc = "Field `CD_TEST_LEVEL` writer - 6:6\\]
This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates \\[card ins or card removal\\]
interrupt when the normal int sts enable bit is set. '0' No Card '1' Card Inserted"]
pub type CdTestLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_SIG_SEL` reader - 7:7\\]
This bit selects source for card detection. '0' SDCD# is selected \\[for normal use\\]
'1' The card detect test level is selected"]
pub type CdSigSelR = crate::BitReader;
#[doc = "Field `CD_SIG_SEL` writer - 7:7\\]
This bit selects source for card detection. '0' SDCD# is selected \\[for normal use\\]
'1' The card detect test level is selected"]
pub type CdSigSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
    #[inline(always)]
    pub fn led_control(&self) -> LedControlR {
        LedControlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card. This bit is not effective in UHS-II mode."]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 \\[default\\], the HC outputs CMD line and DAT lines at the falling edge of the SD clock \\[up to 25 MHz/20MHz for MMC\\]. If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock \\[up to 50 MHz for SD/52MHz for MMC\\]/ 208Mhz \\[for SD3.0\\]
If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again. This bit is not effective in UHS-II mode."]
    #[inline(always)]
    pub fn high_speed_ena(&self) -> HighSpeedEnaR {
        HighSpeedEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field is used to select DMA type. The Host Driver shall check support of DMA modes by referring the Capabilities register. Selected DMA is enabled by DMA Enable of the Transfer Mode register in SD mode and DMA Enable of UHS-II Transfer Mode register in UHS-II mode. \\[1\\]
Up to Version 3.00 When Host Version 4 Enable is set to 0, setting of this field is compatible to Host Controller Version 3.00.SDMA is initiated by writing to the Command register when this field is set to 00b and the SDMA System Address regis-ter \\[32-bit\\]
is used. SDMA does not support 64-bit address-ing.ADMA2 is initiated by writing to the Command register when this field is set to 10b or 11b. Lower 32-bit of the ADMA Sys-tem Address register is used when this field is set to 10b and 64-bit of the ADMA System Address register is used when this field is set to 11b. Support of 64-bit System Addressing is indicated by 64-bit System Address Support for V3 in the Capabilities register. 64-bit AMDA2 uses 96-bit Descriptor. 00 - SDMA is selected 01 - 32-bit Address ADMA1 is selected 10 -32-bit Address ADMA2 is selected 11 - 64-bit Address ADMA2 is selected \\[Optional\\]
\\[2\\]
Version 4.00 or later When Host Version 4 Enable is set to 1, setting of this field is changed as follows. SDMA is initiated by Host Driver writes to the Command reg-ister when this field is set to 00b.ADMA2 is initiated by Host Driver writes to the Command register when this field is set to 10b or 11b and by ADMA3 sets to the ADMA System Address register when this field is set to 11b. ADMA3 is initiated by Host Driver writes to the ADMA3 IDAddress register when this field is set to 11b. 00 - SDMA is selected 01 - Not Used \\[New assignment is not allowed\\]
10 - ADMA2 is selected \\[AMDA3 is not supported or dis-abled\\]
11 - ADMA2 or ADMA3 is selected Support of 64-bit DMA and 128-bit Descriptor is indicated by 64-bit System Address Support for V4 in the Capabilities register. If the support bit is set to 1, all supported DMAs \\[depends on Support, ADMA2 Support and ADMA3 Sup-port\\]
shall support 64-bit addressing. 64-bit Addressing in the Host Controller 2 register selects either 32-bit or 64-bit system addressing of DMAs."]
    #[inline(always)]
    pub fn dma_select(&self) -> DmaSelectR {
        DmaSelectR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot \\[Slot Type is set to 10b in the Capabilities register\\]. In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
    #[inline(always)]
    pub fn ext_data_width(&self) -> ExtDataWidthR {
        ExtDataWidthR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates \\[card ins or card removal\\]
interrupt when the normal int sts enable bit is set. '0' No Card '1' Card Inserted"]
    #[inline(always)]
    pub fn cd_test_level(&self) -> CdTestLevelR {
        CdTestLevelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit selects source for card detection. '0' SDCD# is selected \\[for normal use\\]
'1' The card detect test level is selected"]
    #[inline(always)]
    pub fn cd_sig_sel(&self) -> CdSigSelR {
        CdSigSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to caution the user not to remove the card while the SD card is being accessed. If the software is going to issue multiple SD commands, this bit can be set during all transactions. It is not necessary to change for each transaction."]
    #[inline(always)]
    #[must_use]
    pub fn led_control(&mut self) -> LedControlW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        LedControlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card. This bit is not effective in UHS-II mode."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DataWidthW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        DataWidthW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 \\[default\\], the HC outputs CMD line and DAT lines at the falling edge of the SD clock \\[up to 25 MHz/20MHz for MMC\\]. If this bit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock \\[up to 50 MHz for SD/52MHz for MMC\\]/ 208Mhz \\[for SD3.0\\]
If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again. This bit is not effective in UHS-II mode."]
    #[inline(always)]
    #[must_use]
    pub fn high_speed_ena(&mut self) -> HighSpeedEnaW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        HighSpeedEnaW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
This field is used to select DMA type. The Host Driver shall check support of DMA modes by referring the Capabilities register. Selected DMA is enabled by DMA Enable of the Transfer Mode register in SD mode and DMA Enable of UHS-II Transfer Mode register in UHS-II mode. \\[1\\]
Up to Version 3.00 When Host Version 4 Enable is set to 0, setting of this field is compatible to Host Controller Version 3.00.SDMA is initiated by writing to the Command register when this field is set to 00b and the SDMA System Address regis-ter \\[32-bit\\]
is used. SDMA does not support 64-bit address-ing.ADMA2 is initiated by writing to the Command register when this field is set to 10b or 11b. Lower 32-bit of the ADMA Sys-tem Address register is used when this field is set to 10b and 64-bit of the ADMA System Address register is used when this field is set to 11b. Support of 64-bit System Addressing is indicated by 64-bit System Address Support for V3 in the Capabilities register. 64-bit AMDA2 uses 96-bit Descriptor. 00 - SDMA is selected 01 - 32-bit Address ADMA1 is selected 10 -32-bit Address ADMA2 is selected 11 - 64-bit Address ADMA2 is selected \\[Optional\\]
\\[2\\]
Version 4.00 or later When Host Version 4 Enable is set to 1, setting of this field is changed as follows. SDMA is initiated by Host Driver writes to the Command reg-ister when this field is set to 00b.ADMA2 is initiated by Host Driver writes to the Command register when this field is set to 10b or 11b and by ADMA3 sets to the ADMA System Address register when this field is set to 11b. ADMA3 is initiated by Host Driver writes to the ADMA3 IDAddress register when this field is set to 11b. 00 - SDMA is selected 01 - Not Used \\[New assignment is not allowed\\]
10 - ADMA2 is selected \\[AMDA3 is not supported or dis-abled\\]
11 - ADMA2 or ADMA3 is selected Support of 64-bit DMA and 128-bit Descriptor is indicated by 64-bit System Address Support for V4 in the Capabilities register. If the support bit is set to 1, all supported DMAs \\[depends on Support, ADMA2 Support and ADMA3 Sup-port\\]
shall support 64-bit addressing. 64-bit Addressing in the Host Controller 2 register selects either 32-bit or 64-bit system addressing of DMAs."]
    #[inline(always)]
    #[must_use]
    pub fn dma_select(&mut self) -> DmaSelectW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        DmaSelectW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit controls 8-bit bus width mode for embedded device. Support of this function is indicated in 8-bit Support for Embedded Device in the Capabilities register. If a device supports 8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width is controlled by Data Transfer Width in the Host Control 1 register.This bit is not effective when multiple devices are installed on a bus slot \\[Slot Type is set to 10b in the Capabilities register\\]. In this case, each device bus width is controlled by Bus Width Preset field in the Shared Bus register."]
    #[inline(always)]
    #[must_use]
    pub fn ext_data_width(&mut self) -> ExtDataWidthW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        ExtDataWidthW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit is enabled while the Card Detect Signal Selection is set to 1 and it indicates card inserted or not. Generates \\[card ins or card removal\\]
interrupt when the normal int sts enable bit is set. '0' No Card '1' Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn cd_test_level(&mut self) -> CdTestLevelW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        CdTestLevelW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This bit selects source for card detection. '0' SDCD# is selected \\[for normal use\\]
'1' The card detect test level is selected"]
    #[inline(always)]
    #[must_use]
    pub fn cd_sig_sel(&mut self) -> CdSigSelW<SdhcWrap_CtlCfg_CtlcfgHostControl1Spec> {
        CdSigSelW::new(self, 7)
    }
}
#[doc = "This register is used to program DMA modes, LED Control, Data Transfer Width, High Speed Enable, Card detect test level and signal selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgHostControl1Spec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgHostControl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgHostControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_host_control1::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgHostControl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_host_control1 to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgHostControl1Spec {
    const RESET_VALUE: u8 = 0;
}
