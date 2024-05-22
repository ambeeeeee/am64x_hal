#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec>;
#[doc = "Field `CH_MAP_104` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[40\\]"]
pub type ChMap104R = crate::FieldReader;
#[doc = "Field `CH_MAP_104` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[40\\]"]
pub type ChMap104W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_105` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[41\\]"]
pub type ChMap105R = crate::FieldReader;
#[doc = "Field `CH_MAP_105` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[41\\]"]
pub type ChMap105W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_106` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[42\\]"]
pub type ChMap106R = crate::FieldReader;
#[doc = "Field `CH_MAP_106` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[42\\]"]
pub type ChMap106W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_107` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[43\\]"]
pub type ChMap107R = crate::FieldReader;
#[doc = "Field `CH_MAP_107` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[43\\]"]
pub type ChMap107W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[40\\]"]
    #[inline(always)]
    pub fn ch_map_104(&self) -> ChMap104R {
        ChMap104R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[41\\]"]
    #[inline(always)]
    pub fn ch_map_105(&self) -> ChMap105R {
        ChMap105R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[42\\]"]
    #[inline(always)]
    pub fn ch_map_106(&self) -> ChMap106R {
        ChMap106R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[43\\]"]
    #[inline(always)]
    pub fn ch_map_107(&self) -> ChMap107R {
        ChMap107R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[40\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_104(&mut self) -> ChMap104W<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec> {
        ChMap104W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[41\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_105(&mut self) -> ChMap105W<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec> {
        ChMap105W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[42\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_106(&mut self) -> ChMap106W<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec> {
        ChMap106W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[43\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_107(&mut self) -> ChMap107W<Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec> {
        ChMap107W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg26::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG26 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg26Spec {
    const RESET_VALUE: u32 = 0;
}
