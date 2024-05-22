#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec>;
#[doc = "Field `CH_MAP_40` reader - 4:0\\]
Interrupt Channel Map for intr_in\\[40\\]"]
pub type ChMap40R = crate::FieldReader;
#[doc = "Field `CH_MAP_40` writer - 4:0\\]
Interrupt Channel Map for intr_in\\[40\\]"]
pub type ChMap40W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_41` reader - 12:8\\]
Interrupt Channel Map for intr_in\\[41\\]"]
pub type ChMap41R = crate::FieldReader;
#[doc = "Field `CH_MAP_41` writer - 12:8\\]
Interrupt Channel Map for intr_in\\[41\\]"]
pub type ChMap41W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_42` reader - 20:16\\]
Interrupt Channel Map for intr_in\\[42\\]"]
pub type ChMap42R = crate::FieldReader;
#[doc = "Field `CH_MAP_42` writer - 20:16\\]
Interrupt Channel Map for intr_in\\[42\\]"]
pub type ChMap42W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CH_MAP_43` reader - 28:24\\]
Interrupt Channel Map for intr_in\\[43\\]"]
pub type ChMap43R = crate::FieldReader;
#[doc = "Field `CH_MAP_43` writer - 28:24\\]
Interrupt Channel Map for intr_in\\[43\\]"]
pub type ChMap43W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[40\\]"]
    #[inline(always)]
    pub fn ch_map_40(&self) -> ChMap40R {
        ChMap40R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[41\\]"]
    #[inline(always)]
    pub fn ch_map_41(&self) -> ChMap41R {
        ChMap41R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[42\\]"]
    #[inline(always)]
    pub fn ch_map_42(&self) -> ChMap42R {
        ChMap42R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[43\\]"]
    #[inline(always)]
    pub fn ch_map_43(&self) -> ChMap43R {
        ChMap43R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt Channel Map for intr_in\\[40\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_40(&mut self) -> ChMap40W<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec> {
        ChMap40W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Interrupt Channel Map for intr_in\\[41\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_41(&mut self) -> ChMap41W<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec> {
        ChMap41W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Interrupt Channel Map for intr_in\\[42\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_42(&mut self) -> ChMap42W<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec> {
        ChMap42W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Interrupt Channel Map for intr_in\\[43\\]"]
    #[inline(always)]
    #[must_use]
    pub fn ch_map_43(&mut self) -> ChMap43W<Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec> {
        ChMap43W::new(self, 24)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_ch_map_reg10::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_CH_MAP_REG10 to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsChMapReg10Spec {
    const RESET_VALUE: u32 = 0;
}
