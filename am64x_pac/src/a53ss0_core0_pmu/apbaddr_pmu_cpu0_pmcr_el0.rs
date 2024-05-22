#[doc = "Register `APBADDR_PMU_CPU0_PMCR_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmcrEl0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCR_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmcrEl0Spec>;
#[doc = "Field `E` reader - 0:0\\]
Enable. The possible values of this bit are: 0 All counters, including PMCCNTR_EL0, are disabled. 1 All counters are enabled by PMCNTENSET_EL0. This bit is RW."]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - 0:0\\]
Enable. The possible values of this bit are: 0 All counters, including PMCCNTR_EL0, are disabled. 1 All counters are enabled by PMCNTENSET_EL0. This bit is RW."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P` reader - 1:1\\]
Event counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset all event counters, not including PMCCNTR_EL0, to zero. This bit is always RAZ.Resetting the event counters does not clear any overflow bits to 0."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - 1:1\\]
Event counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset all event counters, not including PMCCNTR_EL0, to zero. This bit is always RAZ.Resetting the event counters does not clear any overflow bits to 0."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C` reader - 2:2\\]
Cycle counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset PMCCNTR_EL0 to zero. This bit is always RAZ.Resetting PMCCNTR_EL0 does not clear the PMCCNTR_EL0 overflow bit to 0."]
pub type CR = crate::BitReader;
#[doc = "Field `C` writer - 2:2\\]
Cycle counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset PMCCNTR_EL0 to zero. This bit is always RAZ.Resetting PMCCNTR_EL0 does not clear the PMCCNTR_EL0 overflow bit to 0."]
pub type CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D` reader - 3:3\\]
Clock divider. The possible values of this bit are: 0 When enabled, PMCCNTR_EL0 counts every clock cycle. 1 When enabled, PMCCNTR_EL0 counts once every 64 clock cycles. This bit is RW.If PMCR_EL0.LC == 1, this bit is ignored and the cycle counter counts every clock cycle.ARM deprecates use of PMCR.D = 1."]
pub type DR = crate::BitReader;
#[doc = "Field `D` writer - 3:3\\]
Clock divider. The possible values of this bit are: 0 When enabled, PMCCNTR_EL0 counts every clock cycle. 1 When enabled, PMCCNTR_EL0 counts once every 64 clock cycles. This bit is RW.If PMCR_EL0.LC == 1, this bit is ignored and the cycle counter counts every clock cycle.ARM deprecates use of PMCR.D = 1."]
pub type DW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X` reader - 4:4\\]
Enable export of events in an IMPLEMENTATION DEFINED event stream. The possible values of this bit are: 0 Do not export events. 1 Export events where not prohibited. This bit is used to permit events to be exported to another debug device, such as an OPTIONAL trace extension, over an event bus. If the implementation does not include such an event bus, this bit is RAZ/WI.This bit does not affect the generation of Performance Monitors overflow interrupt requests or signaling to a cross-trigger interface \\[CTI\\]
that can be implemented as signals exported from the processor.If the implementation does not include an exported event stream, this bit is RAZ/WI. Otherwise this bit is RW."]
pub type XR = crate::BitReader;
#[doc = "Field `X` writer - 4:4\\]
Enable export of events in an IMPLEMENTATION DEFINED event stream. The possible values of this bit are: 0 Do not export events. 1 Export events where not prohibited. This bit is used to permit events to be exported to another debug device, such as an OPTIONAL trace extension, over an event bus. If the implementation does not include such an event bus, this bit is RAZ/WI.This bit does not affect the generation of Performance Monitors overflow interrupt requests or signaling to a cross-trigger interface \\[CTI\\]
that can be implemented as signals exported from the processor.If the implementation does not include an exported event stream, this bit is RAZ/WI. Otherwise this bit is RW."]
pub type XW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DP` reader - 5:5\\]
Disable cycle counter when event counting is prohibited. The possible values of this bit are: 0 PMCCNTR_EL0, if enabled, counts when event counting is prohibited. 1 PMCCNTR_EL0 does not count when event counting is prohibited. Event counting is prohibited when ProfilingProhibited\\[IsSecure\\[\\],PSTATE.EL\\]
== TRUE.This bit is RW."]
pub type DpR = crate::BitReader;
#[doc = "Field `DP` writer - 5:5\\]
Disable cycle counter when event counting is prohibited. The possible values of this bit are: 0 PMCCNTR_EL0, if enabled, counts when event counting is prohibited. 1 PMCCNTR_EL0 does not count when event counting is prohibited. Event counting is prohibited when ProfilingProhibited\\[IsSecure\\[\\],PSTATE.EL\\]
== TRUE.This bit is RW."]
pub type DpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LC` reader - 6:6\\]
Long cycle counter enable. Determines which PMCCNTR_EL0 bit generates an overflow recorded by PMOVSR\\[31\\]. 0 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[31\\]
from 1 to 0. 1 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[63\\]
from 1 to 0. ARM deprecates use of PMCR_EL0.LC = 0."]
pub type LcR = crate::BitReader;
#[doc = "Field `LC` writer - 6:6\\]
Long cycle counter enable. Determines which PMCCNTR_EL0 bit generates an overflow recorded by PMOVSR\\[31\\]. 0 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[31\\]
from 1 to 0. 1 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[63\\]
from 1 to 0. ARM deprecates use of PMCR_EL0.LC = 0."]
pub type LcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_PMCR_EL0_10_7` reader - 10:7\\]
Reserved, RES0."]
pub type Res0PmcrEl0_10_7R = crate::FieldReader;
#[doc = "Field `RES0_PMCR_EL0_10_7` writer - 10:7\\]
Reserved, RES0."]
pub type Res0PmcrEl0_10_7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_PMCR_EL0_31_11` reader - 31:11\\]
Reserved, RAZ/WI."]
pub type Res0PmcrEl0_31_11R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMCR_EL0_31_11` writer - 31:11\\]
Reserved, RAZ/WI."]
pub type Res0PmcrEl0_31_11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable. The possible values of this bit are: 0 All counters, including PMCCNTR_EL0, are disabled. 1 All counters are enabled by PMCNTENSET_EL0. This bit is RW."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Event counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset all event counters, not including PMCCNTR_EL0, to zero. This bit is always RAZ.Resetting the event counters does not clear any overflow bits to 0."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Cycle counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset PMCCNTR_EL0 to zero. This bit is always RAZ.Resetting PMCCNTR_EL0 does not clear the PMCCNTR_EL0 overflow bit to 0."]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clock divider. The possible values of this bit are: 0 When enabled, PMCCNTR_EL0 counts every clock cycle. 1 When enabled, PMCCNTR_EL0 counts once every 64 clock cycles. This bit is RW.If PMCR_EL0.LC == 1, this bit is ignored and the cycle counter counts every clock cycle.ARM deprecates use of PMCR.D = 1."]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable export of events in an IMPLEMENTATION DEFINED event stream. The possible values of this bit are: 0 Do not export events. 1 Export events where not prohibited. This bit is used to permit events to be exported to another debug device, such as an OPTIONAL trace extension, over an event bus. If the implementation does not include such an event bus, this bit is RAZ/WI.This bit does not affect the generation of Performance Monitors overflow interrupt requests or signaling to a cross-trigger interface \\[CTI\\]
that can be implemented as signals exported from the processor.If the implementation does not include an exported event stream, this bit is RAZ/WI. Otherwise this bit is RW."]
    #[inline(always)]
    pub fn x(&self) -> XR {
        XR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Disable cycle counter when event counting is prohibited. The possible values of this bit are: 0 PMCCNTR_EL0, if enabled, counts when event counting is prohibited. 1 PMCCNTR_EL0 does not count when event counting is prohibited. Event counting is prohibited when ProfilingProhibited\\[IsSecure\\[\\],PSTATE.EL\\]
== TRUE.This bit is RW."]
    #[inline(always)]
    pub fn dp(&self) -> DpR {
        DpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Long cycle counter enable. Determines which PMCCNTR_EL0 bit generates an overflow recorded by PMOVSR\\[31\\]. 0 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[31\\]
from 1 to 0. 1 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[63\\]
from 1 to 0. ARM deprecates use of PMCR_EL0.LC = 0."]
    #[inline(always)]
    pub fn lc(&self) -> LcR {
        LcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmcr_el0_10_7(&self) -> Res0PmcrEl0_10_7R {
        Res0PmcrEl0_10_7R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved, RAZ/WI."]
    #[inline(always)]
    pub fn res0_pmcr_el0_31_11(&self) -> Res0PmcrEl0_31_11R {
        Res0PmcrEl0_31_11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable. The possible values of this bit are: 0 All counters, including PMCCNTR_EL0, are disabled. 1 All counters are enabled by PMCNTENSET_EL0. This bit is RW."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> EW<ApbaddrPmuCpu0PmcrEl0Spec> {
        EW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Event counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset all event counters, not including PMCCNTR_EL0, to zero. This bit is always RAZ.Resetting the event counters does not clear any overflow bits to 0."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<ApbaddrPmuCpu0PmcrEl0Spec> {
        PW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Cycle counter reset. This bit is WO. The effects of writing to this bit are: 0 No action. 1 Reset PMCCNTR_EL0 to zero. This bit is always RAZ.Resetting PMCCNTR_EL0 does not clear the PMCCNTR_EL0 overflow bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> CW<ApbaddrPmuCpu0PmcrEl0Spec> {
        CW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clock divider. The possible values of this bit are: 0 When enabled, PMCCNTR_EL0 counts every clock cycle. 1 When enabled, PMCCNTR_EL0 counts once every 64 clock cycles. This bit is RW.If PMCR_EL0.LC == 1, this bit is ignored and the cycle counter counts every clock cycle.ARM deprecates use of PMCR.D = 1."]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> DW<ApbaddrPmuCpu0PmcrEl0Spec> {
        DW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enable export of events in an IMPLEMENTATION DEFINED event stream. The possible values of this bit are: 0 Do not export events. 1 Export events where not prohibited. This bit is used to permit events to be exported to another debug device, such as an OPTIONAL trace extension, over an event bus. If the implementation does not include such an event bus, this bit is RAZ/WI.This bit does not affect the generation of Performance Monitors overflow interrupt requests or signaling to a cross-trigger interface \\[CTI\\]
that can be implemented as signals exported from the processor.If the implementation does not include an exported event stream, this bit is RAZ/WI. Otherwise this bit is RW."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> XW<ApbaddrPmuCpu0PmcrEl0Spec> {
        XW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Disable cycle counter when event counting is prohibited. The possible values of this bit are: 0 PMCCNTR_EL0, if enabled, counts when event counting is prohibited. 1 PMCCNTR_EL0 does not count when event counting is prohibited. Event counting is prohibited when ProfilingProhibited\\[IsSecure\\[\\],PSTATE.EL\\]
== TRUE.This bit is RW."]
    #[inline(always)]
    #[must_use]
    pub fn dp(&mut self) -> DpW<ApbaddrPmuCpu0PmcrEl0Spec> {
        DpW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Long cycle counter enable. Determines which PMCCNTR_EL0 bit generates an overflow recorded by PMOVSR\\[31\\]. 0 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[31\\]
from 1 to 0. 1 Cycle counter overflow on increment that changes PMCCNTR_EL0\\[63\\]
from 1 to 0. ARM deprecates use of PMCR_EL0.LC = 0."]
    #[inline(always)]
    #[must_use]
    pub fn lc(&mut self) -> LcW<ApbaddrPmuCpu0PmcrEl0Spec> {
        LcW::new(self, 6)
    }
    #[doc = "Bits 7:10 - 10:7\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmcr_el0_10_7(&mut self) -> Res0PmcrEl0_10_7W<ApbaddrPmuCpu0PmcrEl0Spec> {
        Res0PmcrEl0_10_7W::new(self, 7)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved, RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmcr_el0_31_11(&mut self) -> Res0PmcrEl0_31_11W<ApbaddrPmuCpu0PmcrEl0Spec> {
        Res0PmcrEl0_31_11W::new(self, 11)
    }
}
#[doc = "Performance Monitors Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcr_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcr_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmcrEl0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmcrEl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmcr_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmcrEl0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmcr_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmcrEl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCR_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0PmcrEl0Spec {
    const RESET_VALUE: u32 = 0;
}
