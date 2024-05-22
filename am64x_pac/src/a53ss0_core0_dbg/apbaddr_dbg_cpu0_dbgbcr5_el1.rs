#[doc = "Register `APBADDR_DBG_CPU0_DBGBCR5_EL1` reader"]
pub type R = crate::R<ApbaddrDbgCpu0Dbgbcr5El1Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_DBGBCR5_EL1` writer"]
pub type W = crate::W<ApbaddrDbgCpu0Dbgbcr5El1Spec>;
#[doc = "Field `E` reader - 0:0\\]
Enable breakpoint DBGBVR&lt;n>_EL1. Possible values are: 0 Breakpoint disabled. 1 Breakpoint enabled."]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - 0:0\\]
Enable breakpoint DBGBVR&lt;n>_EL1. Possible values are: 0 Breakpoint disabled. 1 Breakpoint enabled."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC` reader - 2:1\\]
Privilege mode control. Determines the exception level or levels at which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
pub type PmcR = crate::FieldReader;
#[doc = "Field `PMC` writer - 2:1\\]
Privilege mode control. Determines the exception level or levels at which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
pub type PmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_DBGBCR5_EL1_4_3` reader - 4:3\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_4_3R = crate::FieldReader;
#[doc = "Field `RES0_DBGBCR5_EL1_4_3` writer - 4:3\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BAS` reader - 8:5\\]
Byte address select. Defines which half-words an address-matching breakpoint matches, regardless of the instruction set and execution state. In an AArch64-only implementation, this field is reserved, RES1. Otherwise:BAS\\[2\\]
and BAS\\[0\\]
are read/write.BAS\\[3\\]
and BAS\\[1\\]
are read-only copies of BAS\\[2\\]
and BAS\\[0\\]
respectively.The values 0b0011 and 0b1100 are only supported if AArch32 is supported at any exception level.The permitted values depend on the breakpoint type.For Address match breakpoints in either AArch32 or AArch64 state:BASMatch instruction atConstraint for debuggers0b0011DBGBVR&lt;n>_EL1Use for T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for A64 and A32 instructions.0b0000 is reserved and must behave as if the breakpoint is disabled or map to a permitted value.For Address mismatch breakpoints in an AArch32 stage 1 translation regime:BASStep instruction atConstraint for debuggers0b0000-Use for a match anywhere breakpoint.0b0011DBGBVR&lt;n>_EL1Use for stepping T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for stepping T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for stepping A64 and A32 instructions.For Context matching breakpoints, this field is RES1 and ignored."]
pub type BasR = crate::FieldReader;
#[doc = "Field `BAS` writer - 8:5\\]
Byte address select. Defines which half-words an address-matching breakpoint matches, regardless of the instruction set and execution state. In an AArch64-only implementation, this field is reserved, RES1. Otherwise:BAS\\[2\\]
and BAS\\[0\\]
are read/write.BAS\\[3\\]
and BAS\\[1\\]
are read-only copies of BAS\\[2\\]
and BAS\\[0\\]
respectively.The values 0b0011 and 0b1100 are only supported if AArch32 is supported at any exception level.The permitted values depend on the breakpoint type.For Address match breakpoints in either AArch32 or AArch64 state:BASMatch instruction atConstraint for debuggers0b0011DBGBVR&lt;n>_EL1Use for T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for A64 and A32 instructions.0b0000 is reserved and must behave as if the breakpoint is disabled or map to a permitted value.For Address mismatch breakpoints in an AArch32 stage 1 translation regime:BASStep instruction atConstraint for debuggers0b0000-Use for a match anywhere breakpoint.0b0011DBGBVR&lt;n>_EL1Use for stepping T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for stepping T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for stepping A64 and A32 instructions.For Context matching breakpoints, this field is RES1 and ignored."]
pub type BasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_DBGBCR5_EL1_12_9` reader - 12:9\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_12_9R = crate::FieldReader;
#[doc = "Field `RES0_DBGBCR5_EL1_12_9` writer - 12:9\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_12_9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HMC` reader - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and PMC fields."]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and PMC fields."]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSC` reader - 15:14\\]
Security state control. Determines the security states under which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the HMC and PMC fields."]
pub type SscR = crate::FieldReader;
#[doc = "Field `SSC` writer - 15:14\\]
Security state control. Determines the security states under which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the HMC and PMC fields."]
pub type SscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBN` reader - 19:16\\]
Linked breakpoint number. For Linked address matching breakpoints, this specifies the index of the Context-matching breakpoint linked to."]
pub type LbnR = crate::FieldReader;
#[doc = "Field `LBN` writer - 19:16\\]
Linked breakpoint number. For Linked address matching breakpoints, this specifies the index of the Context-matching breakpoint linked to."]
pub type LbnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BT` reader - 23:20\\]
Breakpoint Type. Possible values are: 0000 Unlinked instruction address match. 0001 Linked instruction address match. 0010 Unlinked context ID match. 0011 Linked context ID match 0100 Unlinked instruction address mismatch. 0101 Linked instruction address mismatch. 1000 Unlinked VMID match. 1001 Linked VMID match. 1010 Unlinked VMID and context ID match. 1011 Linked VMID and context ID match. The field breaks down as follows:BT\\[3:1\\]: Base type.000Match address. DBGBVR&lt;n>_EL1 is the address of an instruction.010Mismatch address. Behaves as type 0b000 if in an AArch64 translation, or if halting debug-mode is enabled and halting is allowed. Otherwise, DBGBVR&lt;n>_EL1 is the address of an instruction to be stepped.001Match context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID.100Match VMID. DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.101Match VMID and context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID, and DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.BT\\[0\\]: Enable linking.If the breakpoint is not context-aware, BT\\[3\\]
and BT\\[1\\]
are RES0. If EL2 is not implemented, BT\\[3\\]
is RES0. If EL1 using AArch32 is not implemented, BT\\[2\\]
is RES0.The values 011x and 11xx are reserved, but must behave as if the breakpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture."]
pub type BtR = crate::FieldReader;
#[doc = "Field `BT` writer - 23:20\\]
Breakpoint Type. Possible values are: 0000 Unlinked instruction address match. 0001 Linked instruction address match. 0010 Unlinked context ID match. 0011 Linked context ID match 0100 Unlinked instruction address mismatch. 0101 Linked instruction address mismatch. 1000 Unlinked VMID match. 1001 Linked VMID match. 1010 Unlinked VMID and context ID match. 1011 Linked VMID and context ID match. The field breaks down as follows:BT\\[3:1\\]: Base type.000Match address. DBGBVR&lt;n>_EL1 is the address of an instruction.010Mismatch address. Behaves as type 0b000 if in an AArch64 translation, or if halting debug-mode is enabled and halting is allowed. Otherwise, DBGBVR&lt;n>_EL1 is the address of an instruction to be stepped.001Match context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID.100Match VMID. DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.101Match VMID and context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID, and DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.BT\\[0\\]: Enable linking.If the breakpoint is not context-aware, BT\\[3\\]
and BT\\[1\\]
are RES0. If EL2 is not implemented, BT\\[3\\]
is RES0. If EL1 using AArch32 is not implemented, BT\\[2\\]
is RES0.The values 011x and 11xx are reserved, but must behave as if the breakpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture."]
pub type BtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_DBGBCR5_EL1_31_24` reader - 31:24\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_31_24R = crate::FieldReader;
#[doc = "Field `RES0_DBGBCR5_EL1_31_24` writer - 31:24\\]
Reserved, RES0."]
pub type Res0Dbgbcr5El1_31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable breakpoint DBGBVR&lt;n>_EL1. Possible values are: 0 Breakpoint disabled. 1 Breakpoint enabled."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Privilege mode control. Determines the exception level or levels at which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
    #[inline(always)]
    pub fn pmc(&self) -> PmcR {
        PmcR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgbcr5_el1_4_3(&self) -> Res0Dbgbcr5El1_4_3R {
        Res0Dbgbcr5El1_4_3R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Byte address select. Defines which half-words an address-matching breakpoint matches, regardless of the instruction set and execution state. In an AArch64-only implementation, this field is reserved, RES1. Otherwise:BAS\\[2\\]
and BAS\\[0\\]
are read/write.BAS\\[3\\]
and BAS\\[1\\]
are read-only copies of BAS\\[2\\]
and BAS\\[0\\]
respectively.The values 0b0011 and 0b1100 are only supported if AArch32 is supported at any exception level.The permitted values depend on the breakpoint type.For Address match breakpoints in either AArch32 or AArch64 state:BASMatch instruction atConstraint for debuggers0b0011DBGBVR&lt;n>_EL1Use for T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for A64 and A32 instructions.0b0000 is reserved and must behave as if the breakpoint is disabled or map to a permitted value.For Address mismatch breakpoints in an AArch32 stage 1 translation regime:BASStep instruction atConstraint for debuggers0b0000-Use for a match anywhere breakpoint.0b0011DBGBVR&lt;n>_EL1Use for stepping T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for stepping T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for stepping A64 and A32 instructions.For Context matching breakpoints, this field is RES1 and ignored."]
    #[inline(always)]
    pub fn bas(&self) -> BasR {
        BasR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgbcr5_el1_12_9(&self) -> Res0Dbgbcr5El1_12_9R {
        Res0Dbgbcr5El1_12_9R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and PMC fields."]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Security state control. Determines the security states under which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the HMC and PMC fields."]
    #[inline(always)]
    pub fn ssc(&self) -> SscR {
        SscR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Linked breakpoint number. For Linked address matching breakpoints, this specifies the index of the Context-matching breakpoint linked to."]
    #[inline(always)]
    pub fn lbn(&self) -> LbnR {
        LbnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Breakpoint Type. Possible values are: 0000 Unlinked instruction address match. 0001 Linked instruction address match. 0010 Unlinked context ID match. 0011 Linked context ID match 0100 Unlinked instruction address mismatch. 0101 Linked instruction address mismatch. 1000 Unlinked VMID match. 1001 Linked VMID match. 1010 Unlinked VMID and context ID match. 1011 Linked VMID and context ID match. The field breaks down as follows:BT\\[3:1\\]: Base type.000Match address. DBGBVR&lt;n>_EL1 is the address of an instruction.010Mismatch address. Behaves as type 0b000 if in an AArch64 translation, or if halting debug-mode is enabled and halting is allowed. Otherwise, DBGBVR&lt;n>_EL1 is the address of an instruction to be stepped.001Match context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID.100Match VMID. DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.101Match VMID and context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID, and DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.BT\\[0\\]: Enable linking.If the breakpoint is not context-aware, BT\\[3\\]
and BT\\[1\\]
are RES0. If EL2 is not implemented, BT\\[3\\]
is RES0. If EL1 using AArch32 is not implemented, BT\\[2\\]
is RES0.The values 011x and 11xx are reserved, but must behave as if the breakpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture."]
    #[inline(always)]
    pub fn bt(&self) -> BtR {
        BtR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_dbgbcr5_el1_31_24(&self) -> Res0Dbgbcr5El1_31_24R {
        Res0Dbgbcr5El1_31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable breakpoint DBGBVR&lt;n>_EL1. Possible values are: 0 Breakpoint disabled. 1 Breakpoint enabled."]
    #[inline(always)]
    #[must_use]
    pub fn e(&mut self) -> EW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Privilege mode control. Determines the exception level or levels at which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and HMC fields."]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PmcW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        PmcW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgbcr5_el1_4_3(&mut self) -> Res0Dbgbcr5El1_4_3W<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        Res0Dbgbcr5El1_4_3W::new(self, 3)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Byte address select. Defines which half-words an address-matching breakpoint matches, regardless of the instruction set and execution state. In an AArch64-only implementation, this field is reserved, RES1. Otherwise:BAS\\[2\\]
and BAS\\[0\\]
are read/write.BAS\\[3\\]
and BAS\\[1\\]
are read-only copies of BAS\\[2\\]
and BAS\\[0\\]
respectively.The values 0b0011 and 0b1100 are only supported if AArch32 is supported at any exception level.The permitted values depend on the breakpoint type.For Address match breakpoints in either AArch32 or AArch64 state:BASMatch instruction atConstraint for debuggers0b0011DBGBVR&lt;n>_EL1Use for T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for A64 and A32 instructions.0b0000 is reserved and must behave as if the breakpoint is disabled or map to a permitted value.For Address mismatch breakpoints in an AArch32 stage 1 translation regime:BASStep instruction atConstraint for debuggers0b0000-Use for a match anywhere breakpoint.0b0011DBGBVR&lt;n>_EL1Use for stepping T32 and T32EE instructions.0b1100DBGBVR&lt;n>_EL1+2Use for stepping T32 and T32EE instructions.0b1111DBGBVR&lt;n>_EL1Use for stepping A64 and A32 instructions.For Context matching breakpoints, this field is RES1 and ignored."]
    #[inline(always)]
    #[must_use]
    pub fn bas(&mut self) -> BasW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        BasW::new(self, 5)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgbcr5_el1_12_9(&mut self) -> Res0Dbgbcr5El1_12_9W<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        Res0Dbgbcr5El1_12_9W::new(self, 9)
    }
    #[doc = "Bit 13 - 13:13\\]
Higher mode control. Determines the debug perspective for deciding when a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the SSC and PMC fields."]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        HmcW::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Security state control. Determines the security states under which a breakpoint debug event for breakpoint n is generated. This field must be interpreted along with the HMC and PMC fields."]
    #[inline(always)]
    #[must_use]
    pub fn ssc(&mut self) -> SscW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        SscW::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Linked breakpoint number. For Linked address matching breakpoints, this specifies the index of the Context-matching breakpoint linked to."]
    #[inline(always)]
    #[must_use]
    pub fn lbn(&mut self) -> LbnW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        LbnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Breakpoint Type. Possible values are: 0000 Unlinked instruction address match. 0001 Linked instruction address match. 0010 Unlinked context ID match. 0011 Linked context ID match 0100 Unlinked instruction address mismatch. 0101 Linked instruction address mismatch. 1000 Unlinked VMID match. 1001 Linked VMID match. 1010 Unlinked VMID and context ID match. 1011 Linked VMID and context ID match. The field breaks down as follows:BT\\[3:1\\]: Base type.000Match address. DBGBVR&lt;n>_EL1 is the address of an instruction.010Mismatch address. Behaves as type 0b000 if in an AArch64 translation, or if halting debug-mode is enabled and halting is allowed. Otherwise, DBGBVR&lt;n>_EL1 is the address of an instruction to be stepped.001Match context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID.100Match VMID. DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.101Match VMID and context ID. DBGBVR&lt;n>_EL1\\[31_0\\]
is a context ID, and DBGBVR&lt;n>_EL1\\[39:32\\]
is a VMID.BT\\[0\\]: Enable linking.If the breakpoint is not context-aware, BT\\[3\\]
and BT\\[1\\]
are RES0. If EL2 is not implemented, BT\\[3\\]
is RES0. If EL1 using AArch32 is not implemented, BT\\[2\\]
is RES0.The values 011x and 11xx are reserved, but must behave as if the breakpoint is disabled. Software must not rely on this property as the behavior of reserved values might change in a future revision of the architecture."]
    #[inline(always)]
    #[must_use]
    pub fn bt(&mut self) -> BtW<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        BtW::new(self, 20)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_dbgbcr5_el1_31_24(
        &mut self,
    ) -> Res0Dbgbcr5El1_31_24W<ApbaddrDbgCpu0Dbgbcr5El1Spec> {
        Res0Dbgbcr5El1_31_24W::new(self, 24)
    }
}
#[doc = "Debug Breakpoint Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_dbgbcr5_el1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_dbgbcr5_el1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0Dbgbcr5El1Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0Dbgbcr5El1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_dbgbcr5_el1::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0Dbgbcr5El1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_dbgbcr5_el1::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0Dbgbcr5El1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_DBGBCR5_EL1 to value 0"]
impl crate::Resettable for ApbaddrDbgCpu0Dbgbcr5El1Spec {
    const RESET_VALUE: u32 = 0;
}
