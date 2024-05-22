#[doc = "Register `APBADDR_ETM_CPU0_TRCCIDR2` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trccidr2Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCIDR2` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trccidr2Spec>;
#[doc = "Field `PRMBL_2` reader - 7:0\\]
Preamble. Must read as 0x05."]
pub type Prmbl2R = crate::FieldReader;
#[doc = "Field `PRMBL_2` writer - 7:0\\]
Preamble. Must read as 0x05."]
pub type Prmbl2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_TRCCIDR2_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trccidr2_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCCIDR2_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trccidr2_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0x05."]
    #[inline(always)]
    pub fn prmbl_2(&self) -> Prmbl2R {
        Prmbl2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trccidr2_31_8(&self) -> Res0Trccidr2_31_8R {
        Res0Trccidr2_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Preamble. Must read as 0x05."]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_2(&mut self) -> Prmbl2W<ApbaddrEtmCpu0Trccidr2Spec> {
        Prmbl2W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trccidr2_31_8(&mut self) -> Res0Trccidr2_31_8W<ApbaddrEtmCpu0Trccidr2Spec> {
        Res0Trccidr2_31_8W::new(self, 8)
    }
}
#[doc = "Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trccidr2Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trccidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trccidr2::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trccidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trccidr2::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trccidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCIDR2 to value 0x05"]
impl crate::Resettable for ApbaddrEtmCpu0Trccidr2Spec {
    const RESET_VALUE: u32 = 0x05;
}
