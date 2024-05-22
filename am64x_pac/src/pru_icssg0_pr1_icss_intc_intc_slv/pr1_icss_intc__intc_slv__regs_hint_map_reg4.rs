#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec>;
#[doc = "Field `HINT_MAP_16` reader - 4:0\\]
Host Interrupt Map for Channel 16"]
pub type HintMap16R = crate::FieldReader;
#[doc = "Field `HINT_MAP_16` writer - 4:0\\]
Host Interrupt Map for Channel 16"]
pub type HintMap16W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_17` reader - 12:8\\]
Host Interrupt Map for Channel 17"]
pub type HintMap17R = crate::FieldReader;
#[doc = "Field `HINT_MAP_17` writer - 12:8\\]
Host Interrupt Map for Channel 17"]
pub type HintMap17W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_18` reader - 20:16\\]
Host Interrupt Map for Channel 18"]
pub type HintMap18R = crate::FieldReader;
#[doc = "Field `HINT_MAP_18` writer - 20:16\\]
Host Interrupt Map for Channel 18"]
pub type HintMap18W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_19` reader - 28:24\\]
Host Interrupt Map for Channel 19"]
pub type HintMap19R = crate::FieldReader;
#[doc = "Field `HINT_MAP_19` writer - 28:24\\]
Host Interrupt Map for Channel 19"]
pub type HintMap19W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 16"]
    #[inline(always)]
    pub fn hint_map_16(&self) -> HintMap16R {
        HintMap16R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 17"]
    #[inline(always)]
    pub fn hint_map_17(&self) -> HintMap17R {
        HintMap17R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 18"]
    #[inline(always)]
    pub fn hint_map_18(&self) -> HintMap18R {
        HintMap18R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 19"]
    #[inline(always)]
    pub fn hint_map_19(&self) -> HintMap19R {
        HintMap19R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 16"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_16(&mut self) -> HintMap16W<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec> {
        HintMap16W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 17"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_17(&mut self) -> HintMap17W<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec> {
        HintMap17W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 18"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_18(&mut self) -> HintMap18W<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec> {
        HintMap18W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 19"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_19(&mut self) -> HintMap19W<Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec> {
        HintMap19W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_hint_map_reg4::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG4 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsHintMapReg4Spec {
    const RESET_VALUE: u32 = 0;
}
