#[doc = "Register `ERR_REGS_err_intr_enable_set` reader"]
pub type R = crate::R<ErrRegsErrIntrEnableSetSpec>;
#[doc = "Register `ERR_REGS_err_intr_enable_set` writer"]
pub type W = crate::W<ErrRegsErrIntrEnableSetSpec>;
#[doc = "Field `INTR_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register"]
pub type IntrEnableSetR = crate::BitReader;
#[doc = "Field `INTR_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register"]
pub type IntrEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register"]
    #[inline(always)]
    pub fn intr_enable_set(&self) -> IntrEnableSetR {
        IntrEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register"]
    #[inline(always)]
    #[must_use]
    pub fn intr_enable_set(&mut self) -> IntrEnableSetW<ErrRegsErrIntrEnableSetSpec> {
        IntrEnableSetW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsErrIntrEnableSetSpec;
impl crate::RegisterSpec for ErrRegsErrIntrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_err_intr_enable_set::R`](R) reader structure"]
impl crate::Readable for ErrRegsErrIntrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_err_intr_enable_set::W`](W) writer structure"]
impl crate::Writable for ErrRegsErrIntrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_err_intr_enable_set to value 0"]
impl crate::Resettable for ErrRegsErrIntrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
