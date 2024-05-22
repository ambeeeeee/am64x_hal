#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_343` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_343` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_1` reader - 9:0\\]
Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
pub type PhyRddmSlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDM_SLAVE_DELAY_1` writer - 9:0\\]
Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
pub type PhyRddmSlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RX_PCLK_CLK_SEL_1` reader - 18:16\\]
RX_PCLK clock frequency selection for slice 1."]
pub type PhyRxPclkClkSel1R = crate::FieldReader;
#[doc = "Field `PHY_RX_PCLK_CLK_SEL_1` writer - 18:16\\]
RX_PCLK clock frequency selection for slice 1."]
pub type PhyRxPclkClkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_RX_CAL_ALL_DLY_1` reader - 28:24\\]
Defines the number of cycles/half cycles that the rx_cal_all_opad signal should be asserted for. There is a phy_rx_cal_all_dly_X parameter for each of the slices of data sent on the DFI data bus for slice 1."]
pub type PhyRxCalAllDly1R = crate::FieldReader;
#[doc = "Field `PHY_RX_CAL_ALL_DLY_1` writer - 28:24\\]
Defines the number of cycles/half cycles that the rx_cal_all_opad signal should be asserted for. There is a phy_rx_cal_all_dly_X parameter for each of the slices of data sent on the DFI data bus for slice 1."]
pub type PhyRxCalAllDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
    #[inline(always)]
    pub fn phy_rddm_slave_delay_1(&self) -> PhyRddmSlaveDelay1R {
        PhyRddmSlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RX_PCLK clock frequency selection for slice 1."]
    #[inline(always)]
    pub fn phy_rx_pclk_clk_sel_1(&self) -> PhyRxPclkClkSel1R {
        PhyRxPclkClkSel1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the number of cycles/half cycles that the rx_cal_all_opad signal should be asserted for. There is a phy_rx_cal_all_dly_X parameter for each of the slices of data sent on the DFI data bus for slice 1."]
    #[inline(always)]
    pub fn phy_rx_cal_all_dly_1(&self) -> PhyRxCalAllDly1R {
        PhyRxCalAllDly1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Read DM/DBI slave delay setting for slice 1. May be used for data swap."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddm_slave_delay_1(
        &mut self,
    ) -> PhyRddmSlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec> {
        PhyRddmSlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
RX_PCLK clock frequency selection for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_pclk_clk_sel_1(
        &mut self,
    ) -> PhyRxPclkClkSel1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec> {
        PhyRxPclkClkSel1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the number of cycles/half cycles that the rx_cal_all_opad signal should be asserted for. There is a phy_rx_cal_all_dly_X parameter for each of the slices of data sent on the DFI data bus for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_all_dly_1(
        &mut self,
    ) -> PhyRxCalAllDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec> {
        PhyRxCalAllDly1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_343\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_343::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_343::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_343::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_343::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_343 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy343Spec {
    const RESET_VALUE: u32 = 0;
}
