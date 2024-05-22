#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE5` reader - "]
pub type Pru1SdSampleSize5R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE5` writer - "]
pub type Pru1SdSampleSize5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_5` reader - "]
pub type Pru1FdWindowSize5R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_5` writer - "]
pub type Pru1FdWindowSize5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_5` reader - "]
pub type Pru1FdOneMinLimit5R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_5` writer - "]
pub type Pru1FdOneMinLimit5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_5` reader - "]
pub type Pru1FdOneMin5R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_5` writer - "]
pub type Pru1FdOneMin5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_5` reader - "]
pub type Pru1FdOneMaxLimit5R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_5` writer - "]
pub type Pru1FdOneMaxLimit5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_5` reader - "]
pub type Pru1FdOneMax5R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_5` writer - "]
pub type Pru1FdOneMax5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_5` reader - "]
pub type Pru1FdEn5R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_5` writer - "]
pub type Pru1FdEn5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size5(&self) -> Pru1SdSampleSize5R {
        Pru1SdSampleSize5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_5(&self) -> Pru1FdWindowSize5R {
        Pru1FdWindowSize5R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_5(&self) -> Pru1FdOneMinLimit5R {
        Pru1FdOneMinLimit5R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_5(&self) -> Pru1FdOneMin5R {
        Pru1FdOneMin5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_5(&self) -> Pru1FdOneMaxLimit5R {
        Pru1FdOneMaxLimit5R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_5(&self) -> Pru1FdOneMax5R {
        Pru1FdOneMax5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_5(&self) -> Pru1FdEn5R {
        Pru1FdEn5R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size5(
        &mut self,
    ) -> Pru1SdSampleSize5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1SdSampleSize5W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_5(
        &mut self,
    ) -> Pru1FdWindowSize5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdWindowSize5W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_5(
        &mut self,
    ) -> Pru1FdOneMinLimit5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdOneMinLimit5W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_5(&mut self) -> Pru1FdOneMin5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdOneMin5W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_5(
        &mut self,
    ) -> Pru1FdOneMaxLimit5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdOneMaxLimit5W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_5(&mut self) -> Pru1FdOneMax5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdOneMax5W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_5(&mut self) -> Pru1FdEn5W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec> {
        Pru1FdEn5W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg5::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg5 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg5Spec {
    const RESET_VALUE: u32 = 0;
}
