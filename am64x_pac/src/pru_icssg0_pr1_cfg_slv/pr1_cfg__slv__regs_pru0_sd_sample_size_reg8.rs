#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE8` reader - "]
pub type Pru0SdSampleSize8R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE8` writer - "]
pub type Pru0SdSampleSize8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_8` reader - "]
pub type Pru0FdWindowSize8R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_8` writer - "]
pub type Pru0FdWindowSize8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_8` reader - "]
pub type Pru0FdOneMinLimit8R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_8` writer - "]
pub type Pru0FdOneMinLimit8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_8` reader - "]
pub type Pru0FdOneMin8R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_8` writer - "]
pub type Pru0FdOneMin8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_8` reader - "]
pub type Pru0FdOneMaxLimit8R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_8` writer - "]
pub type Pru0FdOneMaxLimit8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_8` reader - "]
pub type Pru0FdOneMax8R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_8` writer - "]
pub type Pru0FdOneMax8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_8` reader - "]
pub type Pru0FdEn8R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_8` writer - "]
pub type Pru0FdEn8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size8(&self) -> Pru0SdSampleSize8R {
        Pru0SdSampleSize8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_8(&self) -> Pru0FdWindowSize8R {
        Pru0FdWindowSize8R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_8(&self) -> Pru0FdOneMinLimit8R {
        Pru0FdOneMinLimit8R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_8(&self) -> Pru0FdOneMin8R {
        Pru0FdOneMin8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_8(&self) -> Pru0FdOneMaxLimit8R {
        Pru0FdOneMaxLimit8R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_8(&self) -> Pru0FdOneMax8R {
        Pru0FdOneMax8R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_8(&self) -> Pru0FdEn8R {
        Pru0FdEn8R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size8(
        &mut self,
    ) -> Pru0SdSampleSize8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0SdSampleSize8W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_8(
        &mut self,
    ) -> Pru0FdWindowSize8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdWindowSize8W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_8(
        &mut self,
    ) -> Pru0FdOneMinLimit8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdOneMinLimit8W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_8(&mut self) -> Pru0FdOneMin8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdOneMin8W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_8(
        &mut self,
    ) -> Pru0FdOneMaxLimit8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdOneMaxLimit8W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_8(&mut self) -> Pru0FdOneMax8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdOneMax8W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_8(&mut self) -> Pru0FdEn8W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec> {
        Pru0FdEn8W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg8::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg8 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg8Spec {
    const RESET_VALUE: u32 = 0;
}
