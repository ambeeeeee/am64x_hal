#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1372` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1372` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec>;
#[doc = "Field `PHY_PAD_ACS_IO_CFG` reader - 15:0\\]
Controls PCLK/PARK pin for acs pad."]
pub type PhyPadAcsIoCfgR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_ACS_IO_CFG` writer - 15:0\\]
Controls PCLK/PARK pin for acs pad."]
pub type PhyPadAcsIoCfgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_PAD_ACS_RX_PCLK_CLK_SEL` reader - 18:16\\]
Controls rx_pclk clk selection for acs pad."]
pub type PhyPadAcsRxPclkClkSelR = crate::FieldReader;
#[doc = "Field `PHY_PAD_ACS_RX_PCLK_CLK_SEL` writer - 18:16\\]
Controls rx_pclk clk selection for acs pad."]
pub type PhyPadAcsRxPclkClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Controls PCLK/PARK pin for acs pad."]
    #[inline(always)]
    pub fn phy_pad_acs_io_cfg(&self) -> PhyPadAcsIoCfgR {
        PhyPadAcsIoCfgR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Controls rx_pclk clk selection for acs pad."]
    #[inline(always)]
    pub fn phy_pad_acs_rx_pclk_clk_sel(&self) -> PhyPadAcsRxPclkClkSelR {
        PhyPadAcsRxPclkClkSelR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Controls PCLK/PARK pin for acs pad."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_acs_io_cfg(
        &mut self,
    ) -> PhyPadAcsIoCfgW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec> {
        PhyPadAcsIoCfgW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Controls rx_pclk clk selection for acs pad."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_acs_rx_pclk_clk_sel(
        &mut self,
    ) -> PhyPadAcsRxPclkClkSelW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec> {
        PhyPadAcsRxPclkClkSelW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1372\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1372::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1372::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1372::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1372::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1372 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1372Spec {
    const RESET_VALUE: u32 = 0;
}
