#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_0` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_0` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec>;
#[doc = "Field `PHY_LP4_BOOT_RX_PCLK_CLK_SEL_0` reader - 2:0\\]
RX_PCLK boot clock frequency selection for slice 0."]
pub type PhyLp4BootRxPclkClkSel0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RX_PCLK_CLK_SEL_0` writer - 2:0\\]
RX_PCLK boot clock frequency selection for slice 0."]
pub type PhyLp4BootRxPclkClkSel0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_LP4_BOOT_PAD_DSLICE_IO_CFG_0` reader - 14:8\\]
Controls PCLK/PARK pin for pad for slice 0 with boot frequency."]
pub type PhyLp4BootPadDsliceIoCfg0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_PAD_DSLICE_IO_CFG_0` writer - 14:8\\]
Controls PCLK/PARK pin for pad for slice 0 with boot frequency."]
pub type PhyLp4BootPadDsliceIoCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_0` reader - 26:16\\]
Write data clock bypass mode slave delay setting for slice 0."]
pub type PhyClkWrBypassSlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CLK_WR_BYPASS_SLAVE_DELAY_0` writer - 26:16\\]
Write data clock bypass mode slave delay setting for slice 0."]
pub type PhyClkWrBypassSlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RX_PCLK boot clock frequency selection for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rx_pclk_clk_sel_0(&self) -> PhyLp4BootRxPclkClkSel0R {
        PhyLp4BootRxPclkClkSel0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Controls PCLK/PARK pin for pad for slice 0 with boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_pad_dslice_io_cfg_0(&self) -> PhyLp4BootPadDsliceIoCfg0R {
        PhyLp4BootPadDsliceIoCfg0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_clk_wr_bypass_slave_delay_0(&self) -> PhyClkWrBypassSlaveDelay0R {
        PhyClkWrBypassSlaveDelay0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RX_PCLK boot clock frequency selection for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rx_pclk_clk_sel_0(
        &mut self,
    ) -> PhyLp4BootRxPclkClkSel0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec> {
        PhyLp4BootRxPclkClkSel0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Controls PCLK/PARK pin for pad for slice 0 with boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pad_dslice_io_cfg_0(
        &mut self,
    ) -> PhyLp4BootPadDsliceIoCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec> {
        PhyLp4BootPadDsliceIoCfg0W::new(self, 8)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Write data clock bypass mode slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_clk_wr_bypass_slave_delay_0(
        &mut self,
    ) -> PhyClkWrBypassSlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec> {
        PhyClkWrBypassSlaveDelay0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_0::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_0::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_0 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy0Spec {
    const RESET_VALUE: u32 = 0;
}
