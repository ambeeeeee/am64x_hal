#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec>;
#[doc = "Field `GLB_NEST_LEVEL` reader - 8:0\\]
Global Nesting Level"]
pub type GlbNestLevelR = crate::FieldReader<u16>;
#[doc = "Field `GLB_NEST_LEVEL` writer - 8:0\\]
Global Nesting Level"]
pub type GlbNestLevelW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `GLB_NEST_AUTO_OVR` reader - 31:31\\]
Global Nesting Level Override Automatic"]
pub type GlbNestAutoOvrR = crate::BitReader;
#[doc = "Field `GLB_NEST_AUTO_OVR` writer - 31:31\\]
Global Nesting Level Override Automatic"]
pub type GlbNestAutoOvrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Global Nesting Level"]
    #[inline(always)]
    pub fn glb_nest_level(&self) -> GlbNestLevelR {
        GlbNestLevelR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Global Nesting Level Override Automatic"]
    #[inline(always)]
    pub fn glb_nest_auto_ovr(&self) -> GlbNestAutoOvrR {
        GlbNestAutoOvrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Global Nesting Level"]
    #[inline(always)]
    #[must_use]
    pub fn glb_nest_level(&mut self) -> GlbNestLevelW<Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec> {
        GlbNestLevelW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Global Nesting Level Override Automatic"]
    #[inline(always)]
    #[must_use]
    pub fn glb_nest_auto_ovr(
        &mut self,
    ) -> GlbNestAutoOvrW<Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec> {
        GlbNestAutoOvrW::new(self, 31)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_glb_nest_level_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_GLB_NEST_LEVEL_REG to value 0x0256"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsGlbNestLevelRegSpec {
    const RESET_VALUE: u32 = 0x0256;
}
