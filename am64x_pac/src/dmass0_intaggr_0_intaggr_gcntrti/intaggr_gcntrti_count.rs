#[doc = "Register `INTAGGR_GCNTRTI_count` reader"]
pub type R = crate::R<IntaggrGcntrtiCountSpec>;
#[doc = "Register `INTAGGR_GCNTRTI_count` writer"]
pub type W = crate::W<IntaggrGcntrtiCountSpec>;
#[doc = "Field `CCNT` reader - 31:0\\]
Current count. This field is incremented by the event count for each message received with this event on the Counted ETL Interface. On write, this field will be decremented by the value written. Writing a value greater than the current count is illegal."]
pub type CcntR = crate::FieldReader<u32>;
#[doc = "Field `CCNT` writer - 31:0\\]
Current count. This field is incremented by the event count for each message received with this event on the Counted ETL Interface. On write, this field will be decremented by the value written. Writing a value greater than the current count is illegal."]
pub type CcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current count. This field is incremented by the event count for each message received with this event on the Counted ETL Interface. On write, this field will be decremented by the value written. Writing a value greater than the current count is illegal."]
    #[inline(always)]
    pub fn ccnt(&self) -> CcntR {
        CcntR::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current count. This field is incremented by the event count for each message received with this event on the Counted ETL Interface. On write, this field will be decremented by the value written. Writing a value greater than the current count is illegal."]
    #[inline(always)]
    #[must_use]
    pub fn ccnt(&mut self) -> CcntW<IntaggrGcntrtiCountSpec> {
        CcntW::new(self, 0)
    }
}
#[doc = "The ETL Count register is read by software to determine how many times the event message has been received. This register can be written to decrement the count by a specified amount to acknowledge that a count has been processed by the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intaggr_gcntrti_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intaggr_gcntrti_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntaggrGcntrtiCountSpec;
impl crate::RegisterSpec for IntaggrGcntrtiCountSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`intaggr_gcntrti_count::R`](R) reader structure"]
impl crate::Readable for IntaggrGcntrtiCountSpec {}
#[doc = "`write(|w| ..)` method takes [`intaggr_gcntrti_count::W`](W) writer structure"]
impl crate::Writable for IntaggrGcntrtiCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets INTAGGR_GCNTRTI_count to value 0"]
impl crate::Resettable for IntaggrGcntrtiCountSpec {
    const RESET_VALUE: u64 = 0;
}
