#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcidr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIDR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcidr0Spec>;
#[doc = "Field `RES0_TRCIDR0_0_0` reader - 0:0\\]
Reserved, RES0."]
pub type Res0Trcidr0_0_0R = crate::BitReader;
#[doc = "Field `RES0_TRCIDR0_0_0` writer - 0:0\\]
Reserved, RES0."]
pub type Res0Trcidr0_0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTP0` reader - 2:1\\]
P0 tracing support field. The permitted values are: 00 Tracing of load and store instructions as P0 elements is not supported. 11 Tracing of load and store instructions as P0 elements is supported, so TRCCONFIGR.INSTP0 is supported. All other values are reserved."]
pub type Instp0R = crate::FieldReader;
#[doc = "Field `INSTP0` writer - 2:1\\]
P0 tracing support field. The permitted values are: 00 Tracing of load and store instructions as P0 elements is not supported. 11 Tracing of load and store instructions as P0 elements is supported, so TRCCONFIGR.INSTP0 is supported. All other values are reserved."]
pub type Instp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRCDATA` reader - 4:3\\]
Conditional tracing field. The permitted values are: 00 Data tracing is not supported. 11 Tracing of data addresses and data values is supported, so TRCCONFIGR.DA, TRCCONFIGR.DV, TRCSTALLCTLR.DATADISCARD, TRCSTALLCTLR.INSTPRIORITY, TRCSTALLCTLR.DSTALL, and TRCEVENTCTL1R.DATAEN are supported. All other values are reserved."]
pub type TrcdataR = crate::FieldReader;
#[doc = "Field `TRCDATA` writer - 4:3\\]
Conditional tracing field. The permitted values are: 00 Data tracing is not supported. 11 Tracing of data addresses and data values is supported, so TRCCONFIGR.DA, TRCCONFIGR.DV, TRCSTALLCTLR.DATADISCARD, TRCSTALLCTLR.INSTPRIORITY, TRCSTALLCTLR.DSTALL, and TRCEVENTCTL1R.DATAEN are supported. All other values are reserved."]
pub type TrcdataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRCBB` reader - 5:5\\]
Branch broadcast tracing support bit. Indicates if the trace unit supports branch broadcast tracing: 0 Branch broadcast tracing is not supported. 1 Branch broadcast tracing is supported, so TRCCONFIGR.BB and TRCBBCTLR are supported."]
pub type TrcbbR = crate::BitReader;
#[doc = "Field `TRCBB` writer - 5:5\\]
Branch broadcast tracing support bit. Indicates if the trace unit supports branch broadcast tracing: 0 Branch broadcast tracing is not supported. 1 Branch broadcast tracing is supported, so TRCCONFIGR.BB and TRCBBCTLR are supported."]
pub type TrcbbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCCOND` reader - 6:6\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
pub type TrccondR = crate::BitReader;
#[doc = "Field `TRCCOND` writer - 6:6\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
pub type TrccondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCCCI` reader - 7:7\\]
Cycle counting instruction bit. Indicates if the trace unit supports cycle counting for instructions: 0 Cycle counting in the instruction trace is not implemented. 1 Cycle counting in the instruction trace is implemented, so TRCCONFIGR.CCI and TRCCCCTLR are supported."]
pub type TrccciR = crate::BitReader;
#[doc = "Field `TRCCCI` writer - 7:7\\]
Cycle counting instruction bit. Indicates if the trace unit supports cycle counting for instructions: 0 Cycle counting in the instruction trace is not implemented. 1 Cycle counting in the instruction trace is implemented, so TRCCONFIGR.CCI and TRCCCCTLR are supported."]
pub type TrccciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCIDR0_8_8` reader - 8:8\\]
Reserved, RES0."]
pub type Res0Trcidr0_8_8R = crate::BitReader;
#[doc = "Field `RES0_TRCIDR0_8_8` writer - 8:8\\]
Reserved, RES0."]
pub type Res0Trcidr0_8_8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETSTACK` reader - 9:9\\]
Return stack bit. Indicates if the implementation supports a return stack: 0 Return stack is not implemented. 1 Return stack is implemented, so TRCCONFIGR.RS is supported."]
pub type RetstackR = crate::BitReader;
#[doc = "Field `RETSTACK` writer - 9:9\\]
Return stack bit. Indicates if the implementation supports a return stack: 0 Return stack is not implemented. 1 Return stack is implemented, so TRCCONFIGR.RS is supported."]
pub type RetstackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMEVENT` reader - 11:10\\]
Number of events field. Indicates how many events the trace unit supports: 00 The trace unit supports 1 event. 01 The trace unit supports 2 events. 10 The trace unit supports 3 events. 11 The trace unit supports 4 events. This field controls how many fields are supported in TRCEVENTCTL0R, and indicates the size of TRCEVENTCTL1R.INSTEN."]
pub type NumeventR = crate::FieldReader;
#[doc = "Field `NUMEVENT` writer - 11:10\\]
Number of events field. Indicates how many events the trace unit supports: 00 The trace unit supports 1 event. 01 The trace unit supports 2 events. 10 The trace unit supports 3 events. 11 The trace unit supports 4 events. This field controls how many fields are supported in TRCEVENTCTL0R, and indicates the size of TRCEVENTCTL1R.INSTEN."]
pub type NumeventW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONDTYPE` reader - 13:12\\]
Conditional tracing field. The permitted values are: 00 The trace unit indicates only if a conditional instruction is a pass or fail. 01 The trace unit provides the Current Program Status Register \\[CPSR\\]
status, for a conditional instruction. All other values are reserved."]
pub type CondtypeR = crate::FieldReader;
#[doc = "Field `CONDTYPE` writer - 13:12\\]
Conditional tracing field. The permitted values are: 00 The trace unit indicates only if a conditional instruction is a pass or fail. 01 The trace unit provides the Current Program Status Register \\[CPSR\\]
status, for a conditional instruction. All other values are reserved."]
pub type CondtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QFILT` reader - 14:14\\]
Q element filtering support field. The permitted values are: 0 Q element filtering is not implemented. 1 Q element filtering is implemented. TRCQCTLR is implemented. When QSUPP==0b00, this field is RES0."]
pub type QfiltR = crate::BitReader;
#[doc = "Field `QFILT` writer - 14:14\\]
Q element filtering support field. The permitted values are: 0 Q element filtering is not implemented. 1 Q element filtering is implemented. TRCQCTLR is implemented. When QSUPP==0b00, this field is RES0."]
pub type QfiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSUPP` reader - 16:15\\]
Q element support field. The permitted values are: 00 Q element support is not implemented. TRCCONFIGR.QE is RES0. 01 Q element support is implemented, and only supports Q elements with instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b01. 10 Q element support is implemented, and only supports Q elements without instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b11. 11 Q element support is implemented, and supports both Q elements with instruction counts and Q elements without instruction counts. TRCCONFIGR.QE is fully implemented."]
pub type QsuppR = crate::FieldReader;
#[doc = "Field `QSUPP` writer - 16:15\\]
Q element support field. The permitted values are: 00 Q element support is not implemented. TRCCONFIGR.QE is RES0. 01 Q element support is implemented, and only supports Q elements with instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b01. 10 Q element support is implemented, and only supports Q elements without instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b11. 11 Q element support is implemented, and supports both Q elements with instruction counts and Q elements without instruction counts. TRCCONFIGR.QE is fully implemented."]
pub type QsuppW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_TRCIDR0_23_17` reader - 23:17\\]
Reserved, RES0."]
pub type Res0Trcidr0_23_17R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR0_23_17` writer - 23:17\\]
Reserved, RES0."]
pub type Res0Trcidr0_23_17W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TSSIZE` reader - 28:24\\]
Global timestamp size field. The permitted values are: 00000 Global timestamping is not implemented. 00110 Implementation supports a maximum global timestamp of 48bits. 01000 Implementation supports a maximum global timestamp of 64bits. All other values are reserved.When global timestamping is implemented then TRCCONFIGR.TS and TRCTSCTLR are supported."]
pub type TssizeR = crate::FieldReader;
#[doc = "Field `TSSIZE` writer - 28:24\\]
Global timestamp size field. The permitted values are: 00000 Global timestamping is not implemented. 00110 Implementation supports a maximum global timestamp of 48bits. 01000 Implementation supports a maximum global timestamp of 64bits. All other values are reserved.When global timestamping is implemented then TRCCONFIGR.TS and TRCTSCTLR are supported."]
pub type TssizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMMOPT` reader - 29:29\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
pub type CommoptR = crate::BitReader;
#[doc = "Field `COMMOPT` writer - 29:29\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
pub type CommoptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCIDR0_31_30` reader - 31:30\\]
Reserved, RES0."]
pub type Res0Trcidr0_31_30R = crate::FieldReader;
#[doc = "Field `RES0_TRCIDR0_31_30` writer - 31:30\\]
Reserved, RES0."]
pub type Res0Trcidr0_31_30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr0_0_0(&self) -> Res0Trcidr0_0_0R {
        Res0Trcidr0_0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
P0 tracing support field. The permitted values are: 00 Tracing of load and store instructions as P0 elements is not supported. 11 Tracing of load and store instructions as P0 elements is supported, so TRCCONFIGR.INSTP0 is supported. All other values are reserved."]
    #[inline(always)]
    pub fn instp0(&self) -> Instp0R {
        Instp0R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Conditional tracing field. The permitted values are: 00 Data tracing is not supported. 11 Tracing of data addresses and data values is supported, so TRCCONFIGR.DA, TRCCONFIGR.DV, TRCSTALLCTLR.DATADISCARD, TRCSTALLCTLR.INSTPRIORITY, TRCSTALLCTLR.DSTALL, and TRCEVENTCTL1R.DATAEN are supported. All other values are reserved."]
    #[inline(always)]
    pub fn trcdata(&self) -> TrcdataR {
        TrcdataR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Branch broadcast tracing support bit. Indicates if the trace unit supports branch broadcast tracing: 0 Branch broadcast tracing is not supported. 1 Branch broadcast tracing is supported, so TRCCONFIGR.BB and TRCBBCTLR are supported."]
    #[inline(always)]
    pub fn trcbb(&self) -> TrcbbR {
        TrcbbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
    #[inline(always)]
    pub fn trccond(&self) -> TrccondR {
        TrccondR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Cycle counting instruction bit. Indicates if the trace unit supports cycle counting for instructions: 0 Cycle counting in the instruction trace is not implemented. 1 Cycle counting in the instruction trace is implemented, so TRCCONFIGR.CCI and TRCCCCTLR are supported."]
    #[inline(always)]
    pub fn trccci(&self) -> TrccciR {
        TrccciR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr0_8_8(&self) -> Res0Trcidr0_8_8R {
        Res0Trcidr0_8_8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Return stack bit. Indicates if the implementation supports a return stack: 0 Return stack is not implemented. 1 Return stack is implemented, so TRCCONFIGR.RS is supported."]
    #[inline(always)]
    pub fn retstack(&self) -> RetstackR {
        RetstackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Number of events field. Indicates how many events the trace unit supports: 00 The trace unit supports 1 event. 01 The trace unit supports 2 events. 10 The trace unit supports 3 events. 11 The trace unit supports 4 events. This field controls how many fields are supported in TRCEVENTCTL0R, and indicates the size of TRCEVENTCTL1R.INSTEN."]
    #[inline(always)]
    pub fn numevent(&self) -> NumeventR {
        NumeventR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Conditional tracing field. The permitted values are: 00 The trace unit indicates only if a conditional instruction is a pass or fail. 01 The trace unit provides the Current Program Status Register \\[CPSR\\]
status, for a conditional instruction. All other values are reserved."]
    #[inline(always)]
    pub fn condtype(&self) -> CondtypeR {
        CondtypeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Q element filtering support field. The permitted values are: 0 Q element filtering is not implemented. 1 Q element filtering is implemented. TRCQCTLR is implemented. When QSUPP==0b00, this field is RES0."]
    #[inline(always)]
    pub fn qfilt(&self) -> QfiltR {
        QfiltR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - 16:15\\]
Q element support field. The permitted values are: 00 Q element support is not implemented. TRCCONFIGR.QE is RES0. 01 Q element support is implemented, and only supports Q elements with instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b01. 10 Q element support is implemented, and only supports Q elements without instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b11. 11 Q element support is implemented, and supports both Q elements with instruction counts and Q elements without instruction counts. TRCCONFIGR.QE is fully implemented."]
    #[inline(always)]
    pub fn qsupp(&self) -> QsuppR {
        QsuppR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr0_23_17(&self) -> Res0Trcidr0_23_17R {
        Res0Trcidr0_23_17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Global timestamp size field. The permitted values are: 00000 Global timestamping is not implemented. 00110 Implementation supports a maximum global timestamp of 48bits. 01000 Implementation supports a maximum global timestamp of 64bits. All other values are reserved.When global timestamping is implemented then TRCCONFIGR.TS and TRCTSCTLR are supported."]
    #[inline(always)]
    pub fn tssize(&self) -> TssizeR {
        TssizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
    #[inline(always)]
    pub fn commopt(&self) -> CommoptR {
        CommoptR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcidr0_31_30(&self) -> Res0Trcidr0_31_30R {
        Res0Trcidr0_31_30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr0_0_0(&mut self) -> Res0Trcidr0_0_0W<ApbaddrEtmCpu1Trcidr0Spec> {
        Res0Trcidr0_0_0W::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
P0 tracing support field. The permitted values are: 00 Tracing of load and store instructions as P0 elements is not supported. 11 Tracing of load and store instructions as P0 elements is supported, so TRCCONFIGR.INSTP0 is supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn instp0(&mut self) -> Instp0W<ApbaddrEtmCpu1Trcidr0Spec> {
        Instp0W::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Conditional tracing field. The permitted values are: 00 Data tracing is not supported. 11 Tracing of data addresses and data values is supported, so TRCCONFIGR.DA, TRCCONFIGR.DV, TRCSTALLCTLR.DATADISCARD, TRCSTALLCTLR.INSTPRIORITY, TRCSTALLCTLR.DSTALL, and TRCEVENTCTL1R.DATAEN are supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn trcdata(&mut self) -> TrcdataW<ApbaddrEtmCpu1Trcidr0Spec> {
        TrcdataW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
Branch broadcast tracing support bit. Indicates if the trace unit supports branch broadcast tracing: 0 Branch broadcast tracing is not supported. 1 Branch broadcast tracing is supported, so TRCCONFIGR.BB and TRCBBCTLR are supported."]
    #[inline(always)]
    #[must_use]
    pub fn trcbb(&mut self) -> TrcbbW<ApbaddrEtmCpu1Trcidr0Spec> {
        TrcbbW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
    #[inline(always)]
    #[must_use]
    pub fn trccond(&mut self) -> TrccondW<ApbaddrEtmCpu1Trcidr0Spec> {
        TrccondW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Cycle counting instruction bit. Indicates if the trace unit supports cycle counting for instructions: 0 Cycle counting in the instruction trace is not implemented. 1 Cycle counting in the instruction trace is implemented, so TRCCONFIGR.CCI and TRCCCCTLR are supported."]
    #[inline(always)]
    #[must_use]
    pub fn trccci(&mut self) -> TrccciW<ApbaddrEtmCpu1Trcidr0Spec> {
        TrccciW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr0_8_8(&mut self) -> Res0Trcidr0_8_8W<ApbaddrEtmCpu1Trcidr0Spec> {
        Res0Trcidr0_8_8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Return stack bit. Indicates if the implementation supports a return stack: 0 Return stack is not implemented. 1 Return stack is implemented, so TRCCONFIGR.RS is supported."]
    #[inline(always)]
    #[must_use]
    pub fn retstack(&mut self) -> RetstackW<ApbaddrEtmCpu1Trcidr0Spec> {
        RetstackW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Number of events field. Indicates how many events the trace unit supports: 00 The trace unit supports 1 event. 01 The trace unit supports 2 events. 10 The trace unit supports 3 events. 11 The trace unit supports 4 events. This field controls how many fields are supported in TRCEVENTCTL0R, and indicates the size of TRCEVENTCTL1R.INSTEN."]
    #[inline(always)]
    #[must_use]
    pub fn numevent(&mut self) -> NumeventW<ApbaddrEtmCpu1Trcidr0Spec> {
        NumeventW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Conditional tracing field. The permitted values are: 00 The trace unit indicates only if a conditional instruction is a pass or fail. 01 The trace unit provides the Current Program Status Register \\[CPSR\\]
status, for a conditional instruction. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn condtype(&mut self) -> CondtypeW<ApbaddrEtmCpu1Trcidr0Spec> {
        CondtypeW::new(self, 12)
    }
    #[doc = "Bit 14 - 14:14\\]
Q element filtering support field. The permitted values are: 0 Q element filtering is not implemented. 1 Q element filtering is implemented. TRCQCTLR is implemented. When QSUPP==0b00, this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn qfilt(&mut self) -> QfiltW<ApbaddrEtmCpu1Trcidr0Spec> {
        QfiltW::new(self, 14)
    }
    #[doc = "Bits 15:16 - 16:15\\]
Q element support field. The permitted values are: 00 Q element support is not implemented. TRCCONFIGR.QE is RES0. 01 Q element support is implemented, and only supports Q elements with instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b01. 10 Q element support is implemented, and only supports Q elements without instruction counts. TRCCONFIGR.QE can only take the values 0b00 or 0b11. 11 Q element support is implemented, and supports both Q elements with instruction counts and Q elements without instruction counts. TRCCONFIGR.QE is fully implemented."]
    #[inline(always)]
    #[must_use]
    pub fn qsupp(&mut self) -> QsuppW<ApbaddrEtmCpu1Trcidr0Spec> {
        QsuppW::new(self, 15)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr0_23_17(&mut self) -> Res0Trcidr0_23_17W<ApbaddrEtmCpu1Trcidr0Spec> {
        Res0Trcidr0_23_17W::new(self, 17)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Global timestamp size field. The permitted values are: 00000 Global timestamping is not implemented. 00110 Implementation supports a maximum global timestamp of 48bits. 01000 Implementation supports a maximum global timestamp of 64bits. All other values are reserved.When global timestamping is implemented then TRCCONFIGR.TS and TRCTSCTLR are supported."]
    #[inline(always)]
    #[must_use]
    pub fn tssize(&mut self) -> TssizeW<ApbaddrEtmCpu1Trcidr0Spec> {
        TssizeW::new(self, 24)
    }
    #[doc = "Bit 29 - 29:29\\]
Conditional instruction tracing support bit. Indicates if the trace unit supports conditional instruction tracing: 0 Conditional instruction tracing is not supported. 1 Conditional instruction tracing is supported, so TRCCONFIGR.COND is supported."]
    #[inline(always)]
    #[must_use]
    pub fn commopt(&mut self) -> CommoptW<ApbaddrEtmCpu1Trcidr0Spec> {
        CommoptW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcidr0_31_30(&mut self) -> Res0Trcidr0_31_30W<ApbaddrEtmCpu1Trcidr0Spec> {
        Res0Trcidr0_31_30W::new(self, 30)
    }
}
#[doc = "ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcidr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcidr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcidr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcidr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcidr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIDR0 to value 0x2800_0ea1"]
impl crate::Resettable for ApbaddrEtmCpu1Trcidr0Spec {
    const RESET_VALUE: u32 = 0x2800_0ea1;
}
