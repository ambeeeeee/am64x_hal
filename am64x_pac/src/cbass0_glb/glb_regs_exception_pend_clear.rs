#[doc = "Register `GLB_REGS_exception_pend_clear` reader"]
pub type R = crate::R<GlbRegsExceptionPendClearSpec>;
#[doc = "Register `GLB_REGS_exception_pend_clear` writer"]
pub type W = crate::W<GlbRegsExceptionPendClearSpec>;
#[doc = "Field `PEND_CLR` reader - 0:0\\]
Write a 1 to clear the exception pend signal."]
pub type PendClrR = crate::BitReader;
#[doc = "Field `PEND_CLR` writer - 0:0\\]
Write a 1 to clear the exception pend signal."]
pub type PendClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to clear the exception pend signal."]
    #[inline(always)]
    pub fn pend_clr(&self) -> PendClrR {
        PendClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write a 1 to clear the exception pend signal."]
    #[inline(always)]
    #[must_use]
    pub fn pend_clr(&mut self) -> PendClrW<GlbRegsExceptionPendClearSpec> {
        PendClrW::new(self, 0)
    }
}
#[doc = "The Exception Logging Pending Clear Register allows to clear the pend signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_pend_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_pend_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbRegsExceptionPendClearSpec;
impl crate::RegisterSpec for GlbRegsExceptionPendClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_regs_exception_pend_clear::R`](R) reader structure"]
impl crate::Readable for GlbRegsExceptionPendClearSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_regs_exception_pend_clear::W`](W) writer structure"]
impl crate::Writable for GlbRegsExceptionPendClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_REGS_exception_pend_clear to value 0"]
impl crate::Resettable for GlbRegsExceptionPendClearSpec {
    const RESET_VALUE: u32 = 0;
}
