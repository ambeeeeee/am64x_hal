#[doc = "Register `APBADDR_ETM_CPU1_TRCIMSPEC0` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trcimspec0Spec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCIMSPEC0` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trcimspec0Spec>;
#[doc = "Field `SUPPORT` reader - 3:0\\]
Indicates whether the implementation supports IMPLEMENTATION DEFINED features. The permitted values are: 0000 No IMPLEMENTATION DEFINED features are supported. The EN field is RES0. and any other value, which indicates that IMPLEMENTATION DEFINED features are supported. Use of these values requires written permission from ARM."]
pub type SupportR = crate::FieldReader;
#[doc = "Field `SUPPORT` writer - 3:0\\]
Indicates whether the implementation supports IMPLEMENTATION DEFINED features. The permitted values are: 0000 No IMPLEMENTATION DEFINED features are supported. The EN field is RES0. and any other value, which indicates that IMPLEMENTATION DEFINED features are supported. Use of these values requires written permission from ARM."]
pub type SupportW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN` reader - 7:4\\]
If SUPPORT is not 0b0000, controls whether the IMPLEMENTATION DEFINED features are enabled. The permitted values are: 0000 The IMPLEMENTATION DEFINED features are not enabled. The trace unit must behave as if the IMPLEMENTATION DEFINED features are not supported. and any other value, which indicates that the trace unit behavior is IMPLEMENTATION DEFINED.If SUPPORT is 0b0000, this field is RES0."]
pub type EnR = crate::FieldReader;
#[doc = "Field `EN` writer - 7:4\\]
If SUPPORT is not 0b0000, controls whether the IMPLEMENTATION DEFINED features are enabled. The permitted values are: 0000 The IMPLEMENTATION DEFINED features are not enabled. The trace unit must behave as if the IMPLEMENTATION DEFINED features are not supported. and any other value, which indicates that the trace unit behavior is IMPLEMENTATION DEFINED.If SUPPORT is 0b0000, this field is RES0."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCIMSPEC0_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcimspec0_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCIMSPEC0_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcimspec0_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the implementation supports IMPLEMENTATION DEFINED features. The permitted values are: 0000 No IMPLEMENTATION DEFINED features are supported. The EN field is RES0. and any other value, which indicates that IMPLEMENTATION DEFINED features are supported. Use of these values requires written permission from ARM."]
    #[inline(always)]
    pub fn support(&self) -> SupportR {
        SupportR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
If SUPPORT is not 0b0000, controls whether the IMPLEMENTATION DEFINED features are enabled. The permitted values are: 0000 The IMPLEMENTATION DEFINED features are not enabled. The trace unit must behave as if the IMPLEMENTATION DEFINED features are not supported. and any other value, which indicates that the trace unit behavior is IMPLEMENTATION DEFINED.If SUPPORT is 0b0000, this field is RES0."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcimspec0_31_8(&self) -> Res0Trcimspec0_31_8R {
        Res0Trcimspec0_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the implementation supports IMPLEMENTATION DEFINED features. The permitted values are: 0000 No IMPLEMENTATION DEFINED features are supported. The EN field is RES0. and any other value, which indicates that IMPLEMENTATION DEFINED features are supported. Use of these values requires written permission from ARM."]
    #[inline(always)]
    #[must_use]
    pub fn support(&mut self) -> SupportW<ApbaddrEtmCpu1Trcimspec0Spec> {
        SupportW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
If SUPPORT is not 0b0000, controls whether the IMPLEMENTATION DEFINED features are enabled. The permitted values are: 0000 The IMPLEMENTATION DEFINED features are not enabled. The trace unit must behave as if the IMPLEMENTATION DEFINED features are not supported. and any other value, which indicates that the trace unit behavior is IMPLEMENTATION DEFINED.If SUPPORT is 0b0000, this field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ApbaddrEtmCpu1Trcimspec0Spec> {
        EnW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcimspec0_31_8(&mut self) -> Res0Trcimspec0_31_8W<ApbaddrEtmCpu1Trcimspec0Spec> {
        Res0Trcimspec0_31_8W::new(self, 8)
    }
}
#[doc = "Implementation Specific Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcimspec0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcimspec0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trcimspec0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trcimspec0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcimspec0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trcimspec0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcimspec0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trcimspec0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCIMSPEC0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trcimspec0Spec {
    const RESET_VALUE: u32 = 0;
}
