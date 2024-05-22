#[doc = "Register `CPTS_VBUSP_INT_ENABLE_REG` reader"]
pub type R = crate::R<CptsVbuspIntEnableRegSpec>;
#[doc = "Register `CPTS_VBUSP_INT_ENABLE_REG` writer"]
pub type W = crate::W<CptsVbuspIntEnableRegSpec>;
#[doc = "Field `TS_PEND_EN` reader - 0:0\\]
TS_PEND masked interrupt enable"]
pub type TsPendEnR = crate::BitReader;
#[doc = "Field `TS_PEND_EN` writer - 0:0\\]
TS_PEND masked interrupt enable"]
pub type TsPendEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt enable"]
    #[inline(always)]
    pub fn ts_pend_en(&self) -> TsPendEnR {
        TsPendEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TS_PEND masked interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts_pend_en(&mut self) -> TsPendEnW<CptsVbuspIntEnableRegSpec> {
        TsPendEnW::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_int_enable_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_int_enable_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspIntEnableRegSpec;
impl crate::RegisterSpec for CptsVbuspIntEnableRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_int_enable_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspIntEnableRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_int_enable_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspIntEnableRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_INT_ENABLE_REG to value 0"]
impl crate::Resettable for CptsVbuspIntEnableRegSpec {
    const RESET_VALUE: u32 = 0;
}
