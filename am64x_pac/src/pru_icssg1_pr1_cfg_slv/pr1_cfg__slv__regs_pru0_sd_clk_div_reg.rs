#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec>;
#[doc = "Field `PRU0_SD_SHARE_EN` reader - "]
pub type Pru0SdShareEnR = crate::BitReader;
#[doc = "Field `PRU0_SD_SHARE_EN` writer - "]
pub type Pru0SdShareEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_MAN_EN` reader - "]
pub type Pru0SdManEnR = crate::BitReader;
#[doc = "Field `PRU0_SD_MAN_EN` writer - "]
pub type Pru0SdManEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_MAN_NV_DATA_EN` reader - "]
pub type Pru0SdManNvDataEnR = crate::BitReader;
#[doc = "Field `PRU0_SD_MAN_NV_DATA_EN` writer - "]
pub type Pru0SdManNvDataEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_CH_SEL` reader - "]
pub type Pru0SdChSelR = crate::FieldReader;
#[doc = "Field `PRU0_SD_CH_SEL` writer - "]
pub type Pru0SdChSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRU0_SD_MAN_STATUS` reader - "]
pub type Pru0SdManStatusR = crate::BitReader;
#[doc = "Field `PRU0_SD_MAN_STATUS` writer - "]
pub type Pru0SdManStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_MAN_CLK_CAL_DONE` reader - "]
pub type Pru0SdManClkCalDoneR = crate::BitReader;
#[doc = "Field `PRU0_SD_MAN_CLK_CAL_DONE` writer - "]
pub type Pru0SdManClkCalDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_SD_MAN_REC_CLK_PERIOD` reader - "]
pub type Pru0SdManRecClkPeriodR = crate::FieldReader;
#[doc = "Field `PRU0_SD_MAN_REC_CLK_PERIOD` writer - "]
pub type Pru0SdManRecClkPeriodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pru0_sd_share_en(&self) -> Pru0SdShareEnR {
        Pru0SdShareEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pru0_sd_man_en(&self) -> Pru0SdManEnR {
        Pru0SdManEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pru0_sd_man_nv_data_en(&self) -> Pru0SdManNvDataEnR {
        Pru0SdManNvDataEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn pru0_sd_ch_sel(&self) -> Pru0SdChSelR {
        Pru0SdChSelR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pru0_sd_man_status(&self) -> Pru0SdManStatusR {
        Pru0SdManStatusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pru0_sd_man_clk_cal_done(&self) -> Pru0SdManClkCalDoneR {
        Pru0SdManClkCalDoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn pru0_sd_man_rec_clk_period(&self) -> Pru0SdManRecClkPeriodR {
        Pru0SdManRecClkPeriodR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_share_en(&mut self) -> Pru0SdShareEnW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdShareEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_man_en(&mut self) -> Pru0SdManEnW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdManEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_man_nv_data_en(
        &mut self,
    ) -> Pru0SdManNvDataEnW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdManNvDataEnW::new(self, 10)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_ch_sel(&mut self) -> Pru0SdChSelW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdChSelW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_man_status(&mut self) -> Pru0SdManStatusW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdManStatusW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_man_clk_cal_done(
        &mut self,
    ) -> Pru0SdManClkCalDoneW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdManClkCalDoneW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_sd_man_rec_clk_period(
        &mut self,
    ) -> Pru0SdManRecClkPeriodW<Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec> {
        Pru0SdManRecClkPeriodW::new(self, 24)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_sd_clk_div_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_sd_clk_div_reg to value 0"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0SdClkDivRegSpec {
    const RESET_VALUE: u32 = 0;
}
