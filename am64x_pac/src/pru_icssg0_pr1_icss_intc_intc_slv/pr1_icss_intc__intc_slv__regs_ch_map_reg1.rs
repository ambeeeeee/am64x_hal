#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec>;
#[doc = "Field `CH_MAP_4` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[4\\]"]
pub type ChMap4R = crate::FieldReader;
#[doc = "Field `CH_MAP_4` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[4\\]"]
pub type ChMap4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_5` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[5\\]"]
pub type ChMap5R = crate::FieldReader;
#[doc = "Field `CH_MAP_5` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[5\\]"]
pub type ChMap5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_6` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[6\\]"]
pub type ChMap6R = crate::FieldReader;
#[doc = "Field `CH_MAP_6` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[6\\]"]
pub type ChMap6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_7` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[7\\]"]
pub type ChMap7R = crate::FieldReader;
#[doc = "Field `CH_MAP_7` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[7\\]"]
pub type ChMap7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[4\\]"]
    #[inline(always)]
    pub fn ch_map_4(&self) -> ChMap4R {
        ChMap4R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[5\\]"]
    #[inline(always)]
    pub fn ch_map_5(&self) -> ChMap5R {
        ChMap5R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[6\\]"]
    #[inline(always)]
    pub fn ch_map_6(&self) -> ChMap6R {
        ChMap6R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[7\\]"]
    #[inline(always)]
    pub fn ch_map_7(&self) -> ChMap7R {
        ChMap7R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_4(&mut self) -> ChMap4W<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec> {
        ChMap4W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_5(&mut self) -> ChMap5W<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec> {
        ChMap5W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_6(&mut self) -> ChMap6W<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec> {
        ChMap6W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_7(&mut self) -> ChMap7W<Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec> {
        ChMap7W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg1::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG1 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg1Spec {
    const RESET_VALUE: u32 = 0;
}
