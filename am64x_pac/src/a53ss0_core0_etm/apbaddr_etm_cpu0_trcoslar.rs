#[doc = "Register `APBADDR_ETM_CPU0_TRCOSLAR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcoslarSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCOSLAR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcoslarSpec>;
#[doc = "Field `LOCK` reader - 0:0\\]
OS Lock control bit: 0 Unlocks the OS Lock. 1 Locks the OS Lock. This setting disables the trace unit."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 0:0\\]
OS Lock control bit: 0 Unlocks the OS Lock. 1 Locks the OS Lock. This setting disables the trace unit."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCOSLAR_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Trcoslar31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCOSLAR_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Trcoslar31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
OS Lock control bit: 0 Unlocks the OS Lock. 1 Locks the OS Lock. This setting disables the trace unit."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcoslar_31_1(&self) -> Res0Trcoslar31_1R {
        Res0Trcoslar31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
OS Lock control bit: 0 Unlocks the OS Lock. 1 Locks the OS Lock. This setting disables the trace unit."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<ApbaddrEtmCpu0TrcoslarSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcoslar_31_1(&mut self) -> Res0Trcoslar31_1W<ApbaddrEtmCpu0TrcoslarSpec> {
        Res0Trcoslar31_1W::new(self, 1)
    }
}
#[doc = "OS Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcoslar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcoslar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcoslarSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcoslarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcoslar::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcoslarSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcoslar::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcoslarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCOSLAR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcoslarSpec {
    const RESET_VALUE: u32 = 0;
}
