#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec>;
#[doc = "Field `CH_MAP_52` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[52\\]"]
pub type ChMap52R = crate::FieldReader;
#[doc = "Field `CH_MAP_52` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[52\\]"]
pub type ChMap52W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_53` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[53\\]"]
pub type ChMap53R = crate::FieldReader;
#[doc = "Field `CH_MAP_53` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[53\\]"]
pub type ChMap53W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_54` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[54\\]"]
pub type ChMap54R = crate::FieldReader;
#[doc = "Field `CH_MAP_54` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[54\\]"]
pub type ChMap54W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_55` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[55\\]"]
pub type ChMap55R = crate::FieldReader;
#[doc = "Field `CH_MAP_55` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[55\\]"]
pub type ChMap55W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[52\\]"]
    #[inline(always)]
    pub fn ch_map_52(&self) -> ChMap52R {
        ChMap52R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[53\\]"]
    #[inline(always)]
    pub fn ch_map_53(&self) -> ChMap53R {
        ChMap53R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[54\\]"]
    #[inline(always)]
    pub fn ch_map_54(&self) -> ChMap54R {
        ChMap54R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[55\\]"]
    #[inline(always)]
    pub fn ch_map_55(&self) -> ChMap55R {
        ChMap55R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[52\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_52(&mut self) -> ChMap52W<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec> {
        ChMap52W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[53\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_53(&mut self) -> ChMap53W<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec> {
        ChMap53W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[54\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_54(&mut self) -> ChMap54W<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec> {
        ChMap54W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[55\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_55(&mut self) -> ChMap55W<Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec> {
        ChMap55W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg13::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG13 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg13Spec {
    const RESET_VALUE: u32 = 0;
}
