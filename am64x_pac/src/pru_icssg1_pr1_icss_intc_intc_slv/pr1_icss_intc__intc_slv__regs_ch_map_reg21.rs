#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec>;
#[doc = "Field `CH_MAP_84` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[20\\]"]
pub type ChMap84R = crate::FieldReader;
#[doc = "Field `CH_MAP_84` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[20\\]"]
pub type ChMap84W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_85` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[21\\]"]
pub type ChMap85R = crate::FieldReader;
#[doc = "Field `CH_MAP_85` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[21\\]"]
pub type ChMap85W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_86` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[22\\]"]
pub type ChMap86R = crate::FieldReader;
#[doc = "Field `CH_MAP_86` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[22\\]"]
pub type ChMap86W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_87` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[23\\]"]
pub type ChMap87R = crate::FieldReader;
#[doc = "Field `CH_MAP_87` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[23\\]"]
pub type ChMap87W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[20\\]"]
    #[inline(always)]
    pub fn ch_map_84(&self) -> ChMap84R {
        ChMap84R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[21\\]"]
    #[inline(always)]
    pub fn ch_map_85(&self) -> ChMap85R {
        ChMap85R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[22\\]"]
    #[inline(always)]
    pub fn ch_map_86(&self) -> ChMap86R {
        ChMap86R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[23\\]"]
    #[inline(always)]
    pub fn ch_map_87(&self) -> ChMap87R {
        ChMap87R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[20\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_84(&mut self) -> ChMap84W<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec> {
        ChMap84W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[21\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_85(&mut self) -> ChMap85W<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec> {
        ChMap85W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[22\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_86(&mut self) -> ChMap86W<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec> {
        ChMap86W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[23\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_87(&mut self) -> ChMap87W<Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec> {
        ChMap87W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg21::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG21 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg21Spec {
    const RESET_VALUE: u32 = 0;
}
