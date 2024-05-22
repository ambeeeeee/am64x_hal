#[doc = "Register `APBADDR_PMU_CPU0_PMCIDR0` reader"]
pub type R = crate::R<ApbaddrPmuCpu0Pmcidr0Spec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCIDR0` writer"]
pub type W = crate::W<ApbaddrPmuCpu0Pmcidr0Spec>;
#[doc = "Field `PRMBL_0` reader - 7:0\\]
Preamble. Must read as 0x0D."]
pub type Prmbl0R = crate::FieldReader;
#[doc = "Field `PRMBL_0` writer - 7:0\\]
Preamble. Must read as 0x0D."]
pub type Prmbl0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_PMCIDR0_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Pmcidr0_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMCIDR0_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Pmcidr0_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0x0D."]
    #[inline(always)]
    pub fn prmbl_0(&self) -> Prmbl0R {
        Prmbl0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmcidr0_31_8(&self) -> Res0Pmcidr0_31_8R {
        Res0Pmcidr0_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0x0D."]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_0(&mut self) -> Prmbl0W<ApbaddrPmuCpu0Pmcidr0Spec> {
        Prmbl0W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmcidr0_31_8(&mut self) -> Res0Pmcidr0_31_8W<ApbaddrPmuCpu0Pmcidr0Spec> {
        Res0Pmcidr0_31_8W::new(self, 8)
    }
}
#[doc = "Performance Monitors Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcidr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcidr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0Pmcidr0Spec;
impl crate::RegisterSpec for ApbaddrPmuCpu0Pmcidr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmcidr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0Pmcidr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmcidr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0Pmcidr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCIDR0 to value 0x13"]
impl crate::Resettable for ApbaddrPmuCpu0Pmcidr0Spec {
    const RESET_VALUE: u32 = 0x13;
}
