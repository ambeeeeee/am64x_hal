#[doc = "Register `APBADDR_DBG_CPU1_EDSCR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdscrSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDSCR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdscrSpec>;
#[doc = "Field `STATUS` reader - 5:0\\]
Debug status flags. This field is RO.The possible values of this field are: 000010 Processor is in Non-debug state. 000001 Processor is restarting \\[exiting Debug state\\]. 000111 Breakpoint. 010011 External debug request. 011011 Halting step, normal. 011111 Halting step, exclusive. 100011 OS unlock catch. 100111 Reset catch. 101011 Watchpoint. 101111 HLT instruction. 110011 Software access to debug register. 110111 Exception catch. 111011 Halting step, no syndrome. All other values of STATUS are reserved."]
pub type StatusR = crate::FieldReader;
#[doc = "Field `STATUS` writer - 5:0\\]
Debug status flags. This field is RO.The possible values of this field are: 000010 Processor is in Non-debug state. 000001 Processor is restarting \\[exiting Debug state\\]. 000111 Breakpoint. 010011 External debug request. 011011 Halting step, normal. 011111 Halting step, exclusive. 100011 OS unlock catch. 100111 Reset catch. 101011 Watchpoint. 101111 HLT instruction. 110011 Software access to debug register. 110111 Exception catch. 111011 Halting step, no syndrome. All other values of STATUS are reserved."]
pub type StatusW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ERR` reader - 6:6\\]
Cumulative error flag. This field is RO. It is set to 1 following exceptions in Debug state and on any signaled overrun or underrun on the DTR or EDITR."]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - 6:6\\]
Cumulative error flag. This field is RO. It is set to 1 following exceptions in Debug state and on any signaled overrun or underrun on the DTR or EDITR."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A` reader - 7:7\\]
System Error interrupt pending. Read-only. In Debug state, indicates whether a SError interrupt is pending:If HCR_EL2.{AMO, TGE} = {1, 0} and in Non-secure EL0 or EL1, a virtual SError interrupt.Otherwise, a physical SError interrupt. 0 No SError interrupt pending. 1 SError interrupt pending. A debugger can read EDSCR to check whether a SError interrupt is pending without having to execute further instructions. A pending SError might indicate data from target memory is corrupted.UNKNOWN in Non-debug state."]
pub type AR = crate::BitReader;
#[doc = "Field `A` writer - 7:7\\]
System Error interrupt pending. Read-only. In Debug state, indicates whether a SError interrupt is pending:If HCR_EL2.{AMO, TGE} = {1, 0} and in Non-secure EL0 or EL1, a virtual SError interrupt.Otherwise, a physical SError interrupt. 0 No SError interrupt pending. 1 SError interrupt pending. A debugger can read EDSCR to check whether a SError interrupt is pending without having to execute further instructions. A pending SError might indicate data from target memory is corrupted.UNKNOWN in Non-debug state."]
pub type AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EL` reader - 9:8\\]
Exception level. Read-only. In Debug state, this gives the current EL of the processor.In Non-debug state, this field is RAZ."]
pub type ElR = crate::FieldReader;
#[doc = "Field `EL` writer - 9:8\\]
Exception level. Read-only. In Debug state, this gives the current EL of the processor.In Non-debug state, this field is RAZ."]
pub type ElW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RW` reader - 13:10\\]
Exception level register-width status. Read-only. In Debug state, each bit gives the current register width status of each EL: 1111 All exception levels are AArch64 state. 1110 EL0 is AArch32 state. All other exception levels are AArch64 state. 1100 EL0 and EL1 are AArch32 state. All other exception levels are AArch64 state. Never seen if EL2 is not implemented in the current security state. 1000 EL0, EL1, and, if implemented in the current security state, EL2 are AArch32 state. All other exception levels are AArch64 state. 0000 All exception levels are set to AArch32 state \\[32-bit configuration\\]. However:If not at EL0: RW\\[0\\]
== RW\\[1\\].If EL2 is not implemented in the current security state: RW\\[2\\]
== RW\\[1\\].If EL3 is not implemented: RW\\[3\\]
== RW\\[2\\].In Non-debug state, this field is RAO."]
pub type RwR = crate::FieldReader;
#[doc = "Field `RW` writer - 13:10\\]
Exception level register-width status. Read-only. In Debug state, each bit gives the current register width status of each EL: 1111 All exception levels are AArch64 state. 1110 EL0 is AArch32 state. All other exception levels are AArch64 state. 1100 EL0 and EL1 are AArch32 state. All other exception levels are AArch64 state. Never seen if EL2 is not implemented in the current security state. 1000 EL0, EL1, and, if implemented in the current security state, EL2 are AArch32 state. All other exception levels are AArch64 state. 0000 All exception levels are set to AArch32 state \\[32-bit configuration\\]. However:If not at EL0: RW\\[0\\]
== RW\\[1\\].If EL2 is not implemented in the current security state: RW\\[2\\]
== RW\\[1\\].If EL3 is not implemented: RW\\[3\\]
== RW\\[2\\].In Non-debug state, this field is RAO."]
pub type RwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HDE` reader - 14:14\\]
Halting debug mode enable. Possible values of this bit are: 0 Halting debug mode disabled. 1 Halting debug mode enabled."]
pub type HdeR = crate::BitReader;
#[doc = "Field `HDE` writer - 14:14\\]
Halting debug mode enable. Possible values of this bit are: 0 Halting debug mode disabled. 1 Halting debug mode enabled."]
pub type HdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDSCR_15_15` reader - 15:15\\]
Reserved, RES0."]
pub type Res0Edscr15_15R = crate::BitReader;
#[doc = "Field `RES0_EDSCR_15_15` writer - 15:15\\]
Reserved, RES0."]
pub type Res0Edscr15_15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDD` reader - 16:16\\]
Secure debug disabled. This bit is RO.On entry to Debug state:If entering in Secure state, SDD is set to 0.If entering in Non-secure state, SDD is set to the inverse of ExternalSecureInvasiveDebugEnabled\\[\\].In Debug state, the value of the SDD bit does not change, even if ExternalSecureInvasiveDebugEnabled\\[\\]
changes.In Non-debug state:SDD returns the inverse of ExternalSecureInvasiveDebugEnabled\\[\\]. If the authentication signals that control ExternalSecureInvasiveDebugEnabled\\[\\]
change, a context synchronization operation is required to guarantee their effect.This bit is unaffected by the Security state of the processor.If EL3 is not implemented and the implementation is Non-secure, this bit is RES1."]
pub type SddR = crate::BitReader;
#[doc = "Field `SDD` writer - 16:16\\]
Secure debug disabled. This bit is RO.On entry to Debug state:If entering in Secure state, SDD is set to 0.If entering in Non-secure state, SDD is set to the inverse of ExternalSecureInvasiveDebugEnabled\\[\\].In Debug state, the value of the SDD bit does not change, even if ExternalSecureInvasiveDebugEnabled\\[\\]
changes.In Non-debug state:SDD returns the inverse of ExternalSecureInvasiveDebugEnabled\\[\\]. If the authentication signals that control ExternalSecureInvasiveDebugEnabled\\[\\]
change, a context synchronization operation is required to guarantee their effect.This bit is unaffected by the Security state of the processor.If EL3 is not implemented and the implementation is Non-secure, this bit is RES1."]
pub type SddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDSCR_17_17` reader - 17:17\\]
Reserved, RES0."]
pub type Res0Edscr17_17R = crate::BitReader;
#[doc = "Field `RES0_EDSCR_17_17` writer - 17:17\\]
Reserved, RES0."]
pub type Res0Edscr17_17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - 18:18\\]
Non-secure status. Read-only. When in Debug state, gives the current security state: 0 Secure state, IsSecure\\[\\]
== TRUE 1 Non-secure state, IsSecure\\[\\]
== FALSE. In Non-debug state, this bit is UNKNOWN."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - 18:18\\]
Non-secure status. Read-only. When in Debug state, gives the current security state: 0 Secure state, IsSecure\\[\\]
== TRUE 1 Non-secure state, IsSecure\\[\\]
== FALSE. In Non-debug state, this bit is UNKNOWN."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDSCR_19_19` reader - 19:19\\]
Reserved, RES0."]
pub type Res0Edscr19_19R = crate::BitReader;
#[doc = "Field `RES0_EDSCR_19_19` writer - 19:19\\]
Reserved, RES0."]
pub type Res0Edscr19_19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MA` reader - 20:20\\]
Memory access mode. Controls use of memory-access mode for accessing EDITR and the DCC. This bit is ignored if in Non-debug state and set to zero on entry to Debug state.Possible values of this field are: 0 Normal access mode 1 Memory access mode."]
pub type MaR = crate::BitReader;
#[doc = "Field `MA` writer - 20:20\\]
Memory access mode. Controls use of memory-access mode for accessing EDITR and the DCC. This bit is ignored if in Non-debug state and set to zero on entry to Debug state.Possible values of this field are: 0 Normal access mode 1 Memory access mode."]
pub type MaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDA` reader - 21:21\\]
Trap debug registers accesses."]
pub type TdaR = crate::BitReader;
#[doc = "Field `TDA` writer - 21:21\\]
Trap debug registers accesses."]
pub type TdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTDIS` reader - 23:22\\]
Interrupt disable. Disables taking interrupts \\[including virtual interrupts and System Error interrupts\\]
in Non-Debug state.If external invasive debug is disabled, the value of this field is ignored.If external invasive debug is enabled, the possible values of this field are: 00 Do not disable interrupts 01 Disable interrupts targeting Non-secure EL1. 10 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable interrupts targeting Secure EL1. 11 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable all other interrupts. The value of INTdis does not affect whether an interrupt is a WFI wake-up event, but can mask an interrupt as a WFE wake-up event.If EL3 and EL2 are not implemented, INTdis\\[0\\]
is RO and reads the same value as INTdis\\[1\\], meaning only the values 0b00 and 0b11 can be selected."]
pub type IntdisR = crate::FieldReader;
#[doc = "Field `INTDIS` writer - 23:22\\]
Interrupt disable. Disables taking interrupts \\[including virtual interrupts and System Error interrupts\\]
in Non-Debug state.If external invasive debug is disabled, the value of this field is ignored.If external invasive debug is enabled, the possible values of this field are: 00 Do not disable interrupts 01 Disable interrupts targeting Non-secure EL1. 10 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable interrupts targeting Secure EL1. 11 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable all other interrupts. The value of INTdis does not affect whether an interrupt is a WFI wake-up event, but can mask an interrupt as a WFE wake-up event.If EL3 and EL2 are not implemented, INTdis\\[0\\]
is RO and reads the same value as INTdis\\[1\\], meaning only the values 0b00 and 0b11 can be selected."]
pub type IntdisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ITE` reader - 24:24\\]
ITR empty. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. It is always valid in Debug state."]
pub type IteR = crate::BitReader;
#[doc = "Field `ITE` writer - 24:24\\]
ITR empty. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. It is always valid in Debug state."]
pub type IteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPEADV` reader - 25:25\\]
Pipeline advance. Read-only. Set to 1 every time the processor pipeline retires one or more instructions. Cleared to 0 by a write to EDRCR.CSPA.The architecture does not define precisely when this bit is set to 1. It requires only that this happen periodically in Non-debug state to indicate that software execution is progressing."]
pub type PipeadvR = crate::BitReader;
#[doc = "Field `PIPEADV` writer - 25:25\\]
Pipeline advance. Read-only. Set to 1 every time the processor pipeline retires one or more instructions. Cleared to 0 by a write to EDRCR.CSPA.The architecture does not define precisely when this bit is set to 1. It requires only that this happen periodically in Non-debug state to indicate that software execution is progressing."]
pub type PipeadvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXU` reader - 26:26\\]
DTRTX underrun. This bit is RO."]
pub type TxuR = crate::BitReader;
#[doc = "Field `TXU` writer - 26:26\\]
DTRTX underrun. This bit is RO."]
pub type TxuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXO` reader - 27:27\\]
DTRRX overrun. This bit is RO."]
pub type RxoR = crate::BitReader;
#[doc = "Field `RXO` writer - 27:27\\]
DTRRX overrun. This bit is RO."]
pub type RxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITO` reader - 28:28\\]
EDITR overrun. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. ITO is set to 0 on entry to Debug state."]
pub type ItoR = crate::BitReader;
#[doc = "Field `ITO` writer - 28:28\\]
EDITR overrun. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. ITO is set to 0 on entry to Debug state."]
pub type ItoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULL` reader - 29:29\\]
DTRTX full. This bit is RO."]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXFULL` writer - 29:29\\]
DTRTX full. This bit is RO."]
pub type TxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFULL` reader - 30:30\\]
DTRRX full. This bit is RO."]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXFULL` writer - 30:30\\]
DTRRX full. This bit is RO."]
pub type RxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_EDSCR_31_31` reader - 31:31\\]
Reserved, RES0."]
pub type Res0Edscr31_31R = crate::BitReader;
#[doc = "Field `RES0_EDSCR_31_31` writer - 31:31\\]
Reserved, RES0."]
pub type Res0Edscr31_31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Debug status flags. This field is RO.The possible values of this field are: 000010 Processor is in Non-debug state. 000001 Processor is restarting \\[exiting Debug state\\]. 000111 Breakpoint. 010011 External debug request. 011011 Halting step, normal. 011111 Halting step, exclusive. 100011 OS unlock catch. 100111 Reset catch. 101011 Watchpoint. 101111 HLT instruction. 110011 Software access to debug register. 110111 Exception catch. 111011 Halting step, no syndrome. All other values of STATUS are reserved."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Cumulative error flag. This field is RO. It is set to 1 following exceptions in Debug state and on any signaled overrun or underrun on the DTR or EDITR."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
System Error interrupt pending. Read-only. In Debug state, indicates whether a SError interrupt is pending:If HCR_EL2.{AMO, TGE} = {1, 0} and in Non-secure EL0 or EL1, a virtual SError interrupt.Otherwise, a physical SError interrupt. 0 No SError interrupt pending. 1 SError interrupt pending. A debugger can read EDSCR to check whether a SError interrupt is pending without having to execute further instructions. A pending SError might indicate data from target memory is corrupted.UNKNOWN in Non-debug state."]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Exception level. Read-only. In Debug state, this gives the current EL of the processor.In Non-debug state, this field is RAZ."]
    #[inline(always)]
    pub fn el(&self) -> ElR {
        ElR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Exception level register-width status. Read-only. In Debug state, each bit gives the current register width status of each EL: 1111 All exception levels are AArch64 state. 1110 EL0 is AArch32 state. All other exception levels are AArch64 state. 1100 EL0 and EL1 are AArch32 state. All other exception levels are AArch64 state. Never seen if EL2 is not implemented in the current security state. 1000 EL0, EL1, and, if implemented in the current security state, EL2 are AArch32 state. All other exception levels are AArch64 state. 0000 All exception levels are set to AArch32 state \\[32-bit configuration\\]. However:If not at EL0: RW\\[0\\]
== RW\\[1\\].If EL2 is not implemented in the current security state: RW\\[2\\]
== RW\\[1\\].If EL3 is not implemented: RW\\[3\\]
== RW\\[2\\].In Non-debug state, this field is RAO."]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Halting debug mode enable. Possible values of this bit are: 0 Halting debug mode disabled. 1 Halting debug mode enabled."]
    #[inline(always)]
    pub fn hde(&self) -> HdeR {
        HdeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edscr_15_15(&self) -> Res0Edscr15_15R {
        Res0Edscr15_15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Secure debug disabled. This bit is RO.On entry to Debug state:If entering in Secure state, SDD is set to 0.If entering in Non-secure state, SDD is set to the inverse of ExternalSecureInvasiveDebugEnabled\\[\\].In Debug state, the value of the SDD bit does not change, even if ExternalSecureInvasiveDebugEnabled\\[\\]
changes.In Non-debug state:SDD returns the inverse of ExternalSecureInvasiveDebugEnabled\\[\\]. If the authentication signals that control ExternalSecureInvasiveDebugEnabled\\[\\]
change, a context synchronization operation is required to guarantee their effect.This bit is unaffected by the Security state of the processor.If EL3 is not implemented and the implementation is Non-secure, this bit is RES1."]
    #[inline(always)]
    pub fn sdd(&self) -> SddR {
        SddR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edscr_17_17(&self) -> Res0Edscr17_17R {
        Res0Edscr17_17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Non-secure status. Read-only. When in Debug state, gives the current security state: 0 Secure state, IsSecure\\[\\]
== TRUE 1 Non-secure state, IsSecure\\[\\]
== FALSE. In Non-debug state, this bit is UNKNOWN."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edscr_19_19(&self) -> Res0Edscr19_19R {
        Res0Edscr19_19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Memory access mode. Controls use of memory-access mode for accessing EDITR and the DCC. This bit is ignored if in Non-debug state and set to zero on entry to Debug state.Possible values of this field are: 0 Normal access mode 1 Memory access mode."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Trap debug registers accesses."]
    #[inline(always)]
    pub fn tda(&self) -> TdaR {
        TdaR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Interrupt disable. Disables taking interrupts \\[including virtual interrupts and System Error interrupts\\]
in Non-Debug state.If external invasive debug is disabled, the value of this field is ignored.If external invasive debug is enabled, the possible values of this field are: 00 Do not disable interrupts 01 Disable interrupts targeting Non-secure EL1. 10 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable interrupts targeting Secure EL1. 11 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable all other interrupts. The value of INTdis does not affect whether an interrupt is a WFI wake-up event, but can mask an interrupt as a WFE wake-up event.If EL3 and EL2 are not implemented, INTdis\\[0\\]
is RO and reads the same value as INTdis\\[1\\], meaning only the values 0b00 and 0b11 can be selected."]
    #[inline(always)]
    pub fn intdis(&self) -> IntdisR {
        IntdisR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
ITR empty. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. It is always valid in Debug state."]
    #[inline(always)]
    pub fn ite(&self) -> IteR {
        IteR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Pipeline advance. Read-only. Set to 1 every time the processor pipeline retires one or more instructions. Cleared to 0 by a write to EDRCR.CSPA.The architecture does not define precisely when this bit is set to 1. It requires only that this happen periodically in Non-debug state to indicate that software execution is progressing."]
    #[inline(always)]
    pub fn pipeadv(&self) -> PipeadvR {
        PipeadvR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
DTRTX underrun. This bit is RO."]
    #[inline(always)]
    pub fn txu(&self) -> TxuR {
        TxuR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
DTRRX overrun. This bit is RO."]
    #[inline(always)]
    pub fn rxo(&self) -> RxoR {
        RxoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
EDITR overrun. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. ITO is set to 0 on entry to Debug state."]
    #[inline(always)]
    pub fn ito(&self) -> ItoR {
        ItoR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
DTRTX full. This bit is RO."]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
DTRRX full. This bit is RO."]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_edscr_31_31(&self) -> Res0Edscr31_31R {
        Res0Edscr31_31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Debug status flags. This field is RO.The possible values of this field are: 000010 Processor is in Non-debug state. 000001 Processor is restarting \\[exiting Debug state\\]. 000111 Breakpoint. 010011 External debug request. 011011 Halting step, normal. 011111 Halting step, exclusive. 100011 OS unlock catch. 100111 Reset catch. 101011 Watchpoint. 101111 HLT instruction. 110011 Software access to debug register. 110111 Exception catch. 111011 Halting step, no syndrome. All other values of STATUS are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<ApbaddrDbgCpu1EdscrSpec> {
        StatusW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Cumulative error flag. This field is RO. It is set to 1 following exceptions in Debug state and on any signaled overrun or underrun on the DTR or EDITR."]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<ApbaddrDbgCpu1EdscrSpec> {
        ErrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
System Error interrupt pending. Read-only. In Debug state, indicates whether a SError interrupt is pending:If HCR_EL2.{AMO, TGE} = {1, 0} and in Non-secure EL0 or EL1, a virtual SError interrupt.Otherwise, a physical SError interrupt. 0 No SError interrupt pending. 1 SError interrupt pending. A debugger can read EDSCR to check whether a SError interrupt is pending without having to execute further instructions. A pending SError might indicate data from target memory is corrupted.UNKNOWN in Non-debug state."]
    #[inline(always)]
    #[must_use]
    pub fn a(&mut self) -> AW<ApbaddrDbgCpu1EdscrSpec> {
        AW::new(self, 7)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Exception level. Read-only. In Debug state, this gives the current EL of the processor.In Non-debug state, this field is RAZ."]
    #[inline(always)]
    #[must_use]
    pub fn el(&mut self) -> ElW<ApbaddrDbgCpu1EdscrSpec> {
        ElW::new(self, 8)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Exception level register-width status. Read-only. In Debug state, each bit gives the current register width status of each EL: 1111 All exception levels are AArch64 state. 1110 EL0 is AArch32 state. All other exception levels are AArch64 state. 1100 EL0 and EL1 are AArch32 state. All other exception levels are AArch64 state. Never seen if EL2 is not implemented in the current security state. 1000 EL0, EL1, and, if implemented in the current security state, EL2 are AArch32 state. All other exception levels are AArch64 state. 0000 All exception levels are set to AArch32 state \\[32-bit configuration\\]. However:If not at EL0: RW\\[0\\]
== RW\\[1\\].If EL2 is not implemented in the current security state: RW\\[2\\]
== RW\\[1\\].If EL3 is not implemented: RW\\[3\\]
== RW\\[2\\].In Non-debug state, this field is RAO."]
    #[inline(always)]
    #[must_use]
    pub fn rw(&mut self) -> RwW<ApbaddrDbgCpu1EdscrSpec> {
        RwW::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
Halting debug mode enable. Possible values of this bit are: 0 Halting debug mode disabled. 1 Halting debug mode enabled."]
    #[inline(always)]
    #[must_use]
    pub fn hde(&mut self) -> HdeW<ApbaddrDbgCpu1EdscrSpec> {
        HdeW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edscr_15_15(&mut self) -> Res0Edscr15_15W<ApbaddrDbgCpu1EdscrSpec> {
        Res0Edscr15_15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Secure debug disabled. This bit is RO.On entry to Debug state:If entering in Secure state, SDD is set to 0.If entering in Non-secure state, SDD is set to the inverse of ExternalSecureInvasiveDebugEnabled\\[\\].In Debug state, the value of the SDD bit does not change, even if ExternalSecureInvasiveDebugEnabled\\[\\]
changes.In Non-debug state:SDD returns the inverse of ExternalSecureInvasiveDebugEnabled\\[\\]. If the authentication signals that control ExternalSecureInvasiveDebugEnabled\\[\\]
change, a context synchronization operation is required to guarantee their effect.This bit is unaffected by the Security state of the processor.If EL3 is not implemented and the implementation is Non-secure, this bit is RES1."]
    #[inline(always)]
    #[must_use]
    pub fn sdd(&mut self) -> SddW<ApbaddrDbgCpu1EdscrSpec> {
        SddW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edscr_17_17(&mut self) -> Res0Edscr17_17W<ApbaddrDbgCpu1EdscrSpec> {
        Res0Edscr17_17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Non-secure status. Read-only. When in Debug state, gives the current security state: 0 Secure state, IsSecure\\[\\]
== TRUE 1 Non-secure state, IsSecure\\[\\]
== FALSE. In Non-debug state, this bit is UNKNOWN."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<ApbaddrDbgCpu1EdscrSpec> {
        NsW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edscr_19_19(&mut self) -> Res0Edscr19_19W<ApbaddrDbgCpu1EdscrSpec> {
        Res0Edscr19_19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Memory access mode. Controls use of memory-access mode for accessing EDITR and the DCC. This bit is ignored if in Non-debug state and set to zero on entry to Debug state.Possible values of this field are: 0 Normal access mode 1 Memory access mode."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MaW<ApbaddrDbgCpu1EdscrSpec> {
        MaW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Trap debug registers accesses."]
    #[inline(always)]
    #[must_use]
    pub fn tda(&mut self) -> TdaW<ApbaddrDbgCpu1EdscrSpec> {
        TdaW::new(self, 21)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Interrupt disable. Disables taking interrupts \\[including virtual interrupts and System Error interrupts\\]
in Non-Debug state.If external invasive debug is disabled, the value of this field is ignored.If external invasive debug is enabled, the possible values of this field are: 00 Do not disable interrupts 01 Disable interrupts targeting Non-secure EL1. 10 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable interrupts targeting Secure EL1. 11 Disable interrupts targeting only Non-secure EL1 and Non-secure EL2. If external secure invasive debug is enabled, also disable all other interrupts. The value of INTdis does not affect whether an interrupt is a WFI wake-up event, but can mask an interrupt as a WFE wake-up event.If EL3 and EL2 are not implemented, INTdis\\[0\\]
is RO and reads the same value as INTdis\\[1\\], meaning only the values 0b00 and 0b11 can be selected."]
    #[inline(always)]
    #[must_use]
    pub fn intdis(&mut self) -> IntdisW<ApbaddrDbgCpu1EdscrSpec> {
        IntdisW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
ITR empty. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. It is always valid in Debug state."]
    #[inline(always)]
    #[must_use]
    pub fn ite(&mut self) -> IteW<ApbaddrDbgCpu1EdscrSpec> {
        IteW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Pipeline advance. Read-only. Set to 1 every time the processor pipeline retires one or more instructions. Cleared to 0 by a write to EDRCR.CSPA.The architecture does not define precisely when this bit is set to 1. It requires only that this happen periodically in Non-debug state to indicate that software execution is progressing."]
    #[inline(always)]
    #[must_use]
    pub fn pipeadv(&mut self) -> PipeadvW<ApbaddrDbgCpu1EdscrSpec> {
        PipeadvW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
DTRTX underrun. This bit is RO."]
    #[inline(always)]
    #[must_use]
    pub fn txu(&mut self) -> TxuW<ApbaddrDbgCpu1EdscrSpec> {
        TxuW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
DTRRX overrun. This bit is RO."]
    #[inline(always)]
    #[must_use]
    pub fn rxo(&mut self) -> RxoW<ApbaddrDbgCpu1EdscrSpec> {
        RxoW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
EDITR overrun. This bit is RO.If the processor is not in Debug state, this bit is UNKNOWN. ITO is set to 0 on entry to Debug state."]
    #[inline(always)]
    #[must_use]
    pub fn ito(&mut self) -> ItoW<ApbaddrDbgCpu1EdscrSpec> {
        ItoW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
DTRTX full. This bit is RO."]
    #[inline(always)]
    #[must_use]
    pub fn txfull(&mut self) -> TxfullW<ApbaddrDbgCpu1EdscrSpec> {
        TxfullW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
DTRRX full. This bit is RO."]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RxfullW<ApbaddrDbgCpu1EdscrSpec> {
        RxfullW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_edscr_31_31(&mut self) -> Res0Edscr31_31W<ApbaddrDbgCpu1EdscrSpec> {
        Res0Edscr31_31W::new(self, 31)
    }
}
#[doc = "External Debug Status and Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdscrSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edscr::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdscrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edscr::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdscrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDSCR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdscrSpec {
    const RESET_VALUE: u32 = 0;
}
