#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR3` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcpidr3Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCPIDR3` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcpidr3Spec>;
#[doc = "Field `CMOD` reader - 3:0\\]
Customer modified. Indicates someone other than the Designer has modified the component."]
pub type CmodR = crate::FieldReader;
#[doc = "Field `CMOD` writer - 3:0\\]
Customer modified. Indicates someone other than the Designer has modified the component."]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REVAND` reader - 7:4\\]
The IMPLEMENTATION DEFINED manufacturing revision number for the implementation. After silicon is available, if metal fixes are necessary, the manufacturer can alter the top metal layer so that this field can indicate any post-fab silicon changes."]
pub type RevandR = crate::FieldReader;
#[doc = "Field `REVAND` writer - 7:4\\]
The IMPLEMENTATION DEFINED manufacturing revision number for the implementation. After silicon is available, if metal fixes are necessary, the manufacturer can alter the top metal layer so that this field can indicate any post-fab silicon changes."]
pub type RevandW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES0_TRCPIDR3_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr3_31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCPIDR3_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Trcpidr3_31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Customer modified. Indicates someone other than the Designer has modified the component."]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The IMPLEMENTATION DEFINED manufacturing revision number for the implementation. After silicon is available, if metal fixes are necessary, the manufacturer can alter the top metal layer so that this field can indicate any post-fab silicon changes."]
    #[inline(always)]
    pub fn revand(&self) -> RevandR {
        RevandR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcpidr3_31_8(&self) -> Res0Trcpidr3_31_8R {
        Res0Trcpidr3_31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Customer modified. Indicates someone other than the Designer has modified the component."]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CmodW<ApbaddrEtmCpu0Trcpidr3Spec> {
        CmodW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The IMPLEMENTATION DEFINED manufacturing revision number for the implementation. After silicon is available, if metal fixes are necessary, the manufacturer can alter the top metal layer so that this field can indicate any post-fab silicon changes."]
    #[inline(always)]
    #[must_use]
    pub fn revand(&mut self) -> RevandW<ApbaddrEtmCpu0Trcpidr3Spec> {
        RevandW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcpidr3_31_8(&mut self) -> Res0Trcpidr3_31_8W<ApbaddrEtmCpu0Trcpidr3Spec> {
        Res0Trcpidr3_31_8W::new(self, 8)
    }
}
#[doc = "Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcpidr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcpidr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcpidr3Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcpidr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcpidr3::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcpidr3Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcpidr3::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcpidr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCPIDR3 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcpidr3Spec {
    const RESET_VALUE: u32 = 0;
}
