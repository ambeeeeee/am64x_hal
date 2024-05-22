#[doc = "Register `GLB_REGS_exception_logging_data3` reader"]
pub type R = crate::R<GlbRegsExceptionLoggingData3Spec>;
#[doc = "Register `GLB_REGS_exception_logging_data3` writer"]
pub type W = crate::W<GlbRegsExceptionLoggingData3Spec>;
#[doc = "Field `BYTECNT` reader - 9:0\\]
Byte count."]
pub type BytecntR = crate::FieldReader<u16>;
#[doc = "Field `BYTECNT` writer - 9:0\\]
Byte count."]
pub type BytecntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Byte count."]
    #[inline(always)]
    pub fn bytecnt(&self) -> BytecntR {
        BytecntR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Byte count."]
    #[inline(always)]
    #[must_use]
    pub fn bytecnt(&mut self) -> BytecntW<GlbRegsExceptionLoggingData3Spec> {
        BytecntW::new(self, 0)
    }
}
#[doc = "The Exception Logging Data 3 Register contains the fourth word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbRegsExceptionLoggingData3Spec;
impl crate::RegisterSpec for GlbRegsExceptionLoggingData3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_regs_exception_logging_data3::R`](R) reader structure"]
impl crate::Readable for GlbRegsExceptionLoggingData3Spec {}
#[doc = "`write(|w| ..)` method takes [`glb_regs_exception_logging_data3::W`](W) writer structure"]
impl crate::Writable for GlbRegsExceptionLoggingData3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_REGS_exception_logging_data3 to value 0"]
impl crate::Resettable for GlbRegsExceptionLoggingData3Spec {
    const RESET_VALUE: u32 = 0;
}
