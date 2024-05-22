#[doc = "Register `MEM_TIMEGUARD` reader"]
pub type R = crate::R<MemTimeguardSpec>;
#[doc = "Register `MEM_TIMEGUARD` writer"]
pub type W = crate::W<MemTimeguardSpec>;
#[doc = "Field `TIMEGUARD` reader - 7:0\\]
Specifies the amount of idle baud clocks \\[transmitter bit period\\]
to insert between transmitted bytes, useful when comunicating with slower devices"]
pub type TimeguardR = crate::FieldReader;
#[doc = "Field `TIMEGUARD` writer - 7:0\\]
Specifies the amount of idle baud clocks \\[transmitter bit period\\]
to insert between transmitted bytes, useful when comunicating with slower devices"]
pub type TimeguardW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Specifies the amount of idle baud clocks \\[transmitter bit period\\]
to insert between transmitted bytes, useful when comunicating with slower devices"]
    #[inline(always)]
    pub fn timeguard(&self) -> TimeguardR {
        TimeguardR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Specifies the amount of idle baud clocks \\[transmitter bit period\\]
to insert between transmitted bytes, useful when comunicating with slower devices"]
    #[inline(always)]
    #[must_use]
    pub fn timeguard(&mut self) -> TimeguardW<MemTimeguardSpec> {
        TimeguardW::new(self, 0)
    }
}
#[doc = "Timeguard\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_timeguard::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_timeguard::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTimeguardSpec;
impl crate::RegisterSpec for MemTimeguardSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_timeguard::R`](R) reader structure"]
impl crate::Readable for MemTimeguardSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_timeguard::W`](W) writer structure"]
impl crate::Writable for MemTimeguardSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_TIMEGUARD to value 0"]
impl crate::Resettable for MemTimeguardSpec {
    const RESET_VALUE: u32 = 0;
}
