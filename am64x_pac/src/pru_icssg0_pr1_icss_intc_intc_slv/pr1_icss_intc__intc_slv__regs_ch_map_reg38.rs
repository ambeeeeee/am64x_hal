#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec>;
#[doc = "Field `CH_MAP_152` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[88\\]"]
pub type ChMap152R = crate::FieldReader;
#[doc = "Field `CH_MAP_152` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[88\\]"]
pub type ChMap152W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_153` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[89\\]"]
pub type ChMap153R = crate::FieldReader;
#[doc = "Field `CH_MAP_153` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[89\\]"]
pub type ChMap153W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_154` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[90\\]"]
pub type ChMap154R = crate::FieldReader;
#[doc = "Field `CH_MAP_154` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[90\\]"]
pub type ChMap154W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_155` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[91\\]"]
pub type ChMap155R = crate::FieldReader;
#[doc = "Field `CH_MAP_155` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[91\\]"]
pub type ChMap155W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[88\\]"]
    #[inline(always)]
    pub fn ch_map_152(&self) -> ChMap152R {
        ChMap152R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[89\\]"]
    #[inline(always)]
    pub fn ch_map_153(&self) -> ChMap153R {
        ChMap153R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[90\\]"]
    #[inline(always)]
    pub fn ch_map_154(&self) -> ChMap154R {
        ChMap154R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[91\\]"]
    #[inline(always)]
    pub fn ch_map_155(&self) -> ChMap155R {
        ChMap155R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[88\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_152(&mut self) -> ChMap152W<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec> {
        ChMap152W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[89\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_153(&mut self) -> ChMap153W<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec> {
        ChMap153W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[90\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_154(&mut self) -> ChMap154W<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec> {
        ChMap154W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[91\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_155(&mut self) -> ChMap155W<Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec> {
        ChMap155W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg38::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG38 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg38Spec {
    const RESET_VALUE: u32 = 0;
}
