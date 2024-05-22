#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec>;
#[doc = "Field `CH_MAP_24` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[24\\]"]
pub type ChMap24R = crate::FieldReader;
#[doc = "Field `CH_MAP_24` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[24\\]"]
pub type ChMap24W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_25` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[25\\]"]
pub type ChMap25R = crate::FieldReader;
#[doc = "Field `CH_MAP_25` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[25\\]"]
pub type ChMap25W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_26` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[26\\]"]
pub type ChMap26R = crate::FieldReader;
#[doc = "Field `CH_MAP_26` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[26\\]"]
pub type ChMap26W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_27` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[27\\]"]
pub type ChMap27R = crate::FieldReader;
#[doc = "Field `CH_MAP_27` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[27\\]"]
pub type ChMap27W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[24\\]"]
    #[inline(always)]
    pub fn ch_map_24(&self) -> ChMap24R {
        ChMap24R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[25\\]"]
    #[inline(always)]
    pub fn ch_map_25(&self) -> ChMap25R {
        ChMap25R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[26\\]"]
    #[inline(always)]
    pub fn ch_map_26(&self) -> ChMap26R {
        ChMap26R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[27\\]"]
    #[inline(always)]
    pub fn ch_map_27(&self) -> ChMap27R {
        ChMap27R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[24\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_24(&mut self) -> ChMap24W<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec> {
        ChMap24W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[25\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_25(&mut self) -> ChMap25W<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec> {
        ChMap25W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[26\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_26(&mut self) -> ChMap26W<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec> {
        ChMap26W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[27\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_27(&mut self) -> ChMap27W<Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec> {
        ChMap27W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg6::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG6 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg6Spec {
    const RESET_VALUE: u32 = 0;
}
