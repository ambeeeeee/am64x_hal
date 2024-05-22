#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE7` reader - "]
pub type Pru1SdSampleSize7R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE7` writer - "]
pub type Pru1SdSampleSize7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_7` reader - "]
pub type Pru1FdWindowSize7R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_7` writer - "]
pub type Pru1FdWindowSize7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_7` reader - "]
pub type Pru1FdOneMinLimit7R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_7` writer - "]
pub type Pru1FdOneMinLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_7` reader - "]
pub type Pru1FdOneMin7R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_7` writer - "]
pub type Pru1FdOneMin7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_7` reader - "]
pub type Pru1FdOneMaxLimit7R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_7` writer - "]
pub type Pru1FdOneMaxLimit7W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_7` reader - "]
pub type Pru1FdOneMax7R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_7` writer - "]
pub type Pru1FdOneMax7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_7` reader - "]
pub type Pru1FdEn7R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_7` writer - "]
pub type Pru1FdEn7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size7(&self) -> Pru1SdSampleSize7R {
        Pru1SdSampleSize7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_7(&self) -> Pru1FdWindowSize7R {
        Pru1FdWindowSize7R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_7(&self) -> Pru1FdOneMinLimit7R {
        Pru1FdOneMinLimit7R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_7(&self) -> Pru1FdOneMin7R {
        Pru1FdOneMin7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_7(&self) -> Pru1FdOneMaxLimit7R {
        Pru1FdOneMaxLimit7R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_7(&self) -> Pru1FdOneMax7R {
        Pru1FdOneMax7R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_7(&self) -> Pru1FdEn7R {
        Pru1FdEn7R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size7(
        &mut self,
    ) -> Pru1SdSampleSize7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1SdSampleSize7W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_7(
        &mut self,
    ) -> Pru1FdWindowSize7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdWindowSize7W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_7(
        &mut self,
    ) -> Pru1FdOneMinLimit7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdOneMinLimit7W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_7(&mut self) -> Pru1FdOneMin7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdOneMin7W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_7(
        &mut self,
    ) -> Pru1FdOneMaxLimit7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdOneMaxLimit7W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_7(&mut self) -> Pru1FdOneMax7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdOneMax7W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_7(&mut self) -> Pru1FdEn7W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec> {
        Pru1FdEn7W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg7::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg7 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg7Spec {
    const RESET_VALUE: u32 = 0;
}
