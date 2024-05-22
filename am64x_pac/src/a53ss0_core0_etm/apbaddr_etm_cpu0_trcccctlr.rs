#[doc = "Register `APBADDR_ETM_CPU0_TRCCCCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcccctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCCCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcccctlrSpec>;
#[doc = "Field `THRESHOLD` reader - 11:0\\]
Sets the threshold value for instruction trace cycle counting.The minimum threshold value that can be programmed into THRESHOLD is given in TRCIDR3.CCITMIN.Writing a value of zero might cause UNPREDICTABLE behaviour."]
pub type ThresholdR = crate::FieldReader<u16>;
#[doc = "Field `THRESHOLD` writer - 11:0\\]
Sets the threshold value for instruction trace cycle counting.The minimum threshold value that can be programmed into THRESHOLD is given in TRCIDR3.CCITMIN.Writing a value of zero might cause UNPREDICTABLE behaviour."]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RES0_TRCCCCTLR_31_12` reader - 31:12\\]
Reserved, RES0."]
pub type Res0Trcccctlr31_12R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCCCCTLR_31_12` writer - 31:12\\]
Reserved, RES0."]
pub type Res0Trcccctlr31_12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Sets the threshold value for instruction trace cycle counting.The minimum threshold value that can be programmed into THRESHOLD is given in TRCIDR3.CCITMIN.Writing a value of zero might cause UNPREDICTABLE behaviour."]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcccctlr_31_12(&self) -> Res0Trcccctlr31_12R {
        Res0Trcccctlr31_12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Sets the threshold value for instruction trace cycle counting.The minimum threshold value that can be programmed into THRESHOLD is given in TRCIDR3.CCITMIN.Writing a value of zero might cause UNPREDICTABLE behaviour."]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> ThresholdW<ApbaddrEtmCpu0TrcccctlrSpec> {
        ThresholdW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcccctlr_31_12(&mut self) -> Res0Trcccctlr31_12W<ApbaddrEtmCpu0TrcccctlrSpec> {
        Res0Trcccctlr31_12W::new(self, 12)
    }
}
#[doc = "Cycle Count Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcccctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcccctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcccctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcccctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcccctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcccctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcccctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcccctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCCCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcccctlrSpec {
    const RESET_VALUE: u32 = 0;
}
