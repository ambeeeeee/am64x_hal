#[doc = "Register `APBADDR_ETM_CPU1_TRCVIIECTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcviiectlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCVIIECTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcviiectlrSpec>;
#[doc = "Field `INCLUDE` reader - 7:0\\]
Include range field. Selects which address range comparator pairs are in use with ViewInst include control. Each bit represents an address range comparator pair, so bit\\[m\\]
controls the selection of address range comparator pair m. If bit\\[m\\]
is: 0 The address range that address range comparator pair m defines is not selected for ViewInst include control. 1 The address range that address range comparator pair m defines is selected for ViewInst include control. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI.Selecting no include comparators indicates that all instructions are included by default. The exclude control then indicates which ranges are excluded."]
pub type IncludeR = crate::FieldReader;
#[doc = "Field `INCLUDE` writer - 7:0\\]
Include range field. Selects which address range comparator pairs are in use with ViewInst include control. Each bit represents an address range comparator pair, so bit\\[m\\]
controls the selection of address range comparator pair m. If bit\\[m\\]
is: 0 The address range that address range comparator pair m defines is not selected for ViewInst include control. 1 The address range that address range comparator pair m defines is selected for ViewInst include control. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI.Selecting no include comparators indicates that all instructions are included by default. The exclude control then indicates which ranges are excluded."]
pub type IncludeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCVIIECTLR_15_8` reader - 15:8\\]
Reserved, RES0."]
pub type Res0Trcviiectlr15_8R = crate::FieldReader;
#[doc = "Field `RES0_TRCVIIECTLR_15_8` writer - 15:8\\]
Reserved, RES0."]
pub type Res0Trcviiectlr15_8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXCLUDE` reader - 23:16\\]
0 1 The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type ExcludeR = crate::FieldReader;
#[doc = "Field `EXCLUDE` writer - 23:16\\]
0 1 The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
pub type ExcludeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCVIIECTLR_31_24` reader - 31:24\\]
Reserved, RES0."]
pub type Res0Trcviiectlr31_24R = crate::FieldReader;
#[doc = "Field `RES0_TRCVIIECTLR_31_24` writer - 31:24\\]
Reserved, RES0."]
pub type Res0Trcviiectlr31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Include range field. Selects which address range comparator pairs are in use with ViewInst include control. Each bit represents an address range comparator pair, so bit\\[m\\]
controls the selection of address range comparator pair m. If bit\\[m\\]
is: 0 The address range that address range comparator pair m defines is not selected for ViewInst include control. 1 The address range that address range comparator pair m defines is selected for ViewInst include control. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI.Selecting no include comparators indicates that all instructions are included by default. The exclude control then indicates which ranges are excluded."]
    #[inline(always)]
    pub fn include(&self) -> IncludeR {
        IncludeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcviiectlr_15_8(&self) -> Res0Trcviiectlr15_8R {
        Res0Trcviiectlr15_8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
0 1 The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    pub fn exclude(&self) -> ExcludeR {
        ExcludeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcviiectlr_31_24(&self) -> Res0Trcviiectlr31_24R {
        Res0Trcviiectlr31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Include range field. Selects which address range comparator pairs are in use with ViewInst include control. Each bit represents an address range comparator pair, so bit\\[m\\]
controls the selection of address range comparator pair m. If bit\\[m\\]
is: 0 The address range that address range comparator pair m defines is not selected for ViewInst include control. 1 The address range that address range comparator pair m defines is selected for ViewInst include control. The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI.Selecting no include comparators indicates that all instructions are included by default. The exclude control then indicates which ranges are excluded."]
    #[inline(always)]
    #[must_use]
    pub fn include(&mut self) -> IncludeW<ApbaddrEtmCpu1TrcviiectlrSpec> {
        IncludeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcviiectlr_15_8(&mut self) -> Res0Trcviiectlr15_8W<ApbaddrEtmCpu1TrcviiectlrSpec> {
        Res0Trcviiectlr15_8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
0 1 The implemented width of the field, n, is IMPLEMENTATION DEFINED and is set by the value of TRCIDR4.NUMACPAIRS. Unimplemented bits are RAZ/WI."]
    #[inline(always)]
    #[must_use]
    pub fn exclude(&mut self) -> ExcludeW<ApbaddrEtmCpu1TrcviiectlrSpec> {
        ExcludeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcviiectlr_31_24(
        &mut self,
    ) -> Res0Trcviiectlr31_24W<ApbaddrEtmCpu1TrcviiectlrSpec> {
        Res0Trcviiectlr31_24W::new(self, 24)
    }
}
#[doc = "ViewInst Include-Exclude Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcviiectlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcviiectlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcviiectlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcviiectlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcviiectlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcviiectlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcviiectlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcviiectlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCVIIECTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcviiectlrSpec {
    const RESET_VALUE: u32 = 0;
}
