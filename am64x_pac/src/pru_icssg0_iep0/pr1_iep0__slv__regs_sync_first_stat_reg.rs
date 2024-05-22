#[doc = "Register `PR1_IEP0__SLV__REGS_sync_first_stat_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsSyncFirstStatRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_sync_first_stat_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsSyncFirstStatRegSpec>;
#[doc = "Field `FIRST_SYNC0` reader - "]
pub type FirstSync0R = crate::BitReader;
#[doc = "Field `FIRST_SYNC0` writer - "]
pub type FirstSync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRST_SYNC1` reader - "]
pub type FirstSync1R = crate::BitReader;
#[doc = "Field `FIRST_SYNC1` writer - "]
pub type FirstSync1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn first_sync0(&self) -> FirstSync0R {
        FirstSync0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn first_sync1(&self) -> FirstSync1R {
        FirstSync1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn first_sync0(&mut self) -> FirstSync0W<Pr1Iep0_Slv_RegsSyncFirstStatRegSpec> {
        FirstSync0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn first_sync1(&mut self) -> FirstSync1W<Pr1Iep0_Slv_RegsSyncFirstStatRegSpec> {
        FirstSync1W::new(self, 1)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_sync_first_stat_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_sync_first_stat_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_sync_first_stat_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsSyncFirstStatRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsSyncFirstStatRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_sync_first_stat_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsSyncFirstStatRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_sync_first_stat_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsSyncFirstStatRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_sync_first_stat_reg to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsSyncFirstStatRegSpec {
    const RESET_VALUE: u32 = 0;
}
