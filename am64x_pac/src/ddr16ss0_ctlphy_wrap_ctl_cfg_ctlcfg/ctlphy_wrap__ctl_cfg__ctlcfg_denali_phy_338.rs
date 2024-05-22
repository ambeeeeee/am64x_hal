#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_338` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_338` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec>;
#[doc = "Field `PHY_PAD_DQS_RX_DCD_1` reader - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 1."]
pub type PhyPadDqsRxDcd1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DQS_RX_DCD_1` writer - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 1."]
pub type PhyPadDqsRxDcd1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_FDBK_RX_DCD_1` reader - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 1."]
pub type PhyPadFdbkRxDcd1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_FDBK_RX_DCD_1` writer - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 1."]
pub type PhyPadFdbkRxDcd1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_PAD_DSLICE_IO_CFG_1` reader - 22:16\\]
Controls PCLK/PARK pin for pad for slice 1."]
pub type PhyPadDsliceIoCfg1R = crate::FieldReader;
#[doc = "Field `PHY_PAD_DSLICE_IO_CFG_1` writer - 22:16\\]
Controls PCLK/PARK pin for pad for slice 1."]
pub type PhyPadDsliceIoCfg1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_dqs_rx_dcd_1(&self) -> PhyPadDqsRxDcd1R {
        PhyPadDqsRxDcd1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_fdbk_rx_dcd_1(&self) -> PhyPadFdbkRxDcd1R {
        PhyPadFdbkRxDcd1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Controls PCLK/PARK pin for pad for slice 1."]
    #[inline(always)]
    pub fn phy_pad_dslice_io_cfg_1(&self) -> PhyPadDsliceIoCfg1R {
        PhyPadDsliceIoCfg1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Controls RX_DCD pin for dqs pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dqs_rx_dcd_1(
        &mut self,
    ) -> PhyPadDqsRxDcd1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec> {
        PhyPadDqsRxDcd1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Controls RX_DCD pin for fdbk pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_fdbk_rx_dcd_1(
        &mut self,
    ) -> PhyPadFdbkRxDcd1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec> {
        PhyPadFdbkRxDcd1W::new(self, 8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Controls PCLK/PARK pin for pad for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_dslice_io_cfg_1(
        &mut self,
    ) -> PhyPadDsliceIoCfg1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec> {
        PhyPadDsliceIoCfg1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_338\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_338::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_338::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_338::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_338::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_338 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy338Spec {
    const RESET_VALUE: u32 = 0;
}
