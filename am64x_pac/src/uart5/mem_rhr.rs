#[doc = "Register `MEM_RHR` reader"]
pub type R = crate::R<MemRhrSpec>;
#[doc = "Register `MEM_RHR` writer"]
pub type W = crate::W<MemRhrSpec>;
#[doc = "Field `RHR` reader - 7:0\\]
Receive holding register"]
pub type RhrR = crate::FieldReader;
#[doc = "Field `RHR` writer - 7:0\\]
Receive holding register"]
pub type RhrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Receive holding register"]
    #[inline(always)]
    pub fn rhr(&self) -> RhrR {
        RhrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Receive holding register"]
    #[inline(always)]
    #[must_use]
    pub fn rhr(&mut self) -> RhrW<MemRhrSpec> {
        RhrW::new(self, 0)
    }
}
#[doc = "The receiver section consists of the receiver holding register (RHR) and the receiver shift register. The RHR is actually a 64-byte FIFO. The receiver shift register receives serial data from RX input. The data is converted to parallel data and moved to the RHR. If the FIFO is disabled location zero of the FIFO is used to store the single data character. Note: If an overflow occurs the data in the RHR is not overwritten.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_rhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_rhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemRhrSpec;
impl crate::RegisterSpec for MemRhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_rhr::R`](R) reader structure"]
impl crate::Readable for MemRhrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_rhr::W`](W) writer structure"]
impl crate::Writable for MemRhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_RHR to value 0"]
impl crate::Resettable for MemRhrSpec {
    const RESET_VALUE: u32 = 0;
}
