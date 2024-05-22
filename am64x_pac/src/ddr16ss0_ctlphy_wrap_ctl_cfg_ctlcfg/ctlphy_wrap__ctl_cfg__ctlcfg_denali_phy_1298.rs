#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1298` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1298` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec>;
#[doc = "Field `PHY_SW_TXIO_CTRL_1` reader - 3:0\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_SW_TXIO_CTRL_1` writer - 3:0\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SW_TXIO_CTRL_2` reader - 11:8\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl2R = crate::FieldReader;
#[doc = "Field `PHY_SW_TXIO_CTRL_2` writer - 11:8\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SW_TXIO_CTRL_3` reader - 19:16\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl3R = crate::FieldReader;
#[doc = "Field `PHY_SW_TXIO_CTRL_3` writer - 19:16\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
pub type PhySwTxioCtrl3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_MEMCLK_SW_TXIO_CTRL` reader - 24:24\\]
This register is used to control if clk pads should be shutoff for TX mode."]
pub type PhyMemclkSwTxioCtrlR = crate::BitReader;
#[doc = "Field `PHY_MEMCLK_SW_TXIO_CTRL` writer - 24:24\\]
This register is used to control if clk pads should be shutoff for TX mode."]
pub type PhyMemclkSwTxioCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    pub fn phy_sw_txio_ctrl_1(&self) -> PhySwTxioCtrl1R {
        PhySwTxioCtrl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    pub fn phy_sw_txio_ctrl_2(&self) -> PhySwTxioCtrl2R {
        PhySwTxioCtrl2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    pub fn phy_sw_txio_ctrl_3(&self) -> PhySwTxioCtrl3R {
        PhySwTxioCtrl3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
This register is used to control if clk pads should be shutoff for TX mode."]
    #[inline(always)]
    pub fn phy_memclk_sw_txio_ctrl(&self) -> PhyMemclkSwTxioCtrlR {
        PhyMemclkSwTxioCtrlR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_txio_ctrl_1(
        &mut self,
    ) -> PhySwTxioCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec> {
        PhySwTxioCtrl1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_txio_ctrl_2(
        &mut self,
    ) -> PhySwTxioCtrl2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec> {
        PhySwTxioCtrl2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This register is used to control if command pad \\[CS/RAS...\\]
should be shutoff for TX mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_txio_ctrl_3(
        &mut self,
    ) -> PhySwTxioCtrl3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec> {
        PhySwTxioCtrl3W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
This register is used to control if clk pads should be shutoff for TX mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_memclk_sw_txio_ctrl(
        &mut self,
    ) -> PhyMemclkSwTxioCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec> {
        PhyMemclkSwTxioCtrlW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1298\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1298::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1298::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1298::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1298::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1298 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1298Spec {
    const RESET_VALUE: u32 = 0;
}
