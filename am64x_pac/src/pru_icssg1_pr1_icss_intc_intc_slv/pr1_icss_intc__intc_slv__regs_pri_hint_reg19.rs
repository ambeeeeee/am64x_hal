#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec>;
#[doc = "Field `PRI_HINT_19` reader - 9:0\\]
Host Int 19 Prioritized Interrupt"]
pub type PriHint19R = crate::FieldReader<u16>;
#[doc = "Field `PRI_HINT_19` writer - 9:0\\]
Host Int 19 Prioritized Interrupt"]
pub type PriHint19W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NONE_HINT_19` reader - 31:31\\]
No interrupt pending flag"]
pub type NoneHint19R = crate::BitReader;
#[doc = "Field `NONE_HINT_19` writer - 31:31\\]
No interrupt pending flag"]
pub type NoneHint19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Host Int 19 Prioritized Interrupt"]
    #[inline(always)]
    pub fn pri_hint_19(&self) -> PriHint19R {
        PriHint19R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    pub fn none_hint_19(&self) -> NoneHint19R {
        NoneHint19R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Host Int 19 Prioritized Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_hint_19(&mut self) -> PriHint19W<Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec> {
        PriHint19W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn none_hint_19(&mut self) -> NoneHint19W<Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec> {
        NoneHint19W::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_pri_hint_reg19::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG19 to value 0x8000_0000"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsPriHintReg19Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
