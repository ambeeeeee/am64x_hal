#[doc = "Register `MEM_MAR` reader"]
pub type R = crate::R<MemMarSpec>;
#[doc = "Register `MEM_MAR` writer"]
pub type W = crate::W<MemMarSpec>;
#[doc = "Field `ADDRESS` reader - 7:0\\]
Multidrop match address value"]
pub type AddressR = crate::FieldReader;
#[doc = "Field `ADDRESS` writer - 7:0\\]
Multidrop match address value"]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Multidrop match address value"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Multidrop match address value"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<MemMarSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Multidrop Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMarSpec;
impl crate::RegisterSpec for MemMarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mar::R`](R) reader structure"]
impl crate::Readable for MemMarSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_mar::W`](W) writer structure"]
impl crate::Writable for MemMarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MAR to value 0"]
impl crate::Resettable for MemMarSpec {
    const RESET_VALUE: u32 = 0;
}
