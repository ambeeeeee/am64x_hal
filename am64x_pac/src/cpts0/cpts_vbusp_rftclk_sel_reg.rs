#[doc = "Register `CPTS_VBUSP_RFTCLK_SEL_REG` reader"]
pub type R = crate::R<CptsVbuspRftclkSelRegSpec>;
#[doc = "Register `CPTS_VBUSP_RFTCLK_SEL_REG` writer"]
pub type W = crate::W<CptsVbuspRftclkSelRegSpec>;
#[doc = "Field `RFTCLK_SEL` reader - 4:0\\]
Reference clock select"]
pub type RftclkSelR = crate::FieldReader;
#[doc = "Field `RFTCLK_SEL` writer - 4:0\\]
Reference clock select"]
pub type RftclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Reference clock select"]
    #[inline(always)]
    pub fn rftclk_sel(&self) -> RftclkSelR {
        RftclkSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Reference clock select"]
    #[inline(always)]
    #[must_use]
    pub fn rftclk_sel(&mut self) -> RftclkSelW<CptsVbuspRftclkSelRegSpec> {
        RftclkSelW::new(self, 0)
    }
}
#[doc = "RFTCLK Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_rftclk_sel_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_rftclk_sel_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspRftclkSelRegSpec;
impl crate::RegisterSpec for CptsVbuspRftclkSelRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_rftclk_sel_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspRftclkSelRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_rftclk_sel_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspRftclkSelRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_RFTCLK_SEL_REG to value 0"]
impl crate::Resettable for CptsVbuspRftclkSelRegSpec {
    const RESET_VALUE: u32 = 0;
}
