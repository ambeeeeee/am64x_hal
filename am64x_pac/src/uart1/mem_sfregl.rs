#[doc = "Register `MEM_SFREGL` reader"]
pub type R = crate::R<MemSfreglSpec>;
#[doc = "Register `MEM_SFREGL` writer"]
pub type W = crate::W<MemSfreglSpec>;
#[doc = "Field `SFREGL` reader - 7:0\\]
LSB part of the frame length"]
pub type SfreglR = crate::FieldReader;
#[doc = "Field `SFREGL` writer - 7:0\\]
LSB part of the frame length"]
pub type SfreglW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LSB part of the frame length"]
    #[inline(always)]
    pub fn sfregl(&self) -> SfreglR {
        SfreglR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LSB part of the frame length"]
    #[inline(always)]
    #[must_use]
    pub fn sfregl(&mut self) -> SfreglW<MemSfreglSpec> {
        SfreglW::new(self, 0)
    }
}
#[doc = "IrDA modes only. The frame lengths of received frames are written into the status FIFO. This information can be read by reading the SFREGL and SFREGH registers (i.e. these registers do not physically exist). The least significant bits are read from SFREGL and the most significant bits are read from SFREGH. Reading these registers does not alter the status FIFO read pointer. These registers should be read before the pointer is incremented by reading the SFLSR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_sfregl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_sfregl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemSfreglSpec;
impl crate::RegisterSpec for MemSfreglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_sfregl::R`](R) reader structure"]
impl crate::Readable for MemSfreglSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_sfregl::W`](W) writer structure"]
impl crate::Writable for MemSfreglSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_SFREGL to value 0"]
impl crate::Resettable for MemSfreglSpec {
    const RESET_VALUE: u32 = 0;
}
