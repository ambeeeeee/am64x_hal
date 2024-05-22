#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec>;
#[doc = "Field `PRU1_ED_TX_CLK_SEL` reader - "]
pub type Pru1EdTxClkSelR = crate::BitReader;
#[doc = "Field `PRU1_ED_TX_CLK_SEL` writer - "]
pub type Pru1EdTxClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_BUSY_0` reader - "]
pub type Pru1EdBusy0R = crate::BitReader;
#[doc = "Field `PRU1_ED_BUSY_0` writer - "]
pub type Pru1EdBusy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_BUSY_1` reader - "]
pub type Pru1EdBusy1R = crate::BitReader;
#[doc = "Field `PRU1_ED_BUSY_1` writer - "]
pub type Pru1EdBusy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_BUSY_2` reader - "]
pub type Pru1EdBusy2R = crate::BitReader;
#[doc = "Field `PRU1_ED_BUSY_2` writer - "]
pub type Pru1EdBusy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ENDAT0_CLK_SYNC` reader - "]
pub type Pru1Endat0ClkSyncR = crate::BitReader;
#[doc = "Field `PRU1_ENDAT0_CLK_SYNC` writer - "]
pub type Pru1Endat0ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ENDAT1_CLK_SYNC` reader - "]
pub type Pru1Endat1ClkSyncR = crate::BitReader;
#[doc = "Field `PRU1_ENDAT1_CLK_SYNC` writer - "]
pub type Pru1Endat1ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ENDAT2_CLK_SYNC` reader - "]
pub type Pru1Endat2ClkSyncR = crate::BitReader;
#[doc = "Field `PRU1_ENDAT2_CLK_SYNC` writer - "]
pub type Pru1Endat2ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ENDAT_SHARE_EN` reader - "]
pub type Pru1EndatShareEnR = crate::BitReader;
#[doc = "Field `PRU1_ENDAT_SHARE_EN` writer - "]
pub type Pru1EndatShareEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_TX_DIV_FACTOR_FRAC` reader - "]
pub type Pru1EdTxDivFactorFracR = crate::BitReader;
#[doc = "Field `PRU1_ED_TX_DIV_FACTOR_FRAC` writer - "]
pub type Pru1EdTxDivFactorFracW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_ED_TX_DIV_FACTOR` reader - "]
pub type Pru1EdTxDivFactorR = crate::FieldReader<u16>;
#[doc = "Field `PRU1_ED_TX_DIV_FACTOR` writer - "]
pub type Pru1EdTxDivFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pru1_ed_tx_clk_sel(&self) -> Pru1EdTxClkSelR {
        Pru1EdTxClkSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pru1_ed_busy_0(&self) -> Pru1EdBusy0R {
        Pru1EdBusy0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pru1_ed_busy_1(&self) -> Pru1EdBusy1R {
        Pru1EdBusy1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pru1_ed_busy_2(&self) -> Pru1EdBusy2R {
        Pru1EdBusy2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pru1_endat0_clk_sync(&self) -> Pru1Endat0ClkSyncR {
        Pru1Endat0ClkSyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pru1_endat1_clk_sync(&self) -> Pru1Endat1ClkSyncR {
        Pru1Endat1ClkSyncR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pru1_endat2_clk_sync(&self) -> Pru1Endat2ClkSyncR {
        Pru1Endat2ClkSyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pru1_endat_share_en(&self) -> Pru1EndatShareEnR {
        Pru1EndatShareEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pru1_ed_tx_div_factor_frac(&self) -> Pru1EdTxDivFactorFracR {
        Pru1EdTxDivFactorFracR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pru1_ed_tx_div_factor(&self) -> Pru1EdTxDivFactorR {
        Pru1EdTxDivFactorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_clk_sel(&mut self) -> Pru1EdTxClkSelW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdTxClkSelW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_busy_0(&mut self) -> Pru1EdBusy0W<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdBusy0W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_busy_1(&mut self) -> Pru1EdBusy1W<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdBusy1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_busy_2(&mut self) -> Pru1EdBusy2W<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdBusy2W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_endat0_clk_sync(
        &mut self,
    ) -> Pru1Endat0ClkSyncW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1Endat0ClkSyncW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_endat1_clk_sync(
        &mut self,
    ) -> Pru1Endat1ClkSyncW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1Endat1ClkSyncW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_endat2_clk_sync(
        &mut self,
    ) -> Pru1Endat2ClkSyncW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1Endat2ClkSyncW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_endat_share_en(&mut self) -> Pru1EndatShareEnW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EndatShareEnW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_div_factor_frac(
        &mut self,
    ) -> Pru1EdTxDivFactorFracW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdTxDivFactorFracW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_ed_tx_div_factor(
        &mut self,
    ) -> Pru1EdTxDivFactorW<Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec> {
        Pru1EdTxDivFactorW::new(self, 16)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru1_ed_tx_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru1_ed_tx_cfg_reg to value 0x0700"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru1EdTxCfgRegSpec {
    const RESET_VALUE: u32 = 0x0700;
}
