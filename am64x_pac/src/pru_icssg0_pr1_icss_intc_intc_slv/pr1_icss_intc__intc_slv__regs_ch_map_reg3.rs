#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec>;
#[doc = "Field `CH_MAP_12` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[12\\]"]
pub type ChMap12R = crate::FieldReader;
#[doc = "Field `CH_MAP_12` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[12\\]"]
pub type ChMap12W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_13` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[13\\]"]
pub type ChMap13R = crate::FieldReader;
#[doc = "Field `CH_MAP_13` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[13\\]"]
pub type ChMap13W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_14` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[14\\]"]
pub type ChMap14R = crate::FieldReader;
#[doc = "Field `CH_MAP_14` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[14\\]"]
pub type ChMap14W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_15` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[15\\]"]
pub type ChMap15R = crate::FieldReader;
#[doc = "Field `CH_MAP_15` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[15\\]"]
pub type ChMap15W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[12\\]"]
    #[inline(always)]
    pub fn ch_map_12(&self) -> ChMap12R {
        ChMap12R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[13\\]"]
    #[inline(always)]
    pub fn ch_map_13(&self) -> ChMap13R {
        ChMap13R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[14\\]"]
    #[inline(always)]
    pub fn ch_map_14(&self) -> ChMap14R {
        ChMap14R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[15\\]"]
    #[inline(always)]
    pub fn ch_map_15(&self) -> ChMap15R {
        ChMap15R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_12(&mut self) -> ChMap12W<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec> {
        ChMap12W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[13\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_13(&mut self) -> ChMap13W<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec> {
        ChMap13W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[14\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_14(&mut self) -> ChMap14W<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec> {
        ChMap14W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[15\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_15(&mut self) -> ChMap15W<Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec> {
        ChMap15W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG3 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg3Spec {
    const RESET_VALUE: u32 = 0;
}
