#[doc = "Register `CPSW_NUSS_VBUSP_SOFT_RESET_REG` reader"]
pub type R = crate::R<CpswNussVbuspSoftResetRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_SOFT_RESET_REG` writer"]
pub type W = crate::W<CpswNussVbuspSoftResetRegSpec>;
#[doc = "Field `SOFT_RESET` reader - 0:0\\]
Software reset"]
pub type SoftResetR = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - 0:0\\]
Software reset"]
pub type SoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT_SOFT_RESET` reader - 1:1\\]
Transmit and receive software reset"]
pub type RtSoftResetR = crate::BitReader;
#[doc = "Field `RT_SOFT_RESET` writer - 1:1\\]
Transmit and receive software reset"]
pub type RtSoftResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SoftResetR {
        SoftResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit and receive software reset"]
    #[inline(always)]
    pub fn rt_soft_reset(&self) -> RtSoftResetR {
        RtSoftResetR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SoftResetW<CpswNussVbuspSoftResetRegSpec> {
        SoftResetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit and receive software reset"]
    #[inline(always)]
    #[must_use]
    pub fn rt_soft_reset(&mut self) -> RtSoftResetW<CpswNussVbuspSoftResetRegSpec> {
        RtSoftResetW::new(self, 1)
    }
}
#[doc = "SGMII Soft Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_soft_reset_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_soft_reset_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspSoftResetRegSpec;
impl crate::RegisterSpec for CpswNussVbuspSoftResetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_soft_reset_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspSoftResetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_soft_reset_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspSoftResetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_SOFT_RESET_REG to value 0"]
impl crate::Resettable for CpswNussVbuspSoftResetRegSpec {
    const RESET_VALUE: u32 = 0;
}
