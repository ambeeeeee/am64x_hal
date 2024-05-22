#[doc = "Register `PKTDMA_TCHANRT_TRT_STATUS1` reader"]
pub type R = crate::R<PktdmaTchanrtTrtStatus1Spec>;
#[doc = "Register `PKTDMA_TCHANRT_TRT_STATUS1` writer"]
pub type W = crate::W<PktdmaTchanrtTrtStatus1Spec>;
#[doc = "Field `IN_PACKET_ARRAY` reader - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayR = crate::BitReader;
#[doc = "Field `IN_PACKET_ARRAY` writer - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_BUSY` reader - 6:6\\]
The channel has active work"]
pub type ChannelBusyR = crate::BitReader;
#[doc = "Field `CHANNEL_BUSY` writer - 6:6\\]
The channel has active work"]
pub type ChannelBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_OK` reader - 7:7\\]
The channel is trying to schedule work"]
pub type ChannelOkR = crate::BitReader;
#[doc = "Field `CHANNEL_OK` writer - 7:7\\]
The channel is trying to schedule work"]
pub type ChannelOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDNULL` reader - 8:8\\]
The channel has met the conditions to attempt to teardown"]
pub type TdnullR = crate::BitReader;
#[doc = "Field `TDNULL` writer - 8:8\\]
The channel has met the conditions to attempt to teardown"]
pub type TdnullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVAIL` reader - 24:24\\]
The fifo has space for a burst size"]
pub type WavailR = crate::BitReader;
#[doc = "Field `WAVAIL` writer - 24:24\\]
The fifo has space for a burst size"]
pub type WavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOP_WAVAIL` reader - 25:25\\]
The FIFO has space for the middle of a packet"]
pub type MopWavailR = crate::BitReader;
#[doc = "Field `MOP_WAVAIL` writer - 25:25\\]
The FIFO has space for the middle of a packet"]
pub type MopWavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOP_WAVAIL` reader - 26:26\\]
The FIFO has space for the start of a packet"]
pub type SopWavailR = crate::BitReader;
#[doc = "Field `SOP_WAVAIL` writer - 26:26\\]
The FIFO has space for the start of a packet"]
pub type SopWavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_REQS` reader - 31:31\\]
The channel is sending a schedule request"]
pub type TxReqsR = crate::BitReader;
#[doc = "Field `TX_REQS` writer - 31:31\\]
The channel is sending a schedule request"]
pub type TxReqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
The channel is in a packet"]
    #[inline(always)]
    pub fn in_packet_array(&self) -> InPacketArrayR {
        InPacketArrayR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
The channel has active work"]
    #[inline(always)]
    pub fn channel_busy(&self) -> ChannelBusyR {
        ChannelBusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
The channel is trying to schedule work"]
    #[inline(always)]
    pub fn channel_ok(&self) -> ChannelOkR {
        ChannelOkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
The channel has met the conditions to attempt to teardown"]
    #[inline(always)]
    pub fn tdnull(&self) -> TdnullR {
        TdnullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
The fifo has space for a burst size"]
    #[inline(always)]
    pub fn wavail(&self) -> WavailR {
        WavailR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
The FIFO has space for the middle of a packet"]
    #[inline(always)]
    pub fn mop_wavail(&self) -> MopWavailR {
        MopWavailR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
The FIFO has space for the start of a packet"]
    #[inline(always)]
    pub fn sop_wavail(&self) -> SopWavailR {
        SopWavailR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    pub fn tx_reqs(&self) -> TxReqsR {
        TxReqsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
The channel is in a packet"]
    #[inline(always)]
    #[must_use]
    pub fn in_packet_array(&mut self) -> InPacketArrayW<PktdmaTchanrtTrtStatus1Spec> {
        InPacketArrayW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
The channel has active work"]
    #[inline(always)]
    #[must_use]
    pub fn channel_busy(&mut self) -> ChannelBusyW<PktdmaTchanrtTrtStatus1Spec> {
        ChannelBusyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
The channel is trying to schedule work"]
    #[inline(always)]
    #[must_use]
    pub fn channel_ok(&mut self) -> ChannelOkW<PktdmaTchanrtTrtStatus1Spec> {
        ChannelOkW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
The channel has met the conditions to attempt to teardown"]
    #[inline(always)]
    #[must_use]
    pub fn tdnull(&mut self) -> TdnullW<PktdmaTchanrtTrtStatus1Spec> {
        TdnullW::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
The fifo has space for a burst size"]
    #[inline(always)]
    #[must_use]
    pub fn wavail(&mut self) -> WavailW<PktdmaTchanrtTrtStatus1Spec> {
        WavailW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
The FIFO has space for the middle of a packet"]
    #[inline(always)]
    #[must_use]
    pub fn mop_wavail(&mut self) -> MopWavailW<PktdmaTchanrtTrtStatus1Spec> {
        MopWavailW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
The FIFO has space for the start of a packet"]
    #[inline(always)]
    #[must_use]
    pub fn sop_wavail(&mut self) -> SopWavailW<PktdmaTchanrtTrtStatus1Spec> {
        SopWavailW::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reqs(&mut self) -> TxReqsW<PktdmaTchanrtTrtStatus1Spec> {
        TxReqsW::new(self, 31)
    }
}
#[doc = "The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaTchanrtTrtStatus1Spec;
impl crate::RegisterSpec for PktdmaTchanrtTrtStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_tchanrt_trt_status1::R`](R) reader structure"]
impl crate::Readable for PktdmaTchanrtTrtStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_tchanrt_trt_status1::W`](W) writer structure"]
impl crate::Writable for PktdmaTchanrtTrtStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_TCHANRT_TRT_STATUS1 to value 0"]
impl crate::Resettable for PktdmaTchanrtTrtStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
