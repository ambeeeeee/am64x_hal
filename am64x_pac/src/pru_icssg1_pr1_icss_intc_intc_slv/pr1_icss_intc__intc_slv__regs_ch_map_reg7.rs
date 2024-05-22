#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec>;
#[doc = "Field `CH_MAP_28` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[28\\]"]
pub type ChMap28R = crate::FieldReader;
#[doc = "Field `CH_MAP_28` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[28\\]"]
pub type ChMap28W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_29` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[29\\]"]
pub type ChMap29R = crate::FieldReader;
#[doc = "Field `CH_MAP_29` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[29\\]"]
pub type ChMap29W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_30` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[30\\]"]
pub type ChMap30R = crate::FieldReader;
#[doc = "Field `CH_MAP_30` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[30\\]"]
pub type ChMap30W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_31` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[31\\]"]
pub type ChMap31R = crate::FieldReader;
#[doc = "Field `CH_MAP_31` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[31\\]"]
pub type ChMap31W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[28\\]"]
    #[inline(always)]
    pub fn ch_map_28(&self) -> ChMap28R {
        ChMap28R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[29\\]"]
    #[inline(always)]
    pub fn ch_map_29(&self) -> ChMap29R {
        ChMap29R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[30\\]"]
    #[inline(always)]
    pub fn ch_map_30(&self) -> ChMap30R {
        ChMap30R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[31\\]"]
    #[inline(always)]
    pub fn ch_map_31(&self) -> ChMap31R {
        ChMap31R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[28\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_28(&mut self) -> ChMap28W<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec> {
        ChMap28W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[29\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_29(&mut self) -> ChMap29W<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec> {
        ChMap29W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[30\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_30(&mut self) -> ChMap30W<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec> {
        ChMap30W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[31\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_31(&mut self) -> ChMap31W<Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec> {
        ChMap31W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg7::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG7 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg7Spec {
    const RESET_VALUE: u32 = 0;
}
