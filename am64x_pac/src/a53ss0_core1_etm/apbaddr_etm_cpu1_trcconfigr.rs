#[doc = "Register `APBADDR_ETM_CPU1_TRCCONFIGR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcconfigrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCCONFIGR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcconfigrSpec>;
#[doc = "Field `RES1_TRCCONFIGR_0_0` reader - 0:0\\]
Reserved, RES1."]
pub type Res1Trcconfigr0_0R = crate::BitReader;
#[doc = "Field `RES1_TRCCONFIGR_0_0` writer - 0:0\\]
Reserved, RES1."]
pub type Res1Trcconfigr0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INSTP0` reader - 2:1\\]
Instruction P0 bit. This field controls whether load and store instructions are traced as P0 instructions: 00 Do not trace load and store instructions as P0 instructions. 01 Trace load instructions as P0 instructions. 10 Trace store instructions as P0 instructions. 11 Trace load and store instructions as P0 instructions. TRCIDR0.INSTP0 indicates whether this field is supported. If it is not supported then this field is RES0."]
pub type Instp0R = crate::FieldReader;
#[doc = "Field `INSTP0` writer - 2:1\\]
Instruction P0 bit. This field controls whether load and store instructions are traced as P0 instructions: 00 Do not trace load and store instructions as P0 instructions. 01 Trace load instructions as P0 instructions. 10 Trace store instructions as P0 instructions. 11 Trace load and store instructions as P0 instructions. TRCIDR0.INSTP0 indicates whether this field is supported. If it is not supported then this field is RES0."]
pub type Instp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BB` reader - 3:3\\]
Branch broadcast mode bit: 0 Branch broadcast mode is disabled. 1 Branch broadcast mode is enabled. TRCBBCTLR controls which regions of memory are enabled to use branch broadcasting. TRCIDR0.TRCBB indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type BbR = crate::BitReader;
#[doc = "Field `BB` writer - 3:3\\]
Branch broadcast mode bit: 0 Branch broadcast mode is disabled. 1 Branch broadcast mode is enabled. TRCBBCTLR controls which regions of memory are enabled to use branch broadcasting. TRCIDR0.TRCBB indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCI` reader - 4:4\\]
Cycle counting instruction trace bit: 0 Cycle counting in the instruction trace is disabled. 1 Cycle counting in the instruction trace is enabled. TRCCCCTLR controls the threshold value for cycle counting. TRCIDR0.TRCCCI indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type CciR = crate::BitReader;
#[doc = "Field `CCI` writer - 4:4\\]
Cycle counting instruction trace bit: 0 Cycle counting in the instruction trace is disabled. 1 Cycle counting in the instruction trace is enabled. TRCCCCTLR controls the threshold value for cycle counting. TRCIDR0.TRCCCI indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type CciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCCONFIGR_5_5` reader - 5:5\\]
Reserved, RES0."]
pub type Res0Trcconfigr5_5R = crate::BitReader;
#[doc = "Field `RES0_TRCCONFIGR_5_5` writer - 5:5\\]
Reserved, RES0."]
pub type Res0Trcconfigr5_5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CID` reader - 6:6\\]
Context ID tracing bit: 0 Context ID tracing is disabled. 1 Context ID tracing is enabled. TRCIDR2.CIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type CidR = crate::BitReader;
#[doc = "Field `CID` writer - 6:6\\]
Context ID tracing bit: 0 Context ID tracing is disabled. 1 Context ID tracing is enabled. TRCIDR2.CIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type CidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMID` reader - 7:7\\]
VMID tracing bit: 0 VMID tracing is disabled. 1 VMID tracing is enabled. TRCIDR2.VMIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type VmidR = crate::BitReader;
#[doc = "Field `VMID` writer - 7:7\\]
VMID tracing bit: 0 VMID tracing is disabled. 1 VMID tracing is enabled. TRCIDR2.VMIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type VmidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COND` reader - 10:8\\]
Conditional instruction tracing bit. The permitted values are: 000 Conditional instruction tracing is disabled. 001 Conditional load instructions are traced. 010 Conditional store instructions are traced. 011 Conditional load and store instructions are traced. 111 All conditional instructions are traced. All other values are reserved.TRCIDR0.TRCCOND indicates whether this field is supported. If it is not supported then this field is RES0."]
pub type CondR = crate::FieldReader;
#[doc = "Field `COND` writer - 10:8\\]
Conditional instruction tracing bit. The permitted values are: 000 Conditional instruction tracing is disabled. 001 Conditional load instructions are traced. 010 Conditional store instructions are traced. 011 Conditional load and store instructions are traced. 111 All conditional instructions are traced. All other values are reserved.TRCIDR0.TRCCOND indicates whether this field is supported. If it is not supported then this field is RES0."]
pub type CondW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS` reader - 11:11\\]
Global timestamp tracing bit: 0 Global timestamp tracing is disabled. 1 Global timestamp tracing is enabled. TRCTSCTLR controls the insertion of timestamps in the trace. TRCIDR0.TSSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type TsR = crate::BitReader;
#[doc = "Field `TS` writer - 11:11\\]
Global timestamp tracing bit: 0 Global timestamp tracing is disabled. 1 Global timestamp tracing is enabled. TRCTSCTLR controls the insertion of timestamps in the trace. TRCIDR0.TSSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type TsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - 12:12\\]
Return stack enable bit. 0 Return stack is disabled. 1 Return stack is enabled. TRCIDR0.RETSTACK indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type RsR = crate::BitReader;
#[doc = "Field `RS` writer - 12:12\\]
Return stack enable bit. 0 Return stack is disabled. 1 Return stack is enabled. TRCIDR0.RETSTACK indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type RsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QE` reader - 14:13\\]
Q element enable field: 00 Q elements are disabled. 01 Q elements with instruction counts are enabled. Q elements without instruction counts are disabled. 11 Q elements with and without instruction counts are enabled. The value 0b10 is reserved.TRCIDR0.QSUPP indicates which values of this field are implemented.TRCCONFIGR.QE must be set to 0b00 if any of the following are true:TRCCONFIGR.INSTP0 is not 0b00.TRCCONFIGR.COND is not 0b000.TRCCONFIGR.BB is not 0."]
pub type QeR = crate::FieldReader;
#[doc = "Field `QE` writer - 14:13\\]
Q element enable field: 00 Q elements are disabled. 01 Q elements with instruction counts are enabled. Q elements without instruction counts are disabled. 11 Q elements with and without instruction counts are enabled. The value 0b10 is reserved.TRCIDR0.QSUPP indicates which values of this field are implemented.TRCCONFIGR.QE must be set to 0b00 if any of the following are true:TRCCONFIGR.INSTP0 is not 0b00.TRCCONFIGR.COND is not 0b000.TRCCONFIGR.BB is not 0."]
pub type QeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_TRCCONFIGR_15_15` reader - 15:15\\]
Reserved, RES0."]
pub type Res0Trcconfigr15_15R = crate::BitReader;
#[doc = "Field `RES0_TRCCONFIGR_15_15` writer - 15:15\\]
Reserved, RES0."]
pub type Res0Trcconfigr15_15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - 16:16\\]
Data address tracing bit: 0 Data address tracing is disabled. 1 Data address tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - 16:16\\]
Data address tracing bit: 0 Data address tracing is disabled. 1 Data address tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV` reader - 17:17\\]
Data value tracing bit: 0 Data value tracing is disabled. 1 Data value tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type DvR = crate::BitReader;
#[doc = "Field `DV` writer - 17:17\\]
Data value tracing bit: 0 Data value tracing is disabled. 1 Data value tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
pub type DvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCCONFIGR_31_18` reader - 31:18\\]
Reserved, RES0."]
pub type Res0Trcconfigr31_18R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCCONFIGR_31_18` writer - 31:18\\]
Reserved, RES0."]
pub type Res0Trcconfigr31_18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES1."]
    #[inline(always)]
    pub fn res1_trcconfigr_0_0(&self) -> Res1Trcconfigr0_0R {
        Res1Trcconfigr0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Instruction P0 bit. This field controls whether load and store instructions are traced as P0 instructions: 00 Do not trace load and store instructions as P0 instructions. 01 Trace load instructions as P0 instructions. 10 Trace store instructions as P0 instructions. 11 Trace load and store instructions as P0 instructions. TRCIDR0.INSTP0 indicates whether this field is supported. If it is not supported then this field is RES0."]
    #[inline(always)]
    pub fn instp0(&self) -> Instp0R {
        Instp0R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Branch broadcast mode bit: 0 Branch broadcast mode is disabled. 1 Branch broadcast mode is enabled. TRCBBCTLR controls which regions of memory are enabled to use branch broadcasting. TRCIDR0.TRCBB indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Cycle counting instruction trace bit: 0 Cycle counting in the instruction trace is disabled. 1 Cycle counting in the instruction trace is enabled. TRCCCCTLR controls the threshold value for cycle counting. TRCIDR0.TRCCCI indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn cci(&self) -> CciR {
        CciR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcconfigr_5_5(&self) -> Res0Trcconfigr5_5R {
        Res0Trcconfigr5_5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Context ID tracing bit: 0 Context ID tracing is disabled. 1 Context ID tracing is enabled. TRCIDR2.CIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn cid(&self) -> CidR {
        CidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
VMID tracing bit: 0 VMID tracing is disabled. 1 VMID tracing is enabled. TRCIDR2.VMIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn vmid(&self) -> VmidR {
        VmidR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Conditional instruction tracing bit. The permitted values are: 000 Conditional instruction tracing is disabled. 001 Conditional load instructions are traced. 010 Conditional store instructions are traced. 011 Conditional load and store instructions are traced. 111 All conditional instructions are traced. All other values are reserved.TRCIDR0.TRCCOND indicates whether this field is supported. If it is not supported then this field is RES0."]
    #[inline(always)]
    pub fn cond(&self) -> CondR {
        CondR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
Global timestamp tracing bit: 0 Global timestamp tracing is disabled. 1 Global timestamp tracing is enabled. TRCTSCTLR controls the insertion of timestamps in the trace. TRCIDR0.TSSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Return stack enable bit. 0 Return stack is disabled. 1 Return stack is enabled. TRCIDR0.RETSTACK indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Q element enable field: 00 Q elements are disabled. 01 Q elements with instruction counts are enabled. Q elements without instruction counts are disabled. 11 Q elements with and without instruction counts are enabled. The value 0b10 is reserved.TRCIDR0.QSUPP indicates which values of this field are implemented.TRCCONFIGR.QE must be set to 0b00 if any of the following are true:TRCCONFIGR.INSTP0 is not 0b00.TRCCONFIGR.COND is not 0b000.TRCCONFIGR.BB is not 0."]
    #[inline(always)]
    pub fn qe(&self) -> QeR {
        QeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcconfigr_15_15(&self) -> Res0Trcconfigr15_15R {
        Res0Trcconfigr15_15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Data address tracing bit: 0 Data address tracing is disabled. 1 Data address tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Data value tracing bit: 0 Data value tracing is disabled. 1 Data value tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcconfigr_31_18(&self) -> Res0Trcconfigr31_18R {
        Res0Trcconfigr31_18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES1."]
    #[inline(always)]
    #[must_use]
    pub fn res1_trcconfigr_0_0(&mut self) -> Res1Trcconfigr0_0W<ApbaddrEtmCpu1TrcconfigrSpec> {
        Res1Trcconfigr0_0W::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Instruction P0 bit. This field controls whether load and store instructions are traced as P0 instructions: 00 Do not trace load and store instructions as P0 instructions. 01 Trace load instructions as P0 instructions. 10 Trace store instructions as P0 instructions. 11 Trace load and store instructions as P0 instructions. TRCIDR0.INSTP0 indicates whether this field is supported. If it is not supported then this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn instp0(&mut self) -> Instp0W<ApbaddrEtmCpu1TrcconfigrSpec> {
        Instp0W::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Branch broadcast mode bit: 0 Branch broadcast mode is disabled. 1 Branch broadcast mode is enabled. TRCBBCTLR controls which regions of memory are enabled to use branch broadcasting. TRCIDR0.TRCBB indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BbW<ApbaddrEtmCpu1TrcconfigrSpec> {
        BbW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Cycle counting instruction trace bit: 0 Cycle counting in the instruction trace is disabled. 1 Cycle counting in the instruction trace is enabled. TRCCCCTLR controls the threshold value for cycle counting. TRCIDR0.TRCCCI indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn cci(&mut self) -> CciW<ApbaddrEtmCpu1TrcconfigrSpec> {
        CciW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcconfigr_5_5(&mut self) -> Res0Trcconfigr5_5W<ApbaddrEtmCpu1TrcconfigrSpec> {
        Res0Trcconfigr5_5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Context ID tracing bit: 0 Context ID tracing is disabled. 1 Context ID tracing is enabled. TRCIDR2.CIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn cid(&mut self) -> CidW<ApbaddrEtmCpu1TrcconfigrSpec> {
        CidW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
VMID tracing bit: 0 VMID tracing is disabled. 1 VMID tracing is enabled. TRCIDR2.VMIDSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn vmid(&mut self) -> VmidW<ApbaddrEtmCpu1TrcconfigrSpec> {
        VmidW::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Conditional instruction tracing bit. The permitted values are: 000 Conditional instruction tracing is disabled. 001 Conditional load instructions are traced. 010 Conditional store instructions are traced. 011 Conditional load and store instructions are traced. 111 All conditional instructions are traced. All other values are reserved.TRCIDR0.TRCCOND indicates whether this field is supported. If it is not supported then this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn cond(&mut self) -> CondW<ApbaddrEtmCpu1TrcconfigrSpec> {
        CondW::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
Global timestamp tracing bit: 0 Global timestamp tracing is disabled. 1 Global timestamp tracing is enabled. TRCTSCTLR controls the insertion of timestamps in the trace. TRCIDR0.TSSIZE indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<ApbaddrEtmCpu1TrcconfigrSpec> {
        TsW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Return stack enable bit. 0 Return stack is disabled. 1 Return stack is enabled. TRCIDR0.RETSTACK indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RsW<ApbaddrEtmCpu1TrcconfigrSpec> {
        RsW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Q element enable field: 00 Q elements are disabled. 01 Q elements with instruction counts are enabled. Q elements without instruction counts are disabled. 11 Q elements with and without instruction counts are enabled. The value 0b10 is reserved.TRCIDR0.QSUPP indicates which values of this field are implemented.TRCCONFIGR.QE must be set to 0b00 if any of the following are true:TRCCONFIGR.INSTP0 is not 0b00.TRCCONFIGR.COND is not 0b000.TRCCONFIGR.BB is not 0."]
    #[inline(always)]
    #[must_use]
    pub fn qe(&mut self) -> QeW<ApbaddrEtmCpu1TrcconfigrSpec> {
        QeW::new(self, 13)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcconfigr_15_15(&mut self) -> Res0Trcconfigr15_15W<ApbaddrEtmCpu1TrcconfigrSpec> {
        Res0Trcconfigr15_15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Data address tracing bit: 0 Data address tracing is disabled. 1 Data address tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<ApbaddrEtmCpu1TrcconfigrSpec> {
        DaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Data value tracing bit: 0 Data value tracing is disabled. 1 Data value tracing is enabled when INSTP0 is not 0b00. TRCIDR0.TRCDATA indicates whether this bit is supported. If it is not supported then this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn dv(&mut self) -> DvW<ApbaddrEtmCpu1TrcconfigrSpec> {
        DvW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcconfigr_31_18(&mut self) -> Res0Trcconfigr31_18W<ApbaddrEtmCpu1TrcconfigrSpec> {
        Res0Trcconfigr31_18W::new(self, 18)
    }
}
#[doc = "Trace Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcconfigr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcconfigr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcconfigrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcconfigrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcconfigr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcconfigrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcconfigr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcconfigrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCCONFIGR to value 0x01"]
impl crate::Resettable for ApbaddrEtmCpu1TrcconfigrSpec {
    const RESET_VALUE: u32 = 0x01;
}
