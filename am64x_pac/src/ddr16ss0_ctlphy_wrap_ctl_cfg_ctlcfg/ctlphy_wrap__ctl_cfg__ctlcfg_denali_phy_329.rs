#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_329` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_329` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec>;
#[doc = "Field `PHY_SLV_DLY_CTRL_GATE_DISABLE_1` reader - 0:0\\]
Data slice slv_dly_control block power reduction disable for slice 1."]
pub type PhySlvDlyCtrlGateDisable1R = crate::BitReader;
#[doc = "Field `PHY_SLV_DLY_CTRL_GATE_DISABLE_1` writer - 0:0\\]
Data slice slv_dly_control block power reduction disable for slice 1."]
pub type PhySlvDlyCtrlGateDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RDPATH_GATE_DISABLE_1` reader - 8:8\\]
Data slice read path power reduction disable for slice 1."]
pub type PhyRdpathGateDisable1R = crate::BitReader;
#[doc = "Field `PHY_RDPATH_GATE_DISABLE_1` writer - 8:8\\]
Data slice read path power reduction disable for slice 1."]
pub type PhyRdpathGateDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DCC_RXCAL_CTRL_GATE_DISABLE_1` reader - 16:16\\]
Data slice RX_CAL block power reduction disable for slice 1."]
pub type PhyDccRxcalCtrlGateDisable1R = crate::BitReader;
#[doc = "Field `PHY_DCC_RXCAL_CTRL_GATE_DISABLE_1` writer - 16:16\\]
Data slice RX_CAL block power reduction disable for slice 1."]
pub type PhyDccRxcalCtrlGateDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_1` reader - 24:24\\]
Data slice power reduction disable for slice 1."]
pub type PhySlicePwrRdcDisable1R = crate::BitReader;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_1` writer - 24:24\\]
Data slice power reduction disable for slice 1."]
pub type PhySlicePwrRdcDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data slice slv_dly_control block power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_slv_dly_ctrl_gate_disable_1(&self) -> PhySlvDlyCtrlGateDisable1R {
        PhySlvDlyCtrlGateDisable1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data slice read path power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_rdpath_gate_disable_1(&self) -> PhyRdpathGateDisable1R {
        PhyRdpathGateDisable1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Data slice RX_CAL block power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_dcc_rxcal_ctrl_gate_disable_1(&self) -> PhyDccRxcalCtrlGateDisable1R {
        PhyDccRxcalCtrlGateDisable1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Data slice power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_slice_pwr_rdc_disable_1(&self) -> PhySlicePwrRdcDisable1R {
        PhySlicePwrRdcDisable1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data slice slv_dly_control block power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slv_dly_ctrl_gate_disable_1(
        &mut self,
    ) -> PhySlvDlyCtrlGateDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec> {
        PhySlvDlyCtrlGateDisable1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data slice read path power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdpath_gate_disable_1(
        &mut self,
    ) -> PhyRdpathGateDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec> {
        PhyRdpathGateDisable1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Data slice RX_CAL block power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dcc_rxcal_ctrl_gate_disable_1(
        &mut self,
    ) -> PhyDccRxcalCtrlGateDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec> {
        PhyDccRxcalCtrlGateDisable1W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Data slice power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slice_pwr_rdc_disable_1(
        &mut self,
    ) -> PhySlicePwrRdcDisable1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec> {
        PhySlicePwrRdcDisable1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_329\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_329::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_329::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_329::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_329::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_329 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy329Spec {
    const RESET_VALUE: u32 = 0;
}
