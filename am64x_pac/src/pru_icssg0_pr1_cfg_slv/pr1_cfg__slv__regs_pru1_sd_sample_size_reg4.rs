#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE4` reader - "]
pub type Pru1SdSampleSize4R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE4` writer - "]
pub type Pru1SdSampleSize4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_4` reader - "]
pub type Pru1FdWindowSize4R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_4` writer - "]
pub type Pru1FdWindowSize4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_4` reader - "]
pub type Pru1FdOneMinLimit4R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_4` writer - "]
pub type Pru1FdOneMinLimit4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_4` reader - "]
pub type Pru1FdOneMin4R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_4` writer - "]
pub type Pru1FdOneMin4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_4` reader - "]
pub type Pru1FdOneMaxLimit4R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_4` writer - "]
pub type Pru1FdOneMaxLimit4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_4` reader - "]
pub type Pru1FdOneMax4R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_4` writer - "]
pub type Pru1FdOneMax4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_4` reader - "]
pub type Pru1FdEn4R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_4` writer - "]
pub type Pru1FdEn4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size4(&self) -> Pru1SdSampleSize4R {
        Pru1SdSampleSize4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_4(&self) -> Pru1FdWindowSize4R {
        Pru1FdWindowSize4R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_4(&self) -> Pru1FdOneMinLimit4R {
        Pru1FdOneMinLimit4R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_4(&self) -> Pru1FdOneMin4R {
        Pru1FdOneMin4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_4(&self) -> Pru1FdOneMaxLimit4R {
        Pru1FdOneMaxLimit4R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_4(&self) -> Pru1FdOneMax4R {
        Pru1FdOneMax4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_4(&self) -> Pru1FdEn4R {
        Pru1FdEn4R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size4(
        &mut self,
    ) -> Pru1SdSampleSize4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1SdSampleSize4W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_4(
        &mut self,
    ) -> Pru1FdWindowSize4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdWindowSize4W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_4(
        &mut self,
    ) -> Pru1FdOneMinLimit4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdOneMinLimit4W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_4(&mut self) -> Pru1FdOneMin4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdOneMin4W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_4(
        &mut self,
    ) -> Pru1FdOneMaxLimit4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdOneMaxLimit4W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_4(&mut self) -> Pru1FdOneMax4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdOneMax4W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_4(&mut self) -> Pru1FdEn4W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec> {
        Pru1FdEn4W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg4::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg4 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg4Spec {
    const RESET_VALUE: u32 = 0;
}
