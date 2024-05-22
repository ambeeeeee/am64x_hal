#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec>;
#[doc = "Field `CH_MAP_20` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[20\\]"]
pub type ChMap20R = crate::FieldReader;
#[doc = "Field `CH_MAP_20` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[20\\]"]
pub type ChMap20W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_21` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[21\\]"]
pub type ChMap21R = crate::FieldReader;
#[doc = "Field `CH_MAP_21` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[21\\]"]
pub type ChMap21W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_22` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[22\\]"]
pub type ChMap22R = crate::FieldReader;
#[doc = "Field `CH_MAP_22` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[22\\]"]
pub type ChMap22W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_23` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[23\\]"]
pub type ChMap23R = crate::FieldReader;
#[doc = "Field `CH_MAP_23` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[23\\]"]
pub type ChMap23W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[20\\]"]
    #[inline(always)]
    pub fn ch_map_20(&self) -> ChMap20R {
        ChMap20R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[21\\]"]
    #[inline(always)]
    pub fn ch_map_21(&self) -> ChMap21R {
        ChMap21R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[22\\]"]
    #[inline(always)]
    pub fn ch_map_22(&self) -> ChMap22R {
        ChMap22R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[23\\]"]
    #[inline(always)]
    pub fn ch_map_23(&self) -> ChMap23R {
        ChMap23R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[20\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_20(&mut self) -> ChMap20W<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec> {
        ChMap20W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[21\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_21(&mut self) -> ChMap21W<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec> {
        ChMap21W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[22\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_22(&mut self) -> ChMap22W<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec> {
        ChMap22W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[23\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_23(&mut self) -> ChMap23W<Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec> {
        ChMap23W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg5::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG5 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg5Spec {
    const RESET_VALUE: u32 = 0;
}
