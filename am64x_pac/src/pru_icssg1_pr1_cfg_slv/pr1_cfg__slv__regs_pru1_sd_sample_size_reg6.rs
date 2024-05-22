#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE6` reader - "]
pub type Pru1SdSampleSize6R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE6` writer - "]
pub type Pru1SdSampleSize6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_6` reader - "]
pub type Pru1FdWindowSize6R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_6` writer - "]
pub type Pru1FdWindowSize6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_6` reader - "]
pub type Pru1FdOneMinLimit6R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_6` writer - "]
pub type Pru1FdOneMinLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_6` reader - "]
pub type Pru1FdOneMin6R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_6` writer - "]
pub type Pru1FdOneMin6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_6` reader - "]
pub type Pru1FdOneMaxLimit6R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_6` writer - "]
pub type Pru1FdOneMaxLimit6W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_6` reader - "]
pub type Pru1FdOneMax6R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_6` writer - "]
pub type Pru1FdOneMax6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_6` reader - "]
pub type Pru1FdEn6R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_6` writer - "]
pub type Pru1FdEn6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size6(&self) -> Pru1SdSampleSize6R {
        Pru1SdSampleSize6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_6(&self) -> Pru1FdWindowSize6R {
        Pru1FdWindowSize6R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_6(&self) -> Pru1FdOneMinLimit6R {
        Pru1FdOneMinLimit6R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_6(&self) -> Pru1FdOneMin6R {
        Pru1FdOneMin6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_6(&self) -> Pru1FdOneMaxLimit6R {
        Pru1FdOneMaxLimit6R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_6(&self) -> Pru1FdOneMax6R {
        Pru1FdOneMax6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_6(&self) -> Pru1FdEn6R {
        Pru1FdEn6R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size6(
        &mut self,
    ) -> Pru1SdSampleSize6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1SdSampleSize6W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_6(
        &mut self,
    ) -> Pru1FdWindowSize6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdWindowSize6W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_6(
        &mut self,
    ) -> Pru1FdOneMinLimit6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdOneMinLimit6W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_6(&mut self) -> Pru1FdOneMin6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdOneMin6W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_6(
        &mut self,
    ) -> Pru1FdOneMaxLimit6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdOneMaxLimit6W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_6(&mut self) -> Pru1FdOneMax6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdOneMax6W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_6(&mut self) -> Pru1FdEn6W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec> {
        Pru1FdEn6W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg6::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg6 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg6Spec {
    const RESET_VALUE: u32 = 0;
}
