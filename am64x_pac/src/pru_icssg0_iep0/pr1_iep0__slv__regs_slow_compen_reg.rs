#[doc = "Register `PR1_IEP0__SLV__REGS_slow_compen_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsSlowCompenRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_slow_compen_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsSlowCompenRegSpec>;
#[doc = "Field `SLOW_COMPEN_CNT` reader - "]
pub type SlowCompenCntR = crate::FieldReader<u32>;
#[doc = "Field `SLOW_COMPEN_CNT` writer - "]
pub type SlowCompenCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slow_compen_cnt(&self) -> SlowCompenCntR {
        SlowCompenCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn slow_compen_cnt(&mut self) -> SlowCompenCntW<Pr1Iep0_Slv_RegsSlowCompenRegSpec> {
        SlowCompenCntW::new(self, 0)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_slow_compen_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_slow_compen_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_slow_compen_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsSlowCompenRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsSlowCompenRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_slow_compen_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsSlowCompenRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_slow_compen_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsSlowCompenRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_slow_compen_reg to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsSlowCompenRegSpec {
    const RESET_VALUE: u32 = 0;
}
