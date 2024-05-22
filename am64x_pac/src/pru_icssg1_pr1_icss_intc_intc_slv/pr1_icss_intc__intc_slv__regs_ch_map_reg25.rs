#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec>;
#[doc = "Field `CH_MAP_100` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[36\\]"]
pub type ChMap100R = crate::FieldReader;
#[doc = "Field `CH_MAP_100` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[36\\]"]
pub type ChMap100W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_101` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[37\\]"]
pub type ChMap101R = crate::FieldReader;
#[doc = "Field `CH_MAP_101` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[37\\]"]
pub type ChMap101W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_102` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[38\\]"]
pub type ChMap102R = crate::FieldReader;
#[doc = "Field `CH_MAP_102` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[38\\]"]
pub type ChMap102W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_103` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[39\\]"]
pub type ChMap103R = crate::FieldReader;
#[doc = "Field `CH_MAP_103` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[39\\]"]
pub type ChMap103W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[36\\]"]
    #[inline(always)]
    pub fn ch_map_100(&self) -> ChMap100R {
        ChMap100R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[37\\]"]
    #[inline(always)]
    pub fn ch_map_101(&self) -> ChMap101R {
        ChMap101R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[38\\]"]
    #[inline(always)]
    pub fn ch_map_102(&self) -> ChMap102R {
        ChMap102R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[39\\]"]
    #[inline(always)]
    pub fn ch_map_103(&self) -> ChMap103R {
        ChMap103R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[36\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_100(&mut self) -> ChMap100W<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec> {
        ChMap100W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[37\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_101(&mut self) -> ChMap101W<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec> {
        ChMap101W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[38\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_102(&mut self) -> ChMap102W<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec> {
        ChMap102W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[39\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_103(&mut self) -> ChMap103W<Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec> {
        ChMap103W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg25::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG25 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg25Spec {
    const RESET_VALUE: u32 = 0;
}
