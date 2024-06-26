#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE3` reader - "]
pub type Pru0SdSampleSize3R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE3` writer - "]
pub type Pru0SdSampleSize3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_3` reader - "]
pub type Pru0FdWindowSize3R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_3` writer - "]
pub type Pru0FdWindowSize3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_3` reader - "]
pub type Pru0FdOneMinLimit3R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_3` writer - "]
pub type Pru0FdOneMinLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_3` reader - "]
pub type Pru0FdOneMin3R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_3` writer - "]
pub type Pru0FdOneMin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_3` reader - "]
pub type Pru0FdOneMaxLimit3R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_3` writer - "]
pub type Pru0FdOneMaxLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_3` reader - "]
pub type Pru0FdOneMax3R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_3` writer - "]
pub type Pru0FdOneMax3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_3` reader - "]
pub type Pru0FdEn3R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_3` writer - "]
pub type Pru0FdEn3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size3(&self) -> Pru0SdSampleSize3R {
        Pru0SdSampleSize3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_3(&self) -> Pru0FdWindowSize3R {
        Pru0FdWindowSize3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_3(&self) -> Pru0FdOneMinLimit3R {
        Pru0FdOneMinLimit3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_3(&self) -> Pru0FdOneMin3R {
        Pru0FdOneMin3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_3(&self) -> Pru0FdOneMaxLimit3R {
        Pru0FdOneMaxLimit3R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_3(&self) -> Pru0FdOneMax3R {
        Pru0FdOneMax3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_3(&self) -> Pru0FdEn3R {
        Pru0FdEn3R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size3(
        &mut self,
    ) -> Pru0SdSampleSize3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0SdSampleSize3W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_3(
        &mut self,
    ) -> Pru0FdWindowSize3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdWindowSize3W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_3(
        &mut self,
    ) -> Pru0FdOneMinLimit3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdOneMinLimit3W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_3(&mut self) -> Pru0FdOneMin3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdOneMin3W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_3(
        &mut self,
    ) -> Pru0FdOneMaxLimit3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdOneMaxLimit3W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_3(&mut self) -> Pru0FdOneMax3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdOneMax3W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_3(&mut self) -> Pru0FdEn3W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec> {
        Pru0FdEn3W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg3 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg3Spec {
    const RESET_VALUE: u32 = 0;
}
