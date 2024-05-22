#[doc = "Register `APBADDR_PMU_CPU1_PMAUTHSTATUS` reader"]
pub type R = crate::R<ApbaddrPmuCpu1PmauthstatusSpec>;
#[doc = "Register `APBADDR_PMU_CPU1_PMAUTHSTATUS` writer"]
pub type W = crate::W<ApbaddrPmuCpu1PmauthstatusSpec>;
#[doc = "Field `RES0_PMAUTHSTATUS_1_0` reader - 1:0\\]
Reserved, RES0."]
pub type Res0Pmauthstatus1_0R = crate::FieldReader;
#[doc = "Field `RES0_PMAUTHSTATUS_1_0` writer - 1:0\\]
Reserved, RES0."]
pub type Res0Pmauthstatus1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NSNID` reader - 3:2\\]
Holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
pub type NsnidR = crate::FieldReader;
#[doc = "Field `NSNID` writer - 3:2\\]
Holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_PMAUTHSTATUS_5_4` reader - 5:4\\]
Reserved, RES0."]
pub type Res0Pmauthstatus5_4R = crate::FieldReader;
#[doc = "Field `RES0_PMAUTHSTATUS_5_4` writer - 5:4\\]
Reserved, RES0."]
pub type Res0Pmauthstatus5_4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNID` reader - 7:6\\]
Holds the same value as DBGAUTHSTATUS_EL1.SNID."]
pub type SnidR = crate::FieldReader;
#[doc = "Field `SNID` writer - 7:6\\]
Holds the same value as DBGAUTHSTATUS_EL1.SNID."]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES0_PMAUTHSTATUS_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Pmauthstatus31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_PMAUTHSTATUS_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Pmauthstatus31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmauthstatus_1_0(&self) -> Res0Pmauthstatus1_0R {
        Res0Pmauthstatus1_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmauthstatus_5_4(&self) -> Res0Pmauthstatus5_4R {
        Res0Pmauthstatus5_4R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Holds the same value as DBGAUTHSTATUS_EL1.SNID."]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmauthstatus_31_8(&self) -> Res0Pmauthstatus31_8R {
        Res0Pmauthstatus31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmauthstatus_1_0(
        &mut self,
    ) -> Res0Pmauthstatus1_0W<ApbaddrPmuCpu1PmauthstatusSpec> {
        Res0Pmauthstatus1_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Holds the same value as DBGAUTHSTATUS_EL1.NSNID."]
    #[inline(always)]
    #[must_use]
    pub fn nsnid(&mut self) -> NsnidW<ApbaddrPmuCpu1PmauthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmauthstatus_5_4(
        &mut self,
    ) -> Res0Pmauthstatus5_4W<ApbaddrPmuCpu1PmauthstatusSpec> {
        Res0Pmauthstatus5_4W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Holds the same value as DBGAUTHSTATUS_EL1.SNID."]
    #[inline(always)]
    #[must_use]
    pub fn snid(&mut self) -> SnidW<ApbaddrPmuCpu1PmauthstatusSpec> {
        SnidW::new(self, 6)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmauthstatus_31_8(
        &mut self,
    ) -> Res0Pmauthstatus31_8W<ApbaddrPmuCpu1PmauthstatusSpec> {
        Res0Pmauthstatus31_8W::new(self, 8)
    }
}
#[doc = "Performance Monitors Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu1_pmauthstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu1_pmauthstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu1PmauthstatusSpec;
impl crate::RegisterSpec for ApbaddrPmuCpu1PmauthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu1_pmauthstatus::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu1PmauthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu1_pmauthstatus::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu1PmauthstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU1_PMAUTHSTATUS to value 0x88"]
impl crate::Resettable for ApbaddrPmuCpu1PmauthstatusSpec {
    const RESET_VALUE: u32 = 0x88;
}
