#[doc = "Register `MEM_TIMEOUTH` reader"]
pub type R = crate::R<MemTimeouthSpec>;
#[doc = "Register `MEM_TIMEOUTH` writer"]
pub type W = crate::W<MemTimeouthSpec>;
#[doc = "Field `TIMEOUT_H` reader - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Higher byte of the 16 bit value\\]"]
pub type TimeoutHR = crate::FieldReader;
#[doc = "Field `TIMEOUT_H` writer - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Higher byte of the 16 bit value\\]"]
pub type TimeoutHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Higher byte of the 16 bit value\\]"]
    #[inline(always)]
    pub fn timeout_h(&self) -> TimeoutHR {
        TimeoutHR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Higher byte of the 16 bit value\\]"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_h(&mut self) -> TimeoutHW<MemTimeouthSpec> {
        TimeoutHW::new(self, 0)
    }
}
#[doc = "Timeout higher byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeouth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeouth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTimeouthSpec;
impl crate::RegisterSpec for MemTimeouthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_timeouth::R`](R) reader structure"]
impl crate::Readable for MemTimeouthSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_timeouth::W`](W) writer structure"]
impl crate::Writable for MemTimeouthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TIMEOUTH to value 0"]
impl crate::Resettable for MemTimeouthSpec {
    const RESET_VALUE: u32 = 0;
}
