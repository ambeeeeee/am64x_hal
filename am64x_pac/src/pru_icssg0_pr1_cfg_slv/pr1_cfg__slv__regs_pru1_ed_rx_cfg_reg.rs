#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec>;
#[doc = "Field `PRU1_ED_RX_SAMPLE_SIZE` reader - "]
pub type Pru1EdRxSampleSizeR = crate::FieldReader;
#[doc = "Field `PRU1_ED_RX_SAMPLE_SIZE` writer - "]
pub type Pru1EdRxSampleSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRU1_ED_RX_SB_POL` reader - "]
pub type Pru1EdRxSbPolR = crate::BitReader;
#[doc = "Field `PRU1_ED_RX_SB_POL` writer - "]
pub type Pru1EdRxSbPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_RX_CLK_SEL` reader - "]
pub type Pru1EdRxClkSelR = crate::BitReader;
#[doc = "Field `PRU1_ED_RX_CLK_SEL` writer - "]
pub type Pru1EdRxClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_RX_DIV_FACTOR_FRAC` reader - "]
pub type Pru1EdRxDivFactorFracR = crate::BitReader;
#[doc = "Field `PRU1_ED_RX_DIV_FACTOR_FRAC` writer - "]
pub type Pru1EdRxDivFactorFracW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_RX_DIV_FACTOR` reader - "]
pub type Pru1EdRxDivFactorR = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_RX_DIV_FACTOR` writer - "]
pub type Pru1EdRxDivFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pru1_ed_rx_sample_size(&self) -> Pru1EdRxSampleSizeR {
        Pru1EdRxSampleSizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pru1_ed_rx_sb_pol(&self) -> Pru1EdRxSbPolR {
        Pru1EdRxSbPolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pru1_ed_rx_clk_sel(&self) -> Pru1EdRxClkSelR {
        Pru1EdRxClkSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pru1_ed_rx_div_factor_frac(&self) -> Pru1EdRxDivFactorFracR {
        Pru1EdRxDivFactorFracR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pru1_ed_rx_div_factor(&self) -> Pru1EdRxDivFactorR {
        Pru1EdRxDivFactorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_sample_size(
        &mut self,
    ) -> Pru1EdRxSampleSizeW<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec> {
        Pru1EdRxSampleSizeW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_sb_pol(&mut self) -> Pru1EdRxSbPolW<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec> {
        Pru1EdRxSbPolW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_clk_sel(&mut self) -> Pru1EdRxClkSelW<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec> {
        Pru1EdRxClkSelW::new(self, 4)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_div_factor_frac(
        &mut self,
    ) -> Pru1EdRxDivFactorFracW<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec> {
        Pru1EdRxDivFactorFracW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_rx_div_factor(
        &mut self,
    ) -> Pru1EdRxDivFactorW<Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec> {
        Pru1EdRxDivFactorW::new(self, 16)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_ed_rx_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_ed_rx_cfg_reg to value 0x0f"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1EdRxCfgRegSpec {
    const RESET_VALUE: u32 = 0x0f;
}
