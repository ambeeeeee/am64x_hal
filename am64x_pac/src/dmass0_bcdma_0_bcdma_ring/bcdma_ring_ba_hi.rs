#[doc = "Register `BCDMA_RING_BA_HI` reader"]
pub type R = crate::R<BcdmaRingBaHiSpec>;
#[doc = "Register `BCDMA_RING_BA_HI` writer"]
pub type W = crate::W<BcdmaRingBaHiSpec>;
#[doc = "Field `ADDR_HI` reader - 3:0\\]
Ring base address (MSBs)"]
pub type AddrHiR = crate::FieldReader;
#[doc = "Field `ADDR_HI` writer - 3:0\\]
Ring base address (MSBs)"]
pub type AddrHiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASEL` reader - 19:16\\]
Ring base address select"]
pub type AselR = crate::FieldReader;
#[doc = "Field `ASEL` writer - 19:16\\]
Ring base address select"]
pub type AselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Ring base address (MSBs)"]
    #[inline(always)]
    pub fn addr_hi(&self) -> AddrHiR {
        AddrHiR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Ring base address select"]
    #[inline(always)]
    pub fn asel(&self) -> AselR {
        AselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Ring base address (MSBs)"]
    #[inline(always)]
    #[must_use]
    pub fn addr_hi(&mut self) -> AddrHiW<BcdmaRingBaHiSpec> {
        AddrHiW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Ring base address select"]
    #[inline(always)]
    #[must_use]
    pub fn asel(&mut self) -> AselW<BcdmaRingBaHiSpec> {
        AselW::new(self, 16)
    }
}
#[doc = "The Ring Base Address Hi Register contains the 16 MSBs of the base address for the ring which is used to hand off pending work for the channel from the Host. The base address must be aligned to 0x8. A write to this register will reset the associated ring to clear the occupancies and reset the pointers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_ring_ba_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_ring_ba_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRingBaHiSpec;
impl crate::RegisterSpec for BcdmaRingBaHiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_ring_ba_hi::R`](R) reader structure"]
impl crate::Readable for BcdmaRingBaHiSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_ring_ba_hi::W`](W) writer structure"]
impl crate::Writable for BcdmaRingBaHiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RING_BA_HI to value 0"]
impl crate::Resettable for BcdmaRingBaHiSpec {
    const RESET_VALUE: u32 = 0;
}
