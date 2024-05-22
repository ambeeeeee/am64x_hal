#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec>;
#[doc = "Field `CH_MAP_116` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[52\\]"]
pub type ChMap116R = crate::FieldReader;
#[doc = "Field `CH_MAP_116` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[52\\]"]
pub type ChMap116W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_117` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[53\\]"]
pub type ChMap117R = crate::FieldReader;
#[doc = "Field `CH_MAP_117` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[53\\]"]
pub type ChMap117W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_118` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[54\\]"]
pub type ChMap118R = crate::FieldReader;
#[doc = "Field `CH_MAP_118` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[54\\]"]
pub type ChMap118W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_119` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[55\\]"]
pub type ChMap119R = crate::FieldReader;
#[doc = "Field `CH_MAP_119` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[55\\]"]
pub type ChMap119W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[52\\]"]
    #[inline(always)]
    pub fn ch_map_116(&self) -> ChMap116R {
        ChMap116R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[53\\]"]
    #[inline(always)]
    pub fn ch_map_117(&self) -> ChMap117R {
        ChMap117R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[54\\]"]
    #[inline(always)]
    pub fn ch_map_118(&self) -> ChMap118R {
        ChMap118R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[55\\]"]
    #[inline(always)]
    pub fn ch_map_119(&self) -> ChMap119R {
        ChMap119R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[52\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_116(&mut self) -> ChMap116W<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec> {
        ChMap116W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[53\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_117(&mut self) -> ChMap117W<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec> {
        ChMap117W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[54\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_118(&mut self) -> ChMap118W<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec> {
        ChMap118W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[55\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_119(&mut self) -> ChMap119W<Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec> {
        ChMap119W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg29::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG29 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg29Spec {
    const RESET_VALUE: u32 = 0;
}
