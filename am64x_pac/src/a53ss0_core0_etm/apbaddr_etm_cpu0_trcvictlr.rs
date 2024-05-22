#[doc = "Register `APBADDR_ETM_CPU0_TRCVICTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcvictlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCVICTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcvictlrSpec>;
#[doc = "Field `EVENT` reader - 7:0\\]
An event selector. \\[TODO: Add the bit assignments for EVENT fields into the descriptions directly?\\]"]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - 7:0\\]
An event selector. \\[TODO: Add the bit assignments for EVENT fields into the descriptions directly?\\]"]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCVICTLR_8_8` reader - 8:8\\]
Reserved, RES0."]
pub type Res0Trcvictlr8_8R = crate::BitReader;
#[doc = "Field `RES0_TRCVICTLR_8_8` writer - 8:8\\]
Reserved, RES0."]
pub type Res0Trcvictlr8_8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSTATUS` reader - 9:9\\]
IF TRCIDR4.NUMACPAIRS>0 or TRCIDR.NUMPC>0, this bit returns the status of the start-stop logic: 0 The start-stop logic is in the stopped state. 1 The start-stop logic is in the started state. The bit only returns stable data when TRCSTATR.PMSTABLE==1.Before software enables the trace unit, TRCPRGCTLR.EN==1, it must write to this bit to set the initial state of the start-stop logic. If the start-stop logic is not used then set this bit to 1. ARM recommends that the value of this bit is set before each trace run begins.If TRCIDR4.NUMACPAIRS==0 and TRCIDR4.NUMPC==0, this bit is RES0, indicating that the start-stop logic is not implemented."]
pub type SsstatusR = crate::BitReader;
#[doc = "Field `SSSTATUS` writer - 9:9\\]
IF TRCIDR4.NUMACPAIRS>0 or TRCIDR.NUMPC>0, this bit returns the status of the start-stop logic: 0 The start-stop logic is in the stopped state. 1 The start-stop logic is in the started state. The bit only returns stable data when TRCSTATR.PMSTABLE==1.Before software enables the trace unit, TRCPRGCTLR.EN==1, it must write to this bit to set the initial state of the start-stop logic. If the start-stop logic is not used then set this bit to 1. ARM recommends that the value of this bit is set before each trace run begins.If TRCIDR4.NUMACPAIRS==0 and TRCIDR4.NUMPC==0, this bit is RES0, indicating that the start-stop logic is not implemented."]
pub type SsstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCRESET` reader - 10:10\\]
Controls whether a trace unit must trace a Reset exception: 0 The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception. 1 The trace unit always traces a Reset exception."]
pub type TrcresetR = crate::BitReader;
#[doc = "Field `TRCRESET` writer - 10:10\\]
Controls whether a trace unit must trace a Reset exception: 0 The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception. 1 The trace unit always traces a Reset exception."]
pub type TrcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCERR` reader - 11:11\\]
If TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a system error exception: 0 The trace unit does not trace a system error exception unless it traces the exception or instruction immediately prior to the system error exception. 1 The trace unit always traces a system error exception. If TRCIDR3.TRCERR==0, this bit is RES0."]
pub type TrcerrR = crate::BitReader;
#[doc = "Field `TRCERR` writer - 11:11\\]
If TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a system error exception: 0 The trace unit does not trace a system error exception unless it traces the exception or instruction immediately prior to the system error exception. 1 The trace unit always traces a system error exception. If TRCIDR3.TRCERR==0, this bit is RES0."]
pub type TrcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCVICTLR_15_12` reader - 15:12\\]
Reserved, RES0."]
pub type Res0Trcvictlr15_12R = crate::FieldReader;
#[doc = "Field `RES0_TRCVICTLR_15_12` writer - 15:12\\]
Reserved, RES0."]
pub type Res0Trcvictlr15_12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXLEVEL_S` reader - 19:16\\]
In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Secure state, for exception level n. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
pub type ExlevelSR = crate::FieldReader;
#[doc = "Field `EXLEVEL_S` writer - 19:16\\]
In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Secure state, for exception level n. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
pub type ExlevelSW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXLEVEL_NS` reader - 23:20\\]
In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Non-secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Non-secure state, for exception level n. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
pub type ExlevelNsR = crate::FieldReader;
#[doc = "Field `EXLEVEL_NS` writer - 23:20\\]
In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Non-secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Non-secure state, for exception level n. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
pub type ExlevelNsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCVICTLR_31_24` reader - 31:24\\]
Reserved, RES0."]
pub type Res0Trcvictlr31_24R = crate::FieldReader;
#[doc = "Field `RES0_TRCVICTLR_31_24` writer - 31:24\\]
Reserved, RES0."]
pub type Res0Trcvictlr31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
An event selector. \\[TODO: Add the bit assignments for EVENT fields into the descriptions directly?\\]"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcvictlr_8_8(&self) -> Res0Trcvictlr8_8R {
        Res0Trcvictlr8_8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
IF TRCIDR4.NUMACPAIRS>0 or TRCIDR.NUMPC>0, this bit returns the status of the start-stop logic: 0 The start-stop logic is in the stopped state. 1 The start-stop logic is in the started state. The bit only returns stable data when TRCSTATR.PMSTABLE==1.Before software enables the trace unit, TRCPRGCTLR.EN==1, it must write to this bit to set the initial state of the start-stop logic. If the start-stop logic is not used then set this bit to 1. ARM recommends that the value of this bit is set before each trace run begins.If TRCIDR4.NUMACPAIRS==0 and TRCIDR4.NUMPC==0, this bit is RES0, indicating that the start-stop logic is not implemented."]
    #[inline(always)]
    pub fn ssstatus(&self) -> SsstatusR {
        SsstatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Controls whether a trace unit must trace a Reset exception: 0 The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception. 1 The trace unit always traces a Reset exception."]
    #[inline(always)]
    pub fn trcreset(&self) -> TrcresetR {
        TrcresetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
If TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a system error exception: 0 The trace unit does not trace a system error exception unless it traces the exception or instruction immediately prior to the system error exception. 1 The trace unit always traces a system error exception. If TRCIDR3.TRCERR==0, this bit is RES0."]
    #[inline(always)]
    pub fn trcerr(&self) -> TrcerrR {
        TrcerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcvictlr_15_12(&self) -> Res0Trcvictlr15_12R {
        Res0Trcvictlr15_12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Secure state, for exception level n. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn exlevel_s(&self) -> ExlevelSR {
        ExlevelSR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Non-secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Non-secure state, for exception level n. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn exlevel_ns(&self) -> ExlevelNsR {
        ExlevelNsR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcvictlr_31_24(&self) -> Res0Trcvictlr31_24R {
        Res0Trcvictlr31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
An event selector. \\[TODO: Add the bit assignments for EVENT fields into the descriptions directly?\\]"]
    #[inline(always)]
    #[must_use]
    pub fn event(&mut self) -> EventW<ApbaddrEtmCpu0TrcvictlrSpec> {
        EventW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcvictlr_8_8(&mut self) -> Res0Trcvictlr8_8W<ApbaddrEtmCpu0TrcvictlrSpec> {
        Res0Trcvictlr8_8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
IF TRCIDR4.NUMACPAIRS>0 or TRCIDR.NUMPC>0, this bit returns the status of the start-stop logic: 0 The start-stop logic is in the stopped state. 1 The start-stop logic is in the started state. The bit only returns stable data when TRCSTATR.PMSTABLE==1.Before software enables the trace unit, TRCPRGCTLR.EN==1, it must write to this bit to set the initial state of the start-stop logic. If the start-stop logic is not used then set this bit to 1. ARM recommends that the value of this bit is set before each trace run begins.If TRCIDR4.NUMACPAIRS==0 and TRCIDR4.NUMPC==0, this bit is RES0, indicating that the start-stop logic is not implemented."]
    #[inline(always)]
    #[must_use]
    pub fn ssstatus(&mut self) -> SsstatusW<ApbaddrEtmCpu0TrcvictlrSpec> {
        SsstatusW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Controls whether a trace unit must trace a Reset exception: 0 The trace unit does not trace a Reset exception unless it traces the exception or instruction immediately prior to the Reset exception. 1 The trace unit always traces a Reset exception."]
    #[inline(always)]
    #[must_use]
    pub fn trcreset(&mut self) -> TrcresetW<ApbaddrEtmCpu0TrcvictlrSpec> {
        TrcresetW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
If TRCIDR3.TRCERR==1, this bit controls whether a trace unit must trace a system error exception: 0 The trace unit does not trace a system error exception unless it traces the exception or instruction immediately prior to the system error exception. 1 The trace unit always traces a system error exception. If TRCIDR3.TRCERR==0, this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn trcerr(&mut self) -> TrcerrW<ApbaddrEtmCpu0TrcvictlrSpec> {
        TrcerrW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcvictlr_15_12(&mut self) -> Res0Trcvictlr15_12W<ApbaddrEtmCpu0TrcvictlrSpec> {
        Res0Trcvictlr15_12W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
In Secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Secure state, for exception level n. The exception levels are:Bit\\[16\\]Exception level 0.Bit\\[17\\]Exception level 1.Bit\\[18\\]RAZ/WI. EXLEVEL_S\\[2\\]
is never implemented.Bit\\[19\\]Exception level 3.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_S. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_s(&mut self) -> ExlevelSW<ApbaddrEtmCpu0TrcvictlrSpec> {
        ExlevelSW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
In Non-secure state, each bit controls whether instruction tracing is enabled for the corresponding exception level: 0 The trace unit generates instruction trace, in Non-secure state, for exception level n. 1 The trace unit does not generate instruction trace, in Non-secure state, for exception level n. The exception levels are:Bit\\[20\\]Exception level 0.Bit\\[21\\]Exception level 1.Bit\\[22\\]Exception level 2.Bit\\[23\\]RAZ/WI. EXLEVEL_NS\\[3\\]
is never implemented.The content of the field is IMPLEMENTATION DEFINED and is set by the value of TRCIDR3.EXLEVEL_NS. If instruction tracing is not implemented for a given exception level, the corresponding bit in this field is not implemented. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn exlevel_ns(&mut self) -> ExlevelNsW<ApbaddrEtmCpu0TrcvictlrSpec> {
        ExlevelNsW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcvictlr_31_24(&mut self) -> Res0Trcvictlr31_24W<ApbaddrEtmCpu0TrcvictlrSpec> {
        Res0Trcvictlr31_24W::new(self, 24)
    }
}
#[doc = "ViewInst Main Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcvictlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcvictlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcvictlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcvictlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcvictlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcvictlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcvictlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcvictlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCVICTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcvictlrSpec {
    const RESET_VALUE: u32 = 0;
}
