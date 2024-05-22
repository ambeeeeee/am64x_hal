#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1299` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1299` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec>;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_0` reader - 3:0\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl0R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_0` writer - 3:0\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_1` reader - 11:8\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl1R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_1` writer - 11:8\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_2` reader - 19:16\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl2R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_2` writer - 19:16\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_3` reader - 27:24\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl3R = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_SW_TXPWR_CTRL_3` writer - 27:24\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
pub type PhyAdrctlSwTxpwrCtrl3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    pub fn phy_adrctl_sw_txpwr_ctrl_0(&self) -> PhyAdrctlSwTxpwrCtrl0R {
        PhyAdrctlSwTxpwrCtrl0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    pub fn phy_adrctl_sw_txpwr_ctrl_1(&self) -> PhyAdrctlSwTxpwrCtrl1R {
        PhyAdrctlSwTxpwrCtrl1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    pub fn phy_adrctl_sw_txpwr_ctrl_2(&self) -> PhyAdrctlSwTxpwrCtrl2R {
        PhyAdrctlSwTxpwrCtrl2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    pub fn phy_adrctl_sw_txpwr_ctrl_3(&self) -> PhyAdrctlSwTxpwrCtrl3R {
        PhyAdrctlSwTxpwrCtrl3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_sw_txpwr_ctrl_0(
        &mut self,
    ) -> PhyAdrctlSwTxpwrCtrl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec> {
        PhyAdrctlSwTxpwrCtrl0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_sw_txpwr_ctrl_1(
        &mut self,
    ) -> PhyAdrctlSwTxpwrCtrl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec> {
        PhyAdrctlSwTxpwrCtrl1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_sw_txpwr_ctrl_2(
        &mut self,
    ) -> PhyAdrctlSwTxpwrCtrl2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec> {
        PhyAdrctlSwTxpwrCtrl2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
This register is used to control if address/command pad \\[address/CS/RAS...\\]
should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_sw_txpwr_ctrl_3(
        &mut self,
    ) -> PhyAdrctlSwTxpwrCtrl3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec> {
        PhyAdrctlSwTxpwrCtrl3W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1299\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1299::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1299::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1299::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1299::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1299 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1299Spec {
    const RESET_VALUE: u32 = 0;
}
