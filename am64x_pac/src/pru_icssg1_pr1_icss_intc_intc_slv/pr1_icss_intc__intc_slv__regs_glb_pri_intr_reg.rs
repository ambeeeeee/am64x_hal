#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec>;
#[doc = "Field `GLB_PRI_INTR` reader - 9:0\\]
Prioritized Interrupt"]
pub type GlbPriIntrR = crate::FieldReader<u16>;
#[doc = "Field `GLB_PRI_INTR` writer - 9:0\\]
Prioritized Interrupt"]
pub type GlbPriIntrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `GLB_NONE` reader - 31:31\\]
No interrupt pending flag"]
pub type GlbNoneR = crate::BitReader;
#[doc = "Field `GLB_NONE` writer - 31:31\\]
No interrupt pending flag"]
pub type GlbNoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Prioritized Interrupt"]
    #[inline(always)]
    pub fn glb_pri_intr(&self) -> GlbPriIntrR {
        GlbPriIntrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    pub fn glb_none(&self) -> GlbNoneR {
        GlbNoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Prioritized Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn glb_pri_intr(&mut self) -> GlbPriIntrW<Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec> {
        GlbPriIntrW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
No interrupt pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn glb_none(&mut self) -> GlbNoneW<Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec> {
        GlbNoneW::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_glb_pri_intr_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_GLB_PRI_INTR_REG to value 0x8000_0000"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsGlbPriIntrRegSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
