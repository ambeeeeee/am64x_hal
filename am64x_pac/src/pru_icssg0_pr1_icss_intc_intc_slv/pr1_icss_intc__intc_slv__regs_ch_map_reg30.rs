#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec>;
#[doc = "Field `CH_MAP_120` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[56\\]"]
pub type ChMap120R = crate::FieldReader;
#[doc = "Field `CH_MAP_120` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[56\\]"]
pub type ChMap120W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_121` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[57\\]"]
pub type ChMap121R = crate::FieldReader;
#[doc = "Field `CH_MAP_121` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[57\\]"]
pub type ChMap121W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_122` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[58\\]"]
pub type ChMap122R = crate::FieldReader;
#[doc = "Field `CH_MAP_122` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[58\\]"]
pub type ChMap122W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_123` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[59\\]"]
pub type ChMap123R = crate::FieldReader;
#[doc = "Field `CH_MAP_123` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[59\\]"]
pub type ChMap123W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[56\\]"]
    #[inline(always)]
    pub fn ch_map_120(&self) -> ChMap120R {
        ChMap120R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[57\\]"]
    #[inline(always)]
    pub fn ch_map_121(&self) -> ChMap121R {
        ChMap121R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[58\\]"]
    #[inline(always)]
    pub fn ch_map_122(&self) -> ChMap122R {
        ChMap122R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[59\\]"]
    #[inline(always)]
    pub fn ch_map_123(&self) -> ChMap123R {
        ChMap123R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[56\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_120(&mut self) -> ChMap120W<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec> {
        ChMap120W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[57\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_121(&mut self) -> ChMap121W<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec> {
        ChMap121W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[58\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_122(&mut self) -> ChMap122W<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec> {
        ChMap122W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[59\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_123(&mut self) -> ChMap123W<Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec> {
        ChMap123W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg30::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG30 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg30Spec {
    const RESET_VALUE: u32 = 0;
}
