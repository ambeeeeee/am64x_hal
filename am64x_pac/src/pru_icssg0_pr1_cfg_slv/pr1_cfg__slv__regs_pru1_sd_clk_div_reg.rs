#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec>;
#[doc = "Field `PRU1_SD_SHARE_EN` reader - "]
pub type Pru1SdShareEnR = crate::BitReader;
#[doc = "Field `PRU1_SD_SHARE_EN` writer - "]
pub type Pru1SdShareEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_MAN_EN` reader - "]
pub type Pru1SdManEnR = crate::BitReader;
#[doc = "Field `PRU1_SD_MAN_EN` writer - "]
pub type Pru1SdManEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_MAN_NV_DATA_EN` reader - "]
pub type Pru1SdManNvDataEnR = crate::BitReader;
#[doc = "Field `PRU1_SD_MAN_NV_DATA_EN` writer - "]
pub type Pru1SdManNvDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_CH_SEL` reader - "]
pub type Pru1SdChSelR = crate::FieldReader;
#[doc = "Field `PRU1_SD_CH_SEL` writer - "]
pub type Pru1SdChSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRU1_SD_MAN_STATUS` reader - "]
pub type Pru1SdManStatusR = crate::BitReader;
#[doc = "Field `PRU1_SD_MAN_STATUS` writer - "]
pub type Pru1SdManStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_MAN_CLK_CAL_DONE` reader - "]
pub type Pru1SdManClkCalDoneR = crate::BitReader;
#[doc = "Field `PRU1_SD_MAN_CLK_CAL_DONE` writer - "]
pub type Pru1SdManClkCalDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_SD_MAN_REC_CLK_PERIOD` reader - "]
pub type Pru1SdManRecClkPeriodR = crate::FieldReader;
#[doc = "Field `PRU1_SD_MAN_REC_CLK_PERIOD` writer - "]
pub type Pru1SdManRecClkPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pru1_sd_share_en(&self) -> Pru1SdShareEnR {
        Pru1SdShareEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pru1_sd_man_en(&self) -> Pru1SdManEnR {
        Pru1SdManEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pru1_sd_man_nv_data_en(&self) -> Pru1SdManNvDataEnR {
        Pru1SdManNvDataEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn pru1_sd_ch_sel(&self) -> Pru1SdChSelR {
        Pru1SdChSelR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pru1_sd_man_status(&self) -> Pru1SdManStatusR {
        Pru1SdManStatusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru1_sd_man_clk_cal_done(&self) -> Pru1SdManClkCalDoneR {
        Pru1SdManClkCalDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pru1_sd_man_rec_clk_period(&self) -> Pru1SdManRecClkPeriodR {
        Pru1SdManRecClkPeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_share_en(&mut self) -> Pru1SdShareEnW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdShareEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_man_en(&mut self) -> Pru1SdManEnW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdManEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_man_nv_data_en(
        &mut self,
    ) -> Pru1SdManNvDataEnW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdManNvDataEnW::new(self, 10)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_ch_sel(&mut self) -> Pru1SdChSelW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdChSelW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_man_status(&mut self) -> Pru1SdManStatusW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdManStatusW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_man_clk_cal_done(
        &mut self,
    ) -> Pru1SdManClkCalDoneW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdManClkCalDoneW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_sd_man_rec_clk_period(
        &mut self,
    ) -> Pru1SdManRecClkPeriodW<Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec> {
        Pru1SdManRecClkPeriodW::new(self, 24)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_sd_clk_div_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_sd_clk_div_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1SdClkDivRegSpec {
    const RESET_VALUE: u32 = 0;
}
