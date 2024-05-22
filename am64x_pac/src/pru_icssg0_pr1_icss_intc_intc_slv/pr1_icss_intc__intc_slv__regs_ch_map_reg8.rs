#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec>;
#[doc = "Field `CH_MAP_32` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[32\\]"]
pub type ChMap32R = crate::FieldReader;
#[doc = "Field `CH_MAP_32` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[32\\]"]
pub type ChMap32W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_33` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[33\\]"]
pub type ChMap33R = crate::FieldReader;
#[doc = "Field `CH_MAP_33` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[33\\]"]
pub type ChMap33W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_34` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[34\\]"]
pub type ChMap34R = crate::FieldReader;
#[doc = "Field `CH_MAP_34` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[34\\]"]
pub type ChMap34W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_35` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[35\\]"]
pub type ChMap35R = crate::FieldReader;
#[doc = "Field `CH_MAP_35` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[35\\]"]
pub type ChMap35W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[32\\]"]
    #[inline(always)]
    pub fn ch_map_32(&self) -> ChMap32R {
        ChMap32R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[33\\]"]
    #[inline(always)]
    pub fn ch_map_33(&self) -> ChMap33R {
        ChMap33R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[34\\]"]
    #[inline(always)]
    pub fn ch_map_34(&self) -> ChMap34R {
        ChMap34R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[35\\]"]
    #[inline(always)]
    pub fn ch_map_35(&self) -> ChMap35R {
        ChMap35R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[32\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_32(&mut self) -> ChMap32W<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec> {
        ChMap32W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[33\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_33(&mut self) -> ChMap33W<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec> {
        ChMap33W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[34\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_34(&mut self) -> ChMap34W<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec> {
        ChMap34W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[35\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_35(&mut self) -> ChMap35W<Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec> {
        ChMap35W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg8::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG8 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg8Spec {
    const RESET_VALUE: u32 = 0;
}
