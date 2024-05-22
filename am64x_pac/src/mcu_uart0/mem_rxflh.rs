#[doc = "Register `MEM_RXFLH` reader"]
pub type R = crate::R<MemRxflhSpec>;
#[doc = "Register `MEM_RXFLH` writer"]
pub type W = crate::W<MemRxflhSpec>;
#[doc = "Field `RXFLH` reader - 3:0\\]
MSB register used to specify the frame length in reception"]
pub type RxflhR = crate::FieldReader;
#[doc = "Field `RXFLH` writer - 3:0\\]
MSB register used to specify the frame length in reception"]
pub type RxflhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MSB register used to specify the frame length in reception"]
    #[inline(always)]
    pub fn rxflh(&self) -> RxflhR {
        RxflhR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MSB register used to specify the frame length in reception"]
    #[inline(always)]
    #[must_use]
    pub fn rxflh(&mut self) -> RxflhW<MemRxflhSpec> {
        RxflhW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rxflh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rxflh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemRxflhSpec;
impl crate::RegisterSpec for MemRxflhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rxflh::R`](R) reader structure"]
impl crate::Readable for MemRxflhSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_rxflh::W`](W) writer structure"]
impl crate::Writable for MemRxflhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_RXFLH to value 0"]
impl crate::Resettable for MemRxflhSpec {
    const RESET_VALUE: u32 = 0;
}
