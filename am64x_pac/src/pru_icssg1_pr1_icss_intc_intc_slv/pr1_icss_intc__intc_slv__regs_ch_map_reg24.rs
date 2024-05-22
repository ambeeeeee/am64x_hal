#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec>;
#[doc = "Field `CH_MAP_96` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[32\\]"]
pub type ChMap96R = crate::FieldReader;
#[doc = "Field `CH_MAP_96` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[32\\]"]
pub type ChMap96W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_97` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[33\\]"]
pub type ChMap97R = crate::FieldReader;
#[doc = "Field `CH_MAP_97` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[33\\]"]
pub type ChMap97W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_98` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[34\\]"]
pub type ChMap98R = crate::FieldReader;
#[doc = "Field `CH_MAP_98` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[34\\]"]
pub type ChMap98W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_99` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[35\\]"]
pub type ChMap99R = crate::FieldReader;
#[doc = "Field `CH_MAP_99` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[35\\]"]
pub type ChMap99W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[32\\]"]
    #[inline(always)]
    pub fn ch_map_96(&self) -> ChMap96R {
        ChMap96R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[33\\]"]
    #[inline(always)]
    pub fn ch_map_97(&self) -> ChMap97R {
        ChMap97R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[34\\]"]
    #[inline(always)]
    pub fn ch_map_98(&self) -> ChMap98R {
        ChMap98R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[35\\]"]
    #[inline(always)]
    pub fn ch_map_99(&self) -> ChMap99R {
        ChMap99R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_96(&mut self) -> ChMap96W<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec> {
        ChMap96W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[33\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_97(&mut self) -> ChMap97W<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec> {
        ChMap97W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[34\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_98(&mut self) -> ChMap98W<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec> {
        ChMap98W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[35\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_99(&mut self) -> ChMap99W<Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec> {
        ChMap99W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg24::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG24 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg24Spec {
    const RESET_VALUE: u32 = 0;
}
