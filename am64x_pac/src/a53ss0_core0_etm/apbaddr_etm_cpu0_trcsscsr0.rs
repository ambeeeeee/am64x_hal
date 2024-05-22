#[doc = "Register `APBADDR_ETM_CPU0_TRCSSCSR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcsscsr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCSSCSR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcsscsr0Spec>;
#[doc = "Field `INST` reader - 0:0\\]
Instruction address comparator support bit. Indicates if the trace unit supports instruction address comparisons: 0 Single-shot instruction address comparisons are not supported. 1 Single-shot instruction address comparisons are supported."]
pub type InstR = crate::BitReader;
#[doc = "Field `INST` writer - 0:0\\]
Instruction address comparator support bit. Indicates if the trace unit supports instruction address comparisons: 0 Single-shot instruction address comparisons are not supported. 1 Single-shot instruction address comparisons are supported."]
pub type InstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - 1:1\\]
Data address comparator support bit. Indicates if the trace unit supports data address comparisons: 0 Single-shot data address comparisons are not supported. 1 Single-shot data address comparisons are supported."]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - 1:1\\]
Data address comparator support bit. Indicates if the trace unit supports data address comparisons: 0 Single-shot data address comparisons are not supported. 1 Single-shot data address comparisons are supported."]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DV` reader - 2:2\\]
Data value comparator support bit. Indicates if the trace unit supports data address with data value comparisons: 0 Single-shot data address with data value comparisons are not supported. 1 Single-shot data address with data value comparisons are supported."]
pub type DvR = crate::BitReader;
#[doc = "Field `DV` writer - 2:2\\]
Data value comparator support bit. Indicates if the trace unit supports data address with data value comparisons: 0 Single-shot data address with data value comparisons are not supported. 1 Single-shot data address with data value comparisons are supported."]
pub type DvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCSSCSR0_30_3` reader - 30:3\\]
Reserved, RES0."]
pub type Res0Trcsscsr0_30_3R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCSSCSR0_30_3` writer - 30:3\\]
Reserved, RES0."]
pub type Res0Trcsscsr0_30_3W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `STATUS` reader - 31:31\\]
Single-shot status bit. Indicates if any of the comparators that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects have matched: 0 No match has occurred. 1 One or more matches has occurred. If TRCSSCCRn.RST==0 then there is only one match and no more matches are possible, and software must reset this bit to 0 to re-enable the single-shot control. STATUS must be written to set an initial state when configuring the trace unit, if the single-shot comparator is to be used."]
pub type StatusR = crate::BitReader;
#[doc = "Field `STATUS` writer - 31:31\\]
Single-shot status bit. Indicates if any of the comparators that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects have matched: 0 No match has occurred. 1 One or more matches has occurred. If TRCSSCCRn.RST==0 then there is only one match and no more matches are possible, and software must reset this bit to 0 to re-enable the single-shot control. STATUS must be written to set an initial state when configuring the trace unit, if the single-shot comparator is to be used."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Instruction address comparator support bit. Indicates if the trace unit supports instruction address comparisons: 0 Single-shot instruction address comparisons are not supported. 1 Single-shot instruction address comparisons are supported."]
    #[inline(always)]
    pub fn inst(&self) -> InstR {
        InstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data address comparator support bit. Indicates if the trace unit supports data address comparisons: 0 Single-shot data address comparisons are not supported. 1 Single-shot data address comparisons are supported."]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data value comparator support bit. Indicates if the trace unit supports data address with data value comparisons: 0 Single-shot data address with data value comparisons are not supported. 1 Single-shot data address with data value comparisons are supported."]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:30 - 30:3\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcsscsr0_30_3(&self) -> Res0Trcsscsr0_30_3R {
        Res0Trcsscsr0_30_3R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Single-shot status bit. Indicates if any of the comparators that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects have matched: 0 No match has occurred. 1 One or more matches has occurred. If TRCSSCCRn.RST==0 then there is only one match and no more matches are possible, and software must reset this bit to 0 to re-enable the single-shot control. STATUS must be written to set an initial state when configuring the trace unit, if the single-shot comparator is to be used."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Instruction address comparator support bit. Indicates if the trace unit supports instruction address comparisons: 0 Single-shot instruction address comparisons are not supported. 1 Single-shot instruction address comparisons are supported."]
    #[inline(always)]
    #[must_use]
    pub fn inst(&mut self) -> InstW<ApbaddrEtmCpu0Trcsscsr0Spec> {
        InstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data address comparator support bit. Indicates if the trace unit supports data address comparisons: 0 Single-shot data address comparisons are not supported. 1 Single-shot data address comparisons are supported."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<ApbaddrEtmCpu0Trcsscsr0Spec> {
        DaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Data value comparator support bit. Indicates if the trace unit supports data address with data value comparisons: 0 Single-shot data address with data value comparisons are not supported. 1 Single-shot data address with data value comparisons are supported."]
    #[inline(always)]
    #[must_use]
    pub fn dv(&mut self) -> DvW<ApbaddrEtmCpu0Trcsscsr0Spec> {
        DvW::new(self, 2)
    }
    #[doc = "Bits 3:30 - 30:3\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcsscsr0_30_3(&mut self) -> Res0Trcsscsr0_30_3W<ApbaddrEtmCpu0Trcsscsr0Spec> {
        Res0Trcsscsr0_30_3W::new(self, 3)
    }
    #[doc = "Bit 31 - 31:31\\]
Single-shot status bit. Indicates if any of the comparators that TRCSSCCRn.SAC or TRCSSCCRn.ARC selects have matched: 0 No match has occurred. 1 One or more matches has occurred. If TRCSSCCRn.RST==0 then there is only one match and no more matches are possible, and software must reset this bit to 0 to re-enable the single-shot control. STATUS must be written to set an initial state when configuring the trace unit, if the single-shot comparator is to be used."]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> StatusW<ApbaddrEtmCpu0Trcsscsr0Spec> {
        StatusW::new(self, 31)
    }
}
#[doc = "Single-Shot Comparator Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcsscsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcsscsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcsscsr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcsscsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcsscsr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcsscsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcsscsr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcsscsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCSSCSR0 to value 0x01"]
impl crate::Resettable for ApbaddrEtmCpu0Trcsscsr0Spec {
    const RESET_VALUE: u32 = 0x01;
}
