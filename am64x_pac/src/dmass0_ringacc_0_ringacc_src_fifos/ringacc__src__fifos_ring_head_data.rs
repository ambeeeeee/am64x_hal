#[doc = "Register `RINGACC__SRC__FIFOS_RING_HEAD_DATA` reader"]
pub type R = crate::R<Ringacc_Src_FifosRingHeadDataSpec>;
#[doc = "Register `RINGACC__SRC__FIFOS_RING_HEAD_DATA` writer"]
pub type W = crate::W<Ringacc_Src_FifosRingHeadDataSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Block of ring head or tail element data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Block of ring head or tail element data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Block of ring head or tail element data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Block of ring head or tail element data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Ringacc_Src_FifosRingHeadDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "The Ring Head Entry Data Registers contain the data which is to be written or which was read from the ring head. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring head element for reads or writes. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_ring_head_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_ring_head_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringacc_Src_FifosRingHeadDataSpec;
impl crate::RegisterSpec for Ringacc_Src_FifosRingHeadDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc__src__fifos_ring_head_data::R`](R) reader structure"]
impl crate::Readable for Ringacc_Src_FifosRingHeadDataSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc__src__fifos_ring_head_data::W`](W) writer structure"]
impl crate::Writable for Ringacc_Src_FifosRingHeadDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC__SRC__FIFOS_RING_HEAD_DATA to value 0"]
impl crate::Resettable for Ringacc_Src_FifosRingHeadDataSpec {
    const RESET_VALUE: u32 = 0;
}
