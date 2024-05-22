#[doc = "Register `PR1_IEP1__SLV__REGS_sync1_stat_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsSync1StatRegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_sync1_stat_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsSync1StatRegSpec>;
#[doc = "Field `SYNC1_PEND` reader - "]
pub type Sync1PendR = crate::BitReader;
#[doc = "Field `SYNC1_PEND` writer - "]
pub type Sync1PendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sync1_pend(&self) -> Sync1PendR {
        Sync1PendR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sync1_pend(&mut self) -> Sync1PendW<Pr1Iep1_Slv_RegsSync1StatRegSpec> {
        Sync1PendW::new(self, 0)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_sync1_stat_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_sync1_stat_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_sync1_stat_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsSync1StatRegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsSync1StatRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_sync1_stat_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsSync1StatRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_sync1_stat_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsSync1StatRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_sync1_stat_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsSync1StatRegSpec {
    const RESET_VALUE: u32 = 0;
}
