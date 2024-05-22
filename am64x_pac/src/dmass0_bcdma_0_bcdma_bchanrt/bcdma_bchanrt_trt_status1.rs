#[doc = "Register `BCDMA_BCHANRT_TRT_STATUS1` reader"]
pub type R = crate::R<BcdmaBchanrtTrtStatus1Spec>;
#[doc = "Register `BCDMA_BCHANRT_TRT_STATUS1` writer"]
pub type W = crate::W<BcdmaBchanrtTrtStatus1Spec>;
#[doc = "Field `IN_PACKET_ARRAY` reader - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayR = crate::BitReader;
#[doc = "Field `IN_PACKET_ARRAY` writer - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_BUSY` reader - 6:6\\]
The channel is active"]
pub type ChannelBusyR = crate::BitReader;
#[doc = "Field `CHANNEL_BUSY` writer - 6:6\\]
The channel is active"]
pub type ChannelBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_OK` reader - 7:7\\]
Channel is trying to schedule a transaction"]
pub type ChannelOkR = crate::BitReader;
#[doc = "Field `CHANNEL_OK` writer - 7:7\\]
Channel is trying to schedule a transaction"]
pub type ChannelOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDNULL` reader - 8:8\\]
Channel is trying to teardown and has met conditions"]
pub type TdnullR = crate::BitReader;
#[doc = "Field `TDNULL` writer - 8:8\\]
Channel is trying to teardown and has met conditions"]
pub type TdnullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVAIL` reader - 24:24\\]
The fifo has space for a burst size"]
pub type WavailR = crate::BitReader;
#[doc = "Field `WAVAIL` writer - 24:24\\]
The fifo has space for a burst size"]
pub type WavailW<'a, REG> = crate::BitWriter<'a, REG>;
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
The channel is active"]
    #[inline(always)]
    pub fn channel_busy(&self) -> ChannelBusyR {
        ChannelBusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Channel is trying to schedule a transaction"]
    #[inline(always)]
    pub fn channel_ok(&self) -> ChannelOkR {
        ChannelOkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel is trying to teardown and has met conditions"]
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
    pub fn in_packet_array(&mut self) -> InPacketArrayW<BcdmaBchanrtTrtStatus1Spec> {
        InPacketArrayW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
The channel is active"]
    #[inline(always)]
    #[must_use]
    pub fn channel_busy(&mut self) -> ChannelBusyW<BcdmaBchanrtTrtStatus1Spec> {
        ChannelBusyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Channel is trying to schedule a transaction"]
    #[inline(always)]
    #[must_use]
    pub fn channel_ok(&mut self) -> ChannelOkW<BcdmaBchanrtTrtStatus1Spec> {
        ChannelOkW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel is trying to teardown and has met conditions"]
    #[inline(always)]
    #[must_use]
    pub fn tdnull(&mut self) -> TdnullW<BcdmaBchanrtTrtStatus1Spec> {
        TdnullW::new(self, 8)
    }
    #[doc = "Bit 24 - 24:24\\]
The fifo has space for a burst size"]
    #[inline(always)]
    #[must_use]
    pub fn wavail(&mut self) -> WavailW<BcdmaBchanrtTrtStatus1Spec> {
        WavailW::new(self, 24)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    #[must_use]
    pub fn tx_reqs(&mut self) -> TxReqsW<BcdmaBchanrtTrtStatus1Spec> {
        TxReqsW::new(self, 31)
    }
}
#[doc = "The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaBchanrtTrtStatus1Spec;
impl crate::RegisterSpec for BcdmaBchanrtTrtStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_bchanrt_trt_status1::R`](R) reader structure"]
impl crate::Readable for BcdmaBchanrtTrtStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_bchanrt_trt_status1::W`](W) writer structure"]
impl crate::Writable for BcdmaBchanrtTrtStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_BCHANRT_TRT_STATUS1 to value 0"]
impl crate::Resettable for BcdmaBchanrtTrtStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
