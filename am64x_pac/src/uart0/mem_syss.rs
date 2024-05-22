#[doc = "Register `MEM_SYSS` reader"]
pub type R = crate::R<MemSyssSpec>;
#[doc = "Register `MEM_SYSS` writer"]
pub type W = crate::W<MemSyssSpec>;
#[doc = "Field `RESETDONE` reader - 0:0\\]
Internal Reset Monitoring"]
pub type ResetdoneR = crate::BitReader;
#[doc = "Field `RESETDONE` writer - 0:0\\]
Internal Reset Monitoring"]
pub type ResetdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset Monitoring"]
    #[inline(always)]
    pub fn resetdone(&self) -> ResetdoneR {
        ResetdoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal Reset Monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn resetdone(&mut self) -> ResetdoneW<MemSyssSpec> {
        ResetdoneW::new(self, 0)
    }
}
#[doc = "MEM_SYSS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_syss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_syss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSyssSpec;
impl crate::RegisterSpec for MemSyssSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_syss::R`](R) reader structure"]
impl crate::Readable for MemSyssSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_syss::W`](W) writer structure"]
impl crate::Writable for MemSyssSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SYSS to value 0"]
impl crate::Resettable for MemSyssSpec {
    const RESET_VALUE: u32 = 0;
}
