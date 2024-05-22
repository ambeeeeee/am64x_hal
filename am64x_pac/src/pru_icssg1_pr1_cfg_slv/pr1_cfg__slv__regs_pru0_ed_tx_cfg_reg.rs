#[doc = "Register `PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec>;
#[doc = "Field `PRU0_ED_TX_CLK_SEL` reader - "]
pub type Pru0EdTxClkSelR = crate::BitReader;
#[doc = "Field `PRU0_ED_TX_CLK_SEL` writer - "]
pub type Pru0EdTxClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_BUSY_0` reader - "]
pub type Pru0EdBusy0R = crate::BitReader;
#[doc = "Field `PRU0_ED_BUSY_0` writer - "]
pub type Pru0EdBusy0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_BUSY_1` reader - "]
pub type Pru0EdBusy1R = crate::BitReader;
#[doc = "Field `PRU0_ED_BUSY_1` writer - "]
pub type Pru0EdBusy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_BUSY_2` reader - "]
pub type Pru0EdBusy2R = crate::BitReader;
#[doc = "Field `PRU0_ED_BUSY_2` writer - "]
pub type Pru0EdBusy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ENDAT0_CLK_SYNC` reader - "]
pub type Pru0Endat0ClkSyncR = crate::BitReader;
#[doc = "Field `PRU0_ENDAT0_CLK_SYNC` writer - "]
pub type Pru0Endat0ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ENDAT1_CLK_SYNC` reader - "]
pub type Pru0Endat1ClkSyncR = crate::BitReader;
#[doc = "Field `PRU0_ENDAT1_CLK_SYNC` writer - "]
pub type Pru0Endat1ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ENDAT2_CLK_SYNC` reader - "]
pub type Pru0Endat2ClkSyncR = crate::BitReader;
#[doc = "Field `PRU0_ENDAT2_CLK_SYNC` writer - "]
pub type Pru0Endat2ClkSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ENDAT_SHARE_EN` reader - "]
pub type Pru0EndatShareEnR = crate::BitReader;
#[doc = "Field `PRU0_ENDAT_SHARE_EN` writer - "]
pub type Pru0EndatShareEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_TX_DIV_FACTOR_FRAC` reader - "]
pub type Pru0EdTxDivFactorFracR = crate::BitReader;
#[doc = "Field `PRU0_ED_TX_DIV_FACTOR_FRAC` writer - "]
pub type Pru0EdTxDivFactorFracW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_ED_TX_DIV_FACTOR` reader - "]
pub type Pru0EdTxDivFactorR = crate::FieldReader<u16>;
#[doc = "Field `PRU0_ED_TX_DIV_FACTOR` writer - "]
pub type Pru0EdTxDivFactorW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pru0_ed_tx_clk_sel(&self) -> Pru0EdTxClkSelR {
        Pru0EdTxClkSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pru0_ed_busy_0(&self) -> Pru0EdBusy0R {
        Pru0EdBusy0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pru0_ed_busy_1(&self) -> Pru0EdBusy1R {
        Pru0EdBusy1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pru0_ed_busy_2(&self) -> Pru0EdBusy2R {
        Pru0EdBusy2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pru0_endat0_clk_sync(&self) -> Pru0Endat0ClkSyncR {
        Pru0Endat0ClkSyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pru0_endat1_clk_sync(&self) -> Pru0Endat1ClkSyncR {
        Pru0Endat1ClkSyncR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pru0_endat2_clk_sync(&self) -> Pru0Endat2ClkSyncR {
        Pru0Endat2ClkSyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pru0_endat_share_en(&self) -> Pru0EndatShareEnR {
        Pru0EndatShareEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pru0_ed_tx_div_factor_frac(&self) -> Pru0EdTxDivFactorFracR {
        Pru0EdTxDivFactorFracR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn pru0_ed_tx_div_factor(&self) -> Pru0EdTxDivFactorR {
        Pru0EdTxDivFactorR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_clk_sel(&mut self) -> Pru0EdTxClkSelW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdTxClkSelW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_busy_0(&mut self) -> Pru0EdBusy0W<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdBusy0W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_busy_1(&mut self) -> Pru0EdBusy1W<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdBusy1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_busy_2(&mut self) -> Pru0EdBusy2W<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdBusy2W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_endat0_clk_sync(
        &mut self,
    ) -> Pru0Endat0ClkSyncW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0Endat0ClkSyncW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_endat1_clk_sync(
        &mut self,
    ) -> Pru0Endat1ClkSyncW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0Endat1ClkSyncW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_endat2_clk_sync(
        &mut self,
    ) -> Pru0Endat2ClkSyncW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0Endat2ClkSyncW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_endat_share_en(&mut self) -> Pru0EndatShareEnW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EndatShareEnW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_div_factor_frac(
        &mut self,
    ) -> Pru0EdTxDivFactorFracW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdTxDivFactorFracW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_ed_tx_div_factor(
        &mut self,
    ) -> Pru0EdTxDivFactorW<Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec> {
        Pru0EdTxDivFactorW::new(self, 16)
    }
}
#[doc = "PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_pru0_ed_tx_cfg_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_pru0_ed_tx_cfg_reg to value 0x0700"]
impl crate::Resettable for Pr1Cfg_Slv_RegsPru0EdTxCfgRegSpec {
    const RESET_VALUE: u32 = 0x0700;
}
