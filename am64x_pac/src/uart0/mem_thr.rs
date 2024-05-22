#[doc = "Register `MEM_THR` reader"]
pub type R = crate::R<MemThrSpec>;
#[doc = "Register `MEM_THR` writer"]
pub type W = crate::W<MemThrSpec>;
#[doc = "Field `THR` reader - 7:0\\]
TRANSMIT HOLDING REGISTER"]
pub type ThrR = crate::FieldReader;
#[doc = "Field `THR` writer - 7:0\\]
TRANSMIT HOLDING REGISTER"]
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
TRANSMIT HOLDING REGISTER"]
    #[inline(always)]
    pub fn thr(&self) -> ThrR {
        ThrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
TRANSMIT HOLDING REGISTER"]
    #[inline(always)]
    #[must_use]
    pub fn thr(&mut self) -> ThrW<MemThrSpec> {
        ThrW::new(self, 0)
    }
}
#[doc = "The transmitter section consists of the transmit holding register (THR) and the transmit shift register. The transmit holding register is actually a 64-byte FIFO. The LH writes data to the THR. The data is placed into the transmit shift register where it is shifted out serially on the TX output. If the FIFO is disabled location zero of the FIFO is used to store the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_thr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_thr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemThrSpec;
impl crate::RegisterSpec for MemThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_thr::R`](R) reader structure"]
impl crate::Readable for MemThrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_thr::W`](W) writer structure"]
impl crate::Writable for MemThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_THR to value 0"]
impl crate::Resettable for MemThrSpec {
    const RESET_VALUE: u32 = 0;
}
