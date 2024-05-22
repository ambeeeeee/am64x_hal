#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec>;
#[doc = "Field `PRI_HINT_4` reader - 9:0\\]
Host Int 4 Prioritized Interrupt"]
pub type PriHint4R = crate::FieldReader<u16>;
#[doc = "Field `PRI_HINT_4` writer - 9:0\\]
Host Int 4 Prioritized Interrupt"]
pub type PriHint4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NONE_HINT_4` reader - 31:31\\]
No interrupt pending flag"]
pub type NoneHint4R = crate::BitReader;
#[doc = "Field `NONE_HINT_4` writer - 31:31\\]
No interrupt pending flag"]
pub type NoneHint4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Host Int 4 Prioritized Interrupt"]
    #[inline(always)]
    pub fn pri_hint_4(&self) -> PriHint4R {
        PriHint4R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    pub fn none_hint_4(&self) -> NoneHint4R {
        NoneHint4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Host Int 4 Prioritized Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pri_hint_4(&mut self) -> PriHint4W<Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec> {
        PriHint4W::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn none_hint_4(&mut self) -> NoneHint4W<Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec> {
        NoneHint4W::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_pri_hint_reg4::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_PRI_HINT_REG4 to value 0x8000_0000"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsPriHintReg4Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
