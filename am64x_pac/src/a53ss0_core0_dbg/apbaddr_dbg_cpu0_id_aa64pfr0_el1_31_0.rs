#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_31_0` reader"]
pub type R = crate::R<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec>;
#[doc = "Register `APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_31_0` writer"]
pub type W = crate::W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec>;
#[doc = "Field `EL0` reader - 3:0\\]
EL0 exception level handling. Permitted values are: 0000 EL0 is not implemented. 0001 EL0 can be executed in AArch64 state only. 0010 EL0 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El0R = crate::FieldReader;
#[doc = "Field `EL0` writer - 3:0\\]
EL0 exception level handling. Permitted values are: 0000 EL0 is not implemented. 0001 EL0 can be executed in AArch64 state only. 0010 EL0 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EL1` reader - 7:4\\]
EL1 exception level handling. Permitted values are: 0000 EL1 is not implemented. 0001 EL1 can be executed in AArch64 state only. 0010 EL1 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El1R = crate::FieldReader;
#[doc = "Field `EL1` writer - 7:4\\]
EL1 exception level handling. Permitted values are: 0000 EL1 is not implemented. 0001 EL1 can be executed in AArch64 state only. 0010 EL1 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EL2` reader - 11:8\\]
EL2 exception level handling. Permitted values are: 0000 EL2 is not implemented. 0001 EL2 can be executed in AArch64 state only. 0010 EL2 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El2R = crate::FieldReader;
#[doc = "Field `EL2` writer - 11:8\\]
EL2 exception level handling. Permitted values are: 0000 EL2 is not implemented. 0001 EL2 can be executed in AArch64 state only. 0010 EL2 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EL3` reader - 15:12\\]
EL3 exception level handling. Permitted values are: 0000 EL3 is not implemented. 0001 EL3 can be executed in AArch64 state only. 0010 EL3 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El3R = crate::FieldReader;
#[doc = "Field `EL3` writer - 15:12\\]
EL3 exception level handling. Permitted values are: 0000 EL3 is not implemented. 0001 EL3 can be executed in AArch64 state only. 0010 EL3 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
pub type El3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FP` reader - 19:16\\]
Floating-point. Permitted values are: 0000 Floating-point is implemented. 1111 Floating-point is not implemented. All other values are reserved."]
pub type FpR = crate::FieldReader;
#[doc = "Field `FP` writer - 19:16\\]
Floating-point. Permitted values are: 0000 Floating-point is implemented. 1111 Floating-point is not implemented. All other values are reserved."]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADVSIMD` reader - 23:20\\]
Advanced SIMD. Permitted values are: 0000 Advanced SIMD is implemented. 1111 Advanced SIMD is not implemented. All other values are reserved."]
pub type AdvsimdR = crate::FieldReader;
#[doc = "Field `ADVSIMD` writer - 23:20\\]
Advanced SIMD. Permitted values are: 0000 Advanced SIMD is implemented. 1111 Advanced SIMD is not implemented. All other values are reserved."]
pub type AdvsimdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GIC` reader - 27:24\\]
GIC system register interface. Permitted values are: 0000 No GIC system registers are supported. 0001 GICv3 system registers are supported. All other values are reserved."]
pub type GicR = crate::FieldReader;
#[doc = "Field `GIC` writer - 27:24\\]
GIC system register interface. Permitted values are: 0000 No GIC system registers are supported. 0001 GICv3 system registers are supported. All other values are reserved."]
pub type GicW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_ID_AA64PFR0_EL1_31_0_31_28` reader - 31:28\\]
Reserved, RES0."]
pub type Res0IdAa64pfr0El1_31_0_31_28R = crate::FieldReader;
#[doc = "Field `RES0_ID_AA64PFR0_EL1_31_0_31_28` writer - 31:28\\]
Reserved, RES0."]
pub type Res0IdAa64pfr0El1_31_0_31_28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
EL0 exception level handling. Permitted values are: 0000 EL0 is not implemented. 0001 EL0 can be executed in AArch64 state only. 0010 EL0 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    pub fn el0(&self) -> El0R {
        El0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
EL1 exception level handling. Permitted values are: 0000 EL1 is not implemented. 0001 EL1 can be executed in AArch64 state only. 0010 EL1 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    pub fn el1(&self) -> El1R {
        El1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
EL2 exception level handling. Permitted values are: 0000 EL2 is not implemented. 0001 EL2 can be executed in AArch64 state only. 0010 EL2 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    pub fn el2(&self) -> El2R {
        El2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
EL3 exception level handling. Permitted values are: 0000 EL3 is not implemented. 0001 EL3 can be executed in AArch64 state only. 0010 EL3 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    pub fn el3(&self) -> El3R {
        El3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Floating-point. Permitted values are: 0000 Floating-point is implemented. 1111 Floating-point is not implemented. All other values are reserved."]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Advanced SIMD. Permitted values are: 0000 Advanced SIMD is implemented. 1111 Advanced SIMD is not implemented. All other values are reserved."]
    #[inline(always)]
    pub fn advsimd(&self) -> AdvsimdR {
        AdvsimdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
GIC system register interface. Permitted values are: 0000 No GIC system registers are supported. 0001 GICv3 system registers are supported. All other values are reserved."]
    #[inline(always)]
    pub fn gic(&self) -> GicR {
        GicR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_id_aa64pfr0_el1_31_0_31_28(&self) -> Res0IdAa64pfr0El1_31_0_31_28R {
        Res0IdAa64pfr0El1_31_0_31_28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
EL0 exception level handling. Permitted values are: 0000 EL0 is not implemented. 0001 EL0 can be executed in AArch64 state only. 0010 EL0 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn el0(&mut self) -> El0W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        El0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
EL1 exception level handling. Permitted values are: 0000 EL1 is not implemented. 0001 EL1 can be executed in AArch64 state only. 0010 EL1 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn el1(&mut self) -> El1W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        El1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
EL2 exception level handling. Permitted values are: 0000 EL2 is not implemented. 0001 EL2 can be executed in AArch64 state only. 0010 EL2 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn el2(&mut self) -> El2W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        El2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
EL3 exception level handling. Permitted values are: 0000 EL3 is not implemented. 0001 EL3 can be executed in AArch64 state only. 0010 EL3 can be executed in either AArch64 or AArch32 state. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn el3(&mut self) -> El3W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        El3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Floating-point. Permitted values are: 0000 Floating-point is implemented. 1111 Floating-point is not implemented. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FpW<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        FpW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Advanced SIMD. Permitted values are: 0000 Advanced SIMD is implemented. 1111 Advanced SIMD is not implemented. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn advsimd(&mut self) -> AdvsimdW<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        AdvsimdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
GIC system register interface. Permitted values are: 0000 No GIC system registers are supported. 0001 GICv3 system registers are supported. All other values are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn gic(&mut self) -> GicW<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        GicW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_id_aa64pfr0_el1_31_0_31_28(
        &mut self,
    ) -> Res0IdAa64pfr0El1_31_0_31_28W<ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec> {
        Res0IdAa64pfr0El1_31_0_31_28W::new(self, 28)
    }
}
#[doc = "Processor Feature Register 0 (low word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec;
impl crate::RegisterSpec for ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu0_id_aa64pfr0_el1_31_0::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU0_ID_AA64PFR0_EL1_31_0 to value 0x0100_2222"]
impl crate::Resettable for ApbaddrDbgCpu0IdAa64pfr0El1_31_0Spec {
    const RESET_VALUE: u32 = 0x0100_2222;
}
