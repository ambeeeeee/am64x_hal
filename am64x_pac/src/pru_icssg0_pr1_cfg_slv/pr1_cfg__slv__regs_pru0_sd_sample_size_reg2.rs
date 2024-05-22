#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE2` reader - "]
pub type Pru0SdSampleSize2R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE2` writer - "]
pub type Pru0SdSampleSize2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_2` reader - "]
pub type Pru0FdWindowSize2R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_2` writer - "]
pub type Pru0FdWindowSize2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_2` reader - "]
pub type Pru0FdOneMinLimit2R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_2` writer - "]
pub type Pru0FdOneMinLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_2` reader - "]
pub type Pru0FdOneMin2R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_2` writer - "]
pub type Pru0FdOneMin2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_2` reader - "]
pub type Pru0FdOneMaxLimit2R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_2` writer - "]
pub type Pru0FdOneMaxLimit2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_2` reader - "]
pub type Pru0FdOneMax2R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_2` writer - "]
pub type Pru0FdOneMax2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_2` reader - "]
pub type Pru0FdEn2R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_2` writer - "]
pub type Pru0FdEn2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size2(&self) -> Pru0SdSampleSize2R {
        Pru0SdSampleSize2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_2(&self) -> Pru0FdWindowSize2R {
        Pru0FdWindowSize2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_2(&self) -> Pru0FdOneMinLimit2R {
        Pru0FdOneMinLimit2R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_2(&self) -> Pru0FdOneMin2R {
        Pru0FdOneMin2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_2(&self) -> Pru0FdOneMaxLimit2R {
        Pru0FdOneMaxLimit2R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_2(&self) -> Pru0FdOneMax2R {
        Pru0FdOneMax2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_2(&self) -> Pru0FdEn2R {
        Pru0FdEn2R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size2(
        &mut self,
    ) -> Pru0SdSampleSize2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0SdSampleSize2W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_2(
        &mut self,
    ) -> Pru0FdWindowSize2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdWindowSize2W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_2(
        &mut self,
    ) -> Pru0FdOneMinLimit2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdOneMinLimit2W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_2(&mut self) -> Pru0FdOneMin2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdOneMin2W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_2(
        &mut self,
    ) -> Pru0FdOneMaxLimit2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdOneMaxLimit2W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_2(&mut self) -> Pru0FdOneMax2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdOneMax2W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_2(&mut self) -> Pru0FdEn2W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec> {
        Pru0FdEn2W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg2::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg2 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg2Spec {
    const RESET_VALUE: u32 = 0;
}
