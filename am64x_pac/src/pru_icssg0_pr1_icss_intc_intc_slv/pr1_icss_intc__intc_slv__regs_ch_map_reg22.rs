#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec>;
#[doc = "Field `CH_MAP_88` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[24\\]"]
pub type ChMap88R = crate::FieldReader;
#[doc = "Field `CH_MAP_88` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[24\\]"]
pub type ChMap88W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_89` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[25\\]"]
pub type ChMap89R = crate::FieldReader;
#[doc = "Field `CH_MAP_89` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[25\\]"]
pub type ChMap89W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_90` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[26\\]"]
pub type ChMap90R = crate::FieldReader;
#[doc = "Field `CH_MAP_90` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[26\\]"]
pub type ChMap90W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_91` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[27\\]"]
pub type ChMap91R = crate::FieldReader;
#[doc = "Field `CH_MAP_91` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[27\\]"]
pub type ChMap91W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[24\\]"]
    #[inline(always)]
    pub fn ch_map_88(&self) -> ChMap88R {
        ChMap88R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[25\\]"]
    #[inline(always)]
    pub fn ch_map_89(&self) -> ChMap89R {
        ChMap89R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[26\\]"]
    #[inline(always)]
    pub fn ch_map_90(&self) -> ChMap90R {
        ChMap90R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[27\\]"]
    #[inline(always)]
    pub fn ch_map_91(&self) -> ChMap91R {
        ChMap91R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[24\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_88(&mut self) -> ChMap88W<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec> {
        ChMap88W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[25\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_89(&mut self) -> ChMap89W<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec> {
        ChMap89W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[26\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_90(&mut self) -> ChMap90W<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec> {
        ChMap90W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[27\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_91(&mut self) -> ChMap91W<Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec> {
        ChMap91W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg22::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG22 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg22Spec {
    const RESET_VALUE: u32 = 0;
}
