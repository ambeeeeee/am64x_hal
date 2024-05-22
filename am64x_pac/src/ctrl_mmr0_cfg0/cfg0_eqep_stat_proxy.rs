#[doc = "Register `CFG0_EQEP_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0EqepStatProxySpec>;
#[doc = "Register `CFG0_EQEP_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0EqepStatProxySpec>;
#[doc = "Field `EQEP_STAT_PHASE_ERR0_PROXY` reader - 0:0\\]
EQEP0 Phase error status"]
pub type EqepStatPhaseErr0ProxyR = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR0_PROXY` writer - 0:0\\]
EQEP0 Phase error status"]
pub type EqepStatPhaseErr0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQEP_STAT_PHASE_ERR1_PROXY` reader - 1:1\\]
EQEP1 Phase error status"]
pub type EqepStatPhaseErr1ProxyR = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR1_PROXY` writer - 1:1\\]
EQEP1 Phase error status"]
pub type EqepStatPhaseErr1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQEP_STAT_PHASE_ERR2_PROXY` reader - 2:2\\]
EQEP2 Phase error status"]
pub type EqepStatPhaseErr2ProxyR = crate::BitReader;
#[doc = "Field `EQEP_STAT_PHASE_ERR2_PROXY` writer - 2:2\\]
EQEP2 Phase error status"]
pub type EqepStatPhaseErr2ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EQEP0 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err0_proxy(&self) -> EqepStatPhaseErr0ProxyR {
        EqepStatPhaseErr0ProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EQEP1 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err1_proxy(&self) -> EqepStatPhaseErr1ProxyR {
        EqepStatPhaseErr1ProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
EQEP2 Phase error status"]
    #[inline(always)]
    pub fn eqep_stat_phase_err2_proxy(&self) -> EqepStatPhaseErr2ProxyR {
        EqepStatPhaseErr2ProxyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EQEP0 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err0_proxy(&mut self) -> EqepStatPhaseErr0ProxyW<Cfg0EqepStatProxySpec> {
        EqepStatPhaseErr0ProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EQEP1 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err1_proxy(&mut self) -> EqepStatPhaseErr1ProxyW<Cfg0EqepStatProxySpec> {
        EqepStatPhaseErr1ProxyW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
EQEP2 Phase error status"]
    #[inline(always)]
    #[must_use]
    pub fn eqep_stat_phase_err2_proxy(&mut self) -> EqepStatPhaseErr2ProxyW<Cfg0EqepStatProxySpec> {
        EqepStatPhaseErr2ProxyW::new(self, 2)
    }
}
#[doc = "CFG0_EQEP_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eqep_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eqep_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EqepStatProxySpec;
impl crate::RegisterSpec for Cfg0EqepStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eqep_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0EqepStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eqep_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0EqepStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_EQEP_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0EqepStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
