#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_82` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_82` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec>;
#[doc = "Field `PHY_PAD_DQS_RX_DCD_0` reader - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 0."]
pub type PhyPadDqsRxDcd0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DQS_RX_DCD_0` writer - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 0."]
pub type PhyPadDqsRxDcd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_FDBK_RX_DCD_0` reader - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 0."]
pub type PhyPadFdbkRxDcd0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_FDBK_RX_DCD_0` writer - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 0."]
pub type PhyPadFdbkRxDcd0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_DSLICE_IO_CFG_0` reader - 22:16\\]
Controls PCLK/PARK pin for pad for slice 0."]
pub type PhyPadDsliceIoCfg0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DSLICE_IO_CFG_0` writer - 22:16\\]
Controls PCLK/PARK pin for pad for slice 0."]
pub type PhyPadDsliceIoCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_dqs_rx_dcd_0(&self) -> PhyPadDqsRxDcd0R {
        PhyPadDqsRxDcd0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_fdbk_rx_dcd_0(&self) -> PhyPadFdbkRxDcd0R {
        PhyPadFdbkRxDcd0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Controls PCLK/PARK pin for pad for slice 0."]
    #[inline(always)]
    pub fn phy_pad_dslice_io_cfg_0(&self) -> PhyPadDsliceIoCfg0R {
        PhyPadDsliceIoCfg0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dqs_rx_dcd_0(
        &mut self,
    ) -> PhyPadDqsRxDcd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec> {
        PhyPadDqsRxDcd0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_fdbk_rx_dcd_0(
        &mut self,
    ) -> PhyPadFdbkRxDcd0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec> {
        PhyPadFdbkRxDcd0W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Controls PCLK/PARK pin for pad for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dslice_io_cfg_0(
        &mut self,
    ) -> PhyPadDsliceIoCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec> {
        PhyPadDsliceIoCfg0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_82\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_82::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_82::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_82::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_82::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_82 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy82Spec {
    const RESET_VALUE: u32 = 0;
}
