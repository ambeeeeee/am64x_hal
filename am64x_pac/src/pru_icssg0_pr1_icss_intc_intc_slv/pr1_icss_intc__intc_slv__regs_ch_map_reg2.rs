#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec>;
#[doc = "Field `CH_MAP_8` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[8\\]"]
pub type ChMap8R = crate::FieldReader;
#[doc = "Field `CH_MAP_8` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[8\\]"]
pub type ChMap8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_9` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[9\\]"]
pub type ChMap9R = crate::FieldReader;
#[doc = "Field `CH_MAP_9` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[9\\]"]
pub type ChMap9W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_10` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[10\\]"]
pub type ChMap10R = crate::FieldReader;
#[doc = "Field `CH_MAP_10` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[10\\]"]
pub type ChMap10W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_11` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[11\\]"]
pub type ChMap11R = crate::FieldReader;
#[doc = "Field `CH_MAP_11` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[11\\]"]
pub type ChMap11W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[8\\]"]
    #[inline(always)]
    pub fn ch_map_8(&self) -> ChMap8R {
        ChMap8R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[9\\]"]
    #[inline(always)]
    pub fn ch_map_9(&self) -> ChMap9R {
        ChMap9R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[10\\]"]
    #[inline(always)]
    pub fn ch_map_10(&self) -> ChMap10R {
        ChMap10R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[11\\]"]
    #[inline(always)]
    pub fn ch_map_11(&self) -> ChMap11R {
        ChMap11R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_8(&mut self) -> ChMap8W<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec> {
        ChMap8W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_9(&mut self) -> ChMap9W<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec> {
        ChMap9W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[10\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_10(&mut self) -> ChMap10W<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec> {
        ChMap10W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_11(&mut self) -> ChMap11W<Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec> {
        ChMap11W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG2 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg2Spec {
    const RESET_VALUE: u32 = 0;
}
