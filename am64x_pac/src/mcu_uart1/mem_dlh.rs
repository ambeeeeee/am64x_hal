#[doc = "Register `MEM_DLH` reader"]
pub type R = crate::R<MemDlhSpec>;
#[doc = "Register `MEM_DLH` writer"]
pub type W = crate::W<MemDlhSpec>;
#[doc = "Field `CLOCK_MSB` reader - 7:0\\]
Used to store the 8-bit MSB divisor value"]
pub type ClockMsbR = crate::FieldReader;
#[doc = "Field `CLOCK_MSB` writer - 7:0\\]
Used to store the 8-bit MSB divisor value"]
pub type ClockMsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit MSB divisor value"]
    #[inline(always)]
    pub fn clock_msb(&self) -> ClockMsbR {
        ClockMsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Used to store the 8-bit MSB divisor value"]
    #[inline(always)]
    #[must_use]
    pub fn clock_msb(&mut self) -> ClockMsbW<MemDlhSpec> {
        ClockMsbW::new(self, 0)
    }
}
#[doc = "Divisor Latches High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_dlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_dlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemDlhSpec;
impl crate::RegisterSpec for MemDlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_dlh::R`](R) reader structure"]
impl crate::Readable for MemDlhSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_dlh::W`](W) writer structure"]
impl crate::Writable for MemDlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_DLH to value 0"]
impl crate::Resettable for MemDlhSpec {
    const RESET_VALUE: u32 = 0;
}
