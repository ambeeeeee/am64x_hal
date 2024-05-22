#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec>;
#[doc = "Field `CH_MAP_16` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[16\\]"]
pub type ChMap16R = crate::FieldReader;
#[doc = "Field `CH_MAP_16` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[16\\]"]
pub type ChMap16W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_17` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[17\\]"]
pub type ChMap17R = crate::FieldReader;
#[doc = "Field `CH_MAP_17` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[17\\]"]
pub type ChMap17W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_18` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[18\\]"]
pub type ChMap18R = crate::FieldReader;
#[doc = "Field `CH_MAP_18` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[18\\]"]
pub type ChMap18W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_19` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[19\\]"]
pub type ChMap19R = crate::FieldReader;
#[doc = "Field `CH_MAP_19` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[19\\]"]
pub type ChMap19W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[16\\]"]
    #[inline(always)]
    pub fn ch_map_16(&self) -> ChMap16R {
        ChMap16R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[17\\]"]
    #[inline(always)]
    pub fn ch_map_17(&self) -> ChMap17R {
        ChMap17R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[18\\]"]
    #[inline(always)]
    pub fn ch_map_18(&self) -> ChMap18R {
        ChMap18R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[19\\]"]
    #[inline(always)]
    pub fn ch_map_19(&self) -> ChMap19R {
        ChMap19R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[16\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_16(&mut self) -> ChMap16W<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec> {
        ChMap16W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[17\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_17(&mut self) -> ChMap17W<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec> {
        ChMap17W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[18\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_18(&mut self) -> ChMap18W<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec> {
        ChMap18W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[19\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_19(&mut self) -> ChMap19W<Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec> {
        ChMap19W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg4::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG4 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg4Spec {
    const RESET_VALUE: u32 = 0;
}
