#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec>;
#[doc = "Field `CH_MAP_136` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[72\\]"]
pub type ChMap136R = crate::FieldReader;
#[doc = "Field `CH_MAP_136` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[72\\]"]
pub type ChMap136W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_137` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[73\\]"]
pub type ChMap137R = crate::FieldReader;
#[doc = "Field `CH_MAP_137` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[73\\]"]
pub type ChMap137W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_138` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[74\\]"]
pub type ChMap138R = crate::FieldReader;
#[doc = "Field `CH_MAP_138` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[74\\]"]
pub type ChMap138W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_139` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[75\\]"]
pub type ChMap139R = crate::FieldReader;
#[doc = "Field `CH_MAP_139` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[75\\]"]
pub type ChMap139W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[72\\]"]
    #[inline(always)]
    pub fn ch_map_136(&self) -> ChMap136R {
        ChMap136R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[73\\]"]
    #[inline(always)]
    pub fn ch_map_137(&self) -> ChMap137R {
        ChMap137R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[74\\]"]
    #[inline(always)]
    pub fn ch_map_138(&self) -> ChMap138R {
        ChMap138R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[75\\]"]
    #[inline(always)]
    pub fn ch_map_139(&self) -> ChMap139R {
        ChMap139R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[72\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_136(&mut self) -> ChMap136W<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec> {
        ChMap136W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[73\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_137(&mut self) -> ChMap137W<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec> {
        ChMap137W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[74\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_138(&mut self) -> ChMap138W<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec> {
        ChMap138W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[75\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_139(&mut self) -> ChMap139W<Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec> {
        ChMap139W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg34::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG34 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg34Spec {
    const RESET_VALUE: u32 = 0;
}
