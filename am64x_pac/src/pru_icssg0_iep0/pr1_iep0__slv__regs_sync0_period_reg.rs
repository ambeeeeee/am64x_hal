#[doc = "Register `PR1_IEP0__SLV__REGS_sync0_period_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsSync0PeriodRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_sync0_period_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsSync0PeriodRegSpec>;
#[doc = "Field `SYNC0_PERIOD` reader - "]
pub type Sync0PeriodR = crate::FieldReader<u32>;
#[doc = "Field `SYNC0_PERIOD` writer - "]
pub type Sync0PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sync0_period(&self) -> Sync0PeriodR {
        Sync0PeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sync0_period(&mut self) -> Sync0PeriodW<Pr1Iep0_Slv_RegsSync0PeriodRegSpec> {
        Sync0PeriodW::new(self, 0)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_sync0_period_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_sync0_period_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_sync0_period_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsSync0PeriodRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsSync0PeriodRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_sync0_period_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsSync0PeriodRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_sync0_period_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsSync0PeriodRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_sync0_period_reg to value 0x01"]
impl crate::Resettable for Pr1Iep0_Slv_RegsSync0PeriodRegSpec {
    const RESET_VALUE: u32 = 0x01;
}
