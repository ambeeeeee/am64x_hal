#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec>;
#[doc = "Field `CH_MAP_0` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[0\\]"]
pub type ChMap0R = crate::FieldReader;
#[doc = "Field `CH_MAP_0` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[0\\]"]
pub type ChMap0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_1` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[1\\]"]
pub type ChMap1R = crate::FieldReader;
#[doc = "Field `CH_MAP_1` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[1\\]"]
pub type ChMap1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_2` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[2\\]"]
pub type ChMap2R = crate::FieldReader;
#[doc = "Field `CH_MAP_2` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[2\\]"]
pub type ChMap2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_3` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[3\\]"]
pub type ChMap3R = crate::FieldReader;
#[doc = "Field `CH_MAP_3` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[3\\]"]
pub type ChMap3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[0\\]"]
    #[inline(always)]
    pub fn ch_map_0(&self) -> ChMap0R {
        ChMap0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[1\\]"]
    #[inline(always)]
    pub fn ch_map_1(&self) -> ChMap1R {
        ChMap1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[2\\]"]
    #[inline(always)]
    pub fn ch_map_2(&self) -> ChMap2R {
        ChMap2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[3\\]"]
    #[inline(always)]
    pub fn ch_map_3(&self) -> ChMap3R {
        ChMap3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_0(&mut self) -> ChMap0W<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec> {
        ChMap0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_1(&mut self) -> ChMap1W<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec> {
        ChMap1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_2(&mut self) -> ChMap2W<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec> {
        ChMap2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_3(&mut self) -> ChMap3W<Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec> {
        ChMap3W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg0::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG0 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg0Spec {
    const RESET_VALUE: u32 = 0;
}
