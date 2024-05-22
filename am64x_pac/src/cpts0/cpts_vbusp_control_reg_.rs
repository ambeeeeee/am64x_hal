#[doc = "Register `CPTS_VBUSP_CONTROL_REG_` reader"]
pub type R = crate::R<CptsVbuspControlReg_Spec>;
#[doc = "Register `CPTS_VBUSP_CONTROL_REG_` writer"]
pub type W = crate::W<CptsVbuspControlReg_Spec>;
#[doc = "Field `PPM_DIR` reader - 0:0\\]
Time Stamp Generate Function PPM Direction"]
pub type PpmDirR = crate::BitReader;
#[doc = "Field `PPM_DIR` writer - 0:0\\]
Time Stamp Generate Function PPM Direction"]
pub type PpmDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY_INV` reader - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
pub type PolarityInvR = crate::BitReader;
#[doc = "Field `POLARITY_INV` writer - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
pub type PolarityInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp Generate Function PPM Direction"]
    #[inline(always)]
    pub fn ppm_dir(&self) -> PpmDirR {
        PpmDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
    #[inline(always)]
    pub fn polarity_inv(&self) -> PolarityInvR {
        PolarityInvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time Stamp Generate Function PPM Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ppm_dir(&mut self) -> PpmDirW<CptsVbuspControlReg_Spec> {
        PpmDirW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Time Stamp Generate Function Polarity Invert"]
    #[inline(always)]
    #[must_use]
    pub fn polarity_inv(&mut self) -> PolarityInvW<CptsVbuspControlReg_Spec> {
        PolarityInvW::new(self, 1)
    }
}
#[doc = "Time Stamp Generate Function Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_control_reg_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_control_reg_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspControlReg_Spec;
impl crate::RegisterSpec for CptsVbuspControlReg_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_control_reg_::R`](R) reader structure"]
impl crate::Readable for CptsVbuspControlReg_Spec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_control_reg_::W`](W) writer structure"]
impl crate::Writable for CptsVbuspControlReg_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_CONTROL_REG_ to value 0"]
impl crate::Resettable for CptsVbuspControlReg_Spec {
    const RESET_VALUE: u32 = 0;
}
