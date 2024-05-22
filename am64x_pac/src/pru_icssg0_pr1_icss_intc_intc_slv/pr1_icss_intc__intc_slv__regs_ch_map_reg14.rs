#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec>;
#[doc = "Field `CH_MAP_56` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[56\\]"]
pub type ChMap56R = crate::FieldReader;
#[doc = "Field `CH_MAP_56` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[56\\]"]
pub type ChMap56W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_57` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[57\\]"]
pub type ChMap57R = crate::FieldReader;
#[doc = "Field `CH_MAP_57` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[57\\]"]
pub type ChMap57W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_58` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[58\\]"]
pub type ChMap58R = crate::FieldReader;
#[doc = "Field `CH_MAP_58` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[58\\]"]
pub type ChMap58W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_59` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[59\\]"]
pub type ChMap59R = crate::FieldReader;
#[doc = "Field `CH_MAP_59` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[59\\]"]
pub type ChMap59W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[56\\]"]
    #[inline(always)]
    pub fn ch_map_56(&self) -> ChMap56R {
        ChMap56R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[57\\]"]
    #[inline(always)]
    pub fn ch_map_57(&self) -> ChMap57R {
        ChMap57R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[58\\]"]
    #[inline(always)]
    pub fn ch_map_58(&self) -> ChMap58R {
        ChMap58R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[59\\]"]
    #[inline(always)]
    pub fn ch_map_59(&self) -> ChMap59R {
        ChMap59R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[56\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_56(&mut self) -> ChMap56W<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec> {
        ChMap56W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[57\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_57(&mut self) -> ChMap57W<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec> {
        ChMap57W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[58\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_58(&mut self) -> ChMap58W<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec> {
        ChMap58W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[59\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_59(&mut self) -> ChMap59W<Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec> {
        ChMap59W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg14::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG14 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg14Spec {
    const RESET_VALUE: u32 = 0;
}
