#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec>;
#[doc = "Field `ENABLE_HINT_ANY` reader - 0:0\\]
Global Enable for all Host Ints"]
pub type EnableHintAnyR = crate::BitReader;
#[doc = "Field `ENABLE_HINT_ANY` writer - 0:0\\]
Global Enable for all Host Ints"]
pub type EnableHintAnyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Global Enable for all Host Ints"]
    #[inline(always)]
    pub fn enable_hint_any(&self) -> EnableHintAnyR {
        EnableHintAnyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Global Enable for all Host Ints"]
    #[inline(always)]
    #[must_use]
    pub fn enable_hint_any(
        &mut self,
    ) -> EnableHintAnyW<Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec> {
        EnableHintAnyW::new(self, 0)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_global_enable_hint_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_GLOBAL_ENABLE_HINT_REG to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsGlobalEnableHintRegSpec {
    const RESET_VALUE: u32 = 0;
}
