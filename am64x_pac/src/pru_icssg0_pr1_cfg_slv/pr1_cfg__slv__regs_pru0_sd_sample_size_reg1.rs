#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec>;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE1` reader - "]
pub type Pru0SdSampleSize1R = crate::FieldReader;
#[doc = "Field `PRU0_SD_SAMPLE_SIZE1` writer - "]
pub type Pru0SdSampleSize1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_1` reader - "]
pub type Pru0FdWindowSize1R = crate::FieldReader;
#[doc = "Field `PRU0_FD_WINDOW_SIZE_1` writer - "]
pub type Pru0FdWindowSize1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_1` reader - "]
pub type Pru0FdOneMinLimit1R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MIN_LIMIT_1` writer - "]
pub type Pru0FdOneMinLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MIN_1` reader - "]
pub type Pru0FdOneMin1R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MIN_1` writer - "]
pub type Pru0FdOneMin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_1` reader - "]
pub type Pru0FdOneMaxLimit1R = crate::FieldReader;
#[doc = "Field `PRU0_FD_ONE_MAX_LIMIT_1` writer - "]
pub type Pru0FdOneMaxLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU0_FD_ONE_MAX_1` reader - "]
pub type Pru0FdOneMax1R = crate::BitReader;
#[doc = "Field `PRU0_FD_ONE_MAX_1` writer - "]
pub type Pru0FdOneMax1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_FD_EN_1` reader - "]
pub type Pru0FdEn1R = crate::BitReader;
#[doc = "Field `PRU0_FD_EN_1` writer - "]
pub type Pru0FdEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru0_sd_sample_size1(&self) -> Pru0SdSampleSize1R {
        Pru0SdSampleSize1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru0_fd_window_size_1(&self) -> Pru0FdWindowSize1R {
        Pru0FdWindowSize1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru0_fd_one_min_limit_1(&self) -> Pru0FdOneMinLimit1R {
        Pru0FdOneMinLimit1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_fd_one_min_1(&self) -> Pru0FdOneMin1R {
        Pru0FdOneMin1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru0_fd_one_max_limit_1(&self) -> Pru0FdOneMaxLimit1R {
        Pru0FdOneMaxLimit1R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru0_fd_one_max_1(&self) -> Pru0FdOneMax1R {
        Pru0FdOneMax1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru0_fd_en_1(&self) -> Pru0FdEn1R {
        Pru0FdEn1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_sample_size1(
        &mut self,
    ) -> Pru0SdSampleSize1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0SdSampleSize1W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_window_size_1(
        &mut self,
    ) -> Pru0FdWindowSize1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdWindowSize1W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_limit_1(
        &mut self,
    ) -> Pru0FdOneMinLimit1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdOneMinLimit1W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_min_1(&mut self) -> Pru0FdOneMin1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdOneMin1W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_limit_1(
        &mut self,
    ) -> Pru0FdOneMaxLimit1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdOneMaxLimit1W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_one_max_1(&mut self) -> Pru0FdOneMax1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdOneMax1W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_fd_en_1(&mut self) -> Pru0FdEn1W<Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec> {
        Pru0FdEn1W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_sample_size_reg1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_sample_size_reg1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdSampleSizeReg1Spec {
    const RESET_VALUE: u32 = 0;
}
