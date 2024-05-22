#[doc = "Register `CFG_IRQSTATUS` reader"]
pub type R = crate::R<CfgIrqstatusSpec>;
#[doc = "Register `CFG_IRQSTATUS` writer"]
pub type W = crate::W<CfgIrqstatusSpec>;
#[doc = "Field `TX0_EMPTY` reader - 0:0\\]
Transmitter register empty or almost empty Channel 0"]
pub type Tx0EmptyR = crate::BitReader;
#[doc = "Field `TX0_EMPTY` writer - 0:0\\]
Transmitter register empty or almost empty Channel 0"]
pub type Tx0EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX0_UNDERFLOW` reader - 1:1\\]
Transmitter register underflow Channel 0"]
pub type Tx0UnderflowR = crate::BitReader;
#[doc = "Field `TX0_UNDERFLOW` writer - 1:1\\]
Transmitter register underflow Channel 0"]
pub type Tx0UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX0_FULL` reader - 2:2\\]
Receiver register full or almost full Channel 0"]
pub type Rx0FullR = crate::BitReader;
#[doc = "Field `RX0_FULL` writer - 2:2\\]
Receiver register full or almost full Channel 0"]
pub type Rx0FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX0_OVERFLOW` reader - 3:3\\]
Receiver register overflow \\[slave mode only\\]
Channel 0"]
pub type Rx0OverflowR = crate::BitReader;
#[doc = "Field `RX0_OVERFLOW` writer - 3:3\\]
Receiver register overflow \\[slave mode only\\]
Channel 0"]
pub type Rx0OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1_EMPTY` reader - 4:4\\]
Transmitter register empty or almost empty Channel 1"]
pub type Tx1EmptyR = crate::BitReader;
#[doc = "Field `TX1_EMPTY` writer - 4:4\\]
Transmitter register empty or almost empty Channel 1"]
pub type Tx1EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX1_UNDERFLOW` reader - 5:5\\]
Transmitter register underflow Channel 1"]
pub type Tx1UnderflowR = crate::BitReader;
#[doc = "Field `TX1_UNDERFLOW` writer - 5:5\\]
Transmitter register underflow Channel 1"]
pub type Tx1UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX1_FULL` reader - 6:6\\]
Receiver register full or almost full Channel 1"]
pub type Rx1FullR = crate::BitReader;
#[doc = "Field `RX1_FULL` writer - 6:6\\]
Receiver register full or almost full Channel 1"]
pub type Rx1FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX2_EMPTY` reader - 8:8\\]
Transmitter register empty or almost empty Channel 2"]
pub type Tx2EmptyR = crate::BitReader;
#[doc = "Field `TX2_EMPTY` writer - 8:8\\]
Transmitter register empty or almost empty Channel 2"]
pub type Tx2EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX2_UNDERFLOW` reader - 9:9\\]
Transmitter register underflow Channel 2"]
pub type Tx2UnderflowR = crate::BitReader;
#[doc = "Field `TX2_UNDERFLOW` writer - 9:9\\]
Transmitter register underflow Channel 2"]
pub type Tx2UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX2_FULL` reader - 10:10\\]
Receiver register full or almost full Channel 2"]
pub type Rx2FullR = crate::BitReader;
#[doc = "Field `RX2_FULL` writer - 10:10\\]
Receiver register full or almost full Channel 2"]
pub type Rx2FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX3_EMPTY` reader - 12:12\\]
Transmitter register is empty or almost empty Note: Enabling the channel automatically rises this event"]
pub type Tx3EmptyR = crate::BitReader;
#[doc = "Field `TX3_EMPTY` writer - 12:12\\]
Transmitter register is empty or almost empty Note: Enabling the channel automatically rises this event"]
pub type Tx3EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX3_UNDERFLOW` reader - 13:13\\]
Transmitter register underflow Only when Channel 3 is enabled The transmitter register is empty \\[not updated by Host or DMA with new data\\]
before its time slot assignment Exception: No TX_underflow event when no data has been loaded into the transmitter register since channel has been enabled"]
pub type Tx3UnderflowR = crate::BitReader;
#[doc = "Field `TX3_UNDERFLOW` writer - 13:13\\]
Transmitter register underflow Only when Channel 3 is enabled The transmitter register is empty \\[not updated by Host or DMA with new data\\]
before its time slot assignment Exception: No TX_underflow event when no data has been loaded into the transmitter register since channel has been enabled"]
pub type Tx3UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX3_FULL` reader - 14:14\\]
Receiver register is full or almost full Only when Channel 3 is enabled"]
pub type Rx3FullR = crate::BitReader;
#[doc = "Field `RX3_FULL` writer - 14:14\\]
Receiver register is full or almost full Only when Channel 3 is enabled"]
pub type Rx3FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKS` reader - 16:16\\]
Wake Up event in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]"]
pub type WksR = crate::BitReader;
#[doc = "Field `WKS` writer - 16:16\\]
Wake Up event in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]"]
pub type WksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOW` reader - 17:17\\]
End of word count event when a channel is enabled using the FIFO buffer and the channel had sent the number of SPI word defined by MCSPI_XFERLEVEL\\[WCNT\\]"]
pub type EowR = crate::BitReader;
#[doc = "Field `EOW` writer - 17:17\\]
End of word count event when a channel is enabled using the FIFO buffer and the channel had sent the number of SPI word defined by MCSPI_XFERLEVEL\\[WCNT\\]"]
pub type EowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmitter register empty or almost empty Channel 0"]
    #[inline(always)]
    pub fn tx0_empty(&self) -> Tx0EmptyR {
        Tx0EmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmitter register underflow Channel 0"]
    #[inline(always)]
    pub fn tx0_underflow(&self) -> Tx0UnderflowR {
        Tx0UnderflowR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receiver register full or almost full Channel 0"]
    #[inline(always)]
    pub fn rx0_full(&self) -> Rx0FullR {
        Rx0FullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receiver register overflow \\[slave mode only\\]
Channel 0"]
    #[inline(always)]
    pub fn rx0_overflow(&self) -> Rx0OverflowR {
        Rx0OverflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmitter register empty or almost empty Channel 1"]
    #[inline(always)]
    pub fn tx1_empty(&self) -> Tx1EmptyR {
        Tx1EmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter register underflow Channel 1"]
    #[inline(always)]
    pub fn tx1_underflow(&self) -> Tx1UnderflowR {
        Tx1UnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver register full or almost full Channel 1"]
    #[inline(always)]
    pub fn rx1_full(&self) -> Rx1FullR {
        Rx1FullR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter register empty or almost empty Channel 2"]
    #[inline(always)]
    pub fn tx2_empty(&self) -> Tx2EmptyR {
        Tx2EmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter register underflow Channel 2"]
    #[inline(always)]
    pub fn tx2_underflow(&self) -> Tx2UnderflowR {
        Tx2UnderflowR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Receiver register full or almost full Channel 2"]
    #[inline(always)]
    pub fn rx2_full(&self) -> Rx2FullR {
        Rx2FullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmitter register is empty or almost empty Note: Enabling the channel automatically rises this event"]
    #[inline(always)]
    pub fn tx3_empty(&self) -> Tx3EmptyR {
        Tx3EmptyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Transmitter register underflow Only when Channel 3 is enabled The transmitter register is empty \\[not updated by Host or DMA with new data\\]
before its time slot assignment Exception: No TX_underflow event when no data has been loaded into the transmitter register since channel has been enabled"]
    #[inline(always)]
    pub fn tx3_underflow(&self) -> Tx3UnderflowR {
        Tx3UnderflowR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver register is full or almost full Only when Channel 3 is enabled"]
    #[inline(always)]
    pub fn rx3_full(&self) -> Rx3FullR {
        Rx3FullR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Wake Up event in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]"]
    #[inline(always)]
    pub fn wks(&self) -> WksR {
        WksR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
End of word count event when a channel is enabled using the FIFO buffer and the channel had sent the number of SPI word defined by MCSPI_XFERLEVEL\\[WCNT\\]"]
    #[inline(always)]
    pub fn eow(&self) -> EowR {
        EowR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmitter register empty or almost empty Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx0_empty(&mut self) -> Tx0EmptyW<CfgIrqstatusSpec> {
        Tx0EmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmitter register underflow Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn tx0_underflow(&mut self) -> Tx0UnderflowW<CfgIrqstatusSpec> {
        Tx0UnderflowW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Receiver register full or almost full Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx0_full(&mut self) -> Rx0FullW<CfgIrqstatusSpec> {
        Rx0FullW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Receiver register overflow \\[slave mode only\\]
Channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx0_overflow(&mut self) -> Rx0OverflowW<CfgIrqstatusSpec> {
        Rx0OverflowW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Transmitter register empty or almost empty Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx1_empty(&mut self) -> Tx1EmptyW<CfgIrqstatusSpec> {
        Tx1EmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmitter register underflow Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn tx1_underflow(&mut self) -> Tx1UnderflowW<CfgIrqstatusSpec> {
        Tx1UnderflowW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Receiver register full or almost full Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rx1_full(&mut self) -> Rx1FullW<CfgIrqstatusSpec> {
        Rx1FullW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter register empty or almost empty Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx2_empty(&mut self) -> Tx2EmptyW<CfgIrqstatusSpec> {
        Tx2EmptyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmitter register underflow Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tx2_underflow(&mut self) -> Tx2UnderflowW<CfgIrqstatusSpec> {
        Tx2UnderflowW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Receiver register full or almost full Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx2_full(&mut self) -> Rx2FullW<CfgIrqstatusSpec> {
        Rx2FullW::new(self, 10)
    }
    #[doc = "Bit 12 - 12:12\\]
Transmitter register is empty or almost empty Note: Enabling the channel automatically rises this event"]
    #[inline(always)]
    #[must_use]
    pub fn tx3_empty(&mut self) -> Tx3EmptyW<CfgIrqstatusSpec> {
        Tx3EmptyW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Transmitter register underflow Only when Channel 3 is enabled The transmitter register is empty \\[not updated by Host or DMA with new data\\]
before its time slot assignment Exception: No TX_underflow event when no data has been loaded into the transmitter register since channel has been enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tx3_underflow(&mut self) -> Tx3UnderflowW<CfgIrqstatusSpec> {
        Tx3UnderflowW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Receiver register is full or almost full Only when Channel 3 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rx3_full(&mut self) -> Rx3FullW<CfgIrqstatusSpec> {
        Rx3FullW::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Wake Up event in slave mode when an active control signal is detected on the SPIEN line programmed in the field MCSPI_CH0CONF\\[SPIENSLV\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wks(&mut self) -> WksW<CfgIrqstatusSpec> {
        WksW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
End of word count event when a channel is enabled using the FIFO buffer and the channel had sent the number of SPI word defined by MCSPI_XFERLEVEL\\[WCNT\\]"]
    #[inline(always)]
    #[must_use]
    pub fn eow(&mut self) -> EowW<CfgIrqstatusSpec> {
        EowW::new(self, 17)
    }
}
#[doc = "The interrupt status regroups all the status of the module internal events that can generate an interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIrqstatusSpec;
impl crate::RegisterSpec for CfgIrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_irqstatus::R`](R) reader structure"]
impl crate::Readable for CfgIrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_irqstatus::W`](W) writer structure"]
impl crate::Writable for CfgIrqstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IRQSTATUS to value 0"]
impl crate::Resettable for CfgIrqstatusSpec {
    const RESET_VALUE: u32 = 0;
}
