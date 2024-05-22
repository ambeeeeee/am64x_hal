#[doc = "Register `GLB_REGS_exception_logging_data1` reader"]
pub type R = crate::R<GlbRegsExceptionLoggingData1Spec>;
#[doc = "Register `GLB_REGS_exception_logging_data1` writer"]
pub type W = crate::W<GlbRegsExceptionLoggingData1Spec>;
#[doc = "Field `ADDR_H` reader - 15:0\\]
Address upper 16 bits."]
pub type AddrHR = crate::FieldReader<u16>;
#[doc = "Field `ADDR_H` writer - 15:0\\]
Address upper 16 bits."]
pub type AddrHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Address upper 16 bits."]
    #[inline(always)]
    pub fn addr_h(&self) -> AddrHR {
        AddrHR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Address upper 16 bits."]
    #[inline(always)]
    #[must_use]
    pub fn addr_h(&mut self) -> AddrHW<GlbRegsExceptionLoggingData1Spec> {
        AddrHW::new(self, 0)
    }
}
#[doc = "The Exception Logging Data 1 Register contains the second word of the data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_data1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbRegsExceptionLoggingData1Spec;
impl crate::RegisterSpec for GlbRegsExceptionLoggingData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_regs_exception_logging_data1::R`](R) reader structure"]
impl crate::Readable for GlbRegsExceptionLoggingData1Spec {}
#[doc = "`write(|w| ..)` method takes [`glb_regs_exception_logging_data1::W`](W) writer structure"]
impl crate::Writable for GlbRegsExceptionLoggingData1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_REGS_exception_logging_data1 to value 0"]
impl crate::Resettable for GlbRegsExceptionLoggingData1Spec {
    const RESET_VALUE: u32 = 0;
}
