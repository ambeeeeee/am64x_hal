#[doc = "Register `BCDMA_BCHANRT_TRT_STATUS3` reader"]
pub type R = crate::R<BcdmaBchanrtTrtStatus3Spec>;
#[doc = "Register `BCDMA_BCHANRT_TRT_STATUS3` writer"]
pub type W = crate::W<BcdmaBchanrtTrtStatus3Spec>;
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
    #[doc = "Bit 24 - 24:24\\]
The fifo has data"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_busy(&mut self) -> FifoBusyW<BcdmaBchanrtTrtStatus3Spec> {
        FifoBusyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
The FIFO has enough data for a burst"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_pend(&mut self) -> FifoPendW<BcdmaBchanrtTrtStatus3Spec> {
        FifoPendW::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
The channel is sending a schedule request"]
    #[inline(always)]
    #[must_use]
    pub fn rx_reqs(&mut self) -> RxReqsW<BcdmaBchanrtTrtStatus3Spec> {
        RxReqsW::new(self, 31)
    }
}
#[doc = "The Status Register provides a read only view of channel status bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchanrt_trt_status3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchanrt_trt_status3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaBchanrtTrtStatus3Spec;
impl crate::RegisterSpec for BcdmaBchanrtTrtStatus3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_bchanrt_trt_status3::R`](R) reader structure"]
impl crate::Readable for BcdmaBchanrtTrtStatus3Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_bchanrt_trt_status3::W`](W) writer structure"]
impl crate::Writable for BcdmaBchanrtTrtStatus3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_BCHANRT_TRT_STATUS3 to value 0"]
impl crate::Resettable for BcdmaBchanrtTrtStatus3Spec {
    const RESET_VALUE: u32 = 0;
}
