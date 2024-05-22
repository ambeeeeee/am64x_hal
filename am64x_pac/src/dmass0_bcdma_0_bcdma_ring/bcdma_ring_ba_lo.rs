#[doc = "Register `BCDMA_RING_BA_LO` reader"]
pub type R = crate::R<BcdmaRingBaLoSpec>;
#[doc = "Register `BCDMA_RING_BA_LO` writer"]
pub type W = crate::W<BcdmaRingBaLoSpec>;
#[doc = "Field `ADDR_LO` reader - 31:0\\]
Ring base address (LSBs)"]
pub type AddrLoR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_LO` writer - 31:0\\]
Ring base address (LSBs)"]
pub type AddrLoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Ring base address (LSBs)"]
    #[inline(always)]
    pub fn addr_lo(&self) -> AddrLoR {
        AddrLoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Ring base address (LSBs)"]
    #[inline(always)]
    #[must_use]
    pub fn addr_lo(&mut self) -> AddrLoW<BcdmaRingBaLoSpec> {
        AddrLoW::new(self, 0)
    }
}
#[doc = "The Ring Base Address Lo Register contains the 32 LSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_ba_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_ba_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRingBaLoSpec;
impl crate::RegisterSpec for BcdmaRingBaLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_ring_ba_lo::R`](R) reader structure"]
impl crate::Readable for BcdmaRingBaLoSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_ring_ba_lo::W`](W) writer structure"]
impl crate::Writable for BcdmaRingBaLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RING_BA_LO to value 0"]
impl crate::Resettable for BcdmaRingBaLoSpec {
    const RESET_VALUE: u32 = 0;
}
