#[doc = "Register `MEM_TIMEOUTL` reader"]
pub type R = crate::R<MemTimeoutlSpec>;
#[doc = "Register `MEM_TIMEOUTL` writer"]
pub type W = crate::W<MemTimeoutlSpec>;
#[doc = "Field `TIMEOUT_L` reader - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Lower byte of the 16 bit value\\]"]
pub type TimeoutLR = crate::FieldReader;
#[doc = "Field `TIMEOUT_L` writer - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Lower byte of the 16 bit value\\]"]
pub type TimeoutLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Lower byte of the 16 bit value\\]"]
    #[inline(always)]
    pub fn timeout_l(&self) -> TimeoutLR {
        TimeoutLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Custom timeout period in baud clocks, to override the internal value, when different from 0. \\[Lower byte of the 16 bit value\\]"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_l(&mut self) -> TimeoutLW<MemTimeoutlSpec> {
        TimeoutLW::new(self, 0)
    }
}
#[doc = "Timeout lower byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeoutl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeoutl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTimeoutlSpec;
impl crate::RegisterSpec for MemTimeoutlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_timeoutl::R`](R) reader structure"]
impl crate::Readable for MemTimeoutlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_timeoutl::W`](W) writer structure"]
impl crate::Writable for MemTimeoutlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TIMEOUTL to value 0"]
impl crate::Resettable for MemTimeoutlSpec {
    const RESET_VALUE: u32 = 0;
}
