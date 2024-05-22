#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE2` reader - "]
pub type Pru1SdSampleSize2R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE2` writer - "]
pub type Pru1SdSampleSize2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_2` reader - "]
pub type Pru1FdWindowSize2R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_2` writer - "]
pub type Pru1FdWindowSize2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_2` reader - "]
pub type Pru1FdOneMinLimit2R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_2` writer - "]
pub type Pru1FdOneMinLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_2` reader - "]
pub type Pru1FdOneMin2R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_2` writer - "]
pub type Pru1FdOneMin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_2` reader - "]
pub type Pru1FdOneMaxLimit2R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_2` writer - "]
pub type Pru1FdOneMaxLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_2` reader - "]
pub type Pru1FdOneMax2R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_2` writer - "]
pub type Pru1FdOneMax2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_2` reader - "]
pub type Pru1FdEn2R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_2` writer - "]
pub type Pru1FdEn2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size2(&self) -> Pru1SdSampleSize2R {
        Pru1SdSampleSize2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_2(&self) -> Pru1FdWindowSize2R {
        Pru1FdWindowSize2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_2(&self) -> Pru1FdOneMinLimit2R {
        Pru1FdOneMinLimit2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_2(&self) -> Pru1FdOneMin2R {
        Pru1FdOneMin2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_2(&self) -> Pru1FdOneMaxLimit2R {
        Pru1FdOneMaxLimit2R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_2(&self) -> Pru1FdOneMax2R {
        Pru1FdOneMax2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_2(&self) -> Pru1FdEn2R {
        Pru1FdEn2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size2(
        &mut self,
    ) -> Pru1SdSampleSize2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1SdSampleSize2W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_2(
        &mut self,
    ) -> Pru1FdWindowSize2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdWindowSize2W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_2(
        &mut self,
    ) -> Pru1FdOneMinLimit2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdOneMinLimit2W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_2(&mut self) -> Pru1FdOneMin2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdOneMin2W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_2(
        &mut self,
    ) -> Pru1FdOneMaxLimit2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdOneMaxLimit2W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_2(&mut self) -> Pru1FdOneMax2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdOneMax2W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_2(&mut self) -> Pru1FdEn2W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec> {
        Pru1FdEn2W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg2Spec {
    const RESET_VALUE: u32 = 0;
}
