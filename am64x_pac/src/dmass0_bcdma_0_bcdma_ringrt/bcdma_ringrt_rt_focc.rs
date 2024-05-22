#[doc = "Register `BCDMA_RINGRT_RT_FOCC` reader"]
pub type R = crate::R<BcdmaRingrtRtFoccSpec>;
#[doc = "Register `BCDMA_RINGRT_RT_FOCC` writer"]
pub type W = crate::W<BcdmaRingrtRtFoccSpec>;
#[doc = "Field `OCC` reader - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccR = crate::FieldReader<u32>;
#[doc = "Field `OCC` writer - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
pub type OccW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    pub fn occ(&self) -> OccR {
        OccR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Total number of valid entries on the ring. This value is generally intended to be incremented by doorbell pokes from software and is decremented by the DMA engine as entries are completed."]
    #[inline(always)]
    #[must_use]
    pub fn occ(&mut self) -> OccW<BcdmaRingrtRtFoccSpec> {
        OccW::new(self, 0)
    }
}
#[doc = "The Ring N Occupancy Register can be read by software to determine the total number of valid entries on a ring. The contents of each of these registers are unary ORed in order to create a pending signal for the ring which can be used for triggering hardware operations and/or for generating interrupts to the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ringrt_rt_focc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ringrt_rt_focc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRingrtRtFoccSpec;
impl crate::RegisterSpec for BcdmaRingrtRtFoccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_ringrt_rt_focc::R`](R) reader structure"]
impl crate::Readable for BcdmaRingrtRtFoccSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_ringrt_rt_focc::W`](W) writer structure"]
impl crate::Writable for BcdmaRingrtRtFoccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RINGRT_RT_FOCC to value 0"]
impl crate::Resettable for BcdmaRingrtRtFoccSpec {
    const RESET_VALUE: u32 = 0;
}
