#[doc = "Register `APBADDR_ETM_CPU1_TRCOSLSR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcoslsrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCOSLSR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcoslsrSpec>;
#[doc = "Field `RES0_TRCOSLSR_0_0` reader - 0:0\\]
Reserved, RES0."]
pub type Res0Trcoslsr0_0R = crate::BitReader;
#[doc = "Field `RES0_TRCOSLSR_0_0` writer - 0:0\\]
Reserved, RES0."]
pub type Res0Trcoslsr0_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCKED` reader - 1:1\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. When the trace unit core power domain is powered down the value is UNKNOWN. The TRCPDSR indicates if the trace unit core power domain is powered down."]
pub type LockedR = crate::BitReader;
#[doc = "Field `LOCKED` writer - 1:1\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. When the trace unit core power domain is powered down the value is UNKNOWN. The TRCPDSR indicates if the trace unit core power domain is powered down."]
pub type LockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT32` reader - 2:2\\]
This bit is RES0, which indicates that software must perform a 32-bit write to update the TRCOSLAR."]
pub type Bit32R = crate::BitReader;
#[doc = "Field `BIT32` writer - 2:2\\]
This bit is RES0, which indicates that software must perform a 32-bit write to update the TRCOSLAR."]
pub type Bit32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESENT` reader - 3:3\\]
Indicates whether the OS Lock is implemented.This bit is RES1, which indicates that the OS Lock is always implemented."]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - 3:3\\]
Indicates whether the OS Lock is implemented.This bit is RES1, which indicates that the OS Lock is always implemented."]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCOSLSR_31_4` reader - 31:4\\]
Reserved, RES0."]
pub type Res0Trcoslsr31_4R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCOSLSR_31_4` writer - 31:4\\]
Reserved, RES0."]
pub type Res0Trcoslsr31_4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcoslsr_0_0(&self) -> Res0Trcoslsr0_0R {
        Res0Trcoslsr0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. When the trace unit core power domain is powered down the value is UNKNOWN. The TRCPDSR indicates if the trace unit core power domain is powered down."]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is RES0, which indicates that software must perform a 32-bit write to update the TRCOSLAR."]
    #[inline(always)]
    pub fn bit32(&self) -> Bit32R {
        Bit32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates whether the OS Lock is implemented.This bit is RES1, which indicates that the OS Lock is always implemented."]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcoslsr_31_4(&self) -> Res0Trcoslsr31_4R {
        Res0Trcoslsr31_4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcoslsr_0_0(&mut self) -> Res0Trcoslsr0_0W<ApbaddrEtmCpu1TrcoslsrSpec> {
        Res0Trcoslsr0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. When the trace unit core power domain is powered down the value is UNKNOWN. The TRCPDSR indicates if the trace unit core power domain is powered down."]
    #[inline(always)]
    #[must_use]
    pub fn locked(&mut self) -> LockedW<ApbaddrEtmCpu1TrcoslsrSpec> {
        LockedW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is RES0, which indicates that software must perform a 32-bit write to update the TRCOSLAR."]
    #[inline(always)]
    #[must_use]
    pub fn bit32(&mut self) -> Bit32W<ApbaddrEtmCpu1TrcoslsrSpec> {
        Bit32W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Indicates whether the OS Lock is implemented.This bit is RES1, which indicates that the OS Lock is always implemented."]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<ApbaddrEtmCpu1TrcoslsrSpec> {
        PresentW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcoslsr_31_4(&mut self) -> Res0Trcoslsr31_4W<ApbaddrEtmCpu1TrcoslsrSpec> {
        Res0Trcoslsr31_4W::new(self, 4)
    }
}
#[doc = "OS Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcoslsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcoslsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcoslsrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcoslsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcoslsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcoslsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcoslsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcoslsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCOSLSR to value 0x0a"]
impl crate::Resettable for ApbaddrEtmCpu1TrcoslsrSpec {
    const RESET_VALUE: u32 = 0x0a;
}
