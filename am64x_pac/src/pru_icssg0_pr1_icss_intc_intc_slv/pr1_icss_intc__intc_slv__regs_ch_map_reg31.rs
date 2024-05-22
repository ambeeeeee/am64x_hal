#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec>;
#[doc = "Field `CH_MAP_124` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[60\\]"]
pub type ChMap124R = crate::FieldReader;
#[doc = "Field `CH_MAP_124` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[60\\]"]
pub type ChMap124W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_125` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[61\\]"]
pub type ChMap125R = crate::FieldReader;
#[doc = "Field `CH_MAP_125` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[61\\]"]
pub type ChMap125W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_126` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[62\\]"]
pub type ChMap126R = crate::FieldReader;
#[doc = "Field `CH_MAP_126` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[62\\]"]
pub type ChMap126W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_127` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[63\\]"]
pub type ChMap127R = crate::FieldReader;
#[doc = "Field `CH_MAP_127` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[63\\]"]
pub type ChMap127W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[60\\]"]
    #[inline(always)]
    pub fn ch_map_124(&self) -> ChMap124R {
        ChMap124R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[61\\]"]
    #[inline(always)]
    pub fn ch_map_125(&self) -> ChMap125R {
        ChMap125R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[62\\]"]
    #[inline(always)]
    pub fn ch_map_126(&self) -> ChMap126R {
        ChMap126R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[63\\]"]
    #[inline(always)]
    pub fn ch_map_127(&self) -> ChMap127R {
        ChMap127R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[60\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_124(&mut self) -> ChMap124W<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec> {
        ChMap124W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[61\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_125(&mut self) -> ChMap125W<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec> {
        ChMap125W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[62\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_126(&mut self) -> ChMap126W<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec> {
        ChMap126W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[63\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_127(&mut self) -> ChMap127W<Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec> {
        ChMap127W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg31::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG31 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg31Spec {
    const RESET_VALUE: u32 = 0;
}
