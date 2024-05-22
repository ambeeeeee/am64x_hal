#[doc = "Register `APBADDR_ETM_CPU1_TRCSTATR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcstatrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCSTATR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcstatrSpec>;
#[doc = "Field `IDLE` reader - 0:0\\]
Idle status bit: 0 The trace unit is not idle. 1 The trace unit is idle. The trace unit is idle when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.The trace unit is drained of any trace.With the exception of the programming interfaces, all external interfaces on the trace unit are quiescent."]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 0:0\\]
Idle status bit: 0 The trace unit is not idle. 1 The trace unit is idle. The trace unit is idle when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.The trace unit is drained of any trace.With the exception of the programming interfaces, all external interfaces on the trace unit are quiescent."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMSTABLE` reader - 1:1\\]
Programmer's model stable bit: 0 The programmer's model is not stable. 1 The programmer's model is stable. When polled, the trace unit trace registers return stable data. The programmer's model is stable when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.Reads from trace unit registers return stable data."]
pub type PmstableR = crate::BitReader;
#[doc = "Field `PMSTABLE` writer - 1:1\\]
Programmer's model stable bit: 0 The programmer's model is not stable. 1 The programmer's model is stable. When polled, the trace unit trace registers return stable data. The programmer's model is stable when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.Reads from trace unit registers return stable data."]
pub type PmstableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCSTATR_31_2` reader - 31:2\\]
Reserved, RES0."]
pub type Res0Trcstatr31_2R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCSTATR_31_2` writer - 31:2\\]
Reserved, RES0."]
pub type Res0Trcstatr31_2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Idle status bit: 0 The trace unit is not idle. 1 The trace unit is idle. The trace unit is idle when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.The trace unit is drained of any trace.With the exception of the programming interfaces, all external interfaces on the trace unit are quiescent."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmer's model stable bit: 0 The programmer's model is not stable. 1 The programmer's model is stable. When polled, the trace unit trace registers return stable data. The programmer's model is stable when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.Reads from trace unit registers return stable data."]
    #[inline(always)]
    pub fn pmstable(&self) -> PmstableR {
        PmstableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcstatr_31_2(&self) -> Res0Trcstatr31_2R {
        Res0Trcstatr31_2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Idle status bit: 0 The trace unit is not idle. 1 The trace unit is idle. The trace unit is idle when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.The trace unit is drained of any trace.With the exception of the programming interfaces, all external interfaces on the trace unit are quiescent."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<ApbaddrEtmCpu1TrcstatrSpec> {
        IdleW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Programmer's model stable bit: 0 The programmer's model is not stable. 1 The programmer's model is stable. When polled, the trace unit trace registers return stable data. The programmer's model is stable when all of the following are true:TRCPRGCTLR.EN==0 or the OS Lock is locked.Reads from trace unit registers return stable data."]
    #[inline(always)]
    #[must_use]
    pub fn pmstable(&mut self) -> PmstableW<ApbaddrEtmCpu1TrcstatrSpec> {
        PmstableW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcstatr_31_2(&mut self) -> Res0Trcstatr31_2W<ApbaddrEtmCpu1TrcstatrSpec> {
        Res0Trcstatr31_2W::new(self, 2)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcstatr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcstatr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcstatrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcstatrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcstatr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcstatrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcstatr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcstatrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCSTATR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcstatrSpec {
    const RESET_VALUE: u32 = 0;
}
