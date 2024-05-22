#[doc = "Register `BCDMA_TCHAN_TST_SCHED` reader"]
pub type R = crate::R<BcdmaTchanTstSchedSpec>;
#[doc = "Register `BCDMA_TCHAN_TST_SCHED` writer"]
pub type W = crate::W<BcdmaTchanTstSchedSpec>;
#[doc = "Field `PRIORITY` reader - 1:0\\]
Tx Scheduling Priority: These bits select which scheduling bin the channel will be placed in for bandwidth allocation of the Tx DMA units. This field is encoded as follows: 0 = High priority 1 = Medium - high priority 2 = Medium - low priority 3 = Low priority Arbitration between bins is performed in a strict priority fashion. High priority channels will always be serviced first. If no high priority channels are requesting then all medium-high priority channels will be serviced next. If no high priority or medium-high priority channels are requesting then all medium-low priority channels will be serviced next. When no other channels are requesting, the low priority channels will be serviced. All channels within a given bin are serviced in a round robin order. Only channels which are enabled and which have sufficient free space in their Per Channel FIFO will be included in the round robin arbitration."]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - 1:0\\]
Tx Scheduling Priority: These bits select which scheduling bin the channel will be placed in for bandwidth allocation of the Tx DMA units. This field is encoded as follows: 0 = High priority 1 = Medium - high priority 2 = Medium - low priority 3 = Low priority Arbitration between bins is performed in a strict priority fashion. High priority channels will always be serviced first. If no high priority channels are requesting then all medium-high priority channels will be serviced next. If no high priority or medium-high priority channels are requesting then all medium-low priority channels will be serviced next. When no other channels are requesting, the low priority channels will be serviced. All channels within a given bin are serviced in a round robin order. Only channels which are enabled and which have sufficient free space in their Per Channel FIFO will be included in the round robin arbitration."]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Tx Scheduling Priority: These bits select which scheduling bin the channel will be placed in for bandwidth allocation of the Tx DMA units. This field is encoded as follows: 0 = High priority 1 = Medium - high priority 2 = Medium - low priority 3 = Low priority Arbitration between bins is performed in a strict priority fashion. High priority channels will always be serviced first. If no high priority channels are requesting then all medium-high priority channels will be serviced next. If no high priority or medium-high priority channels are requesting then all medium-low priority channels will be serviced next. When no other channels are requesting, the low priority channels will be serviced. All channels within a given bin are serviced in a round robin order. Only channels which are enabled and which have sufficient free space in their Per Channel FIFO will be included in the round robin arbitration."]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Tx Scheduling Priority: These bits select which scheduling bin the channel will be placed in for bandwidth allocation of the Tx DMA units. This field is encoded as follows: 0 = High priority 1 = Medium - high priority 2 = Medium - low priority 3 = Low priority Arbitration between bins is performed in a strict priority fashion. High priority channels will always be serviced first. If no high priority channels are requesting then all medium-high priority channels will be serviced next. If no high priority or medium-high priority channels are requesting then all medium-low priority channels will be serviced next. When no other channels are requesting, the low priority channels will be serviced. All channels within a given bin are serviced in a round robin order. Only channels which are enabled and which have sufficient free space in their Per Channel FIFO will be included in the round robin arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn priority(&mut self) -> PriorityW<BcdmaTchanTstSchedSpec> {
        PriorityW::new(self, 0)
    }
}
#[doc = "The Tx Channel N Static Scheduler Configuration Register contains static configuration information which affects the conditions under which each channel will be given an opportunity to use the Tx DMA unit(s). The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tst_sched::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tst_sched::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanTstSchedSpec;
impl crate::RegisterSpec for BcdmaTchanTstSchedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchan_tst_sched::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanTstSchedSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchan_tst_sched::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanTstSchedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHAN_TST_SCHED to value 0"]
impl crate::Resettable for BcdmaTchanTstSchedSpec {
    const RESET_VALUE: u32 = 0;
}
