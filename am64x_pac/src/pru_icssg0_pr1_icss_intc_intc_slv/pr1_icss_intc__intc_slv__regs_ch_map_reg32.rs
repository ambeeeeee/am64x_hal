#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec>;
#[doc = "Field `CH_MAP_128` reader - 4:0\\]
Interrupt Channel Map for slv_events_in\\[64\\]"]
pub type ChMap128R = crate::FieldReader;
#[doc = "Field `CH_MAP_128` writer - 4:0\\]
Interrupt Channel Map for slv_events_in\\[64\\]"]
pub type ChMap128W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_129` reader - 12:8\\]
Interrupt Channel Map for slv_events_in\\[65\\]"]
pub type ChMap129R = crate::FieldReader;
#[doc = "Field `CH_MAP_129` writer - 12:8\\]
Interrupt Channel Map for slv_events_in\\[65\\]"]
pub type ChMap129W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_130` reader - 20:16\\]
Interrupt Channel Map for slv_events_in\\[66\\]"]
pub type ChMap130R = crate::FieldReader;
#[doc = "Field `CH_MAP_130` writer - 20:16\\]
Interrupt Channel Map for slv_events_in\\[66\\]"]
pub type ChMap130W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_131` reader - 28:24\\]
Interrupt Channel Map for slv_events_in\\[67\\]"]
pub type ChMap131R = crate::FieldReader;
#[doc = "Field `CH_MAP_131` writer - 28:24\\]
Interrupt Channel Map for slv_events_in\\[67\\]"]
pub type ChMap131W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[64\\]"]
    #[inline(always)]
    pub fn ch_map_128(&self) -> ChMap128R {
        ChMap128R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[65\\]"]
    #[inline(always)]
    pub fn ch_map_129(&self) -> ChMap129R {
        ChMap129R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[66\\]"]
    #[inline(always)]
    pub fn ch_map_130(&self) -> ChMap130R {
        ChMap130R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[67\\]"]
    #[inline(always)]
    pub fn ch_map_131(&self) -> ChMap131R {
        ChMap131R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for slv_events_in\\[64\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_128(&mut self) -> ChMap128W<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec> {
        ChMap128W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for slv_events_in\\[65\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_129(&mut self) -> ChMap129W<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec> {
        ChMap129W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for slv_events_in\\[66\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_130(&mut self) -> ChMap130W<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec> {
        ChMap130W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for slv_events_in\\[67\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_131(&mut self) -> ChMap131W<Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec> {
        ChMap131W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg32::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG32 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg32Spec {
    const RESET_VALUE: u32 = 0;
}
