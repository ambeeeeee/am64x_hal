#[doc = "Register `PKTDMA_RINGRT_RT_ROCC` reader"]
pub type R = crate::R<PktdmaRingrtRtRoccSpec>;
#[doc = "Register `PKTDMA_RINGRT_RT_ROCC` writer"]
pub type W = crate::W<PktdmaRingrtRtRoccSpec>;
#[doc = "Field `OCC` reader - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccR = crate::FieldReader<u32>;
#[doc = "Field `OCC` writer - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `TDOWN_COMPLETE` reader - 31:31\\]
This bit when set indicates that a teardown is complete on the channel. This bit is cleared anytime the tdown_ack bit is written as a 1 in the corresponding Ring N Doorbell Register. This bit is only valid on the reverse rings (rings consumed by the Host SW)."]
pub type TdownCompleteR = crate::BitReader;
#[doc = "Field `TDOWN_COMPLETE` writer - 31:31\\]
This bit when set indicates that a teardown is complete on the channel. This bit is cleared anytime the tdown_ack bit is written as a 1 in the corresponding Ring N Doorbell Register. This bit is only valid on the reverse rings (rings consumed by the Host SW)."]
pub type TdownCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    pub fn occ(&self) -> OccR {
        OccR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit when set indicates that a teardown is complete on the channel. This bit is cleared anytime the tdown_ack bit is written as a 1 in the corresponding Ring N Doorbell Register. This bit is only valid on the reverse rings (rings consumed by the Host SW)."]
    #[inline(always)]
    pub fn tdown_complete(&self) -> TdownCompleteR {
        TdownCompleteR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OccW<PktdmaRingrtRtRoccSpec> {
        OccW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit when set indicates that a teardown is complete on the channel. This bit is cleared anytime the tdown_ack bit is written as a 1 in the corresponding Ring N Doorbell Register. This bit is only valid on the reverse rings (rings consumed by the Host SW)."]
    #[inline(always)]
    #[must_use]
    pub fn tdown_complete(&mut self) -> TdownCompleteW<PktdmaRingrtRtRoccSpec> {
        TdownCompleteW::new(self, 31)
    }
}
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_rocc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_rocc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRingrtRtRoccSpec;
impl crate::RegisterSpec for PktdmaRingrtRtRoccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_ringrt_rt_rocc::R`](R) reader structure"]
impl crate::Readable for PktdmaRingrtRtRoccSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_ringrt_rt_rocc::W`](W) writer structure"]
impl crate::Writable for PktdmaRingrtRtRoccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RINGRT_RT_ROCC to value 0"]
impl crate::Resettable for PktdmaRingrtRtRoccSpec {
    const RESET_VALUE: u32 = 0;
}
