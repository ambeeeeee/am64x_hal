#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec>;
#[doc = "Field `CH_MAP_92` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[28\\]"]
pub type ChMap92R = crate::FieldReader;
#[doc = "Field `CH_MAP_92` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[28\\]"]
pub type ChMap92W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_93` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[29\\]"]
pub type ChMap93R = crate::FieldReader;
#[doc = "Field `CH_MAP_93` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[29\\]"]
pub type ChMap93W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_94` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[30\\]"]
pub type ChMap94R = crate::FieldReader;
#[doc = "Field `CH_MAP_94` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[30\\]"]
pub type ChMap94W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_95` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[31\\]"]
pub type ChMap95R = crate::FieldReader;
#[doc = "Field `CH_MAP_95` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[31\\]"]
pub type ChMap95W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[28\\]"]
    #[inline(always)]
    pub fn ch_map_92(&self) -> ChMap92R {
        ChMap92R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[29\\]"]
    #[inline(always)]
    pub fn ch_map_93(&self) -> ChMap93R {
        ChMap93R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[30\\]"]
    #[inline(always)]
    pub fn ch_map_94(&self) -> ChMap94R {
        ChMap94R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[31\\]"]
    #[inline(always)]
    pub fn ch_map_95(&self) -> ChMap95R {
        ChMap95R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[28\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_92(&mut self) -> ChMap92W<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec> {
        ChMap92W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[29\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_93(&mut self) -> ChMap93W<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec> {
        ChMap93W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[30\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_94(&mut self) -> ChMap94W<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec> {
        ChMap94W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[31\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_95(&mut self) -> ChMap95W<Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec> {
        ChMap95W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg23::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG23 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg23Spec {
    const RESET_VALUE: u32 = 0;
}
