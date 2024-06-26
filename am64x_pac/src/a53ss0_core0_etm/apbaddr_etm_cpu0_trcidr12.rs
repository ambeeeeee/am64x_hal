#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR12` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcidr12Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR12` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcidr12Spec>;
#[doc = "Field `NUMCONDKEY` reader - 31:0\\]
Indicates the number of conditional instruction right-hand keys that the trace unit can use. The number includes normal and special keys."]
pub type NumcondkeyR = crate::FieldReader<u32>;
#[doc = "Field `NUMCONDKEY` writer - 31:0\\]
Indicates the number of conditional instruction right-hand keys that the trace unit can use. The number includes normal and special keys."]
pub type NumcondkeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of conditional instruction right-hand keys that the trace unit can use. The number includes normal and special keys."]
    #[inline(always)]
    pub fn numcondkey(&self) -> NumcondkeyR {
        NumcondkeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of conditional instruction right-hand keys that the trace unit can use. The number includes normal and special keys."]
    #[inline(always)]
    #[must_use]
    pub fn numcondkey(&mut self) -> NumcondkeyW<ApbaddrEtmCpu0Trcidr12Spec> {
        NumcondkeyW::new(self, 0)
    }
}
#[doc = "ID Register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcidr12Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcidr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcidr12::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcidr12Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcidr12::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcidr12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCIDR12 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcidr12Spec {
    const RESET_VALUE: u32 = 0;
}
