#[doc = "Register `PKTDMA_RCHANRT_RRT_SBCNT` reader"]
pub type R = crate::R<PktdmaRchanrtRrtSbcntSpec>;
#[doc = "Register `PKTDMA_RCHANRT_RRT_SBCNT` writer"]
pub type W = crate::W<PktdmaRchanrtRrtSbcntSpec>;
#[doc = "Field `SBCNT` reader - 31:0\\]
Current started byte count for the channel."]
pub type SbcntR = crate::FieldReader<u32>;
#[doc = "Field `SBCNT` writer - 31:0\\]
Current started byte count for the channel."]
pub type SbcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current started byte count for the channel."]
    #[inline(always)]
    pub fn sbcnt(&self) -> SbcntR {
        SbcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current started byte count for the channel."]
    #[inline(always)]
    #[must_use]
    pub fn sbcnt(&mut self) -> SbcntW<PktdmaRchanrtRrtSbcntSpec> {
        SbcntW::new(self, 0)
    }
}
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_sbcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_sbcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRchanrtRrtSbcntSpec;
impl crate::RegisterSpec for PktdmaRchanrtRrtSbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_rchanrt_rrt_sbcnt::R`](R) reader structure"]
impl crate::Readable for PktdmaRchanrtRrtSbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_rchanrt_rrt_sbcnt::W`](W) writer structure"]
impl crate::Writable for PktdmaRchanrtRrtSbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RCHANRT_RRT_SBCNT to value 0"]
impl crate::Resettable for PktdmaRchanrtRrtSbcntSpec {
    const RESET_VALUE: u32 = 0;
}
