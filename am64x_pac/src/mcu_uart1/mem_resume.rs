#[doc = "Register `MEM_RESUME` reader"]
pub type R = crate::R<MemResumeSpec>;
#[doc = "Register `MEM_RESUME` writer"]
pub type W = crate::W<MemResumeSpec>;
#[doc = "Field `RESUME` reader - 7:0\\]
Dummy read to restart the TX or RX"]
pub type ResumeR = crate::FieldReader;
#[doc = "Field `RESUME` writer - 7:0\\]
Dummy read to restart the TX or RX"]
pub type ResumeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Dummy read to restart the TX or RX"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Dummy read to restart the TX or RX"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<MemResumeSpec> {
        ResumeW::new(self, 0)
    }
}
#[doc = "IR-IrDA and IR-CIR modes only. This register is used to clear internal flags, which halt transmission/reception when an underrun/overrun error occurs. Reading this register resumes the halted operation. This register does not physically exist and reads always as 0x00.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_resume::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_resume::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemResumeSpec;
impl crate::RegisterSpec for MemResumeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_resume::R`](R) reader structure"]
impl crate::Readable for MemResumeSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_resume::W`](W) writer structure"]
impl crate::Writable for MemResumeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_RESUME to value 0"]
impl crate::Resettable for MemResumeSpec {
    const RESET_VALUE: u32 = 0;
}
