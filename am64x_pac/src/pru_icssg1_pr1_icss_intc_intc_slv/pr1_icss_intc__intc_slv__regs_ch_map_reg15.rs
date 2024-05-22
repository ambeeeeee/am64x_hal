#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec>;
#[doc = "Field `CH_MAP_60` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[60\\]"]
pub type ChMap60R = crate::FieldReader;
#[doc = "Field `CH_MAP_60` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[60\\]"]
pub type ChMap60W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_61` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[61\\]"]
pub type ChMap61R = crate::FieldReader;
#[doc = "Field `CH_MAP_61` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[61\\]"]
pub type ChMap61W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_62` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[62\\]"]
pub type ChMap62R = crate::FieldReader;
#[doc = "Field `CH_MAP_62` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[62\\]"]
pub type ChMap62W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_63` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[63\\]"]
pub type ChMap63R = crate::FieldReader;
#[doc = "Field `CH_MAP_63` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[63\\]"]
pub type ChMap63W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[60\\]"]
    #[inline(always)]
    pub fn ch_map_60(&self) -> ChMap60R {
        ChMap60R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[61\\]"]
    #[inline(always)]
    pub fn ch_map_61(&self) -> ChMap61R {
        ChMap61R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[62\\]"]
    #[inline(always)]
    pub fn ch_map_62(&self) -> ChMap62R {
        ChMap62R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[63\\]"]
    #[inline(always)]
    pub fn ch_map_63(&self) -> ChMap63R {
        ChMap63R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[60\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_60(&mut self) -> ChMap60W<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec> {
        ChMap60W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[61\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_61(&mut self) -> ChMap61W<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec> {
        ChMap61W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[62\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_62(&mut self) -> ChMap62W<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec> {
        ChMap62W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[63\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_63(&mut self) -> ChMap63W<Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec> {
        ChMap63W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg15::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG15 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg15Spec {
    const RESET_VALUE: u32 = 0;
}
