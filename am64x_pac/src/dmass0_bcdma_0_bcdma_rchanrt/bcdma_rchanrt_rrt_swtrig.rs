#[doc = "Register `BCDMA_RCHANRT_RRT_SWTRIG` reader"]
pub type R = crate::R<BcdmaRchanrtRrtSwtrigSpec>;
#[doc = "Register `BCDMA_RCHANRT_RRT_SWTRIG` writer"]
pub type W = crate::W<BcdmaRchanrtRrtSwtrigSpec>;
#[doc = "Field `TRIGGER` reader - 0:0\\]
Trigger: writing this bit with a value of 1 will cause the trigger event to be sent to this channel"]
pub type TriggerR = crate::BitReader;
#[doc = "Field `TRIGGER` writer - 0:0\\]
Trigger: writing this bit with a value of 1 will cause the trigger event to be sent to this channel"]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trigger: writing this bit with a value of 1 will cause the trigger event to be sent to this channel"]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trigger: writing this bit with a value of 1 will cause the trigger event to be sent to this channel"]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<BcdmaRchanrtRrtSwtrigSpec> {
        TriggerW::new(self, 0)
    }
}
#[doc = "The Software Trigger Register provides a mechanism by which software can directly trigger the channel in a secure way. This register is only used when the tx_chan_type is configured as a Third Party DMA channel. This register has no function when the channel is configured for packet mode transfers. A write to this register will cause an event to be sent to this channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_swtrig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_swtrig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanrtRrtSwtrigSpec;
impl crate::RegisterSpec for BcdmaRchanrtRrtSwtrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchanrt_rrt_swtrig::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanrtRrtSwtrigSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchanrt_rrt_swtrig::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanrtRrtSwtrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHANRT_RRT_SWTRIG to value 0"]
impl crate::Resettable for BcdmaRchanrtRrtSwtrigSpec {
    const RESET_VALUE: u32 = 0;
}
