#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec>;
#[doc = "Field `CH_MAP_80` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[16\\]"]
pub type ChMap80R = crate::FieldReader;
#[doc = "Field `CH_MAP_80` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[16\\]"]
pub type ChMap80W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_81` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[17\\]"]
pub type ChMap81R = crate::FieldReader;
#[doc = "Field `CH_MAP_81` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[17\\]"]
pub type ChMap81W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_82` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[18\\]"]
pub type ChMap82R = crate::FieldReader;
#[doc = "Field `CH_MAP_82` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[18\\]"]
pub type ChMap82W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_83` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[19\\]"]
pub type ChMap83R = crate::FieldReader;
#[doc = "Field `CH_MAP_83` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[19\\]"]
pub type ChMap83W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[16\\]"]
    #[inline(always)]
    pub fn ch_map_80(&self) -> ChMap80R {
        ChMap80R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[17\\]"]
    #[inline(always)]
    pub fn ch_map_81(&self) -> ChMap81R {
        ChMap81R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[18\\]"]
    #[inline(always)]
    pub fn ch_map_82(&self) -> ChMap82R {
        ChMap82R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[19\\]"]
    #[inline(always)]
    pub fn ch_map_83(&self) -> ChMap83R {
        ChMap83R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_80(&mut self) -> ChMap80W<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec> {
        ChMap80W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[17\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_81(&mut self) -> ChMap81W<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec> {
        ChMap81W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[18\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_82(&mut self) -> ChMap82W<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec> {
        ChMap82W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[19\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_83(&mut self) -> ChMap83W<Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec> {
        ChMap83W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg20::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG20 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg20Spec {
    const RESET_VALUE: u32 = 0;
}
