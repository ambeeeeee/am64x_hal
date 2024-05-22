#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR10` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcidr10Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCIDR10` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcidr10Spec>;
#[doc = "Field `NUMP1KEY` reader - 31:0\\]
Indicates the number of P1 right-hand keys that the trace unit can use. The number includes normal and special keys."]
pub type Nump1keyR = crate::FieldReader<u32>;
#[doc = "Field `NUMP1KEY` writer - 31:0\\]
Indicates the number of P1 right-hand keys that the trace unit can use. The number includes normal and special keys."]
pub type Nump1keyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of P1 right-hand keys that the trace unit can use. The number includes normal and special keys."]
    #[inline(always)]
    pub fn nump1key(&self) -> Nump1keyR {
        Nump1keyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Indicates the number of P1 right-hand keys that the trace unit can use. The number includes normal and special keys."]
    #[inline(always)]
    #[must_use]
    pub fn nump1key(&mut self) -> Nump1keyW<ApbaddrEtmCpu0Trcidr10Spec> {
        Nump1keyW::new(self, 0)
    }
}
#[doc = "ID Register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcidr10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcidr10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcidr10Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcidr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcidr10::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcidr10Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcidr10::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcidr10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCIDR10 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcidr10Spec {
    const RESET_VALUE: u32 = 0;
}
