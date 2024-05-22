#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE3` reader - "]
pub type Pru1SdSampleSize3R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE3` writer - "]
pub type Pru1SdSampleSize3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_3` reader - "]
pub type Pru1FdWindowSize3R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_3` writer - "]
pub type Pru1FdWindowSize3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_3` reader - "]
pub type Pru1FdOneMinLimit3R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_3` writer - "]
pub type Pru1FdOneMinLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_3` reader - "]
pub type Pru1FdOneMin3R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_3` writer - "]
pub type Pru1FdOneMin3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_3` reader - "]
pub type Pru1FdOneMaxLimit3R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_3` writer - "]
pub type Pru1FdOneMaxLimit3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_3` reader - "]
pub type Pru1FdOneMax3R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_3` writer - "]
pub type Pru1FdOneMax3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_3` reader - "]
pub type Pru1FdEn3R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_3` writer - "]
pub type Pru1FdEn3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size3(&self) -> Pru1SdSampleSize3R {
        Pru1SdSampleSize3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_3(&self) -> Pru1FdWindowSize3R {
        Pru1FdWindowSize3R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_3(&self) -> Pru1FdOneMinLimit3R {
        Pru1FdOneMinLimit3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_3(&self) -> Pru1FdOneMin3R {
        Pru1FdOneMin3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_3(&self) -> Pru1FdOneMaxLimit3R {
        Pru1FdOneMaxLimit3R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_3(&self) -> Pru1FdOneMax3R {
        Pru1FdOneMax3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_3(&self) -> Pru1FdEn3R {
        Pru1FdEn3R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size3(
        &mut self,
    ) -> Pru1SdSampleSize3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1SdSampleSize3W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_3(
        &mut self,
    ) -> Pru1FdWindowSize3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdWindowSize3W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_3(
        &mut self,
    ) -> Pru1FdOneMinLimit3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdOneMinLimit3W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_3(&mut self) -> Pru1FdOneMin3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdOneMin3W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_3(
        &mut self,
    ) -> Pru1FdOneMaxLimit3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdOneMaxLimit3W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_3(&mut self) -> Pru1FdOneMax3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdOneMax3W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_3(&mut self) -> Pru1FdEn3W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec> {
        Pru1FdEn3W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg3::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg3 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg3Spec {
    const RESET_VALUE: u32 = 0;
}
