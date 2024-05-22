#[doc = "Register `PKTDMA_RINGRT_RT_RDB` reader"]
pub type R = crate::R<PktdmaRingrtRtRdbSpec>;
#[doc = "Register `PKTDMA_RINGRT_RT_RDB` writer"]
pub type W = crate::W<PktdmaRingrtRtRdbSpec>;
#[doc = "Field `ENTRY_CNT` reader - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
pub type EntryCntR = crate::FieldReader;
#[doc = "Field `ENTRY_CNT` writer - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
pub type EntryCntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDOWN_ACK` reader - 31:31\\]
This bit is set to 1 to ackowledge (and clear) the tdown_complete bit in the corresponding Ring N Occupancy Register. this bit is only valid on the reverse rings (rings consumed by the Host SW)"]
pub type TdownAckR = crate::BitReader;
#[doc = "Field `TDOWN_ACK` writer - 31:31\\]
This bit is set to 1 to ackowledge (and clear) the tdown_complete bit in the corresponding Ring N Occupancy Register. this bit is only valid on the reverse rings (rings consumed by the Host SW)"]
pub type TdownAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
    #[inline(always)]
    pub fn entry_cnt(&self) -> EntryCntR {
        EntryCntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set to 1 to ackowledge (and clear) the tdown_complete bit in the corresponding Ring N Occupancy Register. this bit is only valid on the reverse rings (rings consumed by the Host SW)"]
    #[inline(always)]
    pub fn tdown_ack(&self) -> TdownAckR {
        TdownAckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed number of entries by which to increment the ring occupancy. For normal Tx Ring operation, this value should be a positive number. This occ value for the ring is increased by this value each time the doorbell register is written (occ absolute value will increase or decrease based on the sign of the tentry_cnt)."]
    #[inline(always)]
    #[must_use]
    pub fn entry_cnt(&mut self) -> EntryCntW<PktdmaRingrtRtRdbSpec> {
        EntryCntW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set to 1 to ackowledge (and clear) the tdown_complete bit in the corresponding Ring N Occupancy Register. this bit is only valid on the reverse rings (rings consumed by the Host SW)"]
    #[inline(always)]
    #[must_use]
    pub fn tdown_ack(&mut self) -> TdownAckW<PktdmaRingrtRtRdbSpec> {
        TdownAckW::new(self, 31)
    }
}
#[doc = "The Ring N Doorbell Register is written by software to increment or decrement the number of entries on a Ring. One or more entries as specified by the entry_cnt field can be added to a ring with a single write operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_ringrt_rt_rdb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_ringrt_rt_rdb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRingrtRtRdbSpec;
impl crate::RegisterSpec for PktdmaRingrtRtRdbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_ringrt_rt_rdb::R`](R) reader structure"]
impl crate::Readable for PktdmaRingrtRtRdbSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_ringrt_rt_rdb::W`](W) writer structure"]
impl crate::Writable for PktdmaRingrtRtRdbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RINGRT_RT_RDB to value 0"]
impl crate::Resettable for PktdmaRingrtRtRdbSpec {
    const RESET_VALUE: u32 = 0;
}
