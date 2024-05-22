#[doc = "Register `APBADDR_ETM_CPU0_TRCPRGCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcprgctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCPRGCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcprgctlrSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Trace unit enable bit. Possible values are: 0 The trace unit is disabled. All trace resources are inactive and no trace is generated. 1 The trace unit is enabled."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Trace unit enable bit. Possible values are: 0 The trace unit is disabled. All trace resources are inactive and no trace is generated. 1 The trace unit is enabled."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCPRGCTLR_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Trcprgctlr31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPRGCTLR_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Trcprgctlr31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trace unit enable bit. Possible values are: 0 The trace unit is disabled. All trace resources are inactive and no trace is generated. 1 The trace unit is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcprgctlr_31_1(&self) -> Res0Trcprgctlr31_1R {
        Res0Trcprgctlr31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trace unit enable bit. Possible values are: 0 The trace unit is disabled. All trace resources are inactive and no trace is generated. 1 The trace unit is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ApbaddrEtmCpu0TrcprgctlrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcprgctlr_31_1(&mut self) -> Res0Trcprgctlr31_1W<ApbaddrEtmCpu0TrcprgctlrSpec> {
        Res0Trcprgctlr31_1W::new(self, 1)
    }
}
#[doc = "Programming Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcprgctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcprgctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcprgctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcprgctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcprgctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcprgctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcprgctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcprgctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCPRGCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcprgctlrSpec {
    const RESET_VALUE: u32 = 0;
}
