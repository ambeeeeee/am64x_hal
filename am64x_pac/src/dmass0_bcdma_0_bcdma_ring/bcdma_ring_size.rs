#[doc = "Register `BCDMA_RING_SIZE` reader"]
pub type R = crate::R<BcdmaRingSizeSpec>;
#[doc = "Register `BCDMA_RING_SIZE` writer"]
pub type W = crate::W<BcdmaRingSizeSpec>;
#[doc = "Field `SIZE` reader - 15:0\\]
Tx Ring element count. This field configures the size of the ring in elements."]
pub type SizeR = crate::FieldReader<u16>;
#[doc = "Field `SIZE` writer - 15:0\\]
Tx Ring element count. This field configures the size of the ring in elements."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RING_ELSIZE` reader - "]
pub type RingElsizeR = crate::FieldReader;
#[doc = "Field `RING_ELSIZE` writer - "]
pub type RingElsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `QMODE` reader - 31:29\\]
Defines the mode for this ring or queue."]
pub type QmodeR = crate::FieldReader;
#[doc = "Field `QMODE` writer - 31:29\\]
Defines the mode for this ring or queue."]
pub type QmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Tx Ring element count. This field configures the size of the ring in elements."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn ring_elsize(&self) -> RingElsizeR {
        RingElsizeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Defines the mode for this ring or queue."]
    #[inline(always)]
    pub fn qmode(&self) -> QmodeR {
        QmodeR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Tx Ring element count. This field configures the size of the ring in elements."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<BcdmaRingSizeSpec> {
        SizeW::new(self, 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn ring_elsize(&mut self) -> RingElsizeW<BcdmaRingSizeSpec> {
        RingElsizeW::new(self, 24)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Defines the mode for this ring or queue."]
    #[inline(always)]
    #[must_use]
    pub fn qmode(&mut self) -> QmodeW<BcdmaRingSizeSpec> {
        QmodeW::new(self, 29)
    }
}
#[doc = "The Ring Size Register contains the element count for the ring which is used to hand off pending work for the channel from the Host. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRingSizeSpec;
impl crate::RegisterSpec for BcdmaRingSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_ring_size::R`](R) reader structure"]
impl crate::Readable for BcdmaRingSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_ring_size::W`](W) writer structure"]
impl crate::Writable for BcdmaRingSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RING_SIZE to value 0x2100_0000"]
impl crate::Resettable for BcdmaRingSizeSpec {
    const RESET_VALUE: u32 = 0x2100_0000;
}
