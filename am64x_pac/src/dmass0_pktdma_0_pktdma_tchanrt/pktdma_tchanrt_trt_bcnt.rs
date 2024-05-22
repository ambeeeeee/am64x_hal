#[doc = "Register `PKTDMA_TCHANRT_TRT_BCNT` reader"]
pub type R = crate::R<PktdmaTchanrtTrtBcntSpec>;
#[doc = "Register `PKTDMA_TCHANRT_TRT_BCNT` writer"]
pub type W = crate::W<PktdmaTchanrtTrtBcntSpec>;
#[doc = "Field `BCNT` reader - 31:0\\]
Current completed payload byte count for the channel."]
pub type BcntR = crate::FieldReader<u32>;
#[doc = "Field `BCNT` writer - 31:0\\]
Current completed payload byte count for the channel."]
pub type BcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current completed payload byte count for the channel."]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current completed payload byte count for the channel."]
    #[inline(always)]
    #[must_use]
    pub fn bcnt(&mut self) -> BcntW<PktdmaTchanrtTrtBcntSpec> {
        BcntW::new(self, 0)
    }
}
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_bcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_bcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaTchanrtTrtBcntSpec;
impl crate::RegisterSpec for PktdmaTchanrtTrtBcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_tchanrt_trt_bcnt::R`](R) reader structure"]
impl crate::Readable for PktdmaTchanrtTrtBcntSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_tchanrt_trt_bcnt::W`](W) writer structure"]
impl crate::Writable for PktdmaTchanrtTrtBcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_TCHANRT_TRT_BCNT to value 0"]
impl crate::Resettable for PktdmaTchanrtTrtBcntSpec {
    const RESET_VALUE: u32 = 0;
}
