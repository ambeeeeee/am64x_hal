#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec>;
#[doc = "Field `CH_MAP_112` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[48\\]"]
pub type ChMap112R = crate::FieldReader;
#[doc = "Field `CH_MAP_112` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[48\\]"]
pub type ChMap112W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_113` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[49\\]"]
pub type ChMap113R = crate::FieldReader;
#[doc = "Field `CH_MAP_113` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[49\\]"]
pub type ChMap113W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_114` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[50\\]"]
pub type ChMap114R = crate::FieldReader;
#[doc = "Field `CH_MAP_114` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[50\\]"]
pub type ChMap114W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_115` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[51\\]"]
pub type ChMap115R = crate::FieldReader;
#[doc = "Field `CH_MAP_115` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[51\\]"]
pub type ChMap115W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[48\\]"]
    #[inline(always)]
    pub fn ch_map_112(&self) -> ChMap112R {
        ChMap112R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[49\\]"]
    #[inline(always)]
    pub fn ch_map_113(&self) -> ChMap113R {
        ChMap113R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[50\\]"]
    #[inline(always)]
    pub fn ch_map_114(&self) -> ChMap114R {
        ChMap114R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[51\\]"]
    #[inline(always)]
    pub fn ch_map_115(&self) -> ChMap115R {
        ChMap115R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[48\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_112(&mut self) -> ChMap112W<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec> {
        ChMap112W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[49\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_113(&mut self) -> ChMap113W<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec> {
        ChMap113W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[50\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_114(&mut self) -> ChMap114W<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec> {
        ChMap114W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[51\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_115(&mut self) -> ChMap115W<Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec> {
        ChMap115W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg28::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG28 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg28Spec {
    const RESET_VALUE: u32 = 0;
}
