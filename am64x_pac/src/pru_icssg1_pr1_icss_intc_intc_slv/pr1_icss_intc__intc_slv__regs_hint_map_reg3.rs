#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec>;
#[doc = "Field `HINT_MAP_12` reader - 4:0\\]
Host Interrupt Map for Channel 12"]
pub type HintMap12R = crate::FieldReader;
#[doc = "Field `HINT_MAP_12` writer - 4:0\\]
Host Interrupt Map for Channel 12"]
pub type HintMap12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_13` reader - 12:8\\]
Host Interrupt Map for Channel 13"]
pub type HintMap13R = crate::FieldReader;
#[doc = "Field `HINT_MAP_13` writer - 12:8\\]
Host Interrupt Map for Channel 13"]
pub type HintMap13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_14` reader - 20:16\\]
Host Interrupt Map for Channel 14"]
pub type HintMap14R = crate::FieldReader;
#[doc = "Field `HINT_MAP_14` writer - 20:16\\]
Host Interrupt Map for Channel 14"]
pub type HintMap14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_15` reader - 28:24\\]
Host Interrupt Map for Channel 15"]
pub type HintMap15R = crate::FieldReader;
#[doc = "Field `HINT_MAP_15` writer - 28:24\\]
Host Interrupt Map for Channel 15"]
pub type HintMap15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 12"]
    #[inline(always)]
    pub fn hint_map_12(&self) -> HintMap12R {
        HintMap12R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 13"]
    #[inline(always)]
    pub fn hint_map_13(&self) -> HintMap13R {
        HintMap13R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 14"]
    #[inline(always)]
    pub fn hint_map_14(&self) -> HintMap14R {
        HintMap14R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 15"]
    #[inline(always)]
    pub fn hint_map_15(&self) -> HintMap15R {
        HintMap15R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 12"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_12(&mut self) -> HintMap12W<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec> {
        HintMap12W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 13"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_13(&mut self) -> HintMap13W<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec> {
        HintMap13W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 14"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_14(&mut self) -> HintMap14W<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec> {
        HintMap14W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 15"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_15(&mut self) -> HintMap15W<Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec> {
        HintMap15W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_hint_map_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG3 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsHintMapReg3Spec {
    const RESET_VALUE: u32 = 0;
}
