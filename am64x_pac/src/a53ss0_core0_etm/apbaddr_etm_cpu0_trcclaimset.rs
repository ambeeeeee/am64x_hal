#[doc = "Register `APBADDR_ETM_CPU0_TRCCLAIMSET` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcclaimsetSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCLAIMSET` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcclaimsetSpec>;
#[doc = "Field `SET` reader - 3:0\\]
Sets bits in the claim tag and determines the number of claim tag bits implemented."]
pub type SetR = crate::FieldReader;
#[doc = "Field `SET` writer - 3:0\\]
Sets bits in the claim tag and determines the number of claim tag bits implemented."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Sets bits in the claim tag and determines the number of claim tag bits implemented."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Sets bits in the claim tag and determines the number of claim tag bits implemented."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<ApbaddrEtmCpu0TrcclaimsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "Claim Tag Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcclaimset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcclaimset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcclaimsetSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcclaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcclaimset::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcclaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcclaimset::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcclaimsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCLAIMSET to value 0x15"]
impl crate::Resettable for ApbaddrEtmCpu0TrcclaimsetSpec {
    const RESET_VALUE: u32 = 0x15;
}
