#[doc = "Register `APBADDR_PMU_CPU1_PMDEVTYPE` reader"]
pub type R = crate::R<ApbaddrPmuCpu1PmdevtypeSpec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMDEVTYPE` writer"]
pub type W = crate::W<ApbaddrPmuCpu1PmdevtypeSpec>;
#[doc = "Field `MAJOR` reader - 3:0\\]
Major type. Must read as 0x6 to indicate this is a performance monitor component."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - 3:0\\]
Major type. Must read as 0x6 to indicate this is a performance monitor component."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUB` reader - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
pub type SubR = crate::FieldReader;
#[doc = "Field `SUB` writer - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
pub type SubW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_PMDEVTYPE_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Pmdevtype31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMDEVTYPE_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Pmdevtype31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Major type. Must read as 0x6 to indicate this is a performance monitor component."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
    #[inline(always)]
    pub fn sub(&self) -> SubR {
        SubR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmdevtype_31_8(&self) -> Res0Pmdevtype31_8R {
        Res0Pmdevtype31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Major type. Must read as 0x6 to indicate this is a performance monitor component."]
    #[inline(always)]
    #[must_use]
    pub fn major(&mut self) -> MajorW<ApbaddrPmuCpu1PmdevtypeSpec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Subtype. Must read as 0x1 to indicate this is a processor component."]
    #[inline(always)]
    #[must_use]
    pub fn sub(&mut self) -> SubW<ApbaddrPmuCpu1PmdevtypeSpec> {
        SubW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmdevtype_31_8(&mut self) -> Res0Pmdevtype31_8W<ApbaddrPmuCpu1PmdevtypeSpec> {
        Res0Pmdevtype31_8W::new(self, 8)
    }
}
#[doc = "Performance Monitors Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmdevtype::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmdevtype::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1PmdevtypeSpec;
impl crate::RegisterSpec for ApbaddrPmuCpu1PmdevtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmdevtype::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1PmdevtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmdevtype::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1PmdevtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMDEVTYPE to value 0x16"]
impl crate::Resettable for ApbaddrPmuCpu1PmdevtypeSpec {
    const RESET_VALUE: u32 = 0x16;
}
