#[doc = "Register `BCDMA_RCHANRT_RRT_PCNT` reader"]
pub type R = crate::R<BcdmaRchanrtRrtPcntSpec>;
#[doc = "Register `BCDMA_RCHANRT_RRT_PCNT` writer"]
pub type W = crate::W<BcdmaRchanrtRrtPcntSpec>;
#[doc = "Field `PCNT` reader - 31:0\\]
Current completed packet count for the channel."]
pub type PcntR = crate::FieldReader<u32>;
#[doc = "Field `PCNT` writer - 31:0\\]
Current completed packet count for the channel."]
pub type PcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current completed packet count for the channel."]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current completed packet count for the channel."]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PcntW<BcdmaRchanrtRrtPcntSpec> {
        PcntW::new(self, 0)
    }
}
#[doc = "The statistics registers are supplied to give software applications operational progress status for the channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_pcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_pcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanrtRrtPcntSpec;
impl crate::RegisterSpec for BcdmaRchanrtRrtPcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchanrt_rrt_pcnt::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanrtRrtPcntSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchanrt_rrt_pcnt::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanrtRrtPcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHANRT_RRT_PCNT to value 0"]
impl crate::Resettable for BcdmaRchanrtRrtPcntSpec {
    const RESET_VALUE: u32 = 0;
}
