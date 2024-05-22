#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR5` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr5Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR5` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr5Spec>;
#[doc = "Field `NUMEXTIN` reader - 8:0\\]
Indicates how many external inputs are implemented. The permitted values are: 000000000 No external inputs are available. If NUMEXTIN is zero, NUMEXTINSEL must also be zero. 000000001 The implementation has one external input. 000000010 The implementation has two external inputs. and so on up to 0b100000000, which indicates that the implementation has 256 external inputs.All other values >0b100000000 are reserved.See TRCEXTINSELR for how to select an external input."]
pub type NumextinR = crate::FieldReader<u16>;
#[doc = "Field `NUMEXTIN` writer - 8:0\\]
Indicates how many external inputs are implemented. The permitted values are: 000000000 No external inputs are available. If NUMEXTIN is zero, NUMEXTINSEL must also be zero. 000000001 The implementation has one external input. 000000010 The implementation has two external inputs. and so on up to 0b100000000, which indicates that the implementation has 256 external inputs.All other values >0b100000000 are reserved.See TRCEXTINSELR for how to select an external input."]
pub type NumextinW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NUMEXTINSEL` reader - 11:9\\]
Indicates how many external input select resources are implemented. The permitted values are: 000 No external input select resources are available. If NUMEXTINSEL is zero, NUMEXTIN must also be zero. 001 The implementation has one external input select resource. 010 The implementation has two external input select resources. 011 The implementation has three external input select resources. 100 The implementation has four external input select resources. All other values are reserved.See TRCEXTINSELR for how to select an input select resource."]
pub type NumextinselR = crate::FieldReader;
#[doc = "Field `NUMEXTINSEL` writer - 11:9\\]
Indicates how many external input select resources are implemented. The permitted values are: 000 No external input select resources are available. If NUMEXTINSEL is zero, NUMEXTIN must also be zero. 001 The implementation has one external input select resource. 010 The implementation has two external input select resources. 011 The implementation has three external input select resources. 100 The implementation has four external input select resources. All other values are reserved.See TRCEXTINSELR for how to select an input select resource."]
pub type NumextinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES0_TRCIDR5_15_12` reader - 15:12\\]
Reserved, RES0."]
pub type Res0Trcidr5_15_12R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR5_15_12` writer - 15:12\\]
Reserved, RES0."]
pub type Res0Trcidr5_15_12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRACEIDSIZE` reader - 21:16\\]
Indicates the trace ID width. The permitted value is: 111 The implementation supports a 7-bit trace ID. This sets the width of the TRCTRACEIDR.TRACEID field. All other values are reserved.The CoreSight ATB requires a 7-bit trace ID width."]
pub type TraceidsizeR = crate::FieldReader;
#[doc = "Field `TRACEIDSIZE` writer - 21:16\\]
Indicates the trace ID width. The permitted value is: 111 The implementation supports a 7-bit trace ID. This sets the width of the TRCTRACEIDR.TRACEID field. All other values are reserved.The CoreSight ATB requires a 7-bit trace ID width."]
pub type TraceidsizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ATBTRIG` reader - 22:22\\]
Indicates if the implementation can support ATB triggers: 0 The implementation does not support ATB triggers. 1 The implementation supports ATB triggers, and the TRCEVENTCTL1R.ATBTRIG field is implemented."]
pub type AtbtrigR = crate::BitReader;
#[doc = "Field `ATBTRIG` writer - 22:22\\]
Indicates if the implementation can support ATB triggers: 0 The implementation does not support ATB triggers. 1 The implementation supports ATB triggers, and the TRCEVENTCTL1R.ATBTRIG field is implemented."]
pub type AtbtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOVERRIDE` reader - 23:23\\]
Indicates if the implementation can support low-power state override: 0 The implementation does not support low-power state override. 1 The implementation supports low-power state override, and the TRCEVENTCTL1R.LPOVERRIDE field is implemented. The trace unit must support low-power state override if it can enter a low-power mode where the resources and event trace generation are disabled."]
pub type LpoverrideR = crate::BitReader;
#[doc = "Field `LPOVERRIDE` writer - 23:23\\]
Indicates if the implementation can support low-power state override: 0 The implementation does not support low-power state override. 1 The implementation supports low-power state override, and the TRCEVENTCTL1R.LPOVERRIDE field is implemented. The trace unit must support low-power state override if it can enter a low-power mode where the resources and event trace generation are disabled."]
pub type LpoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCIDR5_24_24` reader - 24:24\\]
Reserved, RES0."]
pub type Res0Trcidr5_24_24R = crate::BitReader;
#[doc = "Field `RES0_TRCIDR5_24_24` writer - 24:24\\]
Reserved, RES0."]
pub type Res0Trcidr5_24_24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMSEQSTATE` reader - 27:25\\]
Indicates the number of sequencer states that are implemented. The permitted values are: 000 No sequencer states are implemented. 100 The implementation has four sequencer states. All other values are reserved."]
pub type NumseqstateR = crate::FieldReader;
#[doc = "Field `NUMSEQSTATE` writer - 27:25\\]
Indicates the number of sequencer states that are implemented. The permitted values are: 000 No sequencer states are implemented. 100 The implementation has four sequencer states. All other values are reserved."]
pub type NumseqstateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NUMCNTR` reader - 30:28\\]
Indicates the number of counters that are available for tracing. The permitted values are: 000 No counters are available. 001 The implementation has one counter. 010 The implementation has two counters. 011 The implementation has three counters. 100 The implementation has four counters. All other values are reserved."]
pub type NumcntrR = crate::FieldReader;
#[doc = "Field `NUMCNTR` writer - 30:28\\]
Indicates the number of counters that are available for tracing. The permitted values are: 000 No counters are available. 001 The implementation has one counter. 010 The implementation has two counters. 011 The implementation has three counters. 100 The implementation has four counters. All other values are reserved."]
pub type NumcntrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REDFUNCNTR` reader - 31:31\\]
Indicates if the reduced function counter is implemented: 0 The reduced function counter is not supported. 1 Counter 0 is implemented as a reduced function counter."]
pub type RedfuncntrR = crate::BitReader;
#[doc = "Field `REDFUNCNTR` writer - 31:31\\]
Indicates if the reduced function counter is implemented: 0 The reduced function counter is not supported. 1 Counter 0 is implemented as a reduced function counter."]
pub type RedfuncntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Indicates how many external inputs are implemented. The permitted values are: 000000000 No external inputs are available. If NUMEXTIN is zero, NUMEXTINSEL must also be zero. 000000001 The implementation has one external input. 000000010 The implementation has two external inputs. and so on up to 0b100000000, which indicates that the implementation has 256 external inputs.All other values >0b100000000 are reserved.See TRCEXTINSELR for how to select an external input."]
    #[inline(always)]
    pub fn numextin(&self) -> NumextinR {
        NumextinR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Indicates how many external input select resources are implemented. The permitted values are: 000 No external input select resources are available. If NUMEXTINSEL is zero, NUMEXTIN must also be zero. 001 The implementation has one external input select resource. 010 The implementation has two external input select resources. 011 The implementation has three external input select resources. 100 The implementation has four external input select resources. All other values are reserved.See TRCEXTINSELR for how to select an input select resource."]
    #[inline(always)]
    pub fn numextinsel(&self) -> NumextinselR {
        NumextinselR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr5_15_12(&self) -> Res0Trcidr5_15_12R {
        Res0Trcidr5_15_12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Indicates the trace ID width. The permitted value is: 111 The implementation supports a 7-bit trace ID. This sets the width of the TRCTRACEIDR.TRACEID field. All other values are reserved.The CoreSight ATB requires a 7-bit trace ID width."]
    #[inline(always)]
    pub fn traceidsize(&self) -> TraceidsizeR {
        TraceidsizeR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Indicates if the implementation can support ATB triggers: 0 The implementation does not support ATB triggers. 1 The implementation supports ATB triggers, and the TRCEVENTCTL1R.ATBTRIG field is implemented."]
    #[inline(always)]
    pub fn atbtrig(&self) -> AtbtrigR {
        AtbtrigR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Indicates if the implementation can support low-power state override: 0 The implementation does not support low-power state override. 1 The implementation supports low-power state override, and the TRCEVENTCTL1R.LPOVERRIDE field is implemented. The trace unit must support low-power state override if it can enter a low-power mode where the resources and event trace generation are disabled."]
    #[inline(always)]
    pub fn lpoverride(&self) -> LpoverrideR {
        LpoverrideR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr5_24_24(&self) -> Res0Trcidr5_24_24R {
        Res0Trcidr5_24_24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Indicates the number of sequencer states that are implemented. The permitted values are: 000 No sequencer states are implemented. 100 The implementation has four sequencer states. All other values are reserved."]
    #[inline(always)]
    pub fn numseqstate(&self) -> NumseqstateR {
        NumseqstateR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Indicates the number of counters that are available for tracing. The permitted values are: 000 No counters are available. 001 The implementation has one counter. 010 The implementation has two counters. 011 The implementation has three counters. 100 The implementation has four counters. All other values are reserved."]
    #[inline(always)]
    pub fn numcntr(&self) -> NumcntrR {
        NumcntrR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates if the reduced function counter is implemented: 0 The reduced function counter is not supported. 1 Counter 0 is implemented as a reduced function counter."]
    #[inline(always)]
    pub fn redfuncntr(&self) -> RedfuncntrR {
        RedfuncntrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Indicates how many external inputs are implemented. The permitted values are: 000000000 No external inputs are available. If NUMEXTIN is zero, NUMEXTINSEL must also be zero. 000000001 The implementation has one external input. 000000010 The implementation has two external inputs. and so on up to 0b100000000, which indicates that the implementation has 256 external inputs.All other values >0b100000000 are reserved.See TRCEXTINSELR for how to select an external input."]
    #[inline(always)]
    #[must_use]
    pub fn numextin(&mut self) -> NumextinW<ApbaddrEtmCpu1Trcidr5Spec> {
        NumextinW::new(self, 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Indicates how many external input select resources are implemented. The permitted values are: 000 No external input select resources are available. If NUMEXTINSEL is zero, NUMEXTIN must also be zero. 001 The implementation has one external input select resource. 010 The implementation has two external input select resources. 011 The implementation has three external input select resources. 100 The implementation has four external input select resources. All other values are reserved.See TRCEXTINSELR for how to select an input select resource."]
    #[inline(always)]
    #[must_use]
    pub fn numextinsel(&mut self) -> NumextinselW<ApbaddrEtmCpu1Trcidr5Spec> {
        NumextinselW::new(self, 9)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr5_15_12(&mut self) -> Res0Trcidr5_15_12W<ApbaddrEtmCpu1Trcidr5Spec> {
        Res0Trcidr5_15_12W::new(self, 12)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Indicates the trace ID width. The permitted value is: 111 The implementation supports a 7-bit trace ID. This sets the width of the TRCTRACEIDR.TRACEID field. All other values are reserved.The CoreSight ATB requires a 7-bit trace ID width."]
    #[inline(always)]
    #[must_use]
    pub fn traceidsize(&mut self) -> TraceidsizeW<ApbaddrEtmCpu1Trcidr5Spec> {
        TraceidsizeW::new(self, 16)
    }
    #[doc = "Bit 22 - 22:22\\]
Indicates if the implementation can support ATB triggers: 0 The implementation does not support ATB triggers. 1 The implementation supports ATB triggers, and the TRCEVENTCTL1R.ATBTRIG field is implemented."]
    #[inline(always)]
    #[must_use]
    pub fn atbtrig(&mut self) -> AtbtrigW<ApbaddrEtmCpu1Trcidr5Spec> {
        AtbtrigW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Indicates if the implementation can support low-power state override: 0 The implementation does not support low-power state override. 1 The implementation supports low-power state override, and the TRCEVENTCTL1R.LPOVERRIDE field is implemented. The trace unit must support low-power state override if it can enter a low-power mode where the resources and event trace generation are disabled."]
    #[inline(always)]
    #[must_use]
    pub fn lpoverride(&mut self) -> LpoverrideW<ApbaddrEtmCpu1Trcidr5Spec> {
        LpoverrideW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr5_24_24(&mut self) -> Res0Trcidr5_24_24W<ApbaddrEtmCpu1Trcidr5Spec> {
        Res0Trcidr5_24_24W::new(self, 24)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Indicates the number of sequencer states that are implemented. The permitted values are: 000 No sequencer states are implemented. 100 The implementation has four sequencer states. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numseqstate(&mut self) -> NumseqstateW<ApbaddrEtmCpu1Trcidr5Spec> {
        NumseqstateW::new(self, 25)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Indicates the number of counters that are available for tracing. The permitted values are: 000 No counters are available. 001 The implementation has one counter. 010 The implementation has two counters. 011 The implementation has three counters. 100 The implementation has four counters. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn numcntr(&mut self) -> NumcntrW<ApbaddrEtmCpu1Trcidr5Spec> {
        NumcntrW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates if the reduced function counter is implemented: 0 The reduced function counter is not supported. 1 Counter 0 is implemented as a reduced function counter."]
    #[inline(always)]
    #[must_use]
    pub fn redfuncntr(&mut self) -> RedfuncntrW<ApbaddrEtmCpu1Trcidr5Spec> {
        RedfuncntrW::new(self, 31)
    }
}
#[doc = "ID Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr5Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr5::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr5Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr5::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR5 to value 0x28c7_0830"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr5Spec {
    const RESET_VALUE: u32 = 0x28c7_0830;
}
