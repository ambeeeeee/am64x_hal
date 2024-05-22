#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR3` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr3Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR3` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr3Spec>;
#[doc = "Field `CCITMIN` reader - 11:0\\]
Indicates the minimum value that can be programmed in TRCCCCTLR.THRESHOLD.When cycle counting in the instruction trace is supported, that is TRCIDR0.TRCCCI==1, then the minimum value of this field is 0x001, otherwise it is 0x000."]
pub type CcitminR = crate::FieldReader<u16>;
#[doc = "Field `CCITMIN` writer - 11:0\\]
Indicates the minimum value that can be programmed in TRCCCCTLR.THRESHOLD.When cycle counting in the instruction trace is supported, that is TRCIDR0.TRCCCI==1, then the minimum value of this field is 0x001, otherwise it is 0x000."]
pub type CcitminW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RES0_TRCIDR3_15_12` reader - 15:12\\]
Reserved, RES0."]
pub type Res0Trcidr3_15_12R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR3_15_12` writer - 15:12\\]
Reserved, RES0."]
pub type Res0Trcidr3_15_12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXLEVEL_S` reader - 19:16\\]
In Secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are not supported. 1 In Secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are supported. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]SBZ. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3."]
pub type ExlevelSR = crate::FieldReader;
#[doc = "Field `EXLEVEL_S` writer - 19:16\\]
In Secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are not supported. 1 In Secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are supported. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]SBZ. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3."]
pub type ExlevelSW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXLEVEL_NS` reader - 23:20\\]
In Non-secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Non-secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are not supported. 1 In Non-secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are supported. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]SBZ. EXLEVEL_NS\\[3\\]
is never implemented."]
pub type ExlevelNsR = crate::FieldReader;
#[doc = "Field `EXLEVEL_NS` writer - 23:20\\]
In Non-secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Non-secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are not supported. 1 In Non-secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are supported. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]SBZ. EXLEVEL_NS\\[3\\]
is never implemented."]
pub type ExlevelNsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCERR` reader - 24:24\\]
Indicates if TRCVICTLR.TRCERR is supported: 0 TRCVICTLR.TRCERR is not supported 1 TRCVICTLR.TRCERR is supported."]
pub type TrcerrR = crate::BitReader;
#[doc = "Field `TRCERR` writer - 24:24\\]
Indicates if TRCVICTLR.TRCERR is supported: 0 TRCVICTLR.TRCERR is not supported 1 TRCVICTLR.TRCERR is supported."]
pub type TrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPR` reader - 25:25\\]
Indicates if an implementation has a fixed synchronization period: 0 TRCSYNCPR is read-write so software can change the synchronization period. 1 TRCSYNCPR is read-only so the synchronization period is fixed."]
pub type SyncprR = crate::BitReader;
#[doc = "Field `SYNCPR` writer - 25:25\\]
Indicates if an implementation has a fixed synchronization period: 0 TRCSYNCPR is read-write so software can change the synchronization period. 1 TRCSYNCPR is read-only so the synchronization period is fixed."]
pub type SyncprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLCTL` reader - 26:26\\]
Indicates if TRCSTALLCTLR is supported: 0 TRCSTALLCTLR is not supported. 1 TRCSTALLCTLR is supported."]
pub type StallctlR = crate::BitReader;
#[doc = "Field `STALLCTL` writer - 26:26\\]
Indicates if TRCSTALLCTLR is supported: 0 TRCSTALLCTLR is not supported. 1 TRCSTALLCTLR is supported."]
pub type StallctlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSSTALL` reader - 27:27\\]
Indicates if the implementation can support stall control: 0 The system does not support stall control of the processor. 1 The system can support stall control of the processor. The system supports stalling of the processor only when SYSSTALL==1 and STALLCTL==1."]
pub type SysstallR = crate::BitReader;
#[doc = "Field `SYSSTALL` writer - 27:27\\]
Indicates if the implementation can support stall control: 0 The system does not support stall control of the processor. 1 The system can support stall control of the processor. The system supports stalling of the processor only when SYSSTALL==1 and STALLCTL==1."]
pub type SysstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMPROC` reader - 30:28\\]
Indicates the number of processors available for tracing. The possible values are: 000 The trace unit can trace one processor. 001 The trace unit can trace two processors. 010 The trace unit can trace three processors. and so on up to 0b111, which indicates the trace unit can trace eight processors.This field sets the maximum value of TRCPROCSELR.PROCSEL."]
pub type NumprocR = crate::FieldReader;
#[doc = "Field `NUMPROC` writer - 30:28\\]
Indicates the number of processors available for tracing. The possible values are: 000 The trace unit can trace one processor. 001 The trace unit can trace two processors. 010 The trace unit can trace three processors. and so on up to 0b111, which indicates the trace unit can trace eight processors.This field sets the maximum value of TRCPROCSELR.PROCSEL."]
pub type NumprocW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOOVERFLOW` reader - 31:31\\]
Indicates if TRCSTALLCTLR.NOOVERFLOW is supported: 0 TRCSTALLCTLR.NOOVERFLOW is not supported, or STALLCTL==0. 1 TRCSTALLCTLR.NOOVERFLOW is supported."]
pub type NooverflowR = crate::BitReader;
#[doc = "Field `NOOVERFLOW` writer - 31:31\\]
Indicates if TRCSTALLCTLR.NOOVERFLOW is supported: 0 TRCSTALLCTLR.NOOVERFLOW is not supported, or STALLCTL==0. 1 TRCSTALLCTLR.NOOVERFLOW is supported."]
pub type NooverflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the minimum value that can be programmed in TRCCCCTLR.THRESHOLD.When cycle counting in the instruction trace is supported, that is TRCIDR0.TRCCCI==1, then the minimum value of this field is 0x001, otherwise it is 0x000."]
    #[inline(always)]
    pub fn ccitmin(&self) -> CcitminR {
        CcitminR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr3_15_12(&self) -> Res0Trcidr3_15_12R {
        Res0Trcidr3_15_12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
In Secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are not supported. 1 In Secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are supported. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]SBZ. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3."]
    #[inline(always)]
    pub fn exlevel_s(&self) -> ExlevelSR {
        ExlevelSR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
In Non-secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Non-secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are not supported. 1 In Non-secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are supported. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]SBZ. EXLEVEL_NS\\[3\\]
is never implemented."]
    #[inline(always)]
    pub fn exlevel_ns(&self) -> ExlevelNsR {
        ExlevelNsR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if TRCVICTLR.TRCERR is supported: 0 TRCVICTLR.TRCERR is not supported 1 TRCVICTLR.TRCERR is supported."]
    #[inline(always)]
    pub fn trcerr(&self) -> TrcerrR {
        TrcerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates if an implementation has a fixed synchronization period: 0 TRCSYNCPR is read-write so software can change the synchronization period. 1 TRCSYNCPR is read-only so the synchronization period is fixed."]
    #[inline(always)]
    pub fn syncpr(&self) -> SyncprR {
        SyncprR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates if TRCSTALLCTLR is supported: 0 TRCSTALLCTLR is not supported. 1 TRCSTALLCTLR is supported."]
    #[inline(always)]
    pub fn stallctl(&self) -> StallctlR {
        StallctlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Indicates if the implementation can support stall control: 0 The system does not support stall control of the processor. 1 The system can support stall control of the processor. The system supports stalling of the processor only when SYSSTALL==1 and STALLCTL==1."]
    #[inline(always)]
    pub fn sysstall(&self) -> SysstallR {
        SysstallR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Indicates the number of processors available for tracing. The possible values are: 000 The trace unit can trace one processor. 001 The trace unit can trace two processors. 010 The trace unit can trace three processors. and so on up to 0b111, which indicates the trace unit can trace eight processors.This field sets the maximum value of TRCPROCSELR.PROCSEL."]
    #[inline(always)]
    pub fn numproc(&self) -> NumprocR {
        NumprocR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates if TRCSTALLCTLR.NOOVERFLOW is supported: 0 TRCSTALLCTLR.NOOVERFLOW is not supported, or STALLCTL==0. 1 TRCSTALLCTLR.NOOVERFLOW is supported."]
    #[inline(always)]
    pub fn nooverflow(&self) -> NooverflowR {
        NooverflowR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the minimum value that can be programmed in TRCCCCTLR.THRESHOLD.When cycle counting in the instruction trace is supported, that is TRCIDR0.TRCCCI==1, then the minimum value of this field is 0x001, otherwise it is 0x000."]
    #[inline(always)]
    #[must_use]
    pub fn ccitmin(&mut self) -> CcitminW<ApbaddrEtmCpu1Trcidr3Spec> {
        CcitminW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr3_15_12(&mut self) -> Res0Trcidr3_15_12W<ApbaddrEtmCpu1Trcidr3Spec> {
        Res0Trcidr3_15_12W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
In Secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are not supported. 1 In Secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_S and TRCVICTLR.EXLEVEL_S are supported. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]SBZ. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_s(&mut self) -> ExlevelSW<ApbaddrEtmCpu1Trcidr3Spec> {
        ExlevelSW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
In Non-secure state, each bit indicates whether instruction tracing is supported for the corresponding exception level: 0 In Non-secure state, exception level n is not supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are not supported. 1 In Non-secure state, exception level n is supported so the corresponding bits in TRCACATRn.EXLEVEL_NS and TRCVICTLR.EXLEVEL_NS are supported. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]SBZ. EXLEVEL_NS\\[3\\]
is never implemented."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_ns(&mut self) -> ExlevelNsW<ApbaddrEtmCpu1Trcidr3Spec> {
        ExlevelNsW::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Indicates if TRCVICTLR.TRCERR is supported: 0 TRCVICTLR.TRCERR is not supported 1 TRCVICTLR.TRCERR is supported."]
    #[inline(always)]
    #[must_use]
    pub fn trcerr(&mut self) -> TrcerrW<ApbaddrEtmCpu1Trcidr3Spec> {
        TrcerrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Indicates if an implementation has a fixed synchronization period: 0 TRCSYNCPR is read-write so software can change the synchronization period. 1 TRCSYNCPR is read-only so the synchronization period is fixed."]
    #[inline(always)]
    #[must_use]
    pub fn syncpr(&mut self) -> SyncprW<ApbaddrEtmCpu1Trcidr3Spec> {
        SyncprW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Indicates if TRCSTALLCTLR is supported: 0 TRCSTALLCTLR is not supported. 1 TRCSTALLCTLR is supported."]
    #[inline(always)]
    #[must_use]
    pub fn stallctl(&mut self) -> StallctlW<ApbaddrEtmCpu1Trcidr3Spec> {
        StallctlW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Indicates if the implementation can support stall control: 0 The system does not support stall control of the processor. 1 The system can support stall control of the processor. The system supports stalling of the processor only when SYSSTALL==1 and STALLCTL==1."]
    #[inline(always)]
    #[must_use]
    pub fn sysstall(&mut self) -> SysstallW<ApbaddrEtmCpu1Trcidr3Spec> {
        SysstallW::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Indicates the number of processors available for tracing. The possible values are: 000 The trace unit can trace one processor. 001 The trace unit can trace two processors. 010 The trace unit can trace three processors. and so on up to 0b111, which indicates the trace unit can trace eight processors.This field sets the maximum value of TRCPROCSELR.PROCSEL."]
    #[inline(always)]
    #[must_use]
    pub fn numproc(&mut self) -> NumprocW<ApbaddrEtmCpu1Trcidr3Spec> {
        NumprocW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Indicates if TRCSTALLCTLR.NOOVERFLOW is supported: 0 TRCSTALLCTLR.NOOVERFLOW is not supported, or STALLCTL==0. 1 TRCSTALLCTLR.NOOVERFLOW is supported."]
    #[inline(always)]
    #[must_use]
    pub fn nooverflow(&mut self) -> NooverflowW<ApbaddrEtmCpu1Trcidr3Spec> {
        NooverflowW::new(self, 31)
    }
}
#[doc = "ID Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr3Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr3::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr3::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR3 to value 0x0d71_0004"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr3Spec {
    const RESET_VALUE: u32 = 0x0d71_0004;
}
