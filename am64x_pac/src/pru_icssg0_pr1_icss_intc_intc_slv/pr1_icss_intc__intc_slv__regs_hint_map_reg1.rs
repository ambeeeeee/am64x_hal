#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec>;
#[doc = "Field `HINT_MAP_4` reader - 4:0\\]
Host Interrupt Map for Channel 4"]
pub type HintMap4R = crate::FieldReader;
#[doc = "Field `HINT_MAP_4` writer - 4:0\\]
Host Interrupt Map for Channel 4"]
pub type HintMap4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_5` reader - 12:8\\]
Host Interrupt Map for Channel 5"]
pub type HintMap5R = crate::FieldReader;
#[doc = "Field `HINT_MAP_5` writer - 12:8\\]
Host Interrupt Map for Channel 5"]
pub type HintMap5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_6` reader - 20:16\\]
Host Interrupt Map for Channel 6"]
pub type HintMap6R = crate::FieldReader;
#[doc = "Field `HINT_MAP_6` writer - 20:16\\]
Host Interrupt Map for Channel 6"]
pub type HintMap6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_7` reader - 28:24\\]
Host Interrupt Map for Channel 7"]
pub type HintMap7R = crate::FieldReader;
#[doc = "Field `HINT_MAP_7` writer - 28:24\\]
Host Interrupt Map for Channel 7"]
pub type HintMap7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 4"]
    #[inline(always)]
    pub fn hint_map_4(&self) -> HintMap4R {
        HintMap4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 5"]
    #[inline(always)]
    pub fn hint_map_5(&self) -> HintMap5R {
        HintMap5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 6"]
    #[inline(always)]
    pub fn hint_map_6(&self) -> HintMap6R {
        HintMap6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 7"]
    #[inline(always)]
    pub fn hint_map_7(&self) -> HintMap7R {
        HintMap7R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_4(&mut self) -> HintMap4W<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec> {
        HintMap4W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_5(&mut self) -> HintMap5W<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec> {
        HintMap5W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_6(&mut self) -> HintMap6W<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec> {
        HintMap6W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_7(&mut self) -> HintMap7W<Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec> {
        HintMap7W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_hint_map_reg1::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG1 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsHintMapReg1Spec {
    const RESET_VALUE: u32 = 0;
}
