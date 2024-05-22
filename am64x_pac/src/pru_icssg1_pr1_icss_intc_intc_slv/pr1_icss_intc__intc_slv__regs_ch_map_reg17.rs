#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec>;
#[doc = "Field `CH_MAP_68` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[4\\]"]
pub type ChMap68R = crate::FieldReader;
#[doc = "Field `CH_MAP_68` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[4\\]"]
pub type ChMap68W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_69` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[5\\]"]
pub type ChMap69R = crate::FieldReader;
#[doc = "Field `CH_MAP_69` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[5\\]"]
pub type ChMap69W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_70` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[6\\]"]
pub type ChMap70R = crate::FieldReader;
#[doc = "Field `CH_MAP_70` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[6\\]"]
pub type ChMap70W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_71` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[7\\]"]
pub type ChMap71R = crate::FieldReader;
#[doc = "Field `CH_MAP_71` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[7\\]"]
pub type ChMap71W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[4\\]"]
    #[inline(always)]
    pub fn ch_map_68(&self) -> ChMap68R {
        ChMap68R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[5\\]"]
    #[inline(always)]
    pub fn ch_map_69(&self) -> ChMap69R {
        ChMap69R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[6\\]"]
    #[inline(always)]
    pub fn ch_map_70(&self) -> ChMap70R {
        ChMap70R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[7\\]"]
    #[inline(always)]
    pub fn ch_map_71(&self) -> ChMap71R {
        ChMap71R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_68(&mut self) -> ChMap68W<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec> {
        ChMap68W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_69(&mut self) -> ChMap69W<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec> {
        ChMap69W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_70(&mut self) -> ChMap70W<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec> {
        ChMap70W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_71(&mut self) -> ChMap71W<Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec> {
        ChMap71W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg17::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG17 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg17Spec {
    const RESET_VALUE: u32 = 0;
}
