#[doc = "Register `CPSW_NUSS_VBUSP_EM_CONTROL_REG` reader"]
pub type R = crate::R<CpswNussVbuspEmControlRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_EM_CONTROL_REG` writer"]
pub type W = crate::W<CpswNussVbuspEmControlRegSpec>;
#[doc = "Field `FREE` reader - 0:0\\]
Emulation Free Bit"]
pub type FreeR = crate::BitReader;
#[doc = "Field `FREE` writer - 0:0\\]
Emulation Free Bit"]
pub type FreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFT` reader - 1:1\\]
Emulation Soft Bit"]
pub type SoftR = crate::BitReader;
#[doc = "Field `SOFT` writer - 1:1\\]
Emulation Soft Bit"]
pub type SoftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Emulation Free Bit"]
    #[inline(always)]
    pub fn free(&self) -> FreeR {
        FreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Soft Bit"]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Emulation Free Bit"]
    #[inline(always)]
    #[must_use]
    pub fn free(&mut self) -> FreeW<CpswNussVbuspEmControlRegSpec> {
        FreeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Emulation Soft Bit"]
    #[inline(always)]
    #[must_use]
    pub fn soft(&mut self) -> SoftW<CpswNussVbuspEmControlRegSpec> {
        SoftW::new(self, 1)
    }
}
#[doc = "CPSW Emulation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_em_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_em_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspEmControlRegSpec;
impl crate::RegisterSpec for CpswNussVbuspEmControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_em_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspEmControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_em_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspEmControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_EM_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNussVbuspEmControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
