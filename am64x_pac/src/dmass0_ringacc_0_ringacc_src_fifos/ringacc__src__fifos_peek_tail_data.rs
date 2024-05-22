#[doc = "Register `RINGACC__SRC__FIFOS_PEEK_TAIL_DATA` reader"]
pub type R = crate::R<Ringacc_Src_FifosPeekTailDataSpec>;
#[doc = "Register `RINGACC__SRC__FIFOS_PEEK_TAIL_DATA` writer"]
pub type W = crate::W<Ringacc_Src_FifosPeekTailDataSpec>;
#[doc = "Field `DATA` reader - 31:0\\]
Block of ring head or tail element data. Reserved for rings in ring mode."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Block of ring head or tail element data. Reserved for rings in ring mode."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Block of ring head or tail element data. Reserved for rings in ring mode."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Block of ring head or tail element data. Reserved for rings in ring mode."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Ringacc_Src_FifosPeekTailDataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "The Ring Peek Tail Entry Data Registers contain the data which is to be read from the ring tail without removing the element. These registers are virtual and non-static (i.e. they are just address locations that are used to access the ring tail element for reads. Writes are ignored. The data is right justified.)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc__src__fifos_peek_tail_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc__src__fifos_peek_tail_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ringacc_Src_FifosPeekTailDataSpec;
impl crate::RegisterSpec for Ringacc_Src_FifosPeekTailDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc__src__fifos_peek_tail_data::R`](R) reader structure"]
impl crate::Readable for Ringacc_Src_FifosPeekTailDataSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc__src__fifos_peek_tail_data::W`](W) writer structure"]
impl crate::Writable for Ringacc_Src_FifosPeekTailDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC__SRC__FIFOS_PEEK_TAIL_DATA to value 0"]
impl crate::Resettable for Ringacc_Src_FifosPeekTailDataSpec {
    const RESET_VALUE: u32 = 0;
}
