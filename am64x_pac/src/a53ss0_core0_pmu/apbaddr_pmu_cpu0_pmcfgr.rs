#[doc = "Register `APBADDR_PMU_CPU0_PMCFGR` reader"]
pub type R = crate::R<ApbaddrPmuCpu0PmcfgrSpec>;
#[doc = "Register `APBADDR_PMU_CPU0_PMCFGR` writer"]
pub type W = crate::W<ApbaddrPmuCpu0PmcfgrSpec>;
#[doc = "Field `N` reader - 7:0\\]
Number of counters implemented in addition to the cycle counter, PMCCNTR_EL0. The maximum number of event counters is 31, so bits\\[7:5\\]
are always RES0. 00000000 Only PMCCNTR_EL0 implemented. 00000001 PMCCNTR_EL0 plus one event counter implemented. and so on up to 0b00011111, which indicates PMCCNTR_EL0 and 31 event counters implemented."]
pub type NR = crate::FieldReader;
#[doc = "Field `N` writer - 7:0\\]
Number of counters implemented in addition to the cycle counter, PMCCNTR_EL0. The maximum number of event counters is 31, so bits\\[7:5\\]
are always RES0. 00000000 Only PMCCNTR_EL0 implemented. 00000001 PMCCNTR_EL0 plus one event counter implemented. and so on up to 0b00011111, which indicates PMCCNTR_EL0 and 31 event counters implemented."]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SIZE` reader - 13:8\\]
Size of counters. This field determines the spacing of counters in the memory-map.In v8-A the counters are at doubleword-aligned addresses, and the largest counter is 64-bits, so this field is 0b111111."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 13:8\\]
Size of counters. This field determines the spacing of counters in the memory-map.In v8-A the counters are at doubleword-aligned addresses, and the largest counter is 64-bits, so this field is 0b111111."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CC` reader - 14:14\\]
Dedicated cycle counter \\[counter 31\\]
supported. This bit is RES1."]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - 14:14\\]
Dedicated cycle counter \\[counter 31\\]
supported. This bit is RES1."]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCD` reader - 15:15\\]
Cycle counter has prescale. This is RES1 if AArch32 is supported at any EL, and RES0 otherwise. 0 PMCR_EL0.D is RES0. 1 PMCR_EL0.D is read/write."]
pub type CcdR = crate::BitReader;
#[doc = "Field `CCD` writer - 15:15\\]
Cycle counter has prescale. This is RES1 if AArch32 is supported at any EL, and RES0 otherwise. 0 PMCR_EL0.D is RES0. 1 PMCR_EL0.D is read/write."]
pub type CcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EX` reader - 16:16\\]
Export supported. Value is IMPLEMENTATION DEFINED. 0 PMCR_EL0.X is RES0. 1 PMCR_EL0.X is read/write."]
pub type ExR = crate::BitReader;
#[doc = "Field `EX` writer - 16:16\\]
Export supported. Value is IMPLEMENTATION DEFINED. 0 PMCR_EL0.X is RES0. 1 PMCR_EL0.X is read/write."]
pub type ExW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NA` reader - 17:17\\]
This feature is not supported, so this bit is RES0."]
pub type NaR = crate::BitReader;
#[doc = "Field `NA` writer - 17:17\\]
This feature is not supported, so this bit is RES0."]
pub type NaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WT` reader - 18:18\\]
This feature is not supported, so this bit is RES0."]
pub type WtR = crate::BitReader;
#[doc = "Field `WT` writer - 18:18\\]
This feature is not supported, so this bit is RES0."]
pub type WtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - 19:19\\]
User-mode Enable Register supported. PMUSERENR_EL0 is not visible in the external debug interface, so this bit is RES0."]
pub type UenR = crate::BitReader;
#[doc = "Field `UEN` writer - 19:19\\]
User-mode Enable Register supported. PMUSERENR_EL0 is not visible in the external debug interface, so this bit is RES0."]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_PMCFGR_31_20` reader - 31:20\\]
Reserved, RES0."]
pub type Res0Pmcfgr31_20R = crate::FieldReader<u16>;
#[doc = "Field `RES0_PMCFGR_31_20` writer - 31:20\\]
Reserved, RES0."]
pub type Res0Pmcfgr31_20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of counters implemented in addition to the cycle counter, PMCCNTR_EL0. The maximum number of event counters is 31, so bits\\[7:5\\]
are always RES0. 00000000 Only PMCCNTR_EL0 implemented. 00000001 PMCCNTR_EL0 plus one event counter implemented. and so on up to 0b00011111, which indicates PMCCNTR_EL0 and 31 event counters implemented."]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Size of counters. This field determines the spacing of counters in the memory-map.In v8-A the counters are at doubleword-aligned addresses, and the largest counter is 64-bits, so this field is 0b111111."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Dedicated cycle counter \\[counter 31\\]
supported. This bit is RES1."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Cycle counter has prescale. This is RES1 if AArch32 is supported at any EL, and RES0 otherwise. 0 PMCR_EL0.D is RES0. 1 PMCR_EL0.D is read/write."]
    #[inline(always)]
    pub fn ccd(&self) -> CcdR {
        CcdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Export supported. Value is IMPLEMENTATION DEFINED. 0 PMCR_EL0.X is RES0. 1 PMCR_EL0.X is read/write."]
    #[inline(always)]
    pub fn ex(&self) -> ExR {
        ExR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
This feature is not supported, so this bit is RES0."]
    #[inline(always)]
    pub fn na(&self) -> NaR {
        NaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
This feature is not supported, so this bit is RES0."]
    #[inline(always)]
    pub fn wt(&self) -> WtR {
        WtR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
User-mode Enable Register supported. PMUSERENR_EL0 is not visible in the external debug interface, so this bit is RES0."]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_pmcfgr_31_20(&self) -> Res0Pmcfgr31_20R {
        Res0Pmcfgr31_20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of counters implemented in addition to the cycle counter, PMCCNTR_EL0. The maximum number of event counters is 31, so bits\\[7:5\\]
are always RES0. 00000000 Only PMCCNTR_EL0 implemented. 00000001 PMCCNTR_EL0 plus one event counter implemented. and so on up to 0b00011111, which indicates PMCCNTR_EL0 and 31 event counters implemented."]
    #[inline(always)]
    #[must_use]
    pub fn n(&mut self) -> NW<ApbaddrPmuCpu0PmcfgrSpec> {
        NW::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Size of counters. This field determines the spacing of counters in the memory-map.In v8-A the counters are at doubleword-aligned addresses, and the largest counter is 64-bits, so this field is 0b111111."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<ApbaddrPmuCpu0PmcfgrSpec> {
        SizeW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
Dedicated cycle counter \\[counter 31\\]
supported. This bit is RES1."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<ApbaddrPmuCpu0PmcfgrSpec> {
        CcW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Cycle counter has prescale. This is RES1 if AArch32 is supported at any EL, and RES0 otherwise. 0 PMCR_EL0.D is RES0. 1 PMCR_EL0.D is read/write."]
    #[inline(always)]
    #[must_use]
    pub fn ccd(&mut self) -> CcdW<ApbaddrPmuCpu0PmcfgrSpec> {
        CcdW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Export supported. Value is IMPLEMENTATION DEFINED. 0 PMCR_EL0.X is RES0. 1 PMCR_EL0.X is read/write."]
    #[inline(always)]
    #[must_use]
    pub fn ex(&mut self) -> ExW<ApbaddrPmuCpu0PmcfgrSpec> {
        ExW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
This feature is not supported, so this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn na(&mut self) -> NaW<ApbaddrPmuCpu0PmcfgrSpec> {
        NaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
This feature is not supported, so this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WtW<ApbaddrPmuCpu0PmcfgrSpec> {
        WtW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
User-mode Enable Register supported. PMUSERENR_EL0 is not visible in the external debug interface, so this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UenW<ApbaddrPmuCpu0PmcfgrSpec> {
        UenW::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_pmcfgr_31_20(&mut self) -> Res0Pmcfgr31_20W<ApbaddrPmuCpu0PmcfgrSpec> {
        Res0Pmcfgr31_20W::new(self, 20)
    }
}
#[doc = "Performance Monitors Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_pmu_cpu0_pmcfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_pmu_cpu0_pmcfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrPmuCpu0PmcfgrSpec;
impl crate::RegisterSpec for ApbaddrPmuCpu0PmcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_pmu_cpu0_pmcfgr::R`](R) reader structure"]
impl crate::Readable for ApbaddrPmuCpu0PmcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_pmu_cpu0_pmcfgr::W`](W) writer structure"]
impl crate::Writable for ApbaddrPmuCpu0PmcfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_PMU_CPU0_PMCFGR to value 0x0001_e306"]
impl crate::Resettable for ApbaddrPmuCpu0PmcfgrSpec {
    const RESET_VALUE: u32 = 0x0001_e306;
}
