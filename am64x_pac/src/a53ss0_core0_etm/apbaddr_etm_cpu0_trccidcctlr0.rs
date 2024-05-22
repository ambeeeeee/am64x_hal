#[doc = "Register `APBADDR_ETM_CPU0_TRCCIDCCTLR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trccidcctlr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCIDCCTLR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trccidcctlr0Spec>;
#[doc = "Field `COMP_N` reader - 31:0\\]
Controls the mask value that the trace unit applies to TRCCIDCVRn. Each bit in this field corresponds to a byte in TRCCIDCVRn. When a bit is: 0 The trace unit includes the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. 1 The trace unit ignores the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. Supported only if TRCIDR4.NUMCIDC > n, otherwise the field is RES0."]
pub type CompNR = crate::FieldReader<u32>;
#[doc = "Field `COMP_N` writer - 31:0\\]
Controls the mask value that the trace unit applies to TRCCIDCVRn. Each bit in this field corresponds to a byte in TRCCIDCVRn. When a bit is: 0 The trace unit includes the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. 1 The trace unit ignores the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. Supported only if TRCIDR4.NUMCIDC > n, otherwise the field is RES0."]
pub type CompNW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Controls the mask value that the trace unit applies to TRCCIDCVRn. Each bit in this field corresponds to a byte in TRCCIDCVRn. When a bit is: 0 The trace unit includes the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. 1 The trace unit ignores the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. Supported only if TRCIDR4.NUMCIDC > n, otherwise the field is RES0."]
    #[inline(always)]
    pub fn comp_n(&self) -> CompNR {
        CompNR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Controls the mask value that the trace unit applies to TRCCIDCVRn. Each bit in this field corresponds to a byte in TRCCIDCVRn. When a bit is: 0 The trace unit includes the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. 1 The trace unit ignores the relevant byte in TRCCIDCVRn when it performs the Context ID comparison. Supported only if TRCIDR4.NUMCIDC > n, otherwise the field is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn comp_n(&mut self) -> CompNW<ApbaddrEtmCpu0Trccidcctlr0Spec> {
        CompNW::new(self, 0)
    }
}
#[doc = "Context ID Comparator Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccidcctlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccidcctlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trccidcctlr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trccidcctlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trccidcctlr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trccidcctlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trccidcctlr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trccidcctlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCIDCCTLR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trccidcctlr0Spec {
    const RESET_VALUE: u32 = 0;
}
