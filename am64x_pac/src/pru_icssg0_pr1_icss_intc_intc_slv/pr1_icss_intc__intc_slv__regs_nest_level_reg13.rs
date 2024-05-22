#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec>;
#[doc = "Field `NEST_HINT_13` reader - 8:0\\]
Host Int 13 Nesting Level"]
pub type NestHint13R = crate::FieldReader<u16>;
#[doc = "Field `NEST_HINT_13` writer - 8:0\\]
Host Int 13 Nesting Level"]
pub type NestHint13W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `NEST_AUTO_OVR` reader - 31:31\\]
Nesting Level Override Automatic"]
pub type NestAutoOvrR = crate::BitReader;
#[doc = "Field `NEST_AUTO_OVR` writer - 31:31\\]
Nesting Level Override Automatic"]
pub type NestAutoOvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Host Int 13 Nesting Level"]
    #[inline(always)]
    pub fn nest_hint_13(&self) -> NestHint13R {
        NestHint13R::new((self.bits & 0x01ff) as u16)
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
Host Int 13 Nesting Level"]
    #[inline(always)]
    #[must_use]
    pub fn nest_hint_13(&mut self) -> NestHint13W<Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec> {
        NestHint13W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Nesting Level Override Automatic"]
    #[inline(always)]
    #[must_use]
    pub fn nest_auto_ovr(&mut self) -> NestAutoOvrW<Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec> {
        NestAutoOvrW::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_nest_level_reg13::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_NEST_LEVEL_REG13 to value 0x0256"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsNestLevelReg13Spec {
    const RESET_VALUE: u32 = 0x0256;
}
