#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec>;
#[doc = "Field `CH_MAP_140` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[76\\]"]
pub type ChMap140R = crate::FieldReader;
#[doc = "Field `CH_MAP_140` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[76\\]"]
pub type ChMap140W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_141` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[77\\]"]
pub type ChMap141R = crate::FieldReader;
#[doc = "Field `CH_MAP_141` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[77\\]"]
pub type ChMap141W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_142` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[78\\]"]
pub type ChMap142R = crate::FieldReader;
#[doc = "Field `CH_MAP_142` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[78\\]"]
pub type ChMap142W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_143` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[79\\]"]
pub type ChMap143R = crate::FieldReader;
#[doc = "Field `CH_MAP_143` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[79\\]"]
pub type ChMap143W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[76\\]"]
    #[inline(always)]
    pub fn ch_map_140(&self) -> ChMap140R {
        ChMap140R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[77\\]"]
    #[inline(always)]
    pub fn ch_map_141(&self) -> ChMap141R {
        ChMap141R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[78\\]"]
    #[inline(always)]
    pub fn ch_map_142(&self) -> ChMap142R {
        ChMap142R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[79\\]"]
    #[inline(always)]
    pub fn ch_map_143(&self) -> ChMap143R {
        ChMap143R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[76\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_140(&mut self) -> ChMap140W<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec> {
        ChMap140W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[77\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_141(&mut self) -> ChMap141W<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec> {
        ChMap141W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[78\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_142(&mut self) -> ChMap142W<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec> {
        ChMap142W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[79\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_143(&mut self) -> ChMap143W<Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec> {
        ChMap143W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg35::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG35 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg35Spec {
    const RESET_VALUE: u32 = 0;
}
