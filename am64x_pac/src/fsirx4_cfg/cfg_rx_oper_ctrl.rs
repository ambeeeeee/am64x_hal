#[doc = "Register `CFG_RX_OPER_CTRL` reader"]
pub type R = crate::R<CfgRxOperCtrlSpec>;
#[doc = "Register `CFG_RX_OPER_CTRL` writer"]
pub type W = crate::W<CfgRxOperCtrlSpec>;
#[doc = "Field `DATA_WIDTH` reader - 1:0\\]
Receive Data Width Select bitThese bits decide the number of data lines used for receiving data. 0h \\[R/W\\]
= Data will be received on one data line, RXD0.1h \\[R/W\\]
= Data will be received on two data lines, RXD0 and RXD1.2h, 3h \\[R/W\\]
= Reserved"]
pub type DataWidthR = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - 1:0\\]
Receive Data Width Select bitThese bits decide the number of data lines used for receiving data. 0h \\[R/W\\]
= Data will be received on one data line, RXD0.1h \\[R/W\\]
= Data will be received on two data lines, RXD0 and RXD1.2h, 3h \\[R/W\\]
= Reserved"]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MODE` reader - 2:2\\]
SPI Mode Enable bitThis bit enables and disables the SPI compatibility mode of the FSI RX. The received data must be formatted as an FSI frame in order for the data to properly be received. SPI compatibility mode will allow FSI RX to receive data that is sent using SPI signal format. Refer to the applicable section in the FSI TRM chapter for more information. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
pub type SpiModeR = crate::BitReader;
#[doc = "Field `SPI_MODE` writer - 2:2\\]
SPI Mode Enable bitThis bit enables and disables the SPI compatibility mode of the FSI RX. The received data must be formatted as an FSI frame in order for the data to properly be received. SPI compatibility mode will allow FSI RX to receive data that is sent using SPI signal format. Refer to the applicable section in the FSI TRM chapter for more information. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
pub type SpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `N_WORDS` reader - 6:3\\]
Number of Words to ReceiveThis field defines the number of words which will be received in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the transmitter. Set this bitfield to be one less than the number of words to be received. This value is only applicable when the frame type received is DATA_N_WORD. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
pub type NWordsR = crate::FieldReader;
#[doc = "Field `N_WORDS` writer - 6:3\\]
Number of Words to ReceiveThis field defines the number of words which will be received in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the transmitter. Set this bitfield to be one less than the number of words to be received. This value is only applicable when the frame type received is DATA_N_WORD. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
pub type NWordsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECC_SEL` reader - 7:7\\]
ECC Data Width Select bitThis bit selects between whether the ECC computation is done on 16-bit or 32-bit words. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
pub type EccSelR = crate::BitReader;
#[doc = "Field `ECC_SEL` writer - 7:7\\]
ECC Data Width Select bitThis bit selects between whether the ECC computation is done on 16-bit or 32-bit words. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
pub type EccSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_WD_RST_MODE` reader - 8:8\\]
Ping Watchdog Timeout Mode Select bitThis bit selects the mode by which the ping watchdog counter is reset. The watchdog counter can be reset and restarted only by ping frames or by any received frame. 0h \\[R/W\\]
= The ping watchdog counter will reset and restart only by ping frames.1h \\[R/W\\]
= The ping watchdog counter will reset and restart by any received frame."]
pub type PingWdRstModeR = crate::BitReader;
#[doc = "Field `PING_WD_RST_MODE` writer - 8:8\\]
Ping Watchdog Timeout Mode Select bitThis bit selects the mode by which the ping watchdog counter is reset. The watchdog counter can be reset and restarted only by ping frames or by any received frame. 0h \\[R/W\\]
= The ping watchdog counter will reset and restart only by ping frames.1h \\[R/W\\]
= The ping watchdog counter will reset and restart by any received frame."]
pub type PingWdRstModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Receive Data Width Select bitThese bits decide the number of data lines used for receiving data. 0h \\[R/W\\]
= Data will be received on one data line, RXD0.1h \\[R/W\\]
= Data will be received on two data lines, RXD0 and RXD1.2h, 3h \\[R/W\\]
= Reserved"]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Mode Enable bitThis bit enables and disables the SPI compatibility mode of the FSI RX. The received data must be formatted as an FSI frame in order for the data to properly be received. SPI compatibility mode will allow FSI RX to receive data that is sent using SPI signal format. Refer to the applicable section in the FSI TRM chapter for more information. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
    #[inline(always)]
    pub fn spi_mode(&self) -> SpiModeR {
        SpiModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Number of Words to ReceiveThis field defines the number of words which will be received in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the transmitter. Set this bitfield to be one less than the number of words to be received. This value is only applicable when the frame type received is DATA_N_WORD. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
    #[inline(always)]
    pub fn n_words(&self) -> NWordsR {
        NWordsR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
ECC Data Width Select bitThis bit selects between whether the ECC computation is done on 16-bit or 32-bit words. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
    #[inline(always)]
    pub fn ecc_sel(&self) -> EccSelR {
        EccSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Ping Watchdog Timeout Mode Select bitThis bit selects the mode by which the ping watchdog counter is reset. The watchdog counter can be reset and restarted only by ping frames or by any received frame. 0h \\[R/W\\]
= The ping watchdog counter will reset and restart only by ping frames.1h \\[R/W\\]
= The ping watchdog counter will reset and restart by any received frame."]
    #[inline(always)]
    pub fn ping_wd_rst_mode(&self) -> PingWdRstModeR {
        PingWdRstModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Receive Data Width Select bitThese bits decide the number of data lines used for receiving data. 0h \\[R/W\\]
= Data will be received on one data line, RXD0.1h \\[R/W\\]
= Data will be received on two data lines, RXD0 and RXD1.2h, 3h \\[R/W\\]
= Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DataWidthW<CfgRxOperCtrlSpec> {
        DataWidthW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Mode Enable bitThis bit enables and disables the SPI compatibility mode of the FSI RX. The received data must be formatted as an FSI frame in order for the data to properly be received. SPI compatibility mode will allow FSI RX to receive data that is sent using SPI signal format. Refer to the applicable section in the FSI TRM chapter for more information. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mode(&mut self) -> SpiModeW<CfgRxOperCtrlSpec> {
        SpiModeW::new(self, 2)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Number of Words to ReceiveThis field defines the number of words which will be received in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the transmitter. Set this bitfield to be one less than the number of words to be received. This value is only applicable when the frame type received is DATA_N_WORD. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
    #[inline(always)]
    #[must_use]
    pub fn n_words(&mut self) -> NWordsW<CfgRxOperCtrlSpec> {
        NWordsW::new(self, 3)
    }
    #[doc = "Bit 7 - 7:7\\]
ECC Data Width Select bitThis bit selects between whether the ECC computation is done on 16-bit or 32-bit words. 0h \\[R/W\\]
= 32-bit ECC is used.1h \\[R/W\\]
= 16-bit ECC is used."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_sel(&mut self) -> EccSelW<CfgRxOperCtrlSpec> {
        EccSelW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Ping Watchdog Timeout Mode Select bitThis bit selects the mode by which the ping watchdog counter is reset. The watchdog counter can be reset and restarted only by ping frames or by any received frame. 0h \\[R/W\\]
= The ping watchdog counter will reset and restart only by ping frames.1h \\[R/W\\]
= The ping watchdog counter will reset and restart by any received frame."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_rst_mode(&mut self) -> PingWdRstModeW<CfgRxOperCtrlSpec> {
        PingWdRstModeW::new(self, 8)
    }
}
#[doc = "Receive operation control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_oper_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_oper_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxOperCtrlSpec;
impl crate::RegisterSpec for CfgRxOperCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_oper_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgRxOperCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_oper_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgRxOperCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_OPER_CTRL to value 0"]
impl crate::Resettable for CfgRxOperCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
