#[doc = "Register `APBADDR_DBG_CPU1_EDPRSR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdprsrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDPRSR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdprsrSpec>;
#[doc = "Field `PU` reader - 0:0\\]
Core power-up status bit. Indicates whether the Core power domain debug registers can be accessed: 0 Core is in a low-power or power-down state where the debug registers cannot be accessed. 1 Core is in a power-up state where the debug registers can be accessed."]
pub type PuR = crate::BitReader;
#[doc = "Field `PU` writer - 0:0\\]
Core power-up status bit. Indicates whether the Core power domain debug registers can be accessed: 0 Core is in a low-power or power-down state where the debug registers cannot be accessed. 1 Core is in a power-up state where the debug registers can be accessed."]
pub type PuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPD` reader - 1:1\\]
Sticky core power-down status bit.This bit is set to 1 on Cold reset to indicate the state of the debug registers has been lost. Since a Cold reset is required on powering up the processor, this usually indicates the Core power domain has been completely powered off.Possible values are: 0 If the Core power domain is off \\[EDPRSR.PU is 0\\], it is not known whether the state of the debug registers in the Core power domain is lost. Otherwise, the Core power domain is on, and the state of the debug registers in the Core power domain has not been lost. 1 The state of the debug registers in the Core power domain is lost. This bit is UNKNOWN on reads if both EDPRSR.DLK and EDPRSR.PU are 1.This bit clears to 0 following a read of EDPRSR if the processor is not in the powered down state. There are two logical power off states for the Core power domain:RetentionThe states of the debug registers, including EDPRSR.SPD, in the Core power domain is preserved, and restored on leaving retention state.Power-downThe states of the debug registers in the Core power domain is lost, and a Cold reset is asserted on leaving power-down state.In these states, it is IMPLEMENTATION DEFINED whether:EDPRSR.SPD shows whether the state of the debug registers in the Core power domain has been lost since the last time EDPRSR was read when the Core power domain was on.EDPRSR.SPD reads-as-zero.EDPRSR.SPD is not cleared following a read of EDPRSR in these states.This means it is IMPLEMENTATION DEFINED whether a processor implements EDPRSR.SPD as:Fixed RAZ when in one or both of the retention and power-down states.Retaining its previous value when in the retention state.Fixed RAO in the power-down state.Note that this definition does not allow EDPRSR.SPD to be fixed RAO in the low-power retention state, as the state of the debug registers in the Core power domain is not lost by entering this state. However, the bit can be read as 1 in this state if the state of the registers was lost before entering this state \\[i.e. EDPRSR has not been read since the last Cold reset\\].ARM recommends that an implementation make EDPRSR.SPD fixed RAO when in the power-down state, particularly if it does not support a low-power retention state."]
pub type SpdR = crate::BitReader;
#[doc = "Field `SPD` writer - 1:1\\]
Sticky core power-down status bit.This bit is set to 1 on Cold reset to indicate the state of the debug registers has been lost. Since a Cold reset is required on powering up the processor, this usually indicates the Core power domain has been completely powered off.Possible values are: 0 If the Core power domain is off \\[EDPRSR.PU is 0\\], it is not known whether the state of the debug registers in the Core power domain is lost. Otherwise, the Core power domain is on, and the state of the debug registers in the Core power domain has not been lost. 1 The state of the debug registers in the Core power domain is lost. This bit is UNKNOWN on reads if both EDPRSR.DLK and EDPRSR.PU are 1.This bit clears to 0 following a read of EDPRSR if the processor is not in the powered down state. There are two logical power off states for the Core power domain:RetentionThe states of the debug registers, including EDPRSR.SPD, in the Core power domain is preserved, and restored on leaving retention state.Power-downThe states of the debug registers in the Core power domain is lost, and a Cold reset is asserted on leaving power-down state.In these states, it is IMPLEMENTATION DEFINED whether:EDPRSR.SPD shows whether the state of the debug registers in the Core power domain has been lost since the last time EDPRSR was read when the Core power domain was on.EDPRSR.SPD reads-as-zero.EDPRSR.SPD is not cleared following a read of EDPRSR in these states.This means it is IMPLEMENTATION DEFINED whether a processor implements EDPRSR.SPD as:Fixed RAZ when in one or both of the retention and power-down states.Retaining its previous value when in the retention state.Fixed RAO in the power-down state.Note that this definition does not allow EDPRSR.SPD to be fixed RAO in the low-power retention state, as the state of the debug registers in the Core power domain is not lost by entering this state. However, the bit can be read as 1 in this state if the state of the registers was lost before entering this state \\[i.e. EDPRSR has not been read since the last Cold reset\\].ARM recommends that an implementation make EDPRSR.SPD fixed RAO when in the power-down state, particularly if it does not support a low-power retention state."]
pub type SpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R` reader - 2:2\\]
Core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state. 1 The non-debug logic of the processor is in reset state. This bit is UNKNOWN on reads if either EDPRSR.DLK is 1 or EDPRSR.PU is 0."]
pub type RR = crate::BitReader;
#[doc = "Field `R` writer - 2:2\\]
Core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state. 1 The non-debug logic of the processor is in reset state. This bit is UNKNOWN on reads if either EDPRSR.DLK is 1 or EDPRSR.PU is 0."]
pub type RW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR` reader - 3:3\\]
Sticky core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state and has not been reset since the last time EDPRSR was read. 1 The non-debug logic of the processor is in reset state or has been reset since the last time EDPRSR was read. This bit is UNKNOWN on reads if EDPRSR.DLK is 1 or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR if the non-debug logic of the processor is not in reset state."]
pub type SrR = crate::BitReader;
#[doc = "Field `SR` writer - 3:3\\]
Sticky core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state and has not been reset since the last time EDPRSR was read. 1 The non-debug logic of the processor is in reset state or has been reset since the last time EDPRSR was read. This bit is UNKNOWN on reads if EDPRSR.DLK is 1 or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR if the non-debug logic of the processor is not in reset state."]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALTED` reader - 4:4\\]
Halted status bit. Possible values are: 0 EDSCR.STATUS is 0b000010 \\[processor in Non-debug state\\]. 1 EDSCR.STATUS is not 0b000010. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
pub type HaltedR = crate::BitReader;
#[doc = "Field `HALTED` writer - 4:4\\]
Halted status bit. Possible values are: 0 EDSCR.STATUS is 0b000010 \\[processor in Non-debug state\\]. 1 EDSCR.STATUS is not 0b000010. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
pub type HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSLK` reader - 5:5\\]
OS lock status bit. A read of this bit returns the value of OSLSR_EL1.OSLK.This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1 or EDPRSR.PU is 0."]
pub type OslkR = crate::BitReader;
#[doc = "Field `OSLK` writer - 5:5\\]
OS lock status bit. A read of this bit returns the value of OSLSR_EL1.OSLK.This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1 or EDPRSR.PU is 0."]
pub type OslkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLK` reader - 6:6\\]
OS Double Lock status bit. 0 OSDLR_EL1.DLK == 0 or EDPRCR.CORENPDRQ == 1 or the processor is in Debug state. 1 OSDLR_EL1.DLK == 1 and EDPRCR.CORENPDRQ == 0 and the processor is in Non-debug state. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
pub type DlkR = crate::BitReader;
#[doc = "Field `DLK` writer - 6:6\\]
OS Double Lock status bit. 0 OSDLR_EL1.DLK == 0 or EDPRCR.CORENPDRQ == 1 or the processor is in Debug state. 1 OSDLR_EL1.DLK == 1 and EDPRCR.CORENPDRQ == 0 and the processor is in Non-debug state. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
pub type DlkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDAD` reader - 7:7\\]
External debug access disable status. 0 External debug access enabled. 1 External debug access disabled. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
pub type EdadR = crate::BitReader;
#[doc = "Field `EDAD` writer - 7:7\\]
External debug access disable status. 0 External debug access enabled. 1 External debug access disabled. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
pub type EdadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAD` reader - 8:8\\]
Sticky EDAD error. Set to 1 if an access returns an error because AllowExternalDebugAccess\\[\\]
== FALSE. 0 No accesses to the external debug registers have failed since EDPRSR was last read. 1 At least one access to the external debug registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR."]
pub type SdadR = crate::BitReader;
#[doc = "Field `SDAD` writer - 8:8\\]
Sticky EDAD error. Set to 1 if an access returns an error because AllowExternalDebugAccess\\[\\]
== FALSE. 0 No accesses to the external debug registers have failed since EDPRSR was last read. 1 At least one access to the external debug registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR."]
pub type SdadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMAD` reader - 9:9\\]
External performance monitors access disable status. 0 External performance monitors access enabled. 1 External performance monitors access disabled. If external performance monitors access is not implemented, EPMAD is RAO. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
pub type EpmadR = crate::BitReader;
#[doc = "Field `EPMAD` writer - 9:9\\]
External performance monitors access disable status. 0 External performance monitors access enabled. 1 External performance monitors access disabled. If external performance monitors access is not implemented, EPMAD is RAO. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
pub type EpmadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPMAD` reader - 10:10\\]
Sticky EPMAD error. Set to 1 if an access returns an error because AllowExternalPMUAccess\\[\\]
== FALSE. 0 No accesses to the external performance monitors registers have failed since EDPRSR was last read. 1 At least one access to the external performance monitors registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
pub type SpmadR = crate::BitReader;
#[doc = "Field `SPMAD` writer - 10:10\\]
Sticky EPMAD error. Set to 1 if an access returns an error because AllowExternalPMUAccess\\[\\]
== FALSE. 0 No accesses to the external performance monitors registers have failed since EDPRSR was last read. 1 At least one access to the external performance monitors registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
pub type SpmadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR` reader - 11:11\\]
Sticky debug restart. Set to 1 when the processor exits Debug state and cleared to 0 following reads of EDPRSR. 0 The processor has not restarted since EDPRSR was last read. 1 The processor has restarted since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
pub type SdrR = crate::BitReader;
#[doc = "Field `SDR` writer - 11:11\\]
Sticky debug restart. Set to 1 when the processor exits Debug state and cleared to 0 following reads of EDPRSR. 0 The processor has not restarted since EDPRSR was last read. 1 The processor has restarted since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
pub type SdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDPRSR_31_12` reader - 31:12\\]
Reserved, RES0."]
pub type Res0Edprsr31_12R = crate::FieldReader<u32>;
#[doc = "Field `RES0_EDPRSR_31_12` writer - 31:12\\]
Reserved, RES0."]
pub type Res0Edprsr31_12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Core power-up status bit. Indicates whether the Core power domain debug registers can be accessed: 0 Core is in a low-power or power-down state where the debug registers cannot be accessed. 1 Core is in a power-up state where the debug registers can be accessed."]
    #[inline(always)]
    pub fn pu(&self) -> PuR {
        PuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sticky core power-down status bit.This bit is set to 1 on Cold reset to indicate the state of the debug registers has been lost. Since a Cold reset is required on powering up the processor, this usually indicates the Core power domain has been completely powered off.Possible values are: 0 If the Core power domain is off \\[EDPRSR.PU is 0\\], it is not known whether the state of the debug registers in the Core power domain is lost. Otherwise, the Core power domain is on, and the state of the debug registers in the Core power domain has not been lost. 1 The state of the debug registers in the Core power domain is lost. This bit is UNKNOWN on reads if both EDPRSR.DLK and EDPRSR.PU are 1.This bit clears to 0 following a read of EDPRSR if the processor is not in the powered down state. There are two logical power off states for the Core power domain:RetentionThe states of the debug registers, including EDPRSR.SPD, in the Core power domain is preserved, and restored on leaving retention state.Power-downThe states of the debug registers in the Core power domain is lost, and a Cold reset is asserted on leaving power-down state.In these states, it is IMPLEMENTATION DEFINED whether:EDPRSR.SPD shows whether the state of the debug registers in the Core power domain has been lost since the last time EDPRSR was read when the Core power domain was on.EDPRSR.SPD reads-as-zero.EDPRSR.SPD is not cleared following a read of EDPRSR in these states.This means it is IMPLEMENTATION DEFINED whether a processor implements EDPRSR.SPD as:Fixed RAZ when in one or both of the retention and power-down states.Retaining its previous value when in the retention state.Fixed RAO in the power-down state.Note that this definition does not allow EDPRSR.SPD to be fixed RAO in the low-power retention state, as the state of the debug registers in the Core power domain is not lost by entering this state. However, the bit can be read as 1 in this state if the state of the registers was lost before entering this state \\[i.e. EDPRSR has not been read since the last Cold reset\\].ARM recommends that an implementation make EDPRSR.SPD fixed RAO when in the power-down state, particularly if it does not support a low-power retention state."]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state. 1 The non-debug logic of the processor is in reset state. This bit is UNKNOWN on reads if either EDPRSR.DLK is 1 or EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state and has not been reset since the last time EDPRSR was read. 1 The non-debug logic of the processor is in reset state or has been reset since the last time EDPRSR was read. This bit is UNKNOWN on reads if EDPRSR.DLK is 1 or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR if the non-debug logic of the processor is not in reset state."]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Halted status bit. Possible values are: 0 EDSCR.STATUS is 0b000010 \\[processor in Non-debug state\\]. 1 EDSCR.STATUS is not 0b000010. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn halted(&self) -> HaltedR {
        HaltedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
OS lock status bit. A read of this bit returns the value of OSLSR_EL1.OSLK.This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1 or EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn oslk(&self) -> OslkR {
        OslkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
OS Double Lock status bit. 0 OSDLR_EL1.DLK == 0 or EDPRCR.CORENPDRQ == 1 or the processor is in Debug state. 1 OSDLR_EL1.DLK == 1 and EDPRCR.CORENPDRQ == 0 and the processor is in Non-debug state. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn dlk(&self) -> DlkR {
        DlkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
External debug access disable status. 0 External debug access enabled. 1 External debug access disabled. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn edad(&self) -> EdadR {
        EdadR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sticky EDAD error. Set to 1 if an access returns an error because AllowExternalDebugAccess\\[\\]
== FALSE. 0 No accesses to the external debug registers have failed since EDPRSR was last read. 1 At least one access to the external debug registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR."]
    #[inline(always)]
    pub fn sdad(&self) -> SdadR {
        SdadR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
External performance monitors access disable status. 0 External performance monitors access enabled. 1 External performance monitors access disabled. If external performance monitors access is not implemented, EPMAD is RAO. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
    #[inline(always)]
    pub fn epmad(&self) -> EpmadR {
        EpmadR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Sticky EPMAD error. Set to 1 if an access returns an error because AllowExternalPMUAccess\\[\\]
== FALSE. 0 No accesses to the external performance monitors registers have failed since EDPRSR was last read. 1 At least one access to the external performance monitors registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
    #[inline(always)]
    pub fn spmad(&self) -> SpmadR {
        SpmadR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Sticky debug restart. Set to 1 when the processor exits Debug state and cleared to 0 following reads of EDPRSR. 0 The processor has not restarted since EDPRSR was last read. 1 The processor has restarted since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
    #[inline(always)]
    pub fn sdr(&self) -> SdrR {
        SdrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edprsr_31_12(&self) -> Res0Edprsr31_12R {
        Res0Edprsr31_12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Core power-up status bit. Indicates whether the Core power domain debug registers can be accessed: 0 Core is in a low-power or power-down state where the debug registers cannot be accessed. 1 Core is in a power-up state where the debug registers can be accessed."]
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PuW<ApbaddrDbgCpu1EdprsrSpec> {
        PuW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sticky core power-down status bit.This bit is set to 1 on Cold reset to indicate the state of the debug registers has been lost. Since a Cold reset is required on powering up the processor, this usually indicates the Core power domain has been completely powered off.Possible values are: 0 If the Core power domain is off \\[EDPRSR.PU is 0\\], it is not known whether the state of the debug registers in the Core power domain is lost. Otherwise, the Core power domain is on, and the state of the debug registers in the Core power domain has not been lost. 1 The state of the debug registers in the Core power domain is lost. This bit is UNKNOWN on reads if both EDPRSR.DLK and EDPRSR.PU are 1.This bit clears to 0 following a read of EDPRSR if the processor is not in the powered down state. There are two logical power off states for the Core power domain:RetentionThe states of the debug registers, including EDPRSR.SPD, in the Core power domain is preserved, and restored on leaving retention state.Power-downThe states of the debug registers in the Core power domain is lost, and a Cold reset is asserted on leaving power-down state.In these states, it is IMPLEMENTATION DEFINED whether:EDPRSR.SPD shows whether the state of the debug registers in the Core power domain has been lost since the last time EDPRSR was read when the Core power domain was on.EDPRSR.SPD reads-as-zero.EDPRSR.SPD is not cleared following a read of EDPRSR in these states.This means it is IMPLEMENTATION DEFINED whether a processor implements EDPRSR.SPD as:Fixed RAZ when in one or both of the retention and power-down states.Retaining its previous value when in the retention state.Fixed RAO in the power-down state.Note that this definition does not allow EDPRSR.SPD to be fixed RAO in the low-power retention state, as the state of the debug registers in the Core power domain is not lost by entering this state. However, the bit can be read as 1 in this state if the state of the registers was lost before entering this state \\[i.e. EDPRSR has not been read since the last Cold reset\\].ARM recommends that an implementation make EDPRSR.SPD fixed RAO when in the power-down state, particularly if it does not support a low-power retention state."]
    #[inline(always)]
    #[must_use]
    pub fn spd(&mut self) -> SpdW<ApbaddrDbgCpu1EdprsrSpec> {
        SpdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state. 1 The non-debug logic of the processor is in reset state. This bit is UNKNOWN on reads if either EDPRSR.DLK is 1 or EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> RW<ApbaddrDbgCpu1EdprsrSpec> {
        RW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Sticky core reset status bit. Possible values are: 0 The non-debug logic of the processor is not in reset state and has not been reset since the last time EDPRSR was read. 1 The non-debug logic of the processor is in reset state or has been reset since the last time EDPRSR was read. This bit is UNKNOWN on reads if EDPRSR.DLK is 1 or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR if the non-debug logic of the processor is not in reset state."]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<ApbaddrDbgCpu1EdprsrSpec> {
        SrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Halted status bit. Possible values are: 0 EDSCR.STATUS is 0b000010 \\[processor in Non-debug state\\]. 1 EDSCR.STATUS is not 0b000010. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HaltedW<ApbaddrDbgCpu1EdprsrSpec> {
        HaltedW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
OS lock status bit. A read of this bit returns the value of OSLSR_EL1.OSLK.This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1 or EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn oslk(&mut self) -> OslkW<ApbaddrDbgCpu1EdprsrSpec> {
        OslkW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
OS Double Lock status bit. 0 OSDLR_EL1.DLK == 0 or EDPRCR.CORENPDRQ == 1 or the processor is in Debug state. 1 OSDLR_EL1.DLK == 1 and EDPRCR.CORENPDRQ == 0 and the processor is in Non-debug state. This bit is UNKNOWN on reads if EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn dlk(&mut self) -> DlkW<ApbaddrDbgCpu1EdprsrSpec> {
        DlkW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
External debug access disable status. 0 External debug access enabled. 1 External debug access disabled. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn edad(&mut self) -> EdadW<ApbaddrDbgCpu1EdprsrSpec> {
        EdadW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Sticky EDAD error. Set to 1 if an access returns an error because AllowExternalDebugAccess\\[\\]
== FALSE. 0 No accesses to the external debug registers have failed since EDPRSR was last read. 1 At least one access to the external debug registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 following a read of EDPRSR."]
    #[inline(always)]
    #[must_use]
    pub fn sdad(&mut self) -> SdadW<ApbaddrDbgCpu1EdprsrSpec> {
        SdadW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
External performance monitors access disable status. 0 External performance monitors access enabled. 1 External performance monitors access disabled. If external performance monitors access is not implemented, EPMAD is RAO. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0."]
    #[inline(always)]
    #[must_use]
    pub fn epmad(&mut self) -> EpmadW<ApbaddrDbgCpu1EdprsrSpec> {
        EpmadW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Sticky EPMAD error. Set to 1 if an access returns an error because AllowExternalPMUAccess\\[\\]
== FALSE. 0 No accesses to the external performance monitors registers have failed since EDPRSR was last read. 1 At least one access to the external performance monitors registers has failed since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
    #[inline(always)]
    #[must_use]
    pub fn spmad(&mut self) -> SpmadW<ApbaddrDbgCpu1EdprsrSpec> {
        SpmadW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Sticky debug restart. Set to 1 when the processor exits Debug state and cleared to 0 following reads of EDPRSR. 0 The processor has not restarted since EDPRSR was last read. 1 The processor has restarted since EDPRSR was last read. This bit is UNKNOWN on reads if either of EDPRSR.{DLK, R} is 1, or EDPRSR.PU is 0.This bit clears to 0 when following a read of EDPRSR."]
    #[inline(always)]
    #[must_use]
    pub fn sdr(&mut self) -> SdrW<ApbaddrDbgCpu1EdprsrSpec> {
        SdrW::new(self, 11)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edprsr_31_12(&mut self) -> Res0Edprsr31_12W<ApbaddrDbgCpu1EdprsrSpec> {
        Res0Edprsr31_12W::new(self, 12)
    }
}
#[doc = "External Debug Processor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edprsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edprsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdprsrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdprsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edprsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdprsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edprsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdprsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDPRSR to value 0x03"]
impl crate::Resettable for ApbaddrDbgCpu1EdprsrSpec {
    const RESET_VALUE: u32 = 0x03;
}
