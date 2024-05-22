#[doc = "Register `PR1_IEP0__SLV__REGS_count_reset_val_reg0` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsCountResetValReg0Spec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_count_reset_val_reg0` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsCountResetValReg0Spec>;
#[doc = "Field `RESET_VAL_0` reader - "]
pub type ResetVal0R = crate::FieldReader<u32>;
#[doc = "Field `RESET_VAL_0` writer - "]
pub type ResetVal0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reset_val_0(&self) -> ResetVal0R {
        ResetVal0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn reset_val_0(&mut self) -> ResetVal0W<Pr1Iep0_Slv_RegsCountResetValReg0Spec> {
        ResetVal0W::new(self, 0)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_count_reset_val_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_count_reset_val_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_count_reset_val_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsCountResetValReg0Spec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsCountResetValReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_count_reset_val_reg0::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsCountResetValReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_count_reset_val_reg0::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsCountResetValReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_count_reset_val_reg0 to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsCountResetValReg0Spec {
    const RESET_VALUE: u32 = 0;
}
