#[doc = "Register `GLB_REGS_exception_logging_control` reader"]
pub type R = crate::R<GlbRegsExceptionLoggingControlSpec>;
#[doc = "Register `GLB_REGS_exception_logging_control` writer"]
pub type W = crate::W<GlbRegsExceptionLoggingControlSpec>;
#[doc = "Field `DISABLE_F` reader - 0:0\\]
Disables logging when set."]
pub type DisableFR = crate::BitReader;
#[doc = "Field `DISABLE_F` writer - 0:0\\]
Disables logging when set."]
pub type DisableFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_PEND` reader - 1:1\\]
Disables logging pending when set."]
pub type DisablePendR = crate::BitReader;
#[doc = "Field `DISABLE_PEND` writer - 1:1\\]
Disables logging pending when set."]
pub type DisablePendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disables logging when set."]
    #[inline(always)]
    pub fn disable_f(&self) -> DisableFR {
        DisableFR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables logging pending when set."]
    #[inline(always)]
    pub fn disable_pend(&self) -> DisablePendR {
        DisablePendR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disables logging when set."]
    #[inline(always)]
    #[must_use]
    pub fn disable_f(&mut self) -> DisableFW<GlbRegsExceptionLoggingControlSpec> {
        DisableFW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables logging pending when set."]
    #[inline(always)]
    #[must_use]
    pub fn disable_pend(&mut self) -> DisablePendW<GlbRegsExceptionLoggingControlSpec> {
        DisablePendW::new(self, 1)
    }
}
#[doc = "The Exception Logging Control Register controls the exception logging.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_regs_exception_logging_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_regs_exception_logging_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GlbRegsExceptionLoggingControlSpec;
impl crate::RegisterSpec for GlbRegsExceptionLoggingControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_regs_exception_logging_control::R`](R) reader structure"]
impl crate::Readable for GlbRegsExceptionLoggingControlSpec {}
#[doc = "`write(|w| ..)` method takes [`glb_regs_exception_logging_control::W`](W) writer structure"]
impl crate::Writable for GlbRegsExceptionLoggingControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_REGS_exception_logging_control to value 0"]
impl crate::Resettable for GlbRegsExceptionLoggingControlSpec {
    const RESET_VALUE: u32 = 0;
}
