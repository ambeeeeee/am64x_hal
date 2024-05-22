#[doc = "Register `MEM_DLL` reader"]
pub type R = crate::R<MemDllSpec>;
#[doc = "Register `MEM_DLL` writer"]
pub type W = crate::W<MemDllSpec>;
#[doc = "Field `CLOCK_LSB` reader - 7:0\\]
Used to store the 8-bit LSB divisor value"]
pub type ClockLsbR = crate::FieldReader;
#[doc = "Field `CLOCK_LSB` writer - 7:0\\]
Used to store the 8-bit LSB divisor value"]
pub type ClockLsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit LSB divisor value"]
    #[inline(always)]
    pub fn clock_lsb(&self) -> ClockLsbR {
        ClockLsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit LSB divisor value"]
    #[inline(always)]
    #[must_use]
    pub fn clock_lsb(&mut self) -> ClockLsbW<MemDllSpec> {
        ClockLsbW::new(self, 0)
    }
}
#[doc = "Divisor Latches Low Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDllSpec;
impl crate::RegisterSpec for MemDllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dll::R`](R) reader structure"]
impl crate::Readable for MemDllSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_dll::W`](W) writer structure"]
impl crate::Writable for MemDllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DLL to value 0"]
impl crate::Resettable for MemDllSpec {
    const RESET_VALUE: u32 = 0;
}
