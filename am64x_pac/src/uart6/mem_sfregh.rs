#[doc = "Register `MEM_SFREGH` reader"]
pub type R = crate::R<MemSfreghSpec>;
#[doc = "Register `MEM_SFREGH` writer"]
pub type W = crate::W<MemSfreghSpec>;
#[doc = "Field `SFREGH` reader - 3:0\\]
MSB part of the frame length"]
pub type SfreghR = crate::FieldReader;
#[doc = "Field `SFREGH` writer - 3:0\\]
MSB part of the frame length"]
pub type SfreghW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MSB part of the frame length"]
    #[inline(always)]
    pub fn sfregh(&self) -> SfreghR {
        SfreghR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MSB part of the frame length"]
    #[inline(always)]
    #[must_use]
    pub fn sfregh(&mut self) -> SfreghW<MemSfreghSpec> {
        SfreghW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sfregh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sfregh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSfreghSpec;
impl crate::RegisterSpec for MemSfreghSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sfregh::R`](R) reader structure"]
impl crate::Readable for MemSfreghSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sfregh::W`](W) writer structure"]
impl crate::Writable for MemSfreghSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SFREGH to value 0"]
impl crate::Resettable for MemSfreghSpec {
    const RESET_VALUE: u32 = 0;
}
