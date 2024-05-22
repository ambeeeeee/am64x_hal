#[doc = "Register `APBADDR_PMU_CPU0_PMEVTYPER0_EL0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0Pmevtyper0El0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMEVTYPER0_EL0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0Pmevtyper0El0Spec>;
#[doc = "Field `EVTCOUNT` reader - 9:0\\]
Event to count. The event number of the event that is counted by event counter PMEVCNTR&lt;n>_EL0.Software must program this field with an event defined by the processor or a common event defined by the architecture.If evtCount is programmed to an event that is reserved or not implemented, the behavior depends on the event type.For common architectural and microarchitectural events:No events are counted.The value read back on evtCount is the value written.For IMPLEMENTATION DEFINED events:It is UNPREDICTABLE what event, if any, is counted. UNPREDICTABLE in this case means the event must not expose privileged information.The value read back on evtCount is an UNKNOWN value with the same effect.ARM recommends that the behavior across a family of implementations is defined such that if a given implementation does not include an event from a set of common IMPLEMENTATION DEFINED events, then no event is counted and the value read back on evtCount is the value written."]
pub type EvtcountR = crate::FieldReader<u16>;
#[doc = "Field `EVTCOUNT` writer - 9:0\\]
Event to count. The event number of the event that is counted by event counter PMEVCNTR&lt;n>_EL0.Software must program this field with an event defined by the processor or a common event defined by the architecture.If evtCount is programmed to an event that is reserved or not implemented, the behavior depends on the event type.For common architectural and microarchitectural events:No events are counted.The value read back on evtCount is the value written.For IMPLEMENTATION DEFINED events:It is UNPREDICTABLE what event, if any, is counted. UNPREDICTABLE in this case means the event must not expose privileged information.The value read back on evtCount is an UNKNOWN value with the same effect.ARM recommends that the behavior across a family of implementations is defined such that if a given implementation does not include an event from a set of common IMPLEMENTATION DEFINED events, then no event is counted and the value read back on evtCount is the value written."]
pub type EvtcountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RES0_PMEVTYPER0_EL0_25_10` reader - 25:10\\]
Reserved, RES0."]
pub type Res0Pmevtyper0El0_25_10R = crate::FieldReader<u16>;
#[doc = "Field `RES0_PMEVTYPER0_EL0_25_10` writer - 25:10\\]
Reserved, RES0."]
pub type Res0Pmevtyper0El0_25_10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `M` reader - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Secure EL3 are counted.Otherwise, events in Secure EL3 are not counted."]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Secure EL3 are counted.Otherwise, events in Secure EL3 are not counted."]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSH` reader - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count events in EL2. 1 Count events in EL2."]
pub type NshR = crate::BitReader;
#[doc = "Field `NSH` writer - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count events in EL2. 1 Count events in EL2."]
pub type NshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSU` reader - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, events in Non-secure EL0 are counted.Otherwise, events in Non-secure EL0 are not counted."]
pub type NsuR = crate::BitReader;
#[doc = "Field `NSU` writer - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, events in Non-secure EL0 are counted.Otherwise, events in Non-secure EL0 are not counted."]
pub type NsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSK` reader - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Non-secure EL1 are counted.Otherwise, events in Non-secure EL1 are not counted."]
pub type NskR = crate::BitReader;
#[doc = "Field `NSK` writer - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Non-secure EL1 are counted.Otherwise, events in Non-secure EL1 are not counted."]
pub type NskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U` reader - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count events in EL0. 1 Do not count events in EL0."]
pub type UR = crate::BitReader;
#[doc = "Field `U` writer - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count events in EL0. 1 Do not count events in EL0."]
pub type UW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P` reader - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count events in EL1. 1 Do not count events in EL1."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count events in EL1. 1 Do not count events in EL1."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Event to count. The event number of the event that is counted by event counter PMEVCNTR&lt;n>_EL0.Software must program this field with an event defined by the processor or a common event defined by the architecture.If evtCount is programmed to an event that is reserved or not implemented, the behavior depends on the event type.For common architectural and microarchitectural events:No events are counted.The value read back on evtCount is the value written.For IMPLEMENTATION DEFINED events:It is UNPREDICTABLE what event, if any, is counted. UNPREDICTABLE in this case means the event must not expose privileged information.The value read back on evtCount is an UNKNOWN value with the same effect.ARM recommends that the behavior across a family of implementations is defined such that if a given implementation does not include an event from a set of common IMPLEMENTATION DEFINED events, then no event is counted and the value read back on evtCount is the value written."]
    #[inline(always)]
    pub fn evtcount(&self) -> EvtcountR {
        EvtcountR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:25 - 25:10\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmevtyper0_el0_25_10(&self) -> Res0Pmevtyper0El0_25_10R {
        Res0Pmevtyper0El0_25_10R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Secure EL3 are counted.Otherwise, events in Secure EL3 are not counted."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count events in EL2. 1 Count events in EL2."]
    #[inline(always)]
    pub fn nsh(&self) -> NshR {
        NshR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, events in Non-secure EL0 are counted.Otherwise, events in Non-secure EL0 are not counted."]
    #[inline(always)]
    pub fn nsu(&self) -> NsuR {
        NsuR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Non-secure EL1 are counted.Otherwise, events in Non-secure EL1 are not counted."]
    #[inline(always)]
    pub fn nsk(&self) -> NskR {
        NskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count events in EL0. 1 Do not count events in EL0."]
    #[inline(always)]
    pub fn u(&self) -> UR {
        UR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count events in EL1. 1 Do not count events in EL1."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Event to count. The event number of the event that is counted by event counter PMEVCNTR&lt;n>_EL0.Software must program this field with an event defined by the processor or a common event defined by the architecture.If evtCount is programmed to an event that is reserved or not implemented, the behavior depends on the event type.For common architectural and microarchitectural events:No events are counted.The value read back on evtCount is the value written.For IMPLEMENTATION DEFINED events:It is UNPREDICTABLE what event, if any, is counted. UNPREDICTABLE in this case means the event must not expose privileged information.The value read back on evtCount is an UNKNOWN value with the same effect.ARM recommends that the behavior across a family of implementations is defined such that if a given implementation does not include an event from a set of common IMPLEMENTATION DEFINED events, then no event is counted and the value read back on evtCount is the value written."]
    #[inline(always)]
    #[must_use]
    pub fn evtcount(&mut self) -> EvtcountW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        EvtcountW::new(self, 0)
    }
    #[doc = "Bits 10:25 - 25:10\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmevtyper0_el0_25_10(
        &mut self,
    ) -> Res0Pmevtyper0El0_25_10W<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        Res0Pmevtyper0El0_25_10W::new(self, 10)
    }
    #[doc = "Bit 26 - 26:26\\]
Secure EL3 filtering bit. Most applications can ignore this bit and set the value to zero. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Secure EL3 are counted.Otherwise, events in Secure EL3 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        MW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Non-secure Hyp modes filtering bit. Controls counting in Non-secure EL2. If EL2 is not implemented, this bit is RES0. 0 Do not count events in EL2. 1 Count events in EL2."]
    #[inline(always)]
    #[must_use]
    pub fn nsh(&mut self) -> NshW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        NshW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Non-secure user modes filtering bit. Controls counting in Non-secure EL0. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of U, events in Non-secure EL0 are counted.Otherwise, events in Non-secure EL0 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn nsu(&mut self) -> NsuW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        NsuW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Non-secure kernel modes filtering bit. Controls counting in Non-secure EL1. If EL3 is not implemented, this bit is RES0.If the value of this bit is equal to the value of P, events in Non-secure EL1 are counted.Otherwise, events in Non-secure EL1 are not counted."]
    #[inline(always)]
    #[must_use]
    pub fn nsk(&mut self) -> NskW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        NskW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
EL0 filtering bit. Controls counting in EL0. If EL3 is implemented, then counting in Non-secure EL0 is further controlled by the NSU bit. The possible values of this bit are: 0 Count events in EL0. 1 Do not count events in EL0."]
    #[inline(always)]
    #[must_use]
    pub fn u(&mut self) -> UW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        UW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
EL1 modes filtering bit. Controls counting in EL1. If EL3 is implemented, then counting in Non-secure EL1 is further controlled by the NSK bit. The possible values of this bit are: 0 Count events in EL1. 1 Do not count events in EL1."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<ApbaddrPmuCpu0Pmevtyper0El0Spec> {
        PW::new(self, 31)
    }
}
#[doc = "Performance Monitors Event Type Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmevtyper0_el0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmevtyper0_el0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0Pmevtyper0El0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0Pmevtyper0El0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmevtyper0_el0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0Pmevtyper0El0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmevtyper0_el0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0Pmevtyper0El0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMEVTYPER0_EL0 to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0Pmevtyper0El0Spec {
    const RESET_VALUE: u32 = 0;
}
