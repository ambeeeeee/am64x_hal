#[doc = "Register `REGS_SPLOCK_SYSCONFIG` reader"]
pub type R = crate::R<RegsSplockSysconfigSpec>;
#[doc = "Register `REGS_SPLOCK_SYSCONFIG` writer"]
pub type W = crate::W<RegsSplockSysconfigSpec>;
#[doc = "Field `SOFT_RESET` reader - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0 It has the same effect as the hardware reset Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and free all of the locks"]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0 It has the same effect as the hardware reset Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and free all of the locks"]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0 It has the same effect as the hardware reset Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and free all of the locks"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Module Software Reset The bit is automatically reset by the hardware. During reads, it always returns 0 It has the same effect as the hardware reset Writing a 0 has no effect. Writing a 1 will start a soft reset sequence and free all of the locks"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SoftResetW<RegsSplockSysconfigSpec> {
        SoftResetW::new(self, 1)
    }
}
#[doc = "Provides the SOFTRESET register for backwards compatibility with OMAP Spinlock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_splock_sysconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_splock_sysconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsSplockSysconfigSpec;
impl crate::RegisterSpec for RegsSplockSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_splock_sysconfig::R`](R) reader structure"]
impl crate::Readable for RegsSplockSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_splock_sysconfig::W`](W) writer structure"]
impl crate::Writable for RegsSplockSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_SPLOCK_SYSCONFIG to value 0"]
impl crate::Resettable for RegsSplockSysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
