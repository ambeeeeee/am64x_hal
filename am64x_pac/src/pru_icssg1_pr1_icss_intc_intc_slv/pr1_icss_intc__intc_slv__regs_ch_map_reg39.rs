#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec>;
#[doc = "Field `CH_MAP_156` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[92\\]"]
pub type ChMap156R = crate::FieldReader;
#[doc = "Field `CH_MAP_156` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[92\\]"]
pub type ChMap156W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_157` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[93\\]"]
pub type ChMap157R = crate::FieldReader;
#[doc = "Field `CH_MAP_157` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[93\\]"]
pub type ChMap157W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_158` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[94\\]"]
pub type ChMap158R = crate::FieldReader;
#[doc = "Field `CH_MAP_158` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[94\\]"]
pub type ChMap158W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_159` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[95\\]"]
pub type ChMap159R = crate::FieldReader;
#[doc = "Field `CH_MAP_159` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[95\\]"]
pub type ChMap159W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[92\\]"]
    #[inline(always)]
    pub fn ch_map_156(&self) -> ChMap156R {
        ChMap156R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[93\\]"]
    #[inline(always)]
    pub fn ch_map_157(&self) -> ChMap157R {
        ChMap157R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[94\\]"]
    #[inline(always)]
    pub fn ch_map_158(&self) -> ChMap158R {
        ChMap158R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[95\\]"]
    #[inline(always)]
    pub fn ch_map_159(&self) -> ChMap159R {
        ChMap159R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[92\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_156(&mut self) -> ChMap156W<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec> {
        ChMap156W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[93\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_157(&mut self) -> ChMap157W<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec> {
        ChMap157W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[94\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_158(&mut self) -> ChMap158W<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec> {
        ChMap158W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[95\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_159(&mut self) -> ChMap159W<Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec> {
        ChMap159W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg39::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG39 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg39Spec {
    const RESET_VALUE: u32 = 0;
}
