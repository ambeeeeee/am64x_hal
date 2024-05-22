#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_256` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_256` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec>;
#[doc = "Field `PHY_LP4_BOOT_RX_PCLK_CLK_SEL_1` reader - 2:0\\]
RX_PCLK boot clock frequency selection for slice 1."]
pub type PhyLp4BootRxPclkClkSel1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RX_PCLK_CLK_SEL_1` writer - 2:0\\]
RX_PCLK boot clock frequency selection for slice 1."]
pub type PhyLp4BootRxPclkClkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_LP4_BOOT_PAD_DSLICE_IO_CFG_1` reader - 14:8\\]
Controls PCLK/PARK pin for pad for slice 1 with boot frequency."]
pub type PhyLp4BootPadDsliceIoCfg1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_PAD_DSLICE_IO_CFG_1` writer - 14:8\\]
Controls PCLK/PARK pin for pad for slice 1 with boot frequency."]
pub type PhyLp4BootPadDsliceIoCfg1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_1` reader - 26:16\\]
Write data clock bypass mode slave delay setting for slice 1."]
pub type PhyClkWrBypassSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_1` writer - 26:16\\]
Write data clock bypass mode slave delay setting for slice 1."]
pub type PhyClkWrBypassSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RX_PCLK boot clock frequency selection for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rx_pclk_clk_sel_1(&self) -> PhyLp4BootRxPclkClkSel1R {
        PhyLp4BootRxPclkClkSel1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Controls PCLK/PARK pin for pad for slice 1 with boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_pad_dslice_io_cfg_1(&self) -> PhyLp4BootPadDsliceIoCfg1R {
        PhyLp4BootPadDsliceIoCfg1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_clk_wr_bypass_slave_delay_1(&self) -> PhyClkWrBypassSlaveDelay1R {
        PhyClkWrBypassSlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RX_PCLK boot clock frequency selection for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rx_pclk_clk_sel_1(
        &mut self,
    ) -> PhyLp4BootRxPclkClkSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec> {
        PhyLp4BootRxPclkClkSel1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Controls PCLK/PARK pin for pad for slice 1 with boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pad_dslice_io_cfg_1(
        &mut self,
    ) -> PhyLp4BootPadDsliceIoCfg1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec> {
        PhyLp4BootPadDsliceIoCfg1W::new(self, 8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write data clock bypass mode slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wr_bypass_slave_delay_1(
        &mut self,
    ) -> PhyClkWrBypassSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec> {
        PhyClkWrBypassSlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_256\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_256::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_256::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_256::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_256::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_256 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy256Spec {
    const RESET_VALUE: u32 = 0;
}
