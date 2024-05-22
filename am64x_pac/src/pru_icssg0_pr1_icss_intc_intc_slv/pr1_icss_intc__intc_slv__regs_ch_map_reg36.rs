#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec>;
#[doc = "Field `CH_MAP_144` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[80\\]"]
pub type ChMap144R = crate::FieldReader;
#[doc = "Field `CH_MAP_144` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[80\\]"]
pub type ChMap144W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_145` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[81\\]"]
pub type ChMap145R = crate::FieldReader;
#[doc = "Field `CH_MAP_145` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[81\\]"]
pub type ChMap145W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_146` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[82\\]"]
pub type ChMap146R = crate::FieldReader;
#[doc = "Field `CH_MAP_146` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[82\\]"]
pub type ChMap146W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_147` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[83\\]"]
pub type ChMap147R = crate::FieldReader;
#[doc = "Field `CH_MAP_147` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[83\\]"]
pub type ChMap147W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[80\\]"]
    #[inline(always)]
    pub fn ch_map_144(&self) -> ChMap144R {
        ChMap144R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[81\\]"]
    #[inline(always)]
    pub fn ch_map_145(&self) -> ChMap145R {
        ChMap145R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[82\\]"]
    #[inline(always)]
    pub fn ch_map_146(&self) -> ChMap146R {
        ChMap146R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[83\\]"]
    #[inline(always)]
    pub fn ch_map_147(&self) -> ChMap147R {
        ChMap147R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[80\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_144(&mut self) -> ChMap144W<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec> {
        ChMap144W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[81\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_145(&mut self) -> ChMap145W<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec> {
        ChMap145W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[82\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_146(&mut self) -> ChMap146W<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec> {
        ChMap146W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[83\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_147(&mut self) -> ChMap147W<Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec> {
        ChMap147W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg36::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG36 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg36Spec {
    const RESET_VALUE: u32 = 0;
}
