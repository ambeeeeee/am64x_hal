#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec>;
#[doc = "Field `CH_MAP_64` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[0\\]"]
pub type ChMap64R = crate::FieldReader;
#[doc = "Field `CH_MAP_64` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[0\\]"]
pub type ChMap64W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_65` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[1\\]"]
pub type ChMap65R = crate::FieldReader;
#[doc = "Field `CH_MAP_65` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[1\\]"]
pub type ChMap65W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_66` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[2\\]"]
pub type ChMap66R = crate::FieldReader;
#[doc = "Field `CH_MAP_66` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[2\\]"]
pub type ChMap66W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_67` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[3\\]"]
pub type ChMap67R = crate::FieldReader;
#[doc = "Field `CH_MAP_67` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[3\\]"]
pub type ChMap67W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[0\\]"]
    #[inline(always)]
    pub fn ch_map_64(&self) -> ChMap64R {
        ChMap64R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[1\\]"]
    #[inline(always)]
    pub fn ch_map_65(&self) -> ChMap65R {
        ChMap65R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[2\\]"]
    #[inline(always)]
    pub fn ch_map_66(&self) -> ChMap66R {
        ChMap66R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[3\\]"]
    #[inline(always)]
    pub fn ch_map_67(&self) -> ChMap67R {
        ChMap67R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_64(&mut self) -> ChMap64W<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec> {
        ChMap64W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_65(&mut self) -> ChMap65W<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec> {
        ChMap65W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_66(&mut self) -> ChMap66W<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec> {
        ChMap66W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_67(&mut self) -> ChMap67W<Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec> {
        ChMap67W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg16::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG16 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg16Spec {
    const RESET_VALUE: u32 = 0;
}
