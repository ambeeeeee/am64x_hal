#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec>;
#[doc = "Field `HINT_MAP_8` reader - 4:0\\]
Host Interrupt Map for Channel 8"]
pub type HintMap8R = crate::FieldReader;
#[doc = "Field `HINT_MAP_8` writer - 4:0\\]
Host Interrupt Map for Channel 8"]
pub type HintMap8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_9` reader - 12:8\\]
Host Interrupt Map for Channel 9"]
pub type HintMap9R = crate::FieldReader;
#[doc = "Field `HINT_MAP_9` writer - 12:8\\]
Host Interrupt Map for Channel 9"]
pub type HintMap9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_10` reader - 20:16\\]
Host Interrupt Map for Channel 10"]
pub type HintMap10R = crate::FieldReader;
#[doc = "Field `HINT_MAP_10` writer - 20:16\\]
Host Interrupt Map for Channel 10"]
pub type HintMap10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HINT_MAP_11` reader - 28:24\\]
Host Interrupt Map for Channel 11"]
pub type HintMap11R = crate::FieldReader;
#[doc = "Field `HINT_MAP_11` writer - 28:24\\]
Host Interrupt Map for Channel 11"]
pub type HintMap11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 8"]
    #[inline(always)]
    pub fn hint_map_8(&self) -> HintMap8R {
        HintMap8R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 9"]
    #[inline(always)]
    pub fn hint_map_9(&self) -> HintMap9R {
        HintMap9R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 10"]
    #[inline(always)]
    pub fn hint_map_10(&self) -> HintMap10R {
        HintMap10R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 11"]
    #[inline(always)]
    pub fn hint_map_11(&self) -> HintMap11R {
        HintMap11R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Host Interrupt Map for Channel 8"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_8(&mut self) -> HintMap8W<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec> {
        HintMap8W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Host Interrupt Map for Channel 9"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_9(&mut self) -> HintMap9W<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec> {
        HintMap9W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Host Interrupt Map for Channel 10"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_10(&mut self) -> HintMap10W<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec> {
        HintMap10W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Host Interrupt Map for Channel 11"]
    #[inline(always)]
    #[must_use]
    pub fn hint_map_11(&mut self) -> HintMap11W<Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec> {
        HintMap11W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_hint_map_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_HINT_MAP_REG2 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsHintMapReg2Spec {
    const RESET_VALUE: u32 = 0;
}
