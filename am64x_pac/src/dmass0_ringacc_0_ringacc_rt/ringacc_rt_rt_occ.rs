#[doc = "Register `RINGACC_RT_RT_OCC` reader"]
pub type R = crate::R<RingaccRtRtOccSpec>;
#[doc = "Register `RINGACC_RT_RT_OCC` writer"]
pub type W = crate::W<RingaccRtRtOccSpec>;
#[doc = "Field `OCC` reader - 20:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccR = crate::FieldReader<u32>;
#[doc = "Field `OCC` writer - 20:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    pub fn occ(&self) -> OccR {
        OccR::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OccW<RingaccRtRtOccSpec> {
        OccW::new(self, 0)
    }
}
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_occ::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_occ::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccRtRtOccSpec;
impl crate::RegisterSpec for RingaccRtRtOccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_rt_rt_occ::R`](R) reader structure"]
impl crate::Readable for RingaccRtRtOccSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_rt_rt_occ::W`](W) writer structure"]
impl crate::Writable for RingaccRtRtOccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_RT_RT_OCC to value 0"]
impl crate::Resettable for RingaccRtRtOccSpec {
    const RESET_VALUE: u32 = 0;
}
