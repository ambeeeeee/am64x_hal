#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec>;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE1` reader - "]
pub type Pru1SdSampleSize1R = crate::FieldReader;
#[doc = "Field `PRU1_SD_SAMPLE_SIZE1` writer - "]
pub type Pru1SdSampleSize1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_1` reader - "]
pub type Pru1FdWindowSize1R = crate::FieldReader;
#[doc = "Field `PRU1_FD_WINDOW_SIZE_1` writer - "]
pub type Pru1FdWindowSize1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_1` reader - "]
pub type Pru1FdOneMinLimit1R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MIN_LIMIT_1` writer - "]
pub type Pru1FdOneMinLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MIN_1` reader - "]
pub type Pru1FdOneMin1R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MIN_1` writer - "]
pub type Pru1FdOneMin1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_1` reader - "]
pub type Pru1FdOneMaxLimit1R = crate::FieldReader;
#[doc = "Field `PRU1_FD_ONE_MAX_LIMIT_1` writer - "]
pub type Pru1FdOneMaxLimit1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRU1_FD_ONE_MAX_1` reader - "]
pub type Pru1FdOneMax1R = crate::BitReader;
#[doc = "Field `PRU1_FD_ONE_MAX_1` writer - "]
pub type Pru1FdOneMax1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_FD_EN_1` reader - "]
pub type Pru1FdEn1R = crate::BitReader;
#[doc = "Field `PRU1_FD_EN_1` writer - "]
pub type Pru1FdEn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pru1_sd_sample_size1(&self) -> Pru1SdSampleSize1R {
        Pru1SdSampleSize1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pru1_fd_window_size_1(&self) -> Pru1FdWindowSize1R {
        Pru1FdWindowSize1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn pru1_fd_one_min_limit_1(&self) -> Pru1FdOneMinLimit1R {
        Pru1FdOneMinLimit1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_fd_one_min_1(&self) -> Pru1FdOneMin1R {
        Pru1FdOneMin1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn pru1_fd_one_max_limit_1(&self) -> Pru1FdOneMaxLimit1R {
        Pru1FdOneMaxLimit1R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pru1_fd_one_max_1(&self) -> Pru1FdOneMax1R {
        Pru1FdOneMax1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pru1_fd_en_1(&self) -> Pru1FdEn1R {
        Pru1FdEn1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_sample_size1(
        &mut self,
    ) -> Pru1SdSampleSize1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1SdSampleSize1W::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_window_size_1(
        &mut self,
    ) -> Pru1FdWindowSize1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdWindowSize1W::new(self, 8)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_limit_1(
        &mut self,
    ) -> Pru1FdOneMinLimit1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdOneMinLimit1W::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_min_1(&mut self) -> Pru1FdOneMin1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdOneMin1W::new(self, 16)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_limit_1(
        &mut self,
    ) -> Pru1FdOneMaxLimit1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdOneMaxLimit1W::new(self, 17)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_one_max_1(&mut self) -> Pru1FdOneMax1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdOneMax1W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_fd_en_1(&mut self) -> Pru1FdEn1W<Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec> {
        Pru1FdEn1W::new(self, 23)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_sample_size_reg1::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_sample_size_reg1 to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdSampleSizeReg1Spec {
    const RESET_VALUE: u32 = 0;
}
