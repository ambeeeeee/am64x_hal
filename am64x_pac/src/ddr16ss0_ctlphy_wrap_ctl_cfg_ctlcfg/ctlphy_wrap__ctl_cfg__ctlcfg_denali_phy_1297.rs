#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1297` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1297` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec>;
#[doc = "Field `PHY_LPDDR3_CS` reader - 0:0\\]
Alters reset state polarity for LPDDR chip selects."]
pub type PhyLpddr3CsR = crate::BitReader;
#[doc = "Field `PHY_LPDDR3_CS` writer - 0:0\\]
Alters reset state polarity for LPDDR chip selects."]
pub type PhyLpddr3CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_UPDATE_CLK_CAL_VALUES` reader - 8:8\\]
Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyUpdateClkCalValuesR = crate::BitReader;
#[doc = "Field `SC_PHY_UPDATE_CLK_CAL_VALUES` writer - 8:8\\]
Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyUpdateClkCalValuesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CONTINUOUS_CLK_CAL_UPDATE` reader - 16:16\\]
Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
pub type PhyContinuousClkCalUpdateR = crate::BitReader;
#[doc = "Field `PHY_CONTINUOUS_CLK_CAL_UPDATE` writer - 16:16\\]
Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
pub type PhyContinuousClkCalUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_SW_TXIO_CTRL_0` reader - 27:24\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl0R = crate::FieldReader;
#[doc = "Field `PHY_SW_TXIO_CTRL_0` writer - 27:24\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Alters reset state polarity for LPDDR chip selects."]
    #[inline(always)]
    pub fn phy_lpddr3_cs(&self) -> PhyLpddr3CsR {
        PhyLpddr3CsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_update_clk_cal_values(&self) -> ScPhyUpdateClkCalValuesR {
        ScPhyUpdateClkCalValuesR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
    #[inline(always)]
    pub fn phy_continuous_clk_cal_update(&self) -> PhyContinuousClkCalUpdateR {
        PhyContinuousClkCalUpdateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    pub fn phy_sw_txio_ctrl_0(&self) -> PhySwTxioCtrl0R {
        PhySwTxioCtrl0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Alters reset state polarity for LPDDR chip selects."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpddr3_cs(&mut self) -> PhyLpddr3CsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec> {
        PhyLpddr3CsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Manual update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_update_clk_cal_values(
        &mut self,
    ) -> ScPhyUpdateClkCalValuesW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec> {
        ScPhyUpdateClkCalValuesW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Continuous update of all latest PVTP,PVTN and PVTR values to the CLK IO pads. Set to 1 to keep this enabled."]
    #[inline(always)]
    #[must_use]
    pub fn phy_continuous_clk_cal_update(
        &mut self,
    ) -> PhyContinuousClkCalUpdateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec> {
        PhyContinuousClkCalUpdateW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_txio_ctrl_0(
        &mut self,
    ) -> PhySwTxioCtrl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec> {
        PhySwTxioCtrl0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1297\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1297::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1297::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1297::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1297::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1297 to value 0x01"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1297Spec {
    const RESET_VALUE: u32 = 0x01;
}
