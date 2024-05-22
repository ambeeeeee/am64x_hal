#[doc = "Register `ERR_REGS_err_intr_enabled_stat` reader"]
pub type R = crate::R<ErrRegsErrIntrEnabledStatSpec>;
#[doc = "Register `ERR_REGS_err_intr_enabled_stat` writer"]
pub type W = crate::W<ErrRegsErrIntrEnabledStatSpec>;
#[doc = "Field `ENABLED_INTR` reader - 0:0\\]
Level Enabled Interrupt status"]
pub type EnabledIntrR = crate::BitReader;
#[doc = "Field `ENABLED_INTR` writer - 0:0\\]
Level Enabled Interrupt status"]
pub type EnabledIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Level Enabled Interrupt status"]
    #[inline(always)]
    pub fn enabled_intr(&self) -> EnabledIntrR {
        EnabledIntrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Level Enabled Interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn enabled_intr(&mut self) -> EnabledIntrW<ErrRegsErrIntrEnabledStatSpec> {
        EnabledIntrW::new(self, 0)
    }
}
#[doc = "Global Interrupt Enabled Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enabled_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enabled_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsErrIntrEnabledStatSpec;
impl crate::RegisterSpec for ErrRegsErrIntrEnabledStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_err_intr_enabled_stat::R`](R) reader structure"]
impl crate::Readable for ErrRegsErrIntrEnabledStatSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_err_intr_enabled_stat::W`](W) writer structure"]
impl crate::Writable for ErrRegsErrIntrEnabledStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_err_intr_enabled_stat to value 0"]
impl crate::Resettable for ErrRegsErrIntrEnabledStatSpec {
    const RESET_VALUE: u32 = 0;
}
