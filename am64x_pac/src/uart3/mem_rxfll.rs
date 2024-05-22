#[doc = "Register `MEM_RXFLL` reader"]
pub type R = crate::R<MemRxfllSpec>;
#[doc = "Register `MEM_RXFLL` writer"]
pub type W = crate::W<MemRxfllSpec>;
#[doc = "Field `RXFLL` reader - 7:0\\]
LSB register used to specify the frame length in reception"]
pub type RxfllR = crate::FieldReader;
#[doc = "Field `RXFLL` writer - 7:0\\]
LSB register used to specify the frame length in reception"]
pub type RxfllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LSB register used to specify the frame length in reception"]
    #[inline(always)]
    pub fn rxfll(&self) -> RxfllR {
        RxfllR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LSB register used to specify the frame length in reception"]
    #[inline(always)]
    #[must_use]
    pub fn rxfll(&mut self) -> RxfllW<MemRxfllSpec> {
        RxfllW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The registers RXFLL and RXFLH hold the 12-bit receive maximum frame length. RXFLL holds the least significant bits and RXFLH holds the most significant bits. If the intended maximum receive frame length is n bytes, then program RXFLL and RXFLH to be n + 3 in SIR or MIR modes and n + 6 in FIR mode (+3 and +6 are due to frame format with CRC and stop flag; there are two bytes associated with the FIR stop flag).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rxfll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rxfll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemRxfllSpec;
impl crate::RegisterSpec for MemRxfllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rxfll::R`](R) reader structure"]
impl crate::Readable for MemRxfllSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_rxfll::W`](W) writer structure"]
impl crate::Writable for MemRxfllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_RXFLL to value 0"]
impl crate::Resettable for MemRxfllSpec {
    const RESET_VALUE: u32 = 0;
}
