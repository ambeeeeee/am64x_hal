#[doc = "Register `CFG_TX_OPER_CTRL_LO_ALT1_` reader"]
pub type R = crate::R<CfgTxOperCtrlLoAlt1_Spec>;
#[doc = "Register `CFG_TX_OPER_CTRL_LO_ALT1_` writer"]
pub type W = crate::W<CfgTxOperCtrlLoAlt1_Spec>;
#[doc = "Field `DATA_WIDTH` reader - 1:0\\]
Transmit Data Width Select bitsThese bits define the number of data lines used by the transmitter. 0h \\[R/W\\]
= Data will be transmitted on one data line \\[TXD0\\]1h \\[R/W\\]
= Data will be transmitted on two data lines \\[TXD0 and TXD1\\]. The format of the data is described in the preceeding chapter.2h, 3h \\[R/W\\]
= Reserved"]
pub type DataWidthR = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - 1:0\\]
Transmit Data Width Select bitsThese bits define the number of data lines used by the transmitter. 0h \\[R/W\\]
= Data will be transmitted on one data line \\[TXD0\\]1h \\[R/W\\]
= Data will be transmitted on two data lines \\[TXD0 and TXD1\\]. The format of the data is described in the preceeding chapter.2h, 3h \\[R/W\\]
= Reserved"]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_MODE` reader - 2:2\\]
SPI Mode Select bitThis bit enables and disables SPI compatibility mode. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
pub type SpiModeR = crate::BitReader;
#[doc = "Field `SPI_MODE` writer - 2:2\\]
SPI Mode Select bitThis bit enables and disables SPI compatibility mode. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
pub type SpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_MODE` reader - 5:3\\]
Transmission Start Mode Select bitThese bits select the method by which a new frame transmission is started. 0h \\[R/W\\]
= Only a software write to TX_FRAME_CTRL.START initiate a new transmission.1h \\[R/W\\]
= The configured external trigger will initiate a new transmission.2h \\[R/W\\]
= Either writing to TX_FRAME_CTRL.START or the TX_FRAME_TAG_UDATA register will initiate a new transmission. All other combinations of bits are illegal and reserved for future use."]
pub type StartModeR = crate::FieldReader;
#[doc = "Field `START_MODE` writer - 5:3\\]
Transmission Start Mode Select bitThese bits select the method by which a new frame transmission is started. 0h \\[R/W\\]
= Only a software write to TX_FRAME_CTRL.START initiate a new transmission.1h \\[R/W\\]
= The configured external trigger will initiate a new transmission.2h \\[R/W\\]
= Either writing to TX_FRAME_CTRL.START or the TX_FRAME_TAG_UDATA register will initiate a new transmission. All other combinations of bits are illegal and reserved for future use."]
pub type StartModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_CRC` reader - 6:6\\]
CRC Source Select bitThis bit selects the source of the CRC value that is transmitted. 0h \\[R/W\\]
= The transmitted CRC value is computed by hardware.1h \\[R/W\\]
= The transmitted CRC value is sourced from the value programmed in the TX_USER_CRC register."]
pub type SwCrcR = crate::BitReader;
#[doc = "Field `SW_CRC` writer - 6:6\\]
CRC Source Select bitThis bit selects the source of the CRC value that is transmitted. 0h \\[R/W\\]
= The transmitted CRC value is computed by hardware.1h \\[R/W\\]
= The transmitted CRC value is sourced from the value programmed in the TX_USER_CRC register."]
pub type SwCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TO_MODE` reader - 7:7\\]
Ping Counter Reset Mode Select bitThis bit selects when the ping counter will reset. 0h \\[R/W\\]
= The ping counter will reset and restart only on hardware initiated ping frames, when ping counter has timed out.1h \\[R/W\\]
= The ping counter will reset and restart on any software initiated frame as well as a ping counter timeout"]
pub type PingToModeR = crate::BitReader;
#[doc = "Field `PING_TO_MODE` writer - 7:7\\]
Ping Counter Reset Mode Select bitThis bit selects when the ping counter will reset. 0h \\[R/W\\]
= The ping counter will reset and restart only on hardware initiated ping frames, when ping counter has timed out.1h \\[R/W\\]
= The ping counter will reset and restart on any software initiated frame as well as a ping counter timeout"]
pub type PingToModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_PLLCLK` reader - 8:8\\]
Input Clock Select bitThis bit selects the input clock source for the transmitter core. 0h \\[R/W\\]
= SYSCLK is the source of the transmitter clock into the clock prescaler.1h \\[R/W\\]
= PLLRAWCLK is the source of the transmitter core clock into the clock prescaler."]
pub type SelPllclkR = crate::BitReader;
#[doc = "Field `SEL_PLLCLK` writer - 8:8\\]
Input Clock Select bitThis bit selects the input clock source for the transmitter core. 0h \\[R/W\\]
= SYSCLK is the source of the transmitter clock into the clock prescaler.1h \\[R/W\\]
= PLLRAWCLK is the source of the transmitter core clock into the clock prescaler."]
pub type SelPllclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDM_ENABLE` reader - 9:9\\]
Transmit TDM Mode Enable bit.This bit enables the TDM Mode for multi-slave TDM operation. 0h \\[R/W\\]
Transmit TDM Mode is not enabled.1h \\[R/W\\]
Transmit TDM Mode is enabled."]
pub type TdmEnableR = crate::BitReader;
#[doc = "Field `TDM_ENABLE` writer - 9:9\\]
Transmit TDM Mode Enable bit.This bit enables the TDM Mode for multi-slave TDM operation. 0h \\[R/W\\]
Transmit TDM Mode is not enabled.1h \\[R/W\\]
Transmit TDM Mode is enabled."]
pub type TdmEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Transmit Data Width Select bitsThese bits define the number of data lines used by the transmitter. 0h \\[R/W\\]
= Data will be transmitted on one data line \\[TXD0\\]1h \\[R/W\\]
= Data will be transmitted on two data lines \\[TXD0 and TXD1\\]. The format of the data is described in the preceeding chapter.2h, 3h \\[R/W\\]
= Reserved"]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Mode Select bitThis bit enables and disables SPI compatibility mode. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
    #[inline(always)]
    pub fn spi_mode(&self) -> SpiModeR {
        SpiModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Transmission Start Mode Select bitThese bits select the method by which a new frame transmission is started. 0h \\[R/W\\]
= Only a software write to TX_FRAME_CTRL.START initiate a new transmission.1h \\[R/W\\]
= The configured external trigger will initiate a new transmission.2h \\[R/W\\]
= Either writing to TX_FRAME_CTRL.START or the TX_FRAME_TAG_UDATA register will initiate a new transmission. All other combinations of bits are illegal and reserved for future use."]
    #[inline(always)]
    pub fn start_mode(&self) -> StartModeR {
        StartModeR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
CRC Source Select bitThis bit selects the source of the CRC value that is transmitted. 0h \\[R/W\\]
= The transmitted CRC value is computed by hardware.1h \\[R/W\\]
= The transmitted CRC value is sourced from the value programmed in the TX_USER_CRC register."]
    #[inline(always)]
    pub fn sw_crc(&self) -> SwCrcR {
        SwCrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Ping Counter Reset Mode Select bitThis bit selects when the ping counter will reset. 0h \\[R/W\\]
= The ping counter will reset and restart only on hardware initiated ping frames, when ping counter has timed out.1h \\[R/W\\]
= The ping counter will reset and restart on any software initiated frame as well as a ping counter timeout"]
    #[inline(always)]
    pub fn ping_to_mode(&self) -> PingToModeR {
        PingToModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Input Clock Select bitThis bit selects the input clock source for the transmitter core. 0h \\[R/W\\]
= SYSCLK is the source of the transmitter clock into the clock prescaler.1h \\[R/W\\]
= PLLRAWCLK is the source of the transmitter core clock into the clock prescaler."]
    #[inline(always)]
    pub fn sel_pllclk(&self) -> SelPllclkR {
        SelPllclkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmit TDM Mode Enable bit.This bit enables the TDM Mode for multi-slave TDM operation. 0h \\[R/W\\]
Transmit TDM Mode is not enabled.1h \\[R/W\\]
Transmit TDM Mode is enabled."]
    #[inline(always)]
    pub fn tdm_enable(&self) -> TdmEnableR {
        TdmEnableR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Transmit Data Width Select bitsThese bits define the number of data lines used by the transmitter. 0h \\[R/W\\]
= Data will be transmitted on one data line \\[TXD0\\]1h \\[R/W\\]
= Data will be transmitted on two data lines \\[TXD0 and TXD1\\]. The format of the data is described in the preceeding chapter.2h, 3h \\[R/W\\]
= Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DataWidthW<CfgTxOperCtrlLoAlt1_Spec> {
        DataWidthW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SPI Mode Select bitThis bit enables and disables SPI compatibility mode. 0h \\[R/W\\]
= FSI is in normal mode of operation.1h \\[R/W\\]
= FSI is operating in SPI compatibility mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mode(&mut self) -> SpiModeW<CfgTxOperCtrlLoAlt1_Spec> {
        SpiModeW::new(self, 2)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Transmission Start Mode Select bitThese bits select the method by which a new frame transmission is started. 0h \\[R/W\\]
= Only a software write to TX_FRAME_CTRL.START initiate a new transmission.1h \\[R/W\\]
= The configured external trigger will initiate a new transmission.2h \\[R/W\\]
= Either writing to TX_FRAME_CTRL.START or the TX_FRAME_TAG_UDATA register will initiate a new transmission. All other combinations of bits are illegal and reserved for future use."]
    #[inline(always)]
    #[must_use]
    pub fn start_mode(&mut self) -> StartModeW<CfgTxOperCtrlLoAlt1_Spec> {
        StartModeW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
CRC Source Select bitThis bit selects the source of the CRC value that is transmitted. 0h \\[R/W\\]
= The transmitted CRC value is computed by hardware.1h \\[R/W\\]
= The transmitted CRC value is sourced from the value programmed in the TX_USER_CRC register."]
    #[inline(always)]
    #[must_use]
    pub fn sw_crc(&mut self) -> SwCrcW<CfgTxOperCtrlLoAlt1_Spec> {
        SwCrcW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Ping Counter Reset Mode Select bitThis bit selects when the ping counter will reset. 0h \\[R/W\\]
= The ping counter will reset and restart only on hardware initiated ping frames, when ping counter has timed out.1h \\[R/W\\]
= The ping counter will reset and restart on any software initiated frame as well as a ping counter timeout"]
    #[inline(always)]
    #[must_use]
    pub fn ping_to_mode(&mut self) -> PingToModeW<CfgTxOperCtrlLoAlt1_Spec> {
        PingToModeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Input Clock Select bitThis bit selects the input clock source for the transmitter core. 0h \\[R/W\\]
= SYSCLK is the source of the transmitter clock into the clock prescaler.1h \\[R/W\\]
= PLLRAWCLK is the source of the transmitter core clock into the clock prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn sel_pllclk(&mut self) -> SelPllclkW<CfgTxOperCtrlLoAlt1_Spec> {
        SelPllclkW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmit TDM Mode Enable bit.This bit enables the TDM Mode for multi-slave TDM operation. 0h \\[R/W\\]
Transmit TDM Mode is not enabled.1h \\[R/W\\]
Transmit TDM Mode is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tdm_enable(&mut self) -> TdmEnableW<CfgTxOperCtrlLoAlt1_Spec> {
        TdmEnableW::new(self, 9)
    }
}
#[doc = "Transmit operation control register low. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_oper_ctrl_lo_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_oper_ctrl_lo_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxOperCtrlLoAlt1_Spec;
impl crate::RegisterSpec for CfgTxOperCtrlLoAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_oper_ctrl_lo_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgTxOperCtrlLoAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_oper_ctrl_lo_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgTxOperCtrlLoAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_OPER_CTRL_LO_ALT1_ to value 0"]
impl crate::Resettable for CfgTxOperCtrlLoAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
