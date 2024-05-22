#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec>;
#[doc = "Field `CH_MAP_76` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[12\\]"]
pub type ChMap76R = crate::FieldReader;
#[doc = "Field `CH_MAP_76` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[12\\]"]
pub type ChMap76W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_77` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[13\\]"]
pub type ChMap77R = crate::FieldReader;
#[doc = "Field `CH_MAP_77` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[13\\]"]
pub type ChMap77W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_78` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[14\\]"]
pub type ChMap78R = crate::FieldReader;
#[doc = "Field `CH_MAP_78` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[14\\]"]
pub type ChMap78W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_79` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[15\\]"]
pub type ChMap79R = crate::FieldReader;
#[doc = "Field `CH_MAP_79` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[15\\]"]
pub type ChMap79W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[12\\]"]
    #[inline(always)]
    pub fn ch_map_76(&self) -> ChMap76R {
        ChMap76R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[13\\]"]
    #[inline(always)]
    pub fn ch_map_77(&self) -> ChMap77R {
        ChMap77R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[14\\]"]
    #[inline(always)]
    pub fn ch_map_78(&self) -> ChMap78R {
        ChMap78R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[15\\]"]
    #[inline(always)]
    pub fn ch_map_79(&self) -> ChMap79R {
        ChMap79R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_76(&mut self) -> ChMap76W<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec> {
        ChMap76W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[13\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_77(&mut self) -> ChMap77W<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec> {
        ChMap77W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[14\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_78(&mut self) -> ChMap78W<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec> {
        ChMap78W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[15\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_79(&mut self) -> ChMap79W<Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec> {
        ChMap79W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg19::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG19 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg19Spec {
    const RESET_VALUE: u32 = 0;
}
