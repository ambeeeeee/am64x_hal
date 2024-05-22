#[doc = "Register `PKTDMA_RCHANRT_RRT_STATUS0` reader"]
pub type R = crate::R<PktdmaRchanrtRrtStatus0Spec>;
#[doc = "Register `PKTDMA_RCHANRT_RRT_STATUS0` writer"]
pub type W = crate::W<PktdmaRchanrtRrtStatus0Spec>;
#[doc = "Field `ERR_EVENT_REQS` reader - 16:16\\]
The channel is trying to schedule an error event"]
pub type ErrEventReqsR = crate::BitReader;
#[doc = "Field `ERR_EVENT_REQS` writer - 16:16\\]
The channel is trying to schedule an error event"]
pub type ErrEventReqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REQS` reader - 17:17\\]
The channel is sending a schedule request"]
pub type RxReqsR = crate::BitReader;
#[doc = "Field `RX_REQS` writer - 17:17\\]
The channel is sending a schedule request"]
pub type RxReqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDOWN_MSG_PEND` reader - 18:18\\]
A teardown message is pending"]
pub type TdownMsgPendR = crate::BitReader;
#[doc = "Field `TDOWN_MSG_PEND` writer - 18:18\\]
A teardown message is pending"]
pub type TdownMsgPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAVAIL` reader - 21:21\\]
The fifo for the channel has space to place a burst size entry"]
pub type WavailR = crate::BitReader;
#[doc = "Field `WAVAIL` writer - 21:21\\]
The fifo for the channel has space to place a burst size entry"]
pub type WavailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OK` reader - 22:22\\]
The channel is ready to be scheduled"]
pub type OkR = crate::BitReader;
#[doc = "Field `OK` writer - 22:22\\]
The channel is ready to be scheduled"]
pub type OkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_PACKET` reader - 23:23\\]
The channel is currently in a packet"]
pub type InPacketR = crate::BitReader;
#[doc = "Field `IN_PACKET` writer - 23:23\\]
The channel is currently in a packet"]
pub type InPacketW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSBUSY` reader - 24:24\\]
The channel has an outstanding transaction"]
pub type TransbusyR = crate::BitReader;
#[doc = "Field `TRANSBUSY` writer - 24:24\\]
The channel has an outstanding transaction"]
pub type TransbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - 25:25\\]
The channel is busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 25:25\\]
The channel is busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTID_BUSY` reader - 28:28\\]
There is an outstanding pktid for the channel"]
pub type PktidBusyR = crate::BitReader;
#[doc = "Field `PKTID_BUSY` writer - 28:28\\]
There is an outstanding pktid for the channel"]
pub type PktidBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTID_AVAIL` reader - 29:29\\]
The channel has an available packet id"]
pub type PktidAvailR = crate::BitReader;
#[doc = "Field `PKTID_AVAIL` writer - 29:29\\]
The channel has an available packet id"]
pub type PktidAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXQ_PEND` reader - 30:30\\]
The channel fifo is available"]
pub type RxqPendR = crate::BitReader;
#[doc = "Field `RXQ_PEND` writer - 30:30\\]
The channel fifo is available"]
pub type RxqPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRING_PEND` reader - 31:31\\]
The channel ring has a descriptor"]
pub type RringPendR = crate::BitReader;
#[doc = "Field `RRING_PEND` writer - 31:31\\]
The channel ring has a descriptor"]
pub type RringPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - 16:16\\]
The channel is trying to schedule an error event"]
    #[inline(always)]
    pub fn err_event_reqs(&self) -> ErrEventReqsR {
        ErrEventReqsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
The channel is sending a schedule request"]
    #[inline(always)]
    pub fn rx_reqs(&self) -> RxReqsR {
        RxReqsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
A teardown message is pending"]
    #[inline(always)]
    pub fn tdown_msg_pend(&self) -> TdownMsgPendR {
        TdownMsgPendR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
The fifo for the channel has space to place a burst size entry"]
    #[inline(always)]
    pub fn wavail(&self) -> WavailR {
        WavailR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
The channel is ready to be scheduled"]
    #[inline(always)]
    pub fn ok(&self) -> OkR {
        OkR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
The channel is currently in a packet"]
    #[inline(always)]
    pub fn in_packet(&self) -> InPacketR {
        InPacketR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
The channel has an outstanding transaction"]
    #[inline(always)]
    pub fn transbusy(&self) -> TransbusyR {
        TransbusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
The channel is busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
There is an outstanding pktid for the channel"]
    #[inline(always)]
    pub fn pktid_busy(&self) -> PktidBusyR {
        PktidBusyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
The channel has an available packet id"]
    #[inline(always)]
    pub fn pktid_avail(&self) -> PktidAvailR {
        PktidAvailR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
The channel fifo is available"]
    #[inline(always)]
    pub fn rxq_pend(&self) -> RxqPendR {
        RxqPendR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel ring has a descriptor"]
    #[inline(always)]
    pub fn rring_pend(&self) -> RringPendR {
        RringPendR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - 16:16\\]
The channel is trying to schedule an error event"]
    #[inline(always)]
    #[must_use]
    pub fn err_event_reqs(&mut self) -> ErrEventReqsW<PktdmaRchanrtRrtStatus0Spec> {
        ErrEventReqsW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
The channel is sending a schedule request"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reqs(&mut self) -> RxReqsW<PktdmaRchanrtRrtStatus0Spec> {
        RxReqsW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
A teardown message is pending"]
    #[inline(always)]
    #[must_use]
    pub fn tdown_msg_pend(&mut self) -> TdownMsgPendW<PktdmaRchanrtRrtStatus0Spec> {
        TdownMsgPendW::new(self, 18)
    }
    #[doc = "Bit 21 - 21:21\\]
The fifo for the channel has space to place a burst size entry"]
    #[inline(always)]
    #[must_use]
    pub fn wavail(&mut self) -> WavailW<PktdmaRchanrtRrtStatus0Spec> {
        WavailW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
The channel is ready to be scheduled"]
    #[inline(always)]
    #[must_use]
    pub fn ok(&mut self) -> OkW<PktdmaRchanrtRrtStatus0Spec> {
        OkW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
The channel is currently in a packet"]
    #[inline(always)]
    #[must_use]
    pub fn in_packet(&mut self) -> InPacketW<PktdmaRchanrtRrtStatus0Spec> {
        InPacketW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
The channel has an outstanding transaction"]
    #[inline(always)]
    #[must_use]
    pub fn transbusy(&mut self) -> TransbusyW<PktdmaRchanrtRrtStatus0Spec> {
        TransbusyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
The channel is busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<PktdmaRchanrtRrtStatus0Spec> {
        BusyW::new(self, 25)
    }
    #[doc = "Bit 28 - 28:28\\]
There is an outstanding pktid for the channel"]
    #[inline(always)]
    #[must_use]
    pub fn pktid_busy(&mut self) -> PktidBusyW<PktdmaRchanrtRrtStatus0Spec> {
        PktidBusyW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
The channel has an available packet id"]
    #[inline(always)]
    #[must_use]
    pub fn pktid_avail(&mut self) -> PktidAvailW<PktdmaRchanrtRrtStatus0Spec> {
        PktidAvailW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
The channel fifo is available"]
    #[inline(always)]
    #[must_use]
    pub fn rxq_pend(&mut self) -> RxqPendW<PktdmaRchanrtRrtStatus0Spec> {
        RxqPendW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel ring has a descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn rring_pend(&mut self) -> RringPendW<PktdmaRchanrtRrtStatus0Spec> {
        RringPendW::new(self, 31)
    }
}
#[doc = "The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRchanrtRrtStatus0Spec;
impl crate::RegisterSpec for PktdmaRchanrtRrtStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_rchanrt_rrt_status0::R`](R) reader structure"]
impl crate::Readable for PktdmaRchanrtRrtStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_rchanrt_rrt_status0::W`](W) writer structure"]
impl crate::Writable for PktdmaRchanrtRrtStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RCHANRT_RRT_STATUS0 to value 0"]
impl crate::Resettable for PktdmaRchanrtRrtStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
