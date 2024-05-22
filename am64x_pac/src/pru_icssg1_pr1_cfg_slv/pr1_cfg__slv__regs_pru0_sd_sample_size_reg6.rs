#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE6` reader - "]
pub type Pru0SdSampleSize6R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE6` writer - "]
pub type Pru0SdSampleSize6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_6` reader - "]
pub type Pru0FdWindowSize6R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_6` writer - "]
pub type Pru0FdWindowSize6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_6` reader - "]
pub type Pru0FdOneMinLimit6R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_6` writer - "]
pub type Pru0FdOneMinLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_6` reader - "]
pub type Pru0FdOneMin6R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_6` writer - "]
pub type Pru0FdOneMin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_6` reader - "]
pub type Pru0FdOneMaxLimit6R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_6` writer - "]
pub type Pru0FdOneMaxLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_6` reader - "]
pub type Pru0FdOneMax6R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_6` writer - "]
pub type Pru0FdOneMax6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_6` reader - "]
pub type Pru0FdEn6R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_6` writer - "]
pub type Pru0FdEn6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size6(&self) -> Pru0SdSampleSize6R {
        Pru0SdSampleSize6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_6(&self) -> Pru0FdWindowSize6R {
        Pru0FdWindowSize6R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_6(&self) -> Pru0FdOneMinLimit6R {
        Pru0FdOneMinLimit6R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_6(&self) -> Pru0FdOneMin6R {
        Pru0FdOneMin6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_6(&self) -> Pru0FdOneMaxLimit6R {
        Pru0FdOneMaxLimit6R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_6(&self) -> Pru0FdOneMax6R {
        Pru0FdOneMax6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_6(&self) -> Pru0FdEn6R {
        Pru0FdEn6R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size6(
        &mut self,
    ) -> Pru0SdSampleSize6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0SdSampleSize6W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_6(
        &mut self,
    ) -> Pru0FdWindowSize6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdWindowSize6W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_6(
        &mut self,
    ) -> Pru0FdOneMinLimit6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdOneMinLimit6W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_6(&mut self) -> Pru0FdOneMin6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdOneMin6W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_6(
        &mut self,
    ) -> Pru0FdOneMaxLimit6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdOneMaxLimit6W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_6(&mut self) -> Pru0FdOneMax6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdOneMax6W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_6(&mut self) -> Pru0FdEn6W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec> {
        Pru0FdEn6W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg6::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg6 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg6Spec {
    const RESET_VALUE: u32 = 0;
}
