#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec>;
#[doc = "Field `DEBUGVER` reader - 3:0\\]
Debug architecture version. Indicates presence of v8-A debug architecture. 0110 v8-A debug architecture. All other values are reserved."]
pub type DebugverR = crate::FieldReader;
#[doc = "Field `DEBUGVER` writer - 3:0\\]
Debug architecture version. Indicates presence of v8-A debug architecture. 0110 v8-A debug architecture. All other values are reserved."]
pub type DebugverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRACEVER` reader - 7:4\\]
Trace extension. Indicates whether system register interface to Trace extension is implemented. Permitted values are: 0000 Trace extension system registers not implemented. 0001 Trace extension system registers implemented. All other values are reserved.A value of 0b0000 only indicates that no system register interface to the trace extension is implemented. A trace extension may nevertheless be implemented without a system register interface."]
pub type TraceverR = crate::FieldReader;
#[doc = "Field `TRACEVER` writer - 7:4\\]
Trace extension. Indicates whether system register interface to Trace extension is implemented. Permitted values are: 0000 Trace extension system registers not implemented. 0001 Trace extension system registers implemented. All other values are reserved.A value of 0b0000 only indicates that no system register interface to the trace extension is implemented. A trace extension may nevertheless be implemented without a system register interface."]
pub type TraceverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PMUVER` reader - 11:8\\]
Performance Monitors extension version. Indicates whether system register interface to Performance Monitors extension is implemented. Permitted values are: 0000 Performance Monitors extension system registers not implemented. 0001 Performance Monitors extension system registers implemented, PMUv3. 1111 IMPLEMENTATION DEFINED form of performance monitors supported, PMUv3 not supported. All other values are reserved."]
pub type PmuverR = crate::FieldReader;
#[doc = "Field `PMUVER` writer - 11:8\\]
Performance Monitors extension version. Indicates whether system register interface to Performance Monitors extension is implemented. Permitted values are: 0000 Performance Monitors extension system registers not implemented. 0001 Performance Monitors extension system registers implemented, PMUv3. 1111 IMPLEMENTATION DEFINED form of performance monitors supported, PMUv3 not supported. All other values are reserved."]
pub type PmuverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BRPS` reader - 15:12\\]
Number of breakpoints, minus 1. The value of 0b0000 is reserved."]
pub type BrpsR = crate::FieldReader;
#[doc = "Field `BRPS` writer - 15:12\\]
Number of breakpoints, minus 1. The value of 0b0000 is reserved."]
pub type BrpsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_31_0_19_16` reader - 19:16\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_31_0_19_16R = crate::FieldReader;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_31_0_19_16` writer - 19:16\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_31_0_19_16W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRPS` reader - 23:20\\]
Number of watchpoints, minus 1. The value of 0b0000 is reserved."]
pub type WrpsR = crate::FieldReader;
#[doc = "Field `WRPS` writer - 23:20\\]
Number of watchpoints, minus 1. The value of 0b0000 is reserved."]
pub type WrpsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_31_0_27_24` reader - 27:24\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_31_0_27_24R = crate::FieldReader;
#[doc = "Field `RES0_ID_AA64DFR0_EL1_31_0_27_24` writer - 27:24\\]
Reserved, RES0."]
pub type Res0IdAa64dfr0El1_31_0_27_24W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTX_CMPS` reader - 31:28\\]
Number of breakpoints that are context-aware, minus 1. These are the highest numbered breakpoints."]
pub type CtxCmpsR = crate::FieldReader;
#[doc = "Field `CTX_CMPS` writer - 31:28\\]
Number of breakpoints that are context-aware, minus 1. These are the highest numbered breakpoints."]
pub type CtxCmpsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Debug architecture version. Indicates presence of v8-A debug architecture. 0110 v8-A debug architecture. All other values are reserved."]
    #[inline(always)]
    pub fn debugver(&self) -> DebugverR {
        DebugverR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Trace extension. Indicates whether system register interface to Trace extension is implemented. Permitted values are: 0000 Trace extension system registers not implemented. 0001 Trace extension system registers implemented. All other values are reserved.A value of 0b0000 only indicates that no system register interface to the trace extension is implemented. A trace extension may nevertheless be implemented without a system register interface."]
    #[inline(always)]
    pub fn tracever(&self) -> TraceverR {
        TraceverR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Performance Monitors extension version. Indicates whether system register interface to Performance Monitors extension is implemented. Permitted values are: 0000 Performance Monitors extension system registers not implemented. 0001 Performance Monitors extension system registers implemented, PMUv3. 1111 IMPLEMENTATION DEFINED form of performance monitors supported, PMUv3 not supported. All other values are reserved."]
    #[inline(always)]
    pub fn pmuver(&self) -> PmuverR {
        PmuverR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of breakpoints, minus 1. The value of 0b0000 is reserved."]
    #[inline(always)]
    pub fn brps(&self) -> BrpsR {
        BrpsR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64dfr0_el1_31_0_19_16(&self) -> Res0IdAa64dfr0El1_31_0_19_16R {
        Res0IdAa64dfr0El1_31_0_19_16R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Number of watchpoints, minus 1. The value of 0b0000 is reserved."]
    #[inline(always)]
    pub fn wrps(&self) -> WrpsR {
        WrpsR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64dfr0_el1_31_0_27_24(&self) -> Res0IdAa64dfr0El1_31_0_27_24R {
        Res0IdAa64dfr0El1_31_0_27_24R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Number of breakpoints that are context-aware, minus 1. These are the highest numbered breakpoints."]
    #[inline(always)]
    pub fn ctx_cmps(&self) -> CtxCmpsR {
        CtxCmpsR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Debug architecture version. Indicates presence of v8-A debug architecture. 0110 v8-A debug architecture. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn debugver(&mut self) -> DebugverW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        DebugverW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Trace extension. Indicates whether system register interface to Trace extension is implemented. Permitted values are: 0000 Trace extension system registers not implemented. 0001 Trace extension system registers implemented. All other values are reserved.A value of 0b0000 only indicates that no system register interface to the trace extension is implemented. A trace extension may nevertheless be implemented without a system register interface."]
    #[inline(always)]
    #[must_use]
    pub fn tracever(&mut self) -> TraceverW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        TraceverW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Performance Monitors extension version. Indicates whether system register interface to Performance Monitors extension is implemented. Permitted values are: 0000 Performance Monitors extension system registers not implemented. 0001 Performance Monitors extension system registers implemented, PMUv3. 1111 IMPLEMENTATION DEFINED form of performance monitors supported, PMUv3 not supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn pmuver(&mut self) -> PmuverW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        PmuverW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Number of breakpoints, minus 1. The value of 0b0000 is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn brps(&mut self) -> BrpsW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        BrpsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64dfr0_el1_31_0_19_16(
        &mut self,
    ) -> Res0IdAa64dfr0El1_31_0_19_16W<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        Res0IdAa64dfr0El1_31_0_19_16W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Number of watchpoints, minus 1. The value of 0b0000 is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn wrps(&mut self) -> WrpsW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        WrpsW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64dfr0_el1_31_0_27_24(
        &mut self,
    ) -> Res0IdAa64dfr0El1_31_0_27_24W<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        Res0IdAa64dfr0El1_31_0_27_24W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Number of breakpoints that are context-aware, minus 1. These are the highest numbered breakpoints."]
    #[inline(always)]
    #[must_use]
    pub fn ctx_cmps(&mut self) -> CtxCmpsW<ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec> {
        CtxCmpsW::new(self, 28)
    }
}
#[doc = "Debug Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_id_aa64dfr0_el1_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_ID_AA64DFR0_EL1_31_0 to value 0x1030_5106"]
impl crate::Resettable for ApbaddrDbgCpu0IdAa64dfr0El1_31_0Spec {
    const RESET_VALUE: u32 = 0x1030_5106;
}
