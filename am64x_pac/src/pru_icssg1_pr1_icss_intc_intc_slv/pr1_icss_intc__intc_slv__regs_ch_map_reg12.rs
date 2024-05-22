#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec>;
#[doc = "Field `CH_MAP_48` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[48\\]"]
pub type ChMap48R = crate::FieldReader;
#[doc = "Field `CH_MAP_48` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[48\\]"]
pub type ChMap48W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_49` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[49\\]"]
pub type ChMap49R = crate::FieldReader;
#[doc = "Field `CH_MAP_49` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[49\\]"]
pub type ChMap49W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_50` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[50\\]"]
pub type ChMap50R = crate::FieldReader;
#[doc = "Field `CH_MAP_50` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[50\\]"]
pub type ChMap50W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_51` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[51\\]"]
pub type ChMap51R = crate::FieldReader;
#[doc = "Field `CH_MAP_51` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[51\\]"]
pub type ChMap51W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[48\\]"]
    #[inline(always)]
    pub fn ch_map_48(&self) -> ChMap48R {
        ChMap48R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[49\\]"]
    #[inline(always)]
    pub fn ch_map_49(&self) -> ChMap49R {
        ChMap49R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[50\\]"]
    #[inline(always)]
    pub fn ch_map_50(&self) -> ChMap50R {
        ChMap50R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[51\\]"]
    #[inline(always)]
    pub fn ch_map_51(&self) -> ChMap51R {
        ChMap51R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[48\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_48(&mut self) -> ChMap48W<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec> {
        ChMap48W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[49\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_49(&mut self) -> ChMap49W<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec> {
        ChMap49W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[50\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_50(&mut self) -> ChMap50W<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec> {
        ChMap50W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[51\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_51(&mut self) -> ChMap51W<Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec> {
        ChMap51W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg12::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG12 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg12Spec {
    const RESET_VALUE: u32 = 0;
}
