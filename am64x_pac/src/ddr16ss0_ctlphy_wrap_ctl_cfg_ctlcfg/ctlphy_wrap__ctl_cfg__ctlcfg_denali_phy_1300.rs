#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1300` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1300` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec>;
#[doc = "Field `PHY_MEMCLK_SW_TXPWR_CTRL` reader - 0:0\\]
This register is used to control if clk pads should be shutoff for TX mode in deep sleep mode."]
pub type PhyMemclkSwTxpwrCtrlR = crate::BitReader;
#[doc = "Field `PHY_MEMCLK_SW_TXPWR_CTRL` writer - 0:0\\]
This register is used to control if clk pads should be shutoff for TX mode in deep sleep mode."]
pub type PhyMemclkSwTxpwrCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TOP_STATIC_TOG_DISABLE` reader - 8:8\\]
Disables the generation of the toggle for static clock based paths in the PHY to prevent assymetric aging."]
pub type PhyTopStaticTogDisableR = crate::BitReader;
#[doc = "Field `PHY_TOP_STATIC_TOG_DISABLE` writer - 8:8\\]
Disables the generation of the toggle for static clock based paths in the PHY to prevent assymetric aging."]
pub type PhyTopStaticTogDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_BYTE_DISABLE_STATIC_TOG_DISABLE` reader - 16:16\\]
Control to disable the toggle signal for data slice during static activity when dfi_data_byte_disable is asserted."]
pub type PhyByteDisableStaticTogDisableR = crate::BitReader;
#[doc = "Field `PHY_BYTE_DISABLE_STATIC_TOG_DISABLE` writer - 16:16\\]
Control to disable the toggle signal for data slice during static activity when dfi_data_byte_disable is asserted."]
pub type PhyByteDisableStaticTogDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register is used to control if clk pads should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    pub fn phy_memclk_sw_txpwr_ctrl(&self) -> PhyMemclkSwTxpwrCtrlR {
        PhyMemclkSwTxpwrCtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disables the generation of the toggle for static clock based paths in the PHY to prevent assymetric aging."]
    #[inline(always)]
    pub fn phy_top_static_tog_disable(&self) -> PhyTopStaticTogDisableR {
        PhyTopStaticTogDisableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Control to disable the toggle signal for data slice during static activity when dfi_data_byte_disable is asserted."]
    #[inline(always)]
    pub fn phy_byte_disable_static_tog_disable(&self) -> PhyByteDisableStaticTogDisableR {
        PhyByteDisableStaticTogDisableR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register is used to control if clk pads should be shutoff for TX mode in deep sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn phy_memclk_sw_txpwr_ctrl(
        &mut self,
    ) -> PhyMemclkSwTxpwrCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec> {
        PhyMemclkSwTxpwrCtrlW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disables the generation of the toggle for static clock based paths in the PHY to prevent assymetric aging."]
    #[inline(always)]
    #[must_use]
    pub fn phy_top_static_tog_disable(
        &mut self,
    ) -> PhyTopStaticTogDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec> {
        PhyTopStaticTogDisableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Control to disable the toggle signal for data slice during static activity when dfi_data_byte_disable is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn phy_byte_disable_static_tog_disable(
        &mut self,
    ) -> PhyByteDisableStaticTogDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec> {
        PhyByteDisableStaticTogDisableW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1300\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1300::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1300::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1300::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1300::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1300 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1300Spec {
    const RESET_VALUE: u32 = 0;
}
