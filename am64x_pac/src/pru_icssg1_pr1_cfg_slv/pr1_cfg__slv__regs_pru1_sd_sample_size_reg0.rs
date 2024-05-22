#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE0` reader - "]
pub type Pru1SdSampleSize0R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE0` writer - "]
pub type Pru1SdSampleSize0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_0` reader - "]
pub type Pru1FdWindowSize0R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_0` writer - "]
pub type Pru1FdWindowSize0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_0` reader - "]
pub type Pru1FdOneMinLimit0R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_0` writer - "]
pub type Pru1FdOneMinLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_0` reader - "]
pub type Pru1FdOneMin0R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_0` writer - "]
pub type Pru1FdOneMin0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_0` reader - "]
pub type Pru1FdOneMaxLimit0R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_0` writer - "]
pub type Pru1FdOneMaxLimit0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_0` reader - "]
pub type Pru1FdOneMax0R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_0` writer - "]
pub type Pru1FdOneMax0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_0` reader - "]
pub type Pru1FdEn0R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_0` writer - "]
pub type Pru1FdEn0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size0(&self) -> Pru1SdSampleSize0R {
        Pru1SdSampleSize0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_0(&self) -> Pru1FdWindowSize0R {
        Pru1FdWindowSize0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_0(&self) -> Pru1FdOneMinLimit0R {
        Pru1FdOneMinLimit0R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_0(&self) -> Pru1FdOneMin0R {
        Pru1FdOneMin0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_0(&self) -> Pru1FdOneMaxLimit0R {
        Pru1FdOneMaxLimit0R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_0(&self) -> Pru1FdOneMax0R {
        Pru1FdOneMax0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_0(&self) -> Pru1FdEn0R {
        Pru1FdEn0R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size0(
        &mut self,
    ) -> Pru1SdSampleSize0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1SdSampleSize0W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_0(
        &mut self,
    ) -> Pru1FdWindowSize0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdWindowSize0W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_0(
        &mut self,
    ) -> Pru1FdOneMinLimit0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdOneMinLimit0W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_0(&mut self) -> Pru1FdOneMin0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdOneMin0W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_0(
        &mut self,
    ) -> Pru1FdOneMaxLimit0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdOneMaxLimit0W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_0(&mut self) -> Pru1FdOneMax0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdOneMax0W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_0(&mut self) -> Pru1FdEn0W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec> {
        Pru1FdEn0W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg0::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg0 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg0Spec {
    const RESET_VALUE: u32 = 0;
}
