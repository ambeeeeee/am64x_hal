#[doc = "Register `MEM_BINTEN` reader"]
pub type R = crate::R<MemBintenSpec>;
#[doc = "Register `MEM_BINTEN` writer"]
pub type W = crate::W<MemBintenSpec>;
#[doc = "Field `EN` reader - 15:0\\]
Per bank interrupt enable 0 = disable, 1 = enable"]
pub type EnR = crate::FieldReader<u16>;
#[doc = "Field `EN` writer - 15:0\\]
Per bank interrupt enable 0 = disable, 1 = enable"]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Per bank interrupt enable 0 = disable, 1 = enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Per bank interrupt enable 0 = disable, 1 = enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<MemBintenSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Bit Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_binten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_binten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemBintenSpec;
impl crate::RegisterSpec for MemBintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_binten::R`](R) reader structure"]
impl crate::Readable for MemBintenSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_binten::W`](W) writer structure"]
impl crate::Writable for MemBintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_BINTEN to value 0"]
impl crate::Resettable for MemBintenSpec {
    const RESET_VALUE: u32 = 0;
}
