#[doc = "Register `PR1_IEP1__SLV__REGS_sync1_delay_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsSync1DelayRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_sync1_delay_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsSync1DelayRegSpec>;
#[doc = "Field `SYNC1_DELAY` reader - "]
pub type Sync1DelayR = crate::FieldReader<u32>;
#[doc = "Field `SYNC1_DELAY` writer - "]
pub type Sync1DelayW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sync1_delay(&self) -> Sync1DelayR {
        Sync1DelayR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_delay(&mut self) -> Sync1DelayW<Pr1Iep1_Slv_RegsSync1DelayRegSpec> {
        Sync1DelayW::new(self, 0)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_sync1_delay_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync1_delay_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync1_delay_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsSync1DelayRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsSync1DelayRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_sync1_delay_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsSync1DelayRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_sync1_delay_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsSync1DelayRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_sync1_delay_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsSync1DelayRegSpec {
    const RESET_VALUE: u32 = 0;
}
