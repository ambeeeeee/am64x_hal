#[doc = "Register `PKTDMA_RCHANRT_RRT_DCNT` reader"]
pub type R = crate::R<PktdmaRchanrtRrtDcntSpec>;
#[doc = "Register `PKTDMA_RCHANRT_RRT_DCNT` writer"]
pub type W = crate::W<PktdmaRchanrtRrtDcntSpec>;
#[doc = "Field `DCNT` reader - 31:0\\]
Current dropped packet count for the channel."]
pub type DcntR = crate::FieldReader<u32>;
#[doc = "Field `DCNT` writer - 31:0\\]
Current dropped packet count for the channel."]
pub type DcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current dropped packet count for the channel."]
    #[inline(always)]
    pub fn dcnt(&self) -> DcntR {
        DcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current dropped packet count for the channel."]
    #[inline(always)]
    #[must_use]
    pub fn dcnt(&mut self) -> DcntW<PktdmaRchanrtRrtDcntSpec> {
        DcntW::new(self, 0)
    }
}
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchanrt_rrt_dcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchanrt_rrt_dcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRchanrtRrtDcntSpec;
impl crate::RegisterSpec for PktdmaRchanrtRrtDcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_rchanrt_rrt_dcnt::R`](R) reader structure"]
impl crate::Readable for PktdmaRchanrtRrtDcntSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_rchanrt_rrt_dcnt::W`](W) writer structure"]
impl crate::Writable for PktdmaRchanrtRrtDcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RCHANRT_RRT_DCNT to value 0"]
impl crate::Resettable for PktdmaRchanrtRrtDcntSpec {
    const RESET_VALUE: u32 = 0;
}
