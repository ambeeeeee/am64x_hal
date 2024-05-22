#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec>;
#[doc = "Field `CH_MAP_132` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[68\\]"]
pub type ChMap132R = crate::FieldReader;
#[doc = "Field `CH_MAP_132` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[68\\]"]
pub type ChMap132W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_133` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[69\\]"]
pub type ChMap133R = crate::FieldReader;
#[doc = "Field `CH_MAP_133` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[69\\]"]
pub type ChMap133W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_134` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[70\\]"]
pub type ChMap134R = crate::FieldReader;
#[doc = "Field `CH_MAP_134` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[70\\]"]
pub type ChMap134W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_135` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[71\\]"]
pub type ChMap135R = crate::FieldReader;
#[doc = "Field `CH_MAP_135` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[71\\]"]
pub type ChMap135W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[68\\]"]
    #[inline(always)]
    pub fn ch_map_132(&self) -> ChMap132R {
        ChMap132R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[69\\]"]
    #[inline(always)]
    pub fn ch_map_133(&self) -> ChMap133R {
        ChMap133R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[70\\]"]
    #[inline(always)]
    pub fn ch_map_134(&self) -> ChMap134R {
        ChMap134R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[71\\]"]
    #[inline(always)]
    pub fn ch_map_135(&self) -> ChMap135R {
        ChMap135R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[68\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_132(&mut self) -> ChMap132W<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec> {
        ChMap132W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[69\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_133(&mut self) -> ChMap133W<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec> {
        ChMap133W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[70\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_134(&mut self) -> ChMap134W<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec> {
        ChMap134W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[71\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_135(&mut self) -> ChMap135W<Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec> {
        ChMap135W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg33::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG33 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg33Spec {
    const RESET_VALUE: u32 = 0;
}
