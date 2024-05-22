#[doc = "Register `APBADDR_ETM_CPU0_TRCPDSR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcpdsrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCPDSR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcpdsrSpec>;
#[doc = "Field `POWER` reader - 0:0\\]
Power status bit: 0 The trace unit core power domain is not powered. The trace registers are not accessible and they all return an error response. 1 The trace unit core power domain is powered. The trace registers are accessible."]
pub type PowerR = crate::BitReader;
#[doc = "Field `POWER` writer - 0:0\\]
Power status bit: 0 The trace unit core power domain is not powered. The trace registers are not accessible and they all return an error response. 1 The trace unit core power domain is powered. The trace registers are accessible."]
pub type PowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STICKYPD` reader - 1:1\\]
Sticky powerdown status bit. Indicates whether the trace register state is valid: 0 If POWER==1 then the state of TRCOSLSR and the trace registers are valid. If POWER==0 then it is UNKNOWN whether the state of TRCOSLSR and the trace registers are valid. 1 The state of TRCOSLSR and the trace registers might not be valid. The trace unit sets this bit to 1 if either the trace unit is reset, or the power to the trace unit core power domain is removed and the trace register state is not valid. After this register is read, if the Software Lock is unlocked and the trace unit core power domain is powered up, then the trace unit sets this bit to 0. The TRCLAR controls whether the Software Lock is locked."]
pub type StickypdR = crate::BitReader;
#[doc = "Field `STICKYPD` writer - 1:1\\]
Sticky powerdown status bit. Indicates whether the trace register state is valid: 0 If POWER==1 then the state of TRCOSLSR and the trace registers are valid. If POWER==0 then it is UNKNOWN whether the state of TRCOSLSR and the trace registers are valid. 1 The state of TRCOSLSR and the trace registers might not be valid. The trace unit sets this bit to 1 if either the trace unit is reset, or the power to the trace unit core power domain is removed and the trace register state is not valid. After this register is read, if the Software Lock is unlocked and the trace unit core power domain is powered up, then the trace unit sets this bit to 0. The TRCLAR controls whether the Software Lock is locked."]
pub type StickypdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCPDSR_4_2` reader - 4:2\\]
Reserved, RES0."]
pub type Res0Trcpdsr4_2R = crate::FieldReader;
#[doc = "Field `RES0_TRCPDSR_4_2` writer - 4:2\\]
Reserved, RES0."]
pub type Res0Trcpdsr4_2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LOCKED` reader - 5:5\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. The value is UNKNOWN when the trace unit core power domain is powered down, that is, when POWER==0."]
pub type LockedR = crate::BitReader;
#[doc = "Field `LOCKED` writer - 5:5\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. The value is UNKNOWN when the trace unit core power domain is powered down, that is, when POWER==0."]
pub type LockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCPDSR_31_6` reader - 31:6\\]
Reserved, RES0."]
pub type Res0Trcpdsr31_6R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPDSR_31_6` writer - 31:6\\]
Reserved, RES0."]
pub type Res0Trcpdsr31_6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Power status bit: 0 The trace unit core power domain is not powered. The trace registers are not accessible and they all return an error response. 1 The trace unit core power domain is powered. The trace registers are accessible."]
    #[inline(always)]
    pub fn power(&self) -> PowerR {
        PowerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sticky powerdown status bit. Indicates whether the trace register state is valid: 0 If POWER==1 then the state of TRCOSLSR and the trace registers are valid. If POWER==0 then it is UNKNOWN whether the state of TRCOSLSR and the trace registers are valid. 1 The state of TRCOSLSR and the trace registers might not be valid. The trace unit sets this bit to 1 if either the trace unit is reset, or the power to the trace unit core power domain is removed and the trace register state is not valid. After this register is read, if the Software Lock is unlocked and the trace unit core power domain is powered up, then the trace unit sets this bit to 0. The TRCLAR controls whether the Software Lock is locked."]
    #[inline(always)]
    pub fn stickypd(&self) -> StickypdR {
        StickypdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpdsr_4_2(&self) -> Res0Trcpdsr4_2R {
        Res0Trcpdsr4_2R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. The value is UNKNOWN when the trace unit core power domain is powered down, that is, when POWER==0."]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpdsr_31_6(&self) -> Res0Trcpdsr31_6R {
        Res0Trcpdsr31_6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power status bit: 0 The trace unit core power domain is not powered. The trace registers are not accessible and they all return an error response. 1 The trace unit core power domain is powered. The trace registers are accessible."]
    #[inline(always)]
    #[must_use]
    pub fn power(&mut self) -> PowerW<ApbaddrEtmCpu0TrcpdsrSpec> {
        PowerW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sticky powerdown status bit. Indicates whether the trace register state is valid: 0 If POWER==1 then the state of TRCOSLSR and the trace registers are valid. If POWER==0 then it is UNKNOWN whether the state of TRCOSLSR and the trace registers are valid. 1 The state of TRCOSLSR and the trace registers might not be valid. The trace unit sets this bit to 1 if either the trace unit is reset, or the power to the trace unit core power domain is removed and the trace register state is not valid. After this register is read, if the Software Lock is unlocked and the trace unit core power domain is powered up, then the trace unit sets this bit to 0. The TRCLAR controls whether the Software Lock is locked."]
    #[inline(always)]
    #[must_use]
    pub fn stickypd(&mut self) -> StickypdW<ApbaddrEtmCpu0TrcpdsrSpec> {
        StickypdW::new(self, 1)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpdsr_4_2(&mut self) -> Res0Trcpdsr4_2W<ApbaddrEtmCpu0TrcpdsrSpec> {
        Res0Trcpdsr4_2W::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
OS Lock status bit: 0 The OS Lock is unlocked. 1 The OS Lock is locked. The value is UNKNOWN when the trace unit core power domain is powered down, that is, when POWER==0."]
    #[inline(always)]
    #[must_use]
    pub fn locked(&mut self) -> LockedW<ApbaddrEtmCpu0TrcpdsrSpec> {
        LockedW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpdsr_31_6(&mut self) -> Res0Trcpdsr31_6W<ApbaddrEtmCpu0TrcpdsrSpec> {
        Res0Trcpdsr31_6W::new(self, 6)
    }
}
#[doc = "Power Down Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpdsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpdsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcpdsrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcpdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcpdsr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcpdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcpdsr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcpdsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCPDSR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcpdsrSpec {
    const RESET_VALUE: u32 = 0;
}
