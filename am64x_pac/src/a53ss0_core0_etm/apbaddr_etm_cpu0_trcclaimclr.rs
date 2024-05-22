#[doc = "Register `APBADDR_ETM_CPU0_TRCCLAIMCLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcclaimclrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCLAIMCLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcclaimclrSpec>;
#[doc = "Field `CLR` reader - 3:0\\]
Clears bits in the claim tag and determines the current value of the claim tag."]
pub type ClrR = crate::FieldReader;
#[doc = "Field `CLR` writer - 3:0\\]
Clears bits in the claim tag and determines the current value of the claim tag."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Clears bits in the claim tag and determines the current value of the claim tag."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Clears bits in the claim tag and determines the current value of the claim tag."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<ApbaddrEtmCpu0TrcclaimclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "Claim Tag Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcclaimclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcclaimclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcclaimclrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcclaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcclaimclr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcclaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcclaimclr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcclaimclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCLAIMCLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcclaimclrSpec {
    const RESET_VALUE: u32 = 0;
}
