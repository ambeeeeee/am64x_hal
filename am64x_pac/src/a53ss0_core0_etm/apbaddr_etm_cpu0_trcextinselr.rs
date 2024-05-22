#[doc = "Register `APBADDR_ETM_CPU0_TRCEXTINSELR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrcextinselrSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCEXTINSELR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrcextinselrSpec>;
#[doc = "Field `SEL0` reader - 4:0\\]
Selects an event from the external input bus for External Input Resource 0"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - 4:0\\]
Selects an event from the external input bus for External Input Resource 0"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SEL1` reader - 12:8\\]
Selects an event from the external input bus for External Input Resource 1"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - 12:8\\]
Selects an event from the external input bus for External Input Resource 1"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SEL2` reader - 20:16\\]
Selects an event from the external input bus for External Input Resource 2"]
pub type Sel2R = crate::FieldReader;
#[doc = "Field `SEL2` writer - 20:16\\]
Selects an event from the external input bus for External Input Resource 2"]
pub type Sel2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SEL3` reader - 28:24\\]
Selects an event from the external input bus for External Input Resource 3."]
pub type Sel3R = crate::FieldReader;
#[doc = "Field `SEL3` writer - 28:24\\]
Selects an event from the external input bus for External Input Resource 3."]
pub type Sel3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Selects an event from the external input bus for External Input Resource 0"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Selects an event from the external input bus for External Input Resource 1"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Selects an event from the external input bus for External Input Resource 2"]
    #[inline(always)]
    pub fn sel2(&self) -> Sel2R {
        Sel2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Selects an event from the external input bus for External Input Resource 3."]
    #[inline(always)]
    pub fn sel3(&self) -> Sel3R {
        Sel3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Selects an event from the external input bus for External Input Resource 0"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> Sel0W<ApbaddrEtmCpu0TrcextinselrSpec> {
        Sel0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Selects an event from the external input bus for External Input Resource 1"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> Sel1W<ApbaddrEtmCpu0TrcextinselrSpec> {
        Sel1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Selects an event from the external input bus for External Input Resource 2"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> Sel2W<ApbaddrEtmCpu0TrcextinselrSpec> {
        Sel2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Selects an event from the external input bus for External Input Resource 3."]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> Sel3W<ApbaddrEtmCpu0TrcextinselrSpec> {
        Sel3W::new(self, 24)
    }
}
#[doc = "External Input Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcextinselr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcextinselr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrcextinselrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrcextinselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcextinselr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrcextinselrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcextinselr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrcextinselrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCEXTINSELR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrcextinselrSpec {
    const RESET_VALUE: u32 = 0;
}
