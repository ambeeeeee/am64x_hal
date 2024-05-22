#[doc = "Register `CPSW_NUSS_VBUSP_SOFT_IDLE_REG` reader"]
pub type R = crate::R<CpswNussVbuspSoftIdleRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SOFT_IDLE_REG` writer"]
pub type W = crate::W<CpswNussVbuspSoftIdleRegSpec>;
#[doc = "Field `SOFT_IDLE` reader - 0:0\\]
Software Idle"]
pub type SoftIdleR = crate::BitReader;
#[doc = "Field `SOFT_IDLE` writer - 0:0\\]
Software Idle"]
pub type SoftIdleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software Idle"]
    #[inline(always)]
    pub fn soft_idle(&self) -> SoftIdleR {
        SoftIdleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software Idle"]
    #[inline(always)]
    #[must_use]
    pub fn soft_idle(&mut self) -> SoftIdleW<CpswNussVbuspSoftIdleRegSpec> {
        SoftIdleW::new(self, 0)
    }
}
#[doc = "CPSW Software Idle\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_soft_idle_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_soft_idle_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSoftIdleRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSoftIdleRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_soft_idle_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSoftIdleRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_soft_idle_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSoftIdleRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SOFT_IDLE_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSoftIdleRegSpec {
    const RESET_VALUE: u32 = 0;
}