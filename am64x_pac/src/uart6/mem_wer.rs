#[doc = "Register `MEM_WER` reader"]
pub type R = crate::R<MemWerSpec>;
#[doc = "Register `MEM_WER` writer"]
pub type W = crate::W<MemWerSpec>;
#[doc = "Field `EVENT_0_CTS_ACTIVITY` reader - "]
pub type Event0CtsActivityR = crate::BitReader;
#[doc = "Field `EVENT_0_CTS_ACTIVITY` writer - "]
pub type Event0CtsActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_1_DSR_ACTIVITY` reader - "]
pub type Event1DsrActivityR = crate::BitReader;
#[doc = "Field `EVENT_1_DSR_ACTIVITY` writer - "]
pub type Event1DsrActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_2_RI_ACTIVITY` reader - "]
pub type Event2RiActivityR = crate::BitReader;
#[doc = "Field `EVENT_2_RI_ACTIVITY` writer - "]
pub type Event2RiActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_3_DCD_CD_ACTIVITY` reader - "]
pub type Event3DcdCdActivityR = crate::BitReader;
#[doc = "Field `EVENT_3_DCD_CD_ACTIVITY` writer - "]
pub type Event3DcdCdActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_4_RX_ACTIVITY` reader - "]
pub type Event4RxActivityR = crate::BitReader;
#[doc = "Field `EVENT_4_RX_ACTIVITY` writer - "]
pub type Event4RxActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_5_RHR_INTERRUPT` reader - "]
pub type Event5RhrInterruptR = crate::BitReader;
#[doc = "Field `EVENT_5_RHR_INTERRUPT` writer - "]
pub type Event5RhrInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_6_RECEIVER_LINE_STATUS_INTERRUPT` reader - "]
pub type Event6ReceiverLineStatusInterruptR = crate::BitReader;
#[doc = "Field `EVENT_6_RECEIVER_LINE_STATUS_INTERRUPT` writer - "]
pub type Event6ReceiverLineStatusInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_7_TX_WAKEUP_EN` reader - "]
pub type Event7TxWakeupEnR = crate::BitReader;
#[doc = "Field `EVENT_7_TX_WAKEUP_EN` writer - "]
pub type Event7TxWakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn event_0_cts_activity(&self) -> Event0CtsActivityR {
        Event0CtsActivityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn event_1_dsr_activity(&self) -> Event1DsrActivityR {
        Event1DsrActivityR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn event_2_ri_activity(&self) -> Event2RiActivityR {
        Event2RiActivityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn event_3_dcd_cd_activity(&self) -> Event3DcdCdActivityR {
        Event3DcdCdActivityR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn event_4_rx_activity(&self) -> Event4RxActivityR {
        Event4RxActivityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn event_5_rhr_interrupt(&self) -> Event5RhrInterruptR {
        Event5RhrInterruptR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn event_6_receiver_line_status_interrupt(&self) -> Event6ReceiverLineStatusInterruptR {
        Event6ReceiverLineStatusInterruptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn event_7_tx_wakeup_en(&self) -> Event7TxWakeupEnR {
        Event7TxWakeupEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn event_0_cts_activity(&mut self) -> Event0CtsActivityW<MemWerSpec> {
        Event0CtsActivityW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn event_1_dsr_activity(&mut self) -> Event1DsrActivityW<MemWerSpec> {
        Event1DsrActivityW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn event_2_ri_activity(&mut self) -> Event2RiActivityW<MemWerSpec> {
        Event2RiActivityW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn event_3_dcd_cd_activity(&mut self) -> Event3DcdCdActivityW<MemWerSpec> {
        Event3DcdCdActivityW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn event_4_rx_activity(&mut self) -> Event4RxActivityW<MemWerSpec> {
        Event4RxActivityW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn event_5_rhr_interrupt(&mut self) -> Event5RhrInterruptW<MemWerSpec> {
        Event5RhrInterruptW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn event_6_receiver_line_status_interrupt(
        &mut self,
    ) -> Event6ReceiverLineStatusInterruptW<MemWerSpec> {
        Event6ReceiverLineStatusInterruptW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn event_7_tx_wakeup_en(&mut self) -> Event7TxWakeupEnW<MemWerSpec> {
        Event7TxWakeupEnW::new(self, 7)
    }
}
#[doc = "The UART wakeup enable register is used to mask and unmask a UART event that would subsequently notify the system. The events are any activity in the logic that could cause an interrupt and/ or an activity that would require the system to wakeup. It should be noted that even if the wakeup is disabled for certain events, if these events are also an interrupt to the UART, then the UART will still register the interrupt as such.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_wer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_wer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemWerSpec;
impl crate::RegisterSpec for MemWerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_wer::R`](R) reader structure"]
impl crate::Readable for MemWerSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_wer::W`](W) writer structure"]
impl crate::Writable for MemWerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_WER to value 0xff"]
impl crate::Resettable for MemWerSpec {
    const RESET_VALUE: u32 = 0xff;
}
