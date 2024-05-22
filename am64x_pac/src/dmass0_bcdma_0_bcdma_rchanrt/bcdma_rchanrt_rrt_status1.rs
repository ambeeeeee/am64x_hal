#[doc = "Register `BCDMA_RCHANRT_RRT_STATUS1` reader"]
pub type R = crate::R<BcdmaRchanrtRrtStatus1Spec>;
#[doc = "Register `BCDMA_RCHANRT_RRT_STATUS1` writer"]
pub type W = crate::W<BcdmaRchanrtRrtStatus1Spec>;
#[doc = "Field `IN_PACKET_ARRAY` reader - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayR = crate::BitReader;
#[doc = "Field `IN_PACKET_ARRAY` writer - 3:3\\]
The channel is in a packet"]
pub type InPacketArrayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_BUSY` reader - 6:6\\]
Channel has active transactions"]
pub type ChannelBusyR = crate::BitReader;
#[doc = "Field `CHANNEL_BUSY` writer - 6:6\\]
Channel has active transactions"]
pub type ChannelBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL_OK` reader - 7:7\\]
Channel is trying to send data"]
pub type ChannelOkR = crate::BitReader;
#[doc = "Field `CHANNEL_OK` writer - 7:7\\]
Channel is trying to send data"]
pub type ChannelOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_BUSY` reader - 24:24\\]
The fifo has data"]
pub type FifoBusyR = crate::BitReader;
#[doc = "Field `FIFO_BUSY` writer - 24:24\\]
The fifo has data"]
pub type FifoBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_PEND` reader - 25:25\\]
The FIFO has enough data for a burst"]
pub type FifoPendR = crate::BitReader;
#[doc = "Field `FIFO_PEND` writer - 25:25\\]
The FIFO has enough data for a burst"]
pub type FifoPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_REQS` reader - 31:31\\]
The channel is sending a schedule request"]
pub type RxReqsR = crate::BitReader;
#[doc = "Field `RX_REQS` writer - 31:31\\]
The channel is sending a schedule request"]
pub type RxReqsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
The channel is in a packet"]
    #[inline(always)]
    pub fn in_packet_array(&self) -> InPacketArrayR {
        InPacketArrayR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel has active transactions"]
    #[inline(always)]
    pub fn channel_busy(&self) -> ChannelBusyR {
        ChannelBusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Channel is trying to send data"]
    #[inline(always)]
    pub fn channel_ok(&self) -> ChannelOkR {
        ChannelOkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
The fifo has data"]
    #[inline(always)]
    pub fn fifo_busy(&self) -> FifoBusyR {
        FifoBusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
The FIFO has enough data for a burst"]
    #[inline(always)]
    pub fn fifo_pend(&self) -> FifoPendR {
        FifoPendR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    pub fn rx_reqs(&self) -> RxReqsR {
        RxReqsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
The channel is in a packet"]
    #[inline(always)]
    #[must_use]
    pub fn in_packet_array(&mut self) -> InPacketArrayW<BcdmaRchanrtRrtStatus1Spec> {
        InPacketArrayW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Channel has active transactions"]
    #[inline(always)]
    #[must_use]
    pub fn channel_busy(&mut self) -> ChannelBusyW<BcdmaRchanrtRrtStatus1Spec> {
        ChannelBusyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Channel is trying to send data"]
    #[inline(always)]
    #[must_use]
    pub fn channel_ok(&mut self) -> ChannelOkW<BcdmaRchanrtRrtStatus1Spec> {
        ChannelOkW::new(self, 7)
    }
    #[doc = "Bit 24 - 24:24\\]
The fifo has data"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_busy(&mut self) -> FifoBusyW<BcdmaRchanrtRrtStatus1Spec> {
        FifoBusyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
The FIFO has enough data for a burst"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_pend(&mut self) -> FifoPendW<BcdmaRchanrtRrtStatus1Spec> {
        FifoPendW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reqs(&mut self) -> RxReqsW<BcdmaRchanrtRrtStatus1Spec> {
        RxReqsW::new(self, 31)
    }
}
#[doc = "The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanrtRrtStatus1Spec;
impl crate::RegisterSpec for BcdmaRchanrtRrtStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchanrt_rrt_status1::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanrtRrtStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchanrt_rrt_status1::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanrtRrtStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHANRT_RRT_STATUS1 to value 0"]
impl crate::Resettable for BcdmaRchanrtRrtStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
