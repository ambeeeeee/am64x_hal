#[doc = "Register `MEM_MBR` reader"]
pub type R = crate::R<MemMbrSpec>;
#[doc = "Register `MEM_MBR` writer"]
pub type W = crate::W<MemMbrSpec>;
#[doc = "Field `BROADCAST_ADDRESS` reader - 7:0\\]
Broadcast address for address matching"]
pub type BroadcastAddressR = crate::FieldReader;
#[doc = "Field `BROADCAST_ADDRESS` writer - 7:0\\]
Broadcast address for address matching"]
pub type BroadcastAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Broadcast address for address matching"]
    #[inline(always)]
    pub fn broadcast_address(&self) -> BroadcastAddressR {
        BroadcastAddressR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Broadcast address for address matching"]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_address(&mut self) -> BroadcastAddressW<MemMbrSpec> {
        BroadcastAddressW::new(self, 0)
    }
}
#[doc = "Multidrop Broadcast Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMbrSpec;
impl crate::RegisterSpec for MemMbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mbr::R`](R) reader structure"]
impl crate::Readable for MemMbrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_mbr::W`](W) writer structure"]
impl crate::Writable for MemMbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MBR to value 0"]
impl crate::Resettable for MemMbrSpec {
    const RESET_VALUE: u32 = 0;
}
