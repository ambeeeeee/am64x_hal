#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE0` reader - "]
pub type Pru0SdSampleSize0R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE0` writer - "]
pub type Pru0SdSampleSize0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_0` reader - "]
pub type Pru0FdWindowSize0R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_0` writer - "]
pub type Pru0FdWindowSize0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_0` reader - "]
pub type Pru0FdOneMinLimit0R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_0` writer - "]
pub type Pru0FdOneMinLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_0` reader - "]
pub type Pru0FdOneMin0R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_0` writer - "]
pub type Pru0FdOneMin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_0` reader - "]
pub type Pru0FdOneMaxLimit0R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_0` writer - "]
pub type Pru0FdOneMaxLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_0` reader - "]
pub type Pru0FdOneMax0R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_0` writer - "]
pub type Pru0FdOneMax0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_0` reader - "]
pub type Pru0FdEn0R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_0` writer - "]
pub type Pru0FdEn0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size0(&self) -> Pru0SdSampleSize0R {
        Pru0SdSampleSize0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_0(&self) -> Pru0FdWindowSize0R {
        Pru0FdWindowSize0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_0(&self) -> Pru0FdOneMinLimit0R {
        Pru0FdOneMinLimit0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_0(&self) -> Pru0FdOneMin0R {
        Pru0FdOneMin0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_0(&self) -> Pru0FdOneMaxLimit0R {
        Pru0FdOneMaxLimit0R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_0(&self) -> Pru0FdOneMax0R {
        Pru0FdOneMax0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_0(&self) -> Pru0FdEn0R {
        Pru0FdEn0R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size0(
        &mut self,
    ) -> Pru0SdSampleSize0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0SdSampleSize0W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_0(
        &mut self,
    ) -> Pru0FdWindowSize0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdWindowSize0W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_0(
        &mut self,
    ) -> Pru0FdOneMinLimit0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdOneMinLimit0W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_0(&mut self) -> Pru0FdOneMin0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdOneMin0W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_0(
        &mut self,
    ) -> Pru0FdOneMaxLimit0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdOneMaxLimit0W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_0(&mut self) -> Pru0FdOneMax0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdOneMax0W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_0(&mut self) -> Pru0FdEn0W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec> {
        Pru0FdEn0W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg0Spec {
    const RESET_VALUE: u32 = 0;
}
