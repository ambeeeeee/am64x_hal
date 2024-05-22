#[doc = "Register `APBADDR_ETM_CPU1_TRCTSCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrctsctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCTSCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrctsctlrSpec>;
#[doc = "Field `EVENT` reader - 7:0\\]
An event selector. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams."]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - 7:0\\]
An event selector. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams."]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCTSCTLR_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trctsctlr31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCTSCTLR_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trctsctlr31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
An event selector. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams."]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trctsctlr_31_8(&self) -> Res0Trctsctlr31_8R {
        Res0Trctsctlr31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
An event selector. When the selected event is triggered, the trace unit inserts a global timestamp into the trace streams."]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EventW<ApbaddrEtmCpu1TrctsctlrSpec> {
        EventW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trctsctlr_31_8(&mut self) -> Res0Trctsctlr31_8W<ApbaddrEtmCpu1TrctsctlrSpec> {
        Res0Trctsctlr31_8W::new(self, 8)
    }
}
#[doc = "Global Timestamp Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trctsctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trctsctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrctsctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrctsctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trctsctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrctsctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trctsctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrctsctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCTSCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrctsctlrSpec {
    const RESET_VALUE: u32 = 0;
}
