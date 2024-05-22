#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec>;
#[doc = "Field `CH_MAP_108` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[44\\]"]
pub type ChMap108R = crate::FieldReader;
#[doc = "Field `CH_MAP_108` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[44\\]"]
pub type ChMap108W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_109` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[45\\]"]
pub type ChMap109R = crate::FieldReader;
#[doc = "Field `CH_MAP_109` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[45\\]"]
pub type ChMap109W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_110` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[46\\]"]
pub type ChMap110R = crate::FieldReader;
#[doc = "Field `CH_MAP_110` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[46\\]"]
pub type ChMap110W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_111` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[47\\]"]
pub type ChMap111R = crate::FieldReader;
#[doc = "Field `CH_MAP_111` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[47\\]"]
pub type ChMap111W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[44\\]"]
    #[inline(always)]
    pub fn ch_map_108(&self) -> ChMap108R {
        ChMap108R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[45\\]"]
    #[inline(always)]
    pub fn ch_map_109(&self) -> ChMap109R {
        ChMap109R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[46\\]"]
    #[inline(always)]
    pub fn ch_map_110(&self) -> ChMap110R {
        ChMap110R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[47\\]"]
    #[inline(always)]
    pub fn ch_map_111(&self) -> ChMap111R {
        ChMap111R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[44\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_108(&mut self) -> ChMap108W<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec> {
        ChMap108W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[45\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_109(&mut self) -> ChMap109W<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec> {
        ChMap109W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[46\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_110(&mut self) -> ChMap110W<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec> {
        ChMap110W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[47\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_111(&mut self) -> ChMap111W<Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec> {
        ChMap111W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg27::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG27 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg27Spec {
    const RESET_VALUE: u32 = 0;
}
