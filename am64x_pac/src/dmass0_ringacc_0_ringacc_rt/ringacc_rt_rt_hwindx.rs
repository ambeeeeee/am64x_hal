#[doc = "Register `RINGACC_RT_RT_HWINDX` reader"]
pub type R = crate::R<RingaccRtRtHwindxSpec>;
#[doc = "Register `RINGACC_RT_RT_HWINDX` writer"]
pub type W = crate::W<RingaccRtRtHwindxSpec>;
#[doc = "Field `INDX` reader - 19:0\\]
Current HW owned read index for the ring. This value is initialized to 0 when the ring is set up and will be incremented by HW each time HW processes a ring entry. When the index is incremented to a value equal to the size field in the Ring Size Register for the ring the index will be reset back to 0."]
pub type IndxR = crate::FieldReader<u32>;
#[doc = "Field `INDX` writer - 19:0\\]
Current HW owned read index for the ring. This value is initialized to 0 when the ring is set up and will be incremented by HW each time HW processes a ring entry. When the index is incremented to a value equal to the size field in the Ring Size Register for the ring the index will be reset back to 0."]
pub type IndxW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Current HW owned read index for the ring. This value is initialized to 0 when the ring is set up and will be incremented by HW each time HW processes a ring entry. When the index is incremented to a value equal to the size field in the Ring Size Register for the ring the index will be reset back to 0."]
    #[inline(always)]
    pub fn indx(&self) -> IndxR {
        IndxR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Current HW owned read index for the ring. This value is initialized to 0 when the ring is set up and will be incremented by HW each time HW processes a ring entry. When the index is incremented to a value equal to the size field in the Ring Size Register for the ring the index will be reset back to 0."]
    #[inline(always)]
    #[must_use]
    pub fn indx(&mut self) -> IndxW<RingaccRtRtHwindxSpec> {
        IndxW::new(self, 0)
    }
}
#[doc = "The Ring N Current Index Register can be read by software for debug purposes to determine the current HW read index for the Ring for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_hwindx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_hwindx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccRtRtHwindxSpec;
impl crate::RegisterSpec for RingaccRtRtHwindxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_rt_rt_hwindx::R`](R) reader structure"]
impl crate::Readable for RingaccRtRtHwindxSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_rt_rt_hwindx::W`](W) writer structure"]
impl crate::Writable for RingaccRtRtHwindxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_RT_RT_HWINDX to value 0"]
impl crate::Resettable for RingaccRtRtHwindxSpec {
    const RESET_VALUE: u32 = 0;
}
