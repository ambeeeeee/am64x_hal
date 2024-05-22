#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec>;
#[doc = "Field `NEST_HINT_3` reader - 8:0\\]
Host Int 3 Nesting Level"]
pub type NestHint3R = crate::FieldReader<u16>;
#[doc = "Field `NEST_HINT_3` writer - 8:0\\]
Host Int 3 Nesting Level"]
pub type NestHint3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NEST_AUTO_OVR` reader - 31:31\\]
Nesting Level Override Automatic"]
pub type NestAutoOvrR = crate::BitReader;
#[doc = "Field `NEST_AUTO_OVR` writer - 31:31\\]
Nesting Level Override Automatic"]
pub type NestAutoOvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Host Int 3 Nesting Level"]
    #[inline(always)]
    pub fn nest_hint_3(&self) -> NestHint3R {
        NestHint3R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Nesting Level Override Automatic"]
    #[inline(always)]
    pub fn nest_auto_ovr(&self) -> NestAutoOvrR {
        NestAutoOvrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Host Int 3 Nesting Level"]
    #[inline(always)]
    #[must_use]
    pub fn nest_hint_3(&mut self) -> NestHint3W<Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec> {
        NestHint3W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Nesting Level Override Automatic"]
    #[inline(always)]
    #[must_use]
    pub fn nest_auto_ovr(&mut self) -> NestAutoOvrW<Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec> {
        NestAutoOvrW::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_nest_level_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG3 to value 0x0256"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg3Spec {
    const RESET_VALUE: u32 = 0x0256;
}
