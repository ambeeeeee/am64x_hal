#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec>;
#[doc = "Field `CH_MAP_36` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[36\\]"]
pub type ChMap36R = crate::FieldReader;
#[doc = "Field `CH_MAP_36` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[36\\]"]
pub type ChMap36W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_37` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[37\\]"]
pub type ChMap37R = crate::FieldReader;
#[doc = "Field `CH_MAP_37` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[37\\]"]
pub type ChMap37W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_38` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[38\\]"]
pub type ChMap38R = crate::FieldReader;
#[doc = "Field `CH_MAP_38` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[38\\]"]
pub type ChMap38W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_39` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[39\\]"]
pub type ChMap39R = crate::FieldReader;
#[doc = "Field `CH_MAP_39` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[39\\]"]
pub type ChMap39W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[36\\]"]
    #[inline(always)]
    pub fn ch_map_36(&self) -> ChMap36R {
        ChMap36R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[37\\]"]
    #[inline(always)]
    pub fn ch_map_37(&self) -> ChMap37R {
        ChMap37R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[38\\]"]
    #[inline(always)]
    pub fn ch_map_38(&self) -> ChMap38R {
        ChMap38R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[39\\]"]
    #[inline(always)]
    pub fn ch_map_39(&self) -> ChMap39R {
        ChMap39R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[36\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_36(&mut self) -> ChMap36W<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec> {
        ChMap36W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[37\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_37(&mut self) -> ChMap37W<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec> {
        ChMap37W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[38\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_38(&mut self) -> ChMap38W<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec> {
        ChMap38W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[39\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_39(&mut self) -> ChMap39W<Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec> {
        ChMap39W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg9::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG9 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg9Spec {
    const RESET_VALUE: u32 = 0;
}
