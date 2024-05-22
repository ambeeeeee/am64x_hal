#[doc = "Register `APBADDR_PMU_CPU0_PMITCTRL` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmitctrlSpec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMITCTRL` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmitctrlSpec>;
#[doc = "Field `IME` reader - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
pub type ImeR = crate::BitReader;
#[doc = "Field `IME` writer - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
pub type ImeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_PMITCTRL_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Pmitctrl31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMITCTRL_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Pmitctrl31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
    #[inline(always)]
    pub fn ime(&self) -> ImeR {
        ImeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmitctrl_31_1(&self) -> Res0Pmitctrl31_1R {
        Res0Pmitctrl31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> ImeW<ApbaddrPmuCpu0PmitctrlSpec> {
        ImeW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmitctrl_31_1(&mut self) -> Res0Pmitctrl31_1W<ApbaddrPmuCpu0PmitctrlSpec> {
        Res0Pmitctrl31_1W::new(self, 1)
    }
}
#[doc = "Performance Monitors Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmitctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmitctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmitctrlSpec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmitctrl::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmitctrl::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmitctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMITCTRL to value 0"]
impl crate::Resettable for ApbaddrPmuCpu0PmitctrlSpec {
    const RESET_VALUE: u32 = 0;
}
