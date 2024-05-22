#[doc = "Register `APBADDR_ETM_CPU1_TRCEVENTCTL1R` reader"]
pub type R = crate::R<ApbaddrEtmCpu1Trceventctl1rSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCEVENTCTL1R` writer"]
pub type W = crate::W<ApbaddrEtmCpu1Trceventctl1rSpec>;
#[doc = "Field `EN` reader - 3:0\\]
One bit per event to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type EnR = crate::FieldReader;
#[doc = "Field `EN` writer - 3:0\\]
One bit per event to enable generation of an event element in the instruction trace stream when the selected event occurs"]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ATB` reader - 11:11\\]
ATB trigger enable"]
pub type AtbR = crate::BitReader;
#[doc = "Field `ATB` writer - 11:11\\]
ATB trigger enable"]
pub type AtbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPOVERRIDE` reader - 12:12\\]
Low power state behavior override"]
pub type LpoverrideR = crate::BitReader;
#[doc = "Field `LPOVERRIDE` writer - 12:12\\]
Low power state behavior override"]
pub type LpoverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
One bit per event to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
ATB trigger enable"]
    #[inline(always)]
    pub fn atb(&self) -> AtbR {
        AtbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Low power state behavior override"]
    #[inline(always)]
    pub fn lpoverride(&self) -> LpoverrideR {
        LpoverrideR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
One bit per event to enable generation of an event element in the instruction trace stream when the selected event occurs"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<ApbaddrEtmCpu1Trceventctl1rSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 11 - 11:11\\]
ATB trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn atb(&mut self) -> AtbW<ApbaddrEtmCpu1Trceventctl1rSpec> {
        AtbW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Low power state behavior override"]
    #[inline(always)]
    #[must_use]
    pub fn lpoverride(&mut self) -> LpoverrideW<ApbaddrEtmCpu1Trceventctl1rSpec> {
        LpoverrideW::new(self, 12)
    }
}
#[doc = "Event Control 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trceventctl1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trceventctl1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1Trceventctl1rSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1Trceventctl1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trceventctl1r::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1Trceventctl1rSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trceventctl1r::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1Trceventctl1rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCEVENTCTL1R to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1Trceventctl1rSpec {
    const RESET_VALUE: u32 = 0;
}
