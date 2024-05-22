#[doc = "Register `RINGACC_RT_RT_DB` reader"]
pub type R = crate::R<RingaccRtRtDbSpec>;
#[doc = "Register `RINGACC_RT_RT_DB` writer"]
pub type W = crate::W<RingaccRtRtDbSpec>;
#[doc = "Field `ENTRY_CNT` reader - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
pub type EntryCntR = crate::FieldReader;
#[doc = "Field `ENTRY_CNT` writer - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
pub type EntryCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
    #[inline(always)]
    pub fn entry_cnt(&self) -> EntryCntR {
        EntryCntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
    #[inline(always)]
    #[must_use]
    pub fn entry_cnt(&mut self) -> EntryCntW<RingaccRtRtDbSpec> {
        EntryCntW::new(self, 0)
    }
}
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_rt_rt_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_rt_rt_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccRtRtDbSpec;
impl crate::RegisterSpec for RingaccRtRtDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_rt_rt_db::R`](R) reader structure"]
impl crate::Readable for RingaccRtRtDbSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_rt_rt_db::W`](W) writer structure"]
impl crate::Writable for RingaccRtRtDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_RT_RT_DB to value 0"]
impl crate::Resettable for RingaccRtRtDbSpec {
    const RESET_VALUE: u32 = 0;
}
