#[doc = "Register `APBADDR_ETM_CPU1_TRCCIDR1` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trccidr1Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCCIDR1` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trccidr1Spec>;
#[doc = "Field `PRMBL_1` reader - 3:0\\]
Preamble. Must read as 0x0."]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `PRMBL_1` writer - 3:0\\]
Preamble. Must read as 0x0."]
pub type Prmbl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLASS` reader - 7:4\\]
Component class. Reads as 0x9, to indicate that the ETM is a debug component, with CoreSight architecture compliant management registers."]
pub type ClassR = crate::FieldReader;
#[doc = "Field `CLASS` writer - 7:4\\]
Component class. Reads as 0x9, to indicate that the ETM is a debug component, with CoreSight architecture compliant management registers."]
pub type ClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCCIDR1_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trccidr1_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCCIDR1_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trccidr1_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Preamble. Must read as 0x0."]
    #[inline(always)]
    pub fn prmbl_1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Component class. Reads as 0x9, to indicate that the ETM is a debug component, with CoreSight architecture compliant management registers."]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trccidr1_31_8(&self) -> Res0Trccidr1_31_8R {
        Res0Trccidr1_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Preamble. Must read as 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn prmbl_1(&mut self) -> Prmbl1W<ApbaddrEtmCpu1Trccidr1Spec> {
        Prmbl1W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Component class. Reads as 0x9, to indicate that the ETM is a debug component, with CoreSight architecture compliant management registers."]
    #[inline(always)]
    #[must_use]
    pub fn class(&mut self) -> ClassW<ApbaddrEtmCpu1Trccidr1Spec> {
        ClassW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trccidr1_31_8(&mut self) -> Res0Trccidr1_31_8W<ApbaddrEtmCpu1Trccidr1Spec> {
        Res0Trccidr1_31_8W::new(self, 8)
    }
}
#[doc = "Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trccidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trccidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trccidr1Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trccidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trccidr1::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trccidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trccidr1::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trccidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCCIDR1 to value 0x90"]
impl crate::Resettable for ApbaddrEtmCpu1Trccidr1Spec {
    const RESET_VALUE: u32 = 0x90;
}
