#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec>;
#[doc = "Field `CH_MAP_72` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[8\\]"]
pub type ChMap72R = crate::FieldReader;
#[doc = "Field `CH_MAP_72` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[8\\]"]
pub type ChMap72W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_73` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[9\\]"]
pub type ChMap73R = crate::FieldReader;
#[doc = "Field `CH_MAP_73` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[9\\]"]
pub type ChMap73W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_74` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[10\\]"]
pub type ChMap74R = crate::FieldReader;
#[doc = "Field `CH_MAP_74` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[10\\]"]
pub type ChMap74W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_75` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[11\\]"]
pub type ChMap75R = crate::FieldReader;
#[doc = "Field `CH_MAP_75` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[11\\]"]
pub type ChMap75W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[8\\]"]
    #[inline(always)]
    pub fn ch_map_72(&self) -> ChMap72R {
        ChMap72R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[9\\]"]
    #[inline(always)]
    pub fn ch_map_73(&self) -> ChMap73R {
        ChMap73R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[10\\]"]
    #[inline(always)]
    pub fn ch_map_74(&self) -> ChMap74R {
        ChMap74R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[11\\]"]
    #[inline(always)]
    pub fn ch_map_75(&self) -> ChMap75R {
        ChMap75R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_72(&mut self) -> ChMap72W<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec> {
        ChMap72W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_73(&mut self) -> ChMap73W<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec> {
        ChMap73W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[10\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_74(&mut self) -> ChMap74W<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec> {
        ChMap74W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_75(&mut self) -> ChMap75W<Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec> {
        ChMap75W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg18::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG18 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg18Spec {
    const RESET_VALUE: u32 = 0;
}
