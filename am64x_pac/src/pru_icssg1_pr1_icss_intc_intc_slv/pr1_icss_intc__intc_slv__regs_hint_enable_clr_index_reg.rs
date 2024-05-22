#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec>;
#[doc = "Field `HINT_ENABLE_CLR_INDEX` reader - 9:0\\]
Enable clear for Host Interrupts"]
pub type HintEnableClrIndexR = crate::FieldReader<u16>;
#[doc = "Field `HINT_ENABLE_CLR_INDEX` writer - 9:0\\]
Enable clear for Host Interrupts"]
pub type HintEnableClrIndexW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Enable clear for Host Interrupts"]
    #[inline(always)]
    pub fn hint_enable_clr_index(&self) -> HintEnableClrIndexR {
        HintEnableClrIndexR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Enable clear for Host Interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn hint_enable_clr_index(
        &mut self,
    ) -> HintEnableClrIndexW<Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec> {
        HintEnableClrIndexW::new(self, 0)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_hint_enable_clr_index_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_HINT_ENABLE_CLR_INDEX_REG to value 0"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsHintEnableClrIndexRegSpec {
    const RESET_VALUE: u32 = 0;
}
