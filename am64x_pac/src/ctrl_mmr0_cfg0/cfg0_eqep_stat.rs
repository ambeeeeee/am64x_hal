#[doc = "Register `CFG0_EQEP_STAT` reader"]
pub type R = crate::R<Cfg0EqepStatSpec>;
#[doc = "Register `CFG0_EQEP_STAT` writer"]
pub type W = crate::W<Cfg0EqepStatSpec>;
#[doc = "Field `EQEP_STAT_PHASE_ERR0` reader - 0:0\\]
EQEP0 Phase error status"]
pub type EqepStatPhaseErr0R = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR0` writer - 0:0\\]
EQEP0 Phase error status"]
pub type EqepStatPhaseErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQEP_STAT_PHASE_ERR1` reader - 1:1\\]
EQEP1 Phase error status"]
pub type EqepStatPhaseErr1R = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR1` writer - 1:1\\]
EQEP1 Phase error status"]
pub type EqepStatPhaseErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQEP_STAT_PHASE_ERR2` reader - 2:2\\]
EQEP2 Phase error status"]
pub type EqepStatPhaseErr2R = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR2` writer - 2:2\\]
EQEP2 Phase error status"]
pub type EqepStatPhaseErr2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EQEP0 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err0(&self) -> EqepStatPhaseErr0R {
        EqepStatPhaseErr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EQEP1 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err1(&self) -> EqepStatPhaseErr1R {
        EqepStatPhaseErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
EQEP2 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err2(&self) -> EqepStatPhaseErr2R {
        EqepStatPhaseErr2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EQEP0 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err0(&mut self) -> EqepStatPhaseErr0W<Cfg0EqepStatSpec> {
        EqepStatPhaseErr0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EQEP1 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err1(&mut self) -> EqepStatPhaseErr1W<Cfg0EqepStatSpec> {
        EqepStatPhaseErr1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
EQEP2 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err2(&mut self) -> EqepStatPhaseErr2W<Cfg0EqepStatSpec> {
        EqepStatPhaseErr2W::new(self, 2)
    }
}
#[doc = "CFG0_EQEP_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EqepStatSpec;
impl crate::RegisterSpec for Cfg0EqepStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eqep_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0EqepStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eqep_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0EqepStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EQEP_STAT to value 0"]
impl crate::Resettable for Cfg0EqepStatSpec {
    const RESET_VALUE: u32 = 0;
}
