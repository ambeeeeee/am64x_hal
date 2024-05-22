#[doc = "Register `ERR_REGS_err_intr_enable_clr` reader"]
pub type R = crate::R<ErrRegsErrIntrEnableClrSpec>;
#[doc = "Register `ERR_REGS_err_intr_enable_clr` writer"]
pub type W = crate::W<ErrRegsErrIntrEnableClrSpec>;
#[doc = "Field `INTR_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register"]
pub type IntrEnableClrR = crate::BitReader;
#[doc = "Field `INTR_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register"]
pub type IntrEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register"]
    #[inline(always)]
    pub fn intr_enable_clr(&self) -> IntrEnableClrR {
        IntrEnableClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register"]
    #[inline(always)]
    #[must_use]
    pub fn intr_enable_clr(&mut self) -> IntrEnableClrW<ErrRegsErrIntrEnableClrSpec> {
        IntrEnableClrW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_regs_err_intr_enable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err_regs_err_intr_enable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRegsErrIntrEnableClrSpec;
impl crate::RegisterSpec for ErrRegsErrIntrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_regs_err_intr_enable_clr::R`](R) reader structure"]
impl crate::Readable for ErrRegsErrIntrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`err_regs_err_intr_enable_clr::W`](W) writer structure"]
impl crate::Writable for ErrRegsErrIntrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR_REGS_err_intr_enable_clr to value 0"]
impl crate::Resettable for ErrRegsErrIntrEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
